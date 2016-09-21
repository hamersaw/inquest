#[macro_use]
extern crate chan;
extern crate inquest;
extern crate threadpool;
extern crate time;
extern crate toml;

use std::cmp::{Ordering, PartialOrd};
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::Read;
use std::ops::Add;
use std::sync::{Arc, Mutex, RwLock};

use chan::Sender;
use inquest::inquest_pb::Probe;
use inquest::inquest_pb_grpc::{ProbeCache, ProbeCacheClient};
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
    let host = toml_table.lookup("server.host")
                        .expect("unable to find field 'server.host'")
                        .as_str().expect("unable to parse configuration_server.host into &str");
    let port = toml_table.lookup("server.port")
                        .expect("unable to find field 'server.port'")
                        .as_integer().expect("unable to parse configuration_server.port into integer") as u16;
    let probe_priority = toml_table.lookup("probe.probe_priority")
                        .expect("unable to find field 'probe.probe_priority'")
                        .as_integer().expect("unable to parse probe.probe_priority into integer") as i32;

    //create prober
    //let prober = ThreadedProberImpl::new();
    let prober = ThreadPoolProberImpl::new();

    //open client and start scheduling probes
    let client = ProbeCacheClient::new(host, port).unwrap();

    let tick = chan::tick_ms(5000);
    loop {
        chan_select! {
            tick.recv() => {
                let request = inquest::create_gather_probes_request(Some(probe_priority), prober.get_probe_ids());
                let response = client.GatherProbes(request).unwrap();

                //cancel probes
                for probe_id in response.get_cancel_probe_id() {
                    let _ = prober.cancel_probe(probe_id);
                }

                //schedule new probes
                for probe in response.get_probe() {
                    let _ = prober.schedule_probe(probe);
                }
            }
        }
    }
}

/*struct ThreadedProberImpl {
    probe_map: Arc<RwLock<HashMap<String, Sender<()>>>>,
}

impl ThreadedProberImpl {
    fn new() -> ThreadedProberImpl {
        ThreadedProberImpl {
            probe_map: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn schedule_probe(&self, probe: &Probe) {
        let probe = probe.to_owned();

        //create quit channel
        let (quit_send, quit_receive) = chan::sync(0);
        
        //insert quit channel into probe map
        let mut probe_map = self.probe_map.write().unwrap();
        probe_map.insert(probe.get_probe_id().to_owned(), quit_send);

        //spawn probing thread
        std::thread::spawn(move || {
            let tick = chan::tick_ms(5000);
            loop {
                chan_select! {
                    tick.recv() => {
                        println!("TODO actively probe: {:?}", probe);
                    },
                    quit_receive.recv() => {
                        return;
                    }
                }
            }
        });
    }

    fn cancel_probe(&self, probe_id: &str) {
        let mut probe_map = self.probe_map.write().unwrap();
        match probe_map.remove(probe_id) {
            Some(quit_send) => drop(quit_send),
            None => {}//TODO return error that the object doesn't exist,
        }
    }
    
    fn get_probe_ids(&self) -> Vec<String> {
        let probe_map = self.probe_map.read().unwrap();
        probe_map.keys().map(|key| key.to_owned()).collect()
    }
}*/



struct ThreadPoolProberImpl {
    probe_jobs: Arc<RwLock<BinaryHeap<ProbeJob>>>,
}

impl ThreadPoolProberImpl {
    fn new() -> ThreadPoolProberImpl {
        let probe_jobs = Arc::new(RwLock::new(BinaryHeap::new()));

        //start thread to periodically check probe jobs to execute
        let thread_probe_jobs = probe_jobs.clone();
        std::thread::spawn(move || {
            let pool = ThreadPool::new(4);
            let tick = chan::tick_ms(250);

            loop {
                chan_select! {
                    tick.recv() => {
                        let now = time::now_utc();

                        //loop through probes while they should be executed
                        let mut probe_jobs: std::sync::RwLockWriteGuard<BinaryHeap<ProbeJob>> = thread_probe_jobs.write().unwrap();
                        loop {
                            let next_execution_time = match probe_jobs.peek() {
                                Some(probe_job) => probe_job.get_next_execution_time().to_owned(),
                                None => break,
                            };

                            if next_execution_time.le(&now) {
                                let mut probe_job = probe_jobs.pop().unwrap();

                                //submit probe execution to thread pool
                                let pool_probe_job = probe_job.clone();
                                pool.execute(move || {
                                    let _ = pool_probe_job.execute();
                                });

                                //add probe job back to binary heap with increased execution time
                                let _ = probe_job.inc_execution_time();
                                probe_jobs.push(probe_job);
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        });

        ThreadPoolProberImpl {
            probe_jobs: probe_jobs,
        }
    }

    fn schedule_probe(&self, probe: &Probe) -> Result<(), String> {
        //check if probe_id already exists
        let mut probe_jobs = self.probe_jobs.write().unwrap();
        for probe_job in probe_jobs.iter() {
            if probe_job.probe.get_probe_id() == probe.get_probe_id() {
                return Err(format!("probe_id '{}' already exists", probe.get_probe_id()));
            }
        }

        //schedule probe
        probe_jobs.push(ProbeJob::new(time::now_utc(), probe.to_owned()));
        Ok(())
    }

    fn cancel_probe(&self, probe_id: &str) -> Result<(), &str> {
        let mut probe_jobs = self.probe_jobs.write().unwrap();
        //TODO remove probe_id from probe jobs
        unimplemented!();
    }

    fn get_probe_ids(&self) -> Vec<String> {
        let probe_jobs = self.probe_jobs.read().unwrap();

        probe_jobs.iter()
                .map(|probe_job| probe_job.probe.get_probe_id().to_owned())
                .collect()
    }
}

#[derive(Clone)]
struct ProbeJob {
    next_execution_time: Tm,
    probe: Probe,
}

impl ProbeJob {
    fn new(next_execution_time: Tm, probe: Probe) -> ProbeJob {
        ProbeJob {
            next_execution_time: next_execution_time,
            probe: probe,
        }
    }

    fn execute(&self) -> Result<(), &str> {
        println!("TODO execute probe");
        //TODO execute probe
        Ok(())
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
