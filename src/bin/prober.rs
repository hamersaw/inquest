#[macro_use]
extern crate chan;
extern crate inquest;
extern crate hyper;
extern crate threadpool;
extern crate time;
extern crate toml;

use std::cmp::{Ordering, PartialOrd};
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::Read;
use std::ops::{Add, Sub};
use std::sync::{Arc, RwLock};

use hyper::Client;
use inquest::inquest_pb::{Probe, Probe_Protocol, ProbeResult};
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

        //add probe to probe jobs
        probe_jobs.push(ProbeJob::new(probe.to_owned()));
        Ok(())
    }

    fn cancel_probe(&self, probe_id: &str) -> Result<(), &str> {
        //loop through probe jobs removing the probe_id
        let mut probe_jobs = self.probe_jobs.write().unwrap();
        let mut _probe_jobs = BinaryHeap::new();
        for probe_job in probe_jobs.drain() {
            if probe_job.probe.get_probe_id() != probe_id {
                _probe_jobs.push(probe_job);
            }
        }

        //retain correct probe jobs
        probe_jobs.append(&mut _probe_jobs);
        Ok(())
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
    fn new(probe: Probe) -> ProbeJob {
        ProbeJob {
            next_execution_time: time::now_utc(),
            probe: probe,
        }
    }

    fn execute(&self) -> Result<(), &str> {
        let mut probe_result = ProbeResult::new();
        probe_result.set_probe_id(self.probe.get_probe_id().to_owned());

        //format the url
        let url = format!("{}://{}/{}",
                match self.probe.get_protocol() {
                    Probe_Protocol::HTTP => "http",
                    Probe_Protocol::HTTPS => "https",
                },
                self.probe.get_host(),
                self.probe.get_url_suffix()
            );

        //submit request
        let client = Client::new();
        let start_time = time::now_utc();
        let response = client.get(&url).send();

        //calculate execution time
        let execution_time = time::now_utc().sub(start_time);
        probe_result.set_application_layer_latency_ns(execution_time.num_nanoseconds().unwrap());

        //parse response
        match response {
            Ok(response) => {
                {
                    //populate http status codes and message
                    let status_raw = response.status_raw();
                    probe_result.set_http_status_code(status_raw.0 as i32);
                    //probe_result.set_http_status_msg(status_raw.1.into_owned());
                }

                {
                    //populate application byte counts
                    let byte_count = response.bytes().count();
                    probe_result.set_application_bytes_received(byte_count as i32);
                }
            },
            Err(e) => {
                println!("failed request");
            },
        }

        println!("probe_result: {:?}", probe_result);
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
