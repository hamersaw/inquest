extern crate grpc;
extern crate inquest;

use std::collections::{HashMap, BTreeMap};
use std::hash::{Hash, Hasher, SipHasher};
use std::sync::{Arc, RwLock};

use grpc::error::GrpcError;
use grpc::result::GrpcResult;

use inquest::inquest_pb::{CancelProbeRequest, GetBucketKeysRequest, GetProbesRequest, SearchRequest, ScheduleProbeRequest};
use inquest::inquest_pb::{CancelProbeReply, GetBucketKeysReply, GetProbesReply, SearchReply, ScheduleProbeReply};
use inquest::inquest_pb::{Probe, Protocol};
use inquest::inquest_pb_grpc::{ProbeCache, ProbeCacheServer, Scheduler, SchedulerServer};

fn main() {
    let probe_map = Arc::new(RwLock::new(BTreeMap::new()));

    {
        //add buckets to probe_map
        let bucket_count = 1000;
        let mut counter = 0;
        let delta = u64::max_value() / bucket_count;
        let mut probe_map = probe_map.write().unwrap();
        for _ in 0..bucket_count {
            probe_map.insert(counter, HashMap::new());
            counter += delta;
        }
    }

    let _probe_cache_server = ProbeCacheServer::new(52890, ProbeCacheImpl::new(probe_map.clone()));
    let _scheduler_server = SchedulerServer::new(12289, SchedulerImpl::new(probe_map.clone()));

    loop {
        std::thread::park();
    }
}

struct ProbeCacheImpl {
    probe_map: Arc<RwLock<BTreeMap<u64, HashMap<String, HashMap<Protocol, Vec<Probe>>>>>>, //map<domain_hash, map<domain, vec<probe>>>
}

impl ProbeCacheImpl {
    fn new(probe_map: Arc<RwLock<BTreeMap<u64, HashMap<String, HashMap<Protocol, Vec<Probe>>>>>>) -> ProbeCacheImpl {
        ProbeCacheImpl {
            probe_map: probe_map,
        }
    }
}

impl ProbeCache for ProbeCacheImpl {
    fn GetBucketKeys(&self, _: GetBucketKeysRequest) -> GrpcResult<GetBucketKeysReply> {
        let bucket_keys;
        {
            let probe_map = self.probe_map.read().unwrap();
            bucket_keys = probe_map.keys().cloned().collect();
        }

        Ok(inquest::create_get_bucket_keys_reply(bucket_keys))
    }

    fn GetProbes(&self, request: GetProbesRequest) -> GrpcResult<GetProbesReply> {
        //compute local bucket hashes
        let mut bucket_hashes = HashMap::new();
        {
            let probe_map = self.probe_map.read().unwrap();
            for (bucket_key, domain_map) in probe_map.iter() {
                let mut hasher = SipHasher::new();
                for protocol_map in domain_map.values() {
                    for probes in protocol_map.values() {
                        for probe in probes {
                            probe.get_probe_id().hash(&mut hasher);
                        }
                    }
                }

                bucket_hashes.insert(*bucket_key, hasher.finish());
            }
        }

        //compare hashes
        let mut bucket_probes = HashMap::new();
        for bucket_hash in request.get_bucket_hash() {
            match bucket_hashes.get(&bucket_hash.get_bucket_key()) {
                Some(local_bucket_hash) => {
                    //check if bucket hashes differ
                    if bucket_hash.get_hash() != *local_bucket_hash {
                        println!("GET LOCAL PROBES FOR BUCKET KEY:{}", bucket_hash.get_bucket_key());
                        let reply_probes = bucket_probes.entry(*local_bucket_hash).or_insert(Vec::new());

                        //add all local probes
                        let probe_map = self.probe_map.read().unwrap();
                        for domain_map in probe_map.get(&bucket_hash.get_bucket_key()) {
                            for protocol_map in domain_map.values() {
                                for probes in protocol_map.values() {
                                    for probe in probes {
                                        reply_probes.push(probe.clone());
                                    }
                                }
                            }
                        }
                    }
                },
                None => continue,
            }
        }

        Ok(inquest::create_get_probes_reply(bucket_probes))
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
        let remove_domain;
        {
            let mut protocol_map = match domain_map.get_mut(request.get_domain()) {
                Some(protocol_map) => protocol_map,
                None => return Err(GrpcError::Other("domain doesn't exist")),
            };

            //loop over protocols
            for protocol in request.get_protocol() {
                let remove_protocol;
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

                    remove_protocol = probes.len() == 0;
                }

                //remove protocol if there are no probes scheduled
                if remove_protocol {
                    protocol_map.remove(protocol);
                }
            }

            remove_domain = protocol_map.len() == 0;
        }

        //remove domain if there are no probes scheduled
        if remove_domain {
            domain_map.remove(request.get_domain());
        }

        Ok(inquest::create_cancel_probe_reply())
    }

    fn Search(&self, request: SearchRequest) -> GrpcResult<SearchReply> {
        let key = inquest::compute_domain_hash(request.get_domain());
        let probe_map = self.probe_map.read().unwrap();

        //determine correct bucket key
        let mut bucket_key = 0;
        for map_key in probe_map.keys() {
            if *map_key > key {
                break;
            }

            bucket_key = *map_key;
        }

        //find map containing protocols pertaining to the given domain
        let domain_map = probe_map.get(&bucket_key).unwrap();
        let protocol_map = match domain_map.get(request.get_domain()) {
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
}
