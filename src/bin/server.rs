extern crate grpc;
extern crate inquest;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use grpc::error::GrpcError;
use grpc::result::GrpcResult;

use inquest::inquest_pb::{CancelProbeRequest, DescribeProbeRequest, GatherProbesRequest, ListProbeIdsRequest, ScheduleProbeRequest};
use inquest::inquest_pb::{CancelProbeReply, DescribeProbeReply, GatherProbesReply, ListProbeIdsReply, ScheduleProbeReply};
use inquest::inquest_pb::Probe;
use inquest::inquest_pb_grpc::{ProbeCache, ProbeCacheServer, Scheduler, SchedulerServer};

fn main() {
    let probe_map = Arc::new(RwLock::new(HashMap::new()));
    let _probe_cache_server = ProbeCacheServer::new(52890, ProbeCacheImpl::new(probe_map.clone()));
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
        let probe_priority = request.get_probe_priority(); //if request has no priority 0 is returned
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

                    probe.get_probe_priority() >= probe_priority
                }).collect();

        let cancel_probes = probe_ids.iter()
                .filter(|probe_id| !probe_map.contains_key(probe_id.to_owned()))
                .collect();

        Ok(inquest::create_gather_probes_reply(probes, cancel_probes))
    }
}

struct SchedulerImpl {
    probe_map: Arc<RwLock<HashMap<String, Probe>>>,
}

impl SchedulerImpl {
    fn new(probe_map: Arc<RwLock<HashMap<String, Probe>>>) -> SchedulerImpl {
        SchedulerImpl {
            //probe_map: Arc::new(RwLock::new(HashMap::new())),
            probe_map: probe_map,
        }
    }
}

impl Scheduler for SchedulerImpl {
    fn CancelProbe(&self, request: CancelProbeRequest) -> GrpcResult<CancelProbeReply> {
        //check for a probe id
        if !request.has_probe_id() {
            return Err(GrpcError::Other("request field probe_id is required"));
        }

        //remove probe if exists
        let mut probe_map = self.probe_map.write().unwrap();
        if probe_map.remove(request.get_probe_id()).is_none() {
            return Err(GrpcError::Other("probe does not exist"));
        }

        Ok(inquest::create_cancel_probe_reply())
    }

    fn DescribeProbe(&self, request: DescribeProbeRequest) -> GrpcResult<DescribeProbeReply> {
        //check for a probe id
        if !request.has_probe_id() {
            return Err(GrpcError::Other("request field probe_id is required"));
        }

        //retrieve probe if exists
        let probe_map = self.probe_map.read().unwrap();
        let probe = match probe_map.get(request.get_probe_id()) {
            Some(probe) => probe,
            None => return Err(GrpcError::Other("probe does not exist")),
        };

        Ok(inquest::create_describe_probe_reply(probe))
    }

    fn ListProbeIds(&self, request: ListProbeIdsRequest) -> GrpcResult<ListProbeIdsReply> {
        //get probe priority
        let probe_priority = request.get_probe_priority(); //if request has no priority 0 is returned

        //get all the probe ids where probe has priority over what is provided
        let probe_map = self.probe_map.read().unwrap();
        let probe_ids = probe_map.iter()
                .filter(|entry| entry.1.get_probe_priority() >= probe_priority)
                .map(|entry| entry.0.to_owned())
                .collect::<Vec<_>>();

        Ok(inquest::create_list_probe_ids_reply(probe_ids))
    }

    fn ScheduleProbe(&self, request: ScheduleProbeRequest) -> GrpcResult<ScheduleProbeReply> {
        //check for field 'probe'
        if !request.has_probe() {
            return Err(GrpcError::Other("request field probe is required"));
        }

        let probe = request.get_probe();

        //check for field 'probe_id'
        if !probe.has_probe_id() {
            return Err(GrpcError::Other("request field probe_id is required"));
        }

        let probe_id = probe.get_probe_id();

        //check if probe already exists
        let mut probe_map = self.probe_map.write().unwrap();
        if probe_map.contains_key(probe_id) {
            return Err(GrpcError::Other("probe id already exists"));
        }

        //add probe to map
        probe_map.insert(probe_id.to_owned(), probe.to_owned());

        Ok(inquest::create_schedule_probe_reply())
    }
}
