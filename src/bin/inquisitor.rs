extern crate inquest;
extern crate grpc;

use inquest::inquest_pb::{ScheduleProbeRequest, ScheduleProbeReply};
use inquest::inquest_pb_grpc::{Inquest, InquestClient};

fn main() {
    let client = InquestClient::new("localhost", 12289).unwrap();

    let mut request = ScheduleProbeRequest::new();
    let response = client.ScheduleProbe(request);

    println!("response: {:?}", response);
}
