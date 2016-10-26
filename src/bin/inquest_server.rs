extern crate grpc;
extern crate inquest;

use std::collections::{HashMap, BTreeMap};
use std::hash;
use std::sync::{Arc, RwLock};

use grpc::error::GrpcError;
use grpc::result::GrpcResult;

use inquest::inquest_pb::{CancelProbeRequest, GatherProbesRequest, SearchRequest, ScheduleProbeRequest};
use inquest::inquest_pb::{CancelProbeReply, GatherProbesReply, SearchReply, ScheduleProbeReply};
use inquest::inquest_pb::{Probe, Protocol};
use inquest::inquest_pb_grpc::{ProbeCache, ProbeCacheServer, Scheduler, SchedulerServer};

fn main() {
    let probe_map = Arc::new(RwLock::new(BTreeMap::new()));

    {
        //add buckets to probe_map
        let bucket_count = 10;
        let mut counter = 0;
        let delta = u64::max_value() / bucket_count;
        let mut probe_map = probe_map.write().unwrap();
        for i in 0..bucket_count {
            probe_map.insert(counter, HashMap::new());
            counter += delta;
        }
    }

    let _probe_cache_server = ProbeCacheServer::new(52890, ProbeCacheImpl::new(Arc::new(RwLock::new(HashMap::new()))));
    let _scheduler_server = SchedulerServer::new(12289, SchedulerImpl::new(probe_map.clone()));

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
        /*let probe_ids = request.get_scheduled_probe_id();
        
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

        Ok(inquest::create_gather_probes_reply(probes, cancel_probes))*/
        Err(GrpcError::Other("unimplemented!"))
    }
}

struct SchedulerImpl {
    probe_map: Arc<RwLock<BTreeMap<u64, HashMap<String, HashMap<Protocol, Vec<Probe>>>>>>, //map<domain_hash, map<domain, vec<probe>>>
}

impl SchedulerImpl {
    fn new(probe_map: Arc<RwLock<BTreeMap<u64, HashMap<String, HashMap<Protocol, Vec<Probe>>>>>>) -> SchedulerImpl {
        SchedulerImpl {
            probe_map: probe_map,
        }
    }
}

impl Scheduler for SchedulerImpl {
    fn CancelProbe(&self, request: CancelProbeRequest) -> GrpcResult<CancelProbeReply> {
        let key = inquest::compute_domain_hash(request.get_domain());
        let mut probe_map = self.probe_map.write().unwrap();

        //determine correct bucket key
        let mut bucket_key = 0;
        for map_key in probe_map.keys() {
            if *map_key > key {
                break;
            }

            bucket_key = *map_key;
        }

        let mut domain_map = probe_map.get_mut(&bucket_key).unwrap();
        let mut protocol_map = match domain_map.get_mut(request.get_domain()) {
            Some(protocol_map) => protocol_map,
            None => return Err(GrpcError::Other("domain doesn't exist")),
        };

        //loop over protocols
        for protocol in request.get_protocol() {
            let mut remove_protocol = false;
            {
                let mut probes = match protocol_map.get_mut(protocol) {
                    Some(probes) => probes,
                    None => continue,
                };

                match protocol {
                    &Protocol::HTTP => {
                        if request.has_url_suffix() {
                            let mut index = -1;
                            for (i, p) in probes.iter().enumerate() {
                                if p.get_url_suffix() == request.get_url_suffix() {
                                    index = i as i16;
                                    break;
                                }
                            }

                            if index != -1 {
                                probes.remove(index as usize);
                            }
                        } else {
                            probes.clear();
                        }
                    },
                    _ => probes.clear(),
                }

                //check if there are no probes left for the protocol
                if probes.len() == 0 {
                    remove_protocol = true;
                }
            }

            //remove protocol if there are no probes scheduled
            if remove_protocol {
                protocol_map.remove(protocol);
            }
        }

        Ok(inquest::create_cancel_probe_reply())
    }

    fn Search(&self, request: SearchRequest) -> GrpcResult<SearchReply> {
        let key = inquest::compute_domain_hash(request.get_domain());
        let mut probe_map = self.probe_map.write().unwrap();

        //determine correct bucket key
        let mut bucket_key = 0;
        for map_key in probe_map.keys() {
            if *map_key > key {
                break;
            }

            bucket_key = *map_key;
        }

        //find map containing protocols pertaining to the given domain
        let mut domain_map = probe_map.get_mut(&bucket_key).unwrap();
        let mut protocol_map = match domain_map.get_mut(request.get_domain()) {
            Some(protocol_map) => protocol_map,
            None => return Err(GrpcError::Other("domain does not exist")),
        };

        //loop over protocol arguments and return respective probes
        let mut search_probes: Vec<Probe> = Vec::new();
        for protocol in request.get_protocol() {
            match protocol_map.get(protocol) {
                Some(probes) => {
                    for p in probes {
                        search_probes.push(p.clone());
                    }
                },
                None => continue,
            }
        }

        Ok(inquest::create_search_reply(search_probes))
    }

    fn ScheduleProbe(&self, request: ScheduleProbeRequest) -> GrpcResult<ScheduleProbeReply> {
        for probe in request.get_probe() {
            let key = inquest::compute_domain_hash(probe.get_domain());
            let probe_id = inquest::compute_probe_hash(probe);

            let mut probe_map = self.probe_map.write().unwrap();

            //determine correct bucket key
            let mut bucket_key = 0;
            for map_key in probe_map.keys() {
                if *map_key > key {
                    break;
                }

                bucket_key = *map_key;
            }

            //see if probe already exists
            let mut domain_map = probe_map.get_mut(&bucket_key).unwrap();
            let mut protocol_map = domain_map.entry(probe.get_domain().to_owned()).or_insert(HashMap::new());
            let mut probes = protocol_map.entry(probe.get_protocol()).or_insert(Vec::new());
            
            let mut found = false;
            for p in probes.iter() {
                if p.get_probe_id() == probe_id {
                    found = true;
                    break;
                }
            }

            //add probe if needed
            if !found {
                let mut probe = probe.clone();
                probe.set_probe_id(probe_id);
                probes.push(probe);
            }
        }

        Ok(inquest::create_schedule_probe_reply())
    }

    /*fn CancelProbe(&self, request: CancelProbeRequest) -> GrpcResult<CancelProbeReply> {
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
    }*/
}
