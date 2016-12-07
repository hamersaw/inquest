#[macro_use]
extern crate chan;
extern crate chrono;
extern crate docopt;
extern crate inquest;
extern crate rustc_serialize;
extern crate threadpool;

use std::cmp::{Ordering, PartialOrd};
use std::collections::{BinaryHeap, BTreeMap, HashMap};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, RwLock};

use chrono::offset::utc::UTC;
use docopt::Docopt;
use inquest::pb::proddle::{Probe, ProbeResult};
use inquest::pb::proddle_grpc::{ProbeCache, ProbeCacheClient};
use threadpool::ThreadPool;

const USAGE: &'static str = "
Inquest Prober

Usage:
    inquest_prober (-h | --help)
    inquest_prober <hostname> [--server-host=<shost>] [--server-port=<sport>] [--thread-count=<thread-count>] [--probe-poll-seconds=<probe-poll-seconds>] [--results-buffer-size=<results-buffer-size>]

Options:
    -h --help                                       Display this screen.
    --server-host=<server-host>                     Host of server to connect to [default: 127.0.0.1].
    --server-port=<server-port>                     Port of server to connect to [default: 52890].
    --thread-count=<thread-count>                   Number of threads to use for probing pool [default: 16].
    --probe-poll-seconds=<probe-poll-seconds>       Interval at which prober polls configuration server for probe changes [default: 1200]. 
    --results-buffer-size=<results-buffer-size>     Size of results buffer for returning results to configuration server [default: 100].
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_hostname: String,
    flag_server_host: String,
    flag_server_port: u16,
    flag_thread_count: usize,
    flag_probe_poll_seconds: u32,
    flag_results_buffer_size: usize,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                        .and_then(|d| d.decode())
                        .unwrap_or_else(|e| e.exit());

    //initialize prober variables
    let client = ProbeCacheClient::new(&args.flag_server_host, args.flag_server_port, false).unwrap();
    let probe_jobs: Arc<RwLock<BTreeMap<u64, BinaryHeap<ProbeJob>>>> = Arc::new(RwLock::new(BTreeMap::new())); //map<domain_hash, binary_heap<probe>>
    let probe_results: Arc<RwLock<Vec<ProbeResult>>> = Arc::new(RwLock::new(Vec::new()));

    //get bucket keys
    let request = inquest::create_get_bucket_keys_request();
    let reply = client.GetBucketKeys(request).unwrap();

    {
        let mut probe_jobs = probe_jobs.write().unwrap();
        for bucket_key in reply.get_bucket_key() {
            probe_jobs.insert(*bucket_key, BinaryHeap::new());
        }
    }

    //add initial probes to bucket
    let _ = get_probes(&client, probe_jobs.clone());
    
    //start execution threadpool loop
    let thread_probe_jobs = probe_jobs.clone();
    let thread_probe_results  = probe_results.clone();
    let thread_hostname = args.arg_hostname;
    let thread_thread_count = args.flag_thread_count;
    std::thread::spawn(move || {
        let pool = ThreadPool::new(thread_thread_count);
        let tick = chan::tick_ms(1000);

        loop {
            chan_select! {
                tick.recv() => {
                    let now = UTC::now().timestamp();

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
                                let pool_probe_results = thread_probe_results.clone();
                                let pool_probe_job = probe_job.clone();
                                let pool_prober_hostname = thread_hostname.to_owned();

                                //add probe job back to binary heap with increased execution time
                                let _ = probe_job.inc_execution_time();
                                probe_jobs.push(probe_job);

                                //execute probe
                                pool.execute(move || {
                                    match inquest::execute_probe(&pool_probe_job.probe) {
                                        Ok(mut probe_result) => {
                                            probe_result.set_prober_hostname(pool_prober_hostname);

                                            let mut probe_results = pool_probe_results.write().unwrap();
                                            probe_results.push(probe_result);
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
    let get_probes_tick = chan::tick_ms(args.flag_probe_poll_seconds * 1000);
    let check_results_tick = chan::tick_ms(5 * 1000);
    loop {
        chan_select! {
            get_probes_tick.recv() => {
                let _ = get_probes(&client, probe_jobs.clone());
            },
            check_results_tick.recv() => {
                let mut probe_results = probe_results.write().unwrap();
                if probe_results.len() >= args.flag_results_buffer_size {
                    let request = inquest::create_send_probe_results_request(&probe_results);
                    let _ = client.SendProbeResults(request).unwrap();
                    probe_results.clear();
                }
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
            let mut hasher = DefaultHasher::new();

            //add all probe ids to binary heap
            let mut probe_ids = BinaryHeap::new();
            for probe_job in probe_jobs_heap {
                probe_ids.push(probe_job.probe.get_probe_id());
            }

            //loop over probe ids in order
            while let Some(probe_id) = probe_ids.pop() {
                probe_id.hash(&mut hasher);
            }

            bucket_hashes.insert(*bucket_key, hasher.finish());
        }
    }

    let request = inquest::create_get_probes_request(bucket_hashes);
    let reply = client.GetProbes(request).unwrap();

    //loop through bucket probes and add
    for bucket_probes in reply.get_bucket_probes() {
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
    next_execution_time: i64,
    probe: Probe,
}

impl ProbeJob {
    fn new(probe: Probe) -> ProbeJob {
        let now = UTC::now().timestamp();

        ProbeJob {
            next_execution_time: now - (now % probe.get_probe_interval_seconds() as i64) + probe.get_probe_interval_seconds() as i64,
            probe: probe,
        }
    }

    fn inc_execution_time(&mut self) -> Result<(), &str> {
        self.next_execution_time = self.next_execution_time + (self.probe.get_probe_interval_seconds() as i64);
        Ok(())
    }

    fn get_next_execution_time(&self) -> i64 {
        self.next_execution_time
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
