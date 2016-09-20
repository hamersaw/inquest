extern crate inquest;
extern crate grpc;

use std::collections::HashMap;

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
    probe_map: HashMap<String, ScheduleProbeRequest>,            
}

impl InquestImpl {
    fn new() -> InquestImpl {
        InquestImpl {
            probe_map: HashMap::new(),
        }
    }
}

impl Inquest for InquestImpl {
    fn DescribeProbe(&self, request: DescribeProbeRequest) -> GrpcResult<DescribeProbeReply> {
        Ok(inquest::describe_probe_reply_failure("unimplemented"))
    }

    fn ListProbeIds(&self, request: ListProbeIdsRequest) -> GrpcResult<ListProbeIdsReply> {
        Ok(inquest::list_probe_ids_reply_failure("unimplemented"))
    }

    fn ScheduleProbe(&self, request: ScheduleProbeRequest) -> GrpcResult<ScheduleProbeReply> {
        //check for field 'probe'
        if !request.has_probe() {
            return Ok(inquest::schedule_probe_reply_failure("request field probe is empty"));
        }

        let probe = request.get_probe();

        //check for field 'probe_id'
        if !probe.has_probe_id() {
            return Ok(inquest::schedule_probe_reply_failure("request field probe_id is empty"));
        }

        let probe_id = probe.get_probe_id();

        //check if probe already exists
        if self.probe_map.contains_key(probe_id) {
            return Ok(inquest::schedule_probe_reply_failure("probe id already exists"));
        }

        //TODO add probe to map
        //self.probe_map.insert(probe_id.to_owned(), request);

        Ok(inquest::schedule_probe_reply_success())
    }
}
