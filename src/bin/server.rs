extern crate grpc;
extern crate inquest;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use grpc::result::GrpcResult;

use inquest::inquest_pb::{DescribeProbeRequest, DescribeProbeReply};
use inquest::inquest_pb::{ListProbeIdsRequest, ListProbeIdsReply};
use inquest::inquest_pb::{Probe};
use inquest::inquest_pb::{ScheduleProbeRequest, ScheduleProbeReply};
use inquest::inquest_pb_grpc::{Inquest, InquestServer};

fn main() {
    let _server = InquestServer::new(12289, InquestImpl::new());

    loop {
        std::thread::park();
    }
}

struct InquestImpl {
    probe_map: Arc<RwLock<HashMap<String, Probe>>>,
}

impl InquestImpl {
    fn new() -> InquestImpl {
        InquestImpl {
            probe_map: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Inquest for InquestImpl {
    fn DescribeProbe(&self, request: DescribeProbeRequest) -> GrpcResult<DescribeProbeReply> {
        if !request.has_probe_id() {
            return Ok(inquest::describe_probe_reply_failure("request field probe_id is required"));
        }

        //check if probe exists
        let probe_map = self.probe_map.read().unwrap();
        let probe = match probe_map.get(request.get_probe_id()) {
            Some(probe) => probe,
            None => return Ok(inquest::describe_probe_reply_failure("probe_id does not exist")),
        };

        Ok(inquest::describe_probe_reply_success(probe))
    }

    fn ListProbeIds(&self, request: ListProbeIdsRequest) -> GrpcResult<ListProbeIdsReply> {
        //rget probe priority
        let probe_priority = request.get_probe_priority(); //if request has no priority 0 is returned

        //get all the probe ids where the probe has priority over what is given
        let probe_map = self.probe_map.read().unwrap();
        let probe_ids = probe_map.iter()
                .filter(|entry| {entry.1.get_probe_priority() >= probe_priority})
                .map(|entry| {entry.0.to_owned()})
                .collect::<Vec<_>>();

        Ok(inquest::list_probe_ids_reply_success(probe_ids))
    }

    fn ScheduleProbe(&self, request: ScheduleProbeRequest) -> GrpcResult<ScheduleProbeReply> {
        //check for field 'probe'
        if !request.has_probe() {
            return Ok(inquest::schedule_probe_reply_failure("request field probe is required"));
        }

        let probe = request.get_probe();

        //check for field 'probe_id'
        if !probe.has_probe_id() {
            return Ok(inquest::schedule_probe_reply_failure("request field probe_id is required"));
        }

        let probe_id = probe.get_probe_id();

        //check if probe already exists
        let mut probe_map = self.probe_map.write().unwrap();
        if probe_map.contains_key(probe_id) {
            return Ok(inquest::schedule_probe_reply_failure("probe id already exists"));
        }

        //add probe to map
        probe_map.insert(probe_id.to_owned(), probe.to_owned());

        Ok(inquest::schedule_probe_reply_success())
    }
}
