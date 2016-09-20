extern crate protobuf;
extern crate grpc;
extern crate futures;
extern crate futures_cpupool;

pub mod inquest_pb;
pub mod inquest_pb_grpc;

use inquest_pb::{DescribeProbeRequest, DescribeProbeReply};
use inquest_pb::{ListProbeIdsRequest, ListProbeIdsReply};
use inquest_pb::{Probe};
use inquest_pb::{ScheduleProbeRequest, ScheduleProbeReply};

pub fn describe_probe_reply_success(probe: Probe) -> DescribeProbeReply {
    let mut reply = DescribeProbeReply::new();
    reply.set_probe(probe);
    reply
}

pub fn describe_probe_reply_failure(msg: &str) -> DescribeProbeReply {
    let mut reply = DescribeProbeReply::new();
    reply.set_error_msg(msg.to_owned());
    reply
}

pub fn list_probe_ids_reply_success(probe_ids: Vec<&str>) -> ListProbeIdsReply {
    let mut reply = ListProbeIdsReply::new();
    //TODO add probe_ids
    reply
}

pub fn list_probe_ids_reply_failure(msg: &str) -> ListProbeIdsReply {
    let mut reply = ListProbeIdsReply::new();
    reply.set_error_msg(msg.to_owned());
    reply
}

pub fn schedule_probe_reply_success() -> ScheduleProbeReply {
    ScheduleProbeReply::new()
}

pub fn schedule_probe_reply_failure(msg: &str) -> ScheduleProbeReply {
    let mut reply = ScheduleProbeReply::new();
    reply.set_error_msg(msg.to_owned());
    reply
}
