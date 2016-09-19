extern crate inquest;
extern crate grpc;

use grpc::result::GrpcResult;

use inquest::inquest_pb::{ScheduleProbeRequest, ScheduleProbeReply};
use inquest::inquest_pb_grpc::{Inquest, InquestServer};

fn main() {
    let _server = InquestServer::new(12289, InquestImpl::new());

    loop {
        std::thread::park();
    }
}

struct InquestImpl {
        
}

impl InquestImpl {
    fn new() -> InquestImpl {
        InquestImpl {

        }
    }
}

impl Inquest for InquestImpl {
    fn ScheduleProbe(&self, request: ScheduleProbeRequest) -> GrpcResult<ScheduleProbeReply> {
        let mut reply = ScheduleProbeReply::new();
        reply.set_success(false);
        reply.set_error_msg("Unimplemented".to_string());
        Ok(reply)
    }
}
