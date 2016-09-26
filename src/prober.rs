use std;
use std::cmp::{Ordering, PartialOrd};
use std::collections::BinaryHeap;
use std::ops::Add;
use std::sync::{Arc, Mutex, RwLock};

use inquest_pb::Probe;
use writer::Writer;

use chan;
use threadpool::ThreadPool;
use time;
use time::{Duration, Tm};

pub trait Prober {
    fn schedule_probe(&self, probe: &Probe) -> Result<(), String>;
    fn cancel_probe(&self, probe_id: &str) -> Result<(), &str>;
    fn get_probe_ids(&self) -> Vec<String>;
}

pub struct ThreadPoolProberImpl {
    probe_jobs: Arc<RwLock<BinaryHeap<ProbeJob>>>,
}

impl ThreadPoolProberImpl {
    pub fn new(writer: Box<Writer + Send>, probe_threads: usize) -> ThreadPoolProberImpl {
        let probe_jobs = Arc::new(RwLock::new(BinaryHeap::new()));

        //start thread to periodically check probe jobs to execute
        let thread_probe_jobs = probe_jobs.clone();
        let thread_writer = Arc::new(Mutex::new(writer));
        std::thread::spawn(move || {
            let pool = ThreadPool::new(probe_threads);
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
                                let pool_writer = thread_writer.clone();
                                pool.execute(move || {
                                    match super::execute_probe(&pool_probe_job.probe) {
                                        Ok(probe_result) => {
                                            let mut writer = pool_writer.lock().unwrap();
                                            let _ = writer.write_probe_result(&probe_result);
                                        },
                                        Err(e) => println!("ERROR: {}", e),
                                    }
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
}

impl Prober for ThreadPoolProberImpl {
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
