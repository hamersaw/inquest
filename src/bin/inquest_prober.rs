#[macro_use]
extern crate chan;
extern crate inquest;
extern crate hyper;
extern crate threadpool;
extern crate time;
extern crate toml;

use std::cmp::{Ordering, PartialOrd};
use std::collections::{BinaryHeap, BTreeMap, HashMap};
use std::fs::File;
use std::hash::{Hash, Hasher, SipHasher};
use std::io::Read;
use std::ops::Add;
use std::sync::{Arc, Mutex, RwLock};

use inquest::inquest_pb::{Probe};
use inquest::inquest_pb_grpc::{ProbeCache, ProbeCacheClient};
use inquest::writer::{FileWriter, PrintWriter, Writer};
use threadpool::ThreadPool;
use time::{Duration, Tm};
use toml::Parser;
use toml::Value::Table;

fn main() {
    //read arguments
    let mut args = std::env::args();
    if args.len() != 2 {
        panic!("Usage: {} <configuration-filename>", args.nth(0).unwrap());
    }

    //read toml configuration file
    let mut input = String::new();
    let filename = args.nth(1).unwrap();
    File::open(&filename).and_then(|mut f| {
        f.read_to_string(&mut input)
    }).unwrap();

    //parse into toml table
    let mut parser = Parser::new(&input);
    let toml = match parser.parse() {
        Some(toml) => toml,
        None => {
            for err in &parser.errors {
                println!("unable to parse configuration server:{} {:?} - {:?} : '{}'", filename, parser.to_linecol(err.lo), parser.to_linecol(err.hi), err.desc);
            }
            return
        }
    };

    //parse toml values
    let toml_table = Table(toml);
    let prober_hostname = toml_table.lookup("prober_hostname")
                        .expect("unable to find field 'prober_hostname'")
                        .as_str().expect("unable to parse prober_hostname into &str").to_owned();
    let probe_threads = toml_table.lookup("probe_threads")
                        .expect("unable to find field 'probe_threads'")
                        .as_integer().expect("unable to parse probe_threads into integer") as usize;
    let writer_str = toml_table.lookup("writer.type")
                        .expect("unable to find field 'writer.type'")
                        .as_str().expect("unable to parse writer.type into &str");
    let host = toml_table.lookup("server.host")
                        .expect("unable to find field 'server.host'")
                        .as_str().expect("unable to parse configuration_server.host into &str");
    let port = toml_table.lookup("server.port")
                        .expect("unable to find field 'server.port'")
                        .as_integer().expect("unable to parse configuration_server.port into integer") as u16;
    let probe_poll_seconds = toml_table.lookup("server.probe_poll_seconds")
                        .expect("unable to find field 'server.probe_poll_seconds'")
                        .as_integer().expect("unable to parse server.probe_poll_seconds into integer") as u32;

    //create prober
    let writer = match writer_str {
        "PrintWriter" => Arc::new(Mutex::new(Box::new(PrintWriter::new()) as Box<Writer + Send>)),
        "FileWriter" => {
            let directory = toml_table.lookup("writer.directory")
                                .expect("unable to find field 'writer.directory'")
                                .as_str().expect("unable to parse writer.directory into &str");

            let max_filesize = toml_table.lookup("writer.max_filesize")
                                .expect("unable to find field 'writer.max_filesize'")
                                .as_integer().expect("unable to parse writer.max_filesize into integer") as u32;

            Arc::new(Mutex::new(Box::new(FileWriter::new(directory, max_filesize)) as Box<Writer + Send>))
        }
        _ => panic!("unknown writer type '{}'", writer_str),
    };

    //initialize prober variables
    let client = ProbeCacheClient::new(host, port, false).unwrap();
    let probe_jobs: Arc<RwLock<BTreeMap<u64, BinaryHeap<ProbeJob>>>> = Arc::new(RwLock::new(BTreeMap::new())); //map<domain_hash, binary_heap<probe>>

    //get bucket keys
    let request = inquest::create_get_bucket_keys_request();
    let response = client.GetBucketKeys(request).unwrap();

    {
        let mut probe_jobs = probe_jobs.write().unwrap();
        for bucket_key in response.get_bucket_key() {
            probe_jobs.insert(*bucket_key, BinaryHeap::new());
        }
    }

    //add initial probes to bucket
    let _ = get_probes(&client, probe_jobs.clone());
    
    //start execution threadpool loop
    let thread_probe_jobs = probe_jobs.clone();
    std::thread::spawn(move || {
        let pool = ThreadPool::new(probe_threads);
        let tick = chan::tick_ms(1000);

        loop {
            chan_select! {
                tick.recv() => {
                    let now = time::now_utc();

                    //loop through probes while they should be executed
                    let mut probe_jobs: std::sync::RwLockWriteGuard<BTreeMap<u64, BinaryHeap<ProbeJob>>> = thread_probe_jobs.write().unwrap();
                    for (_, probe_jobs) in probe_jobs.iter_mut() {
                        loop {
                            let next_execution_time = match probe_jobs.peek() {
                                Some(probe_job) => probe_job.get_next_execution_time().to_owned(),
                                None => break,
                            };

                            if next_execution_time.le(&now) {
                                let mut probe_job = probe_jobs.pop().unwrap();

                                //initialize threadpool variables
                                let pool_probe_job = probe_job.clone();
                                let pool_writer = writer.clone();
                                let pool_prober_hostname = prober_hostname.to_owned();

                                //add probe job back to binary heap with increased execution time
                                let _ = probe_job.inc_execution_time();
                                probe_jobs.push(probe_job);

                                //execute probe
                                pool.execute(move || {
                                    match inquest::execute_probe(&pool_probe_job.probe) {
                                        Ok(mut probe_result) => {
                                            probe_result.set_prober_hostname(pool_prober_hostname);
                                            let mut writer = pool_writer.lock().unwrap();
                                            let _ = writer.write_probe_result(&probe_result);
                                        },
                                        Err(e) => println!("ERROR: {}", e),
                                    }
                                });
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        }
    });

    //start probe schedule poll loop
    let tick = chan::tick_ms(probe_poll_seconds * 1000);
    loop {
        chan_select! {
            tick.recv() => {
                let _ = get_probes(&client, probe_jobs.clone());
            },
        }
    }
}

//fn get_probes(client: &ProbeCacheClient, probe_map: Arc<RwLock<BTreeMap<u64, HashMap<String, Vec<Probe>>>>>) -> Result<(), String> {
fn get_probes(client: &ProbeCacheClient, probe_jobs: Arc<RwLock<BTreeMap<u64, BinaryHeap<ProbeJob>>>>) -> Result<(), String> {
    let mut bucket_hashes = HashMap::new();
    {
        let probe_jobs = probe_jobs.read().unwrap();
        for (bucket_key, probe_jobs_heap) in probe_jobs.iter() {
            let mut hasher = SipHasher::new();
            for probe_job in probe_jobs_heap {
                probe_job.probe.get_probe_id().hash(&mut hasher);
            }

            bucket_hashes.insert(*bucket_key, hasher.finish());
        }
    }

    let request = inquest::create_get_probes_request(bucket_hashes);
    let response = client.GetProbes(request).unwrap();

    //loop through bucket probes and add
    for bucket_probes in response.get_bucket_probes() {
        let mut probe_jobs_heap = BinaryHeap::new();
        for probe in bucket_probes.get_probe() {
            probe_jobs_heap.push(ProbeJob::new(probe.to_owned()));
        }

        {
            let mut probe_jobs = probe_jobs.write().unwrap();
            probe_jobs.insert(bucket_probes.get_bucket_key(), probe_jobs_heap);
        }
    }

    Ok(())
}

/*
 * ProbeJob definition
 */
#[derive(Clone)]
struct ProbeJob {
    next_execution_time: Tm,
    probe: Probe,
}

impl ProbeJob {
    fn new(probe: Probe) -> ProbeJob {
        ProbeJob {
            next_execution_time: time::now_utc(),
            probe: probe,
        }
    }

    fn inc_execution_time(&mut self) -> Result<(), &str> {
        let duration = Duration::seconds(self.probe.get_probe_interval_seconds() as i64);
        self.next_execution_time = self.next_execution_time.add(duration);
        Ok(())
    }

    fn get_next_execution_time(&self) -> &Tm {
        &self.next_execution_time
    }
}

impl PartialEq for ProbeJob {
    fn eq(&self, other: &ProbeJob) -> bool {
        self.next_execution_time == other.next_execution_time
    }
}

impl Eq for ProbeJob {}

impl Ord for ProbeJob {
    fn cmp(&self, other: &ProbeJob) -> Ordering {
        other.next_execution_time.cmp(&self.next_execution_time)
    }
}

impl PartialOrd for ProbeJob {
    fn partial_cmp(&self, other: &ProbeJob) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
