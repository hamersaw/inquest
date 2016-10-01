extern crate docopt;
extern crate grpc;
extern crate inquest;
extern crate rustc_serialize;

use docopt::Docopt;
use inquest::inquest_pb_grpc::{Scheduler, SchedulerClient};

const USAGE: &'static str = "
Client application to inquest

Usage:
    inquisitor cancel <probe-id>
    inquisitor describe <probe-id>
    inquisitor list [--priority=<priority>]
    inquisitor schedule <probe-id> (--http | --https) <host> [--url-suffix=<url-suffix>] [--follow] [--interval=<interval>] [--priority=<priority>]
    inquisitor (-h | --help)

Options:
    -h --help               Show this screen.
    --follow                Follow HTTP/HTTPS redirects.
    --http                  Use HTTP for probe protocol.
    --https                 Use HTTPS for probe protocol.
    --interval=<interval>   Probe interval in seconds [default: 10].
    --priority=<priority>   Probe priority [default: 0].
";

#[derive(Debug, RustcDecodable)]
struct Args {
    cmd_cancel: bool,
    cmd_describe: bool,
    cmd_list: bool,
    cmd_schedule: bool,
    arg_probe_id: String,
    arg_host: String,
    flag_interval: Option<i32>,
    flag_priority: Option<i32>,
    flag_url_suffix: Option<String>,
    flag_follow: bool,
    flag_http: bool,
    flag_https: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                        .and_then(|d| d.decode())
                        .unwrap_or_else(|e| e.exit());

    let client = SchedulerClient::new("localhost", 12289, false).unwrap();

    if args.cmd_cancel {
        let request = inquest::create_cancel_probe_request(&args.arg_probe_id);
        let response = client.CancelProbe(request);

        println!("response: {:?}", response);
    } else if args.cmd_describe {
        let request = inquest::create_describe_probe_request(&args.arg_probe_id);
        let response = client.DescribeProbe(request);

        println!("response: {:?}", response);
    } else if args.cmd_list {
        let request = inquest::create_list_probe_ids_request(args.flag_priority);
        let response = client.ListProbeIds(request);

        println!("response: {:?}", response);
    } else if args.cmd_schedule {
        let request = inquest::create_schedule_probe_request(&args.arg_probe_id, args.flag_http, args.flag_https, &args.arg_host, args.flag_url_suffix, args.flag_interval, args.flag_priority, args.flag_follow);
        let response = client.ScheduleProbe(request);

        println!("response: {:?}", response);
    }
}
