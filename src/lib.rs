extern crate protobuf;
extern crate grpc;
extern crate futures;
extern crate futures_cpupool;

pub mod inquest_pb;
pub mod inquest_pb_grpc;

use inquest_pb::{ScheduleProbeRequest, ScheduleProbeReply};

pub fn schedule_probe_reply_success() -> ScheduleProbeReply {
    let mut reply = ScheduleProbeReply::new();
    reply.set_success(true);
    reply
}

pub fn schedule_probe_reply_failure(msg: &str) -> ScheduleProbeReply {
    let mut reply = ScheduleProbeReply::new();
    reply.set_success(false);
    reply.set_error_msg(msg.to_owned());
    reply
}
