extern crate protobuf;
extern crate grpc;
extern crate futures;
extern crate futures_cpupool;

pub mod inquest_pb;
pub mod inquest_pb_grpc;

use protobuf::RepeatedField;

use inquest_pb::{DescribeProbeRequest, DescribeProbeReply};
use inquest_pb::{GatherProbesRequest, GatherProbesReply};
use inquest_pb::{ListProbeIdsRequest, ListProbeIdsReply};
use inquest_pb::{Probe};
use inquest_pb::{ScheduleProbeRequest, ScheduleProbeReply};

pub fn create_describe_probe_reply(probe: &Probe) -> DescribeProbeReply {
    let mut reply = DescribeProbeReply::new();
    reply.set_probe(probe.clone());
    reply
}

pub fn create_gather_probes_reply(probes: Vec<&Probe>) -> GatherProbesReply {
    let mut repeated_field: RepeatedField<Probe> = RepeatedField::new();
    for probe in probes {
        repeated_field.push(probe.clone());
    }

    let mut reply = GatherProbesReply::new();
    reply.set_probe(repeated_field);
    reply
}

pub fn create_list_probe_ids_reply(probe_ids: Vec<&String>) -> ListProbeIdsReply {
    let mut repeated_field: RepeatedField<String> = RepeatedField::new();
    for probe_id in probe_ids {
        repeated_field.push(probe_id.to_owned());
    }

    let mut reply = ListProbeIdsReply::new();
    reply.set_probe_id(repeated_field);
    reply
}

pub fn create_schedule_probe_reply() -> ScheduleProbeReply {
    ScheduleProbeReply::new()
}
