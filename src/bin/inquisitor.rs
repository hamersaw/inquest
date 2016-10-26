extern crate docopt;
extern crate grpc;
extern crate inquest;
extern crate rustc_serialize;

use docopt::Docopt;
use inquest::inquest_pb_grpc::{Scheduler, SchedulerClient};

const USAGE: &'static str = "
Client application to inquest

Usage:
    inquisitor cancel <domain> [--dns] [--http [--url-suffix=<url-suffix>]] [--https] [--ping] [--traceroute]
    inquisitor search <domain> [--dns] [--http] [--https] [--ping] [--traceroute]
    inquisitor schedule-dns <domain> [--interval=<interval>]
    inquisitor schedule-http <domain> [--url-suffix=<url-suffix>] [--follow] [--interval=<interval>]
    inquisitor schedule-https <domain> [--url-suffix=<url-suffix>] [--follow] [--interval=<interval>]
    inquisitor schedule-ping <domain> [--interval=<interval>]
    inquisitor schedule-traceroute <domain> [--interval=<interval>]
    inquisitor (-h | --help)

Options:
    --dns                       Search for probes using the DNS protocol.
    --follow                    Follow HTTP/HTTPS redirects.
    --http                      Search for probes using the HTTP protocol.
    --https                     Search for probes using the HTTPS protocol.
    --interval=<interval>       Probe interval in seconds [default: 3600].
    --ping                      Search for probes using the PING protcol.
    --traceroute                Search for probes using the TRACEROUTE protocol.
    --url-suffix=<url_suffix>   Suffix for url.
    -h --help                   Show this screen.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    cmd_cancel: bool,
    cmd_search: bool,
    cmd_schedule_dns: bool,
    cmd_schedule_http: bool,
    cmd_schedule_https: bool,
    cmd_schedule_ping: bool,
    cmd_schedule_traceroute: bool,
    arg_domain: String,
    flag_dns: bool,
    flag_follow: bool,
    flag_http: bool,
    flag_https: bool,
    flag_interval: i32,
    flag_ping: bool,
    flag_traceroute: bool,
    flag_url_suffix: Option<String>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                        .and_then(|d| d.decode())
                        .unwrap_or_else(|e| e.exit());

    let client = SchedulerClient::new("localhost", 12289, false).unwrap();

    if args.cmd_cancel {
        let request = inquest::create_cancel_probe_request(&args.arg_domain, args.flag_dns, args.flag_http, args.flag_https, args.flag_ping, args.flag_traceroute, args.flag_url_suffix);
        let response = client.CancelProbe(request);

        println!("response: {:?}", response);
    } else if args.cmd_search {
        let request = inquest::create_search_request(&args.arg_domain, args.flag_dns, args.flag_http, args.flag_https, args.flag_ping, args.flag_traceroute);
        let response = client.Search(request);

        println!("response: {:?}", response);
    } else if args.cmd_schedule_dns {
        let probe = inquest::create_dns_probe(&args.arg_domain, args.flag_interval);
        let request = inquest::create_schedule_probe_request(vec!(probe));
        let response = client.ScheduleProbe(request);

        println!("response: {:?}", response);
    } else if args.cmd_schedule_http {
        let probe = inquest::create_http_probe(&args.arg_domain, args.flag_interval, args.flag_url_suffix, args.flag_follow);
        let request = inquest::create_schedule_probe_request(vec!(probe));
        let response = client.ScheduleProbe(request);

        println!("response: {:?}", response);
    } else if args.cmd_schedule_https {
        unimplemented!();
    } else if args.cmd_schedule_ping {
        unimplemented!();
    } else if args.cmd_schedule_traceroute {
        unimplemented!();
    }
}
