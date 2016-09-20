extern crate docopt;
extern crate grpc;
extern crate inquest;
extern crate rustc_serialize;

use docopt::Docopt;
use inquest::inquest_pb::{DescribeProbeRequest, ListProbeIdsRequest, Probe, ScheduleProbeRequest};
use inquest::inquest_pb_grpc::{Inquest, InquestClient};

const USAGE: &'static str = "
Client application to inquest

Usage:
    inquisitor describe <probe-id>
    inquisitor list [--priority=<priority>]
    inquisitor schedule <probe-id> <host> [--priority=<priority>]
    inquisitor (-h | --help)

Options:
    -h --help               Show this screen.
    --priority=<priority>   Probe priority [default: 0].
";

#[derive(Debug, RustcDecodable)]
struct Args {
    cmd_describe: bool,
    cmd_list: bool,
    cmd_schedule: bool,
    arg_probe_id: String,
    arg_host: String,
    flag_priority: Option<i32>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                        .and_then(|d| d.decode())
                        .unwrap_or_else(|e| e.exit());

    let client = InquestClient::new("localhost", 12289).unwrap();

    if args.cmd_describe {
        let mut request = DescribeProbeRequest::new();
        request.set_probe_id(args.arg_probe_id);
        let response = client.DescribeProbe(request);

        println!("response: {:?}", response);
    } else if args.cmd_list {
        let mut request = ListProbeIdsRequest::new();
        if args.flag_priority.is_some() {
            request.set_probe_priority(args.flag_priority.unwrap());
        }
        let response = client.ListProbeIds(request);

        println!("response: {:?}", response);
    } else if args.cmd_schedule {
        let mut probe = Probe::new();
        probe.set_probe_id(args.arg_probe_id);
        probe.set_host(args.arg_host);
        if args.flag_priority.is_some() {
            probe.set_probe_priority(args.flag_priority.unwrap());
        }

        let mut request = ScheduleProbeRequest::new();
        request.set_probe(probe);
        let response = client.ScheduleProbe(request);

        println!("response: {:?}", response);
    }
}
