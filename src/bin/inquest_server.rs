extern crate grpc;
extern crate inquest;
extern crate uuid;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use grpc::error::GrpcError;
use grpc::result::GrpcResult;

use inquest::inquest_pb::{CancelProbeRequest, GatherProbesRequest, SearchRequest, ScheduleProbeRequest};
use inquest::inquest_pb::{CancelProbeReply, GatherProbesReply, SearchReply, ScheduleProbeReply};
use inquest::inquest_pb::{Probe, Protocol};
use inquest::inquest_pb_grpc::{ProbeCache, ProbeCacheServer, Scheduler, SchedulerServer};
use uuid::Uuid;

fn main() {
    let probe_map = Arc::new(RwLock::new(HashMap::new()));
    let probe_index = Arc::new(RwLock::new(HashMap::new()));
    let _probe_cache_server = ProbeCacheServer::new(52890, ProbeCacheImpl::new(Arc::new(RwLock::new(HashMap::new()))));
    let _scheduler_server = SchedulerServer::new(12289, SchedulerImpl::new(probe_map.clone(), probe_index.clone()));

    loop {
        std::thread::park();
    }
}

struct ProbeCacheImpl {
    probe_map: Arc<RwLock<HashMap<String, Probe>>>,
}

impl ProbeCacheImpl {
    fn new(probe_map: Arc<RwLock<HashMap<String, Probe>>>) -> ProbeCacheImpl {
        ProbeCacheImpl {
            probe_map: probe_map,
        }
    }
}

impl ProbeCache for ProbeCacheImpl {
    fn GatherProbes(&self, request: GatherProbesRequest) -> GrpcResult<GatherProbesReply> {
        let probe_ids = request.get_scheduled_probe_id();
        
        //get all the probes where probe has priority over what is provided
        let probe_map = self.probe_map.read().unwrap();
        let probes = probe_map.values()
                .filter(|probe| {
                    for probe_id in probe_ids {
                        if probe.get_probe_id() == probe_id {
                            return false
                        }
                    }

                    true
                }).collect();

        let cancel_probes = probe_ids.iter()
                .filter(|probe_id| !probe_map.contains_key(probe_id.to_owned()))
                .collect();

        Ok(inquest::create_gather_probes_reply(probes, cancel_probes))
    }
}

struct SchedulerImpl {
    //probe_map: Arc<RwLock<HashMap<String, Probe>>>,
    probe_map: Arc<RwLock<HashMap<Protocol, HashMap<String, Vec<Probe>>>>>, //map<protocol, map<domain, vec<probe>>>
    probe_index: Arc<RwLock<HashMap<String, (Protocol, String)>>>, //map<probe_id, (protocol, domain)>
}

impl SchedulerImpl {
    //fn new(probe_map: Arc<RwLock<HashMap<String, Probe>>>) -> SchedulerImpl {
    fn new(probe_map: Arc<RwLock<HashMap<Protocol, HashMap<String, Vec<Probe>>>>>, probe_index: Arc<RwLock<HashMap<String, (Protocol, String)>>>) -> SchedulerImpl {
        SchedulerImpl {
            probe_map: probe_map,
            probe_index: probe_index,
        }
    }
}

impl Scheduler for SchedulerImpl {
    fn CancelProbe(&self, request: CancelProbeRequest) -> GrpcResult<CancelProbeReply> {
        //check for a probe id
        if !request.has_probe_id() {
            return Err(GrpcError::Other("request field probe_id is required"));
        }

        //check for probe in probe index
        let mut probe_index = self.probe_index.write().unwrap();
        let (protocol, domain) = match probe_index.remove(request.get_probe_id()) {
            Some(x) => x,
            None => return Err(GrpcError::Other("probe does not exist")),
        };

        //remove probe if exists
        let mut probe_map = self.probe_map.write().unwrap();
        let mut map = probe_map.get_mut(&protocol).unwrap();
        let mut probes = map.get_mut(&domain).unwrap();

        let mut index = 0;
        for (i, p) in probes.iter().enumerate() {
            if p.get_probe_id() == request.get_probe_id() {
                index = i;
                break;
            }
        }

        probes.remove(index);

        Ok(inquest::create_cancel_probe_reply())
    }

    fn Search(&self, request: SearchRequest) -> GrpcResult<SearchReply> {
        let mut probe_vec = Vec::new();
        for protocol in request.get_protocol() {
            let probe_map = self.probe_map.read().unwrap();
            let map = match probe_map.get(protocol) {
                Some(map) => map,
                None => continue,
            };

            let probes = match map.get(request.get_domain()) {
                Some(probes) => probes,
                None => continue,
            };

            for p in probes {
                probe_vec.push(p.clone());
            }
        }

        Ok(inquest::create_search_reply(probe_vec))
    }

    fn ScheduleProbe(&self, request: ScheduleProbeRequest) -> GrpcResult<ScheduleProbeReply> {
        for probe in request.get_probe() {
            //add to probe map
            let mut probe_map = self.probe_map.write().unwrap();
            let mut map = probe_map.entry(probe.get_protocol()).or_insert(HashMap::new());
            let mut probes = map.entry(probe.get_domain().to_owned()).or_insert(Vec::new());

            //check if probe already exists
            for p in probes.iter() {
                let equality = match probe.get_protocol() {
                    Protocol::DNS => true, //change once we have DNS for multiple record types
                    Protocol::HTTP => probe.get_url_suffix() == p.get_url_suffix(),
                    _ => true,
                };

                if equality {
                    continue;
                }
            }

            //generate probe id
            let probe_id = Uuid::new_v4().hyphenated().to_string();
            let mut probe = probe.clone();
            probe.set_probe_id(probe_id.clone());

            //add probe to structures
            let mut probe_index = self.probe_index.write().unwrap();
            probe_index.insert(probe_id, (probe.get_protocol(), probe.get_domain().to_owned()));
            probes.push(probe);
        }

        Ok(inquest::create_schedule_probe_reply())
    }
}
