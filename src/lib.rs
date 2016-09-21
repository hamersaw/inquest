extern crate protobuf;
extern crate grpc;
extern crate futures;
extern crate futures_cpupool;

pub mod inquest_pb;
pub mod inquest_pb_grpc;

use protobuf::RepeatedField;

use inquest_pb::{CancelProbeRequest, DescribeProbeRequest, GatherProbesRequest, ListProbeIdsRequest, ScheduleProbeRequest};
use inquest_pb::{CancelProbeReply, DescribeProbeReply, GatherProbesReply, ListProbeIdsReply, ScheduleProbeReply};
use inquest_pb::Probe;

pub fn create_cancel_probe_request(probe_id: &str) -> CancelProbeRequest {
    let mut request = CancelProbeRequest::new();
    request.set_probe_id(probe_id.to_owned());
    request
}

pub fn create_cancel_probe_reply() -> CancelProbeReply {
    CancelProbeReply::new()
}

pub fn create_describe_probe_request(probe_id: &str) -> DescribeProbeRequest {
    let mut request = DescribeProbeRequest::new();
    request.set_probe_id(probe_id.to_owned());
    request
}

pub fn create_describe_probe_reply(probe: &Probe) -> DescribeProbeReply {
    let mut reply = DescribeProbeReply::new();
    reply.set_probe(probe.clone());
    reply
}

pub fn create_gather_probes_request(probe_priority: Option<i32>, probe_ids: Vec<String>) -> GatherProbesRequest {
    let mut repeated_probe_id: RepeatedField<String> = RepeatedField::new();
    for probe_id in probe_ids {
        repeated_probe_id.push(probe_id);
    }

    let mut request = GatherProbesRequest::new();
    request.set_scheduled_probe_id(repeated_probe_id);
    if probe_priority.is_some() {
        request.set_probe_priority(probe_priority.unwrap());
    }


    request
}

pub fn create_gather_probes_reply(probes: Vec<&Probe>, probe_ids: Vec<&String>) -> GatherProbesReply {
    let mut repeated_probe: RepeatedField<Probe> = RepeatedField::new();
    for probe in probes {
        repeated_probe.push(probe.clone());
    }

    let mut repeated_probe_id: RepeatedField<String> = RepeatedField::new();
    for probe_id in probe_ids {
        repeated_probe_id.push(probe_id.to_owned());
    }

    let mut reply = GatherProbesReply::new();
    reply.set_probe(repeated_probe);
    reply.set_cancel_probe_id(repeated_probe_id);
    reply
}

pub fn create_list_probe_ids_request(probe_priority: Option<i32>) -> ListProbeIdsRequest {
    let mut request = ListProbeIdsRequest::new();
    if probe_priority.is_some() {
        request.set_probe_priority(probe_priority.unwrap());
    }

    request
}

pub fn create_list_probe_ids_reply(probe_ids: Vec<String>) -> ListProbeIdsReply {
    let mut repeated_probe_id: RepeatedField<String> = RepeatedField::new();
    for probe_id in probe_ids {
        repeated_probe_id.push(probe_id);
    }

    let mut reply = ListProbeIdsReply::new();
    reply.set_probe_id(repeated_probe_id);
    reply
}

pub fn create_schedule_probe_request(probe_id: &str, host: &str, probe_priority: Option<i32>) -> ScheduleProbeRequest {
    let mut probe = Probe::new();
    probe.set_probe_id(probe_id.to_owned());
    probe.set_host(host.to_owned());
    if probe_priority.is_some() {
        probe.set_probe_priority(probe_priority.unwrap());
    }

    let mut request = ScheduleProbeRequest::new();
    request.set_probe(probe);
    request
}

pub fn create_schedule_probe_reply() -> ScheduleProbeReply {
    ScheduleProbeReply::new()
}
