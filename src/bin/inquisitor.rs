extern crate docopt;
extern crate grpc;
extern crate inquest;
extern crate rustc_serialize;

use std::fs::File;
use std::io::{BufRead, BufReader};

use docopt::Docopt;
use inquest::inquest_pb_grpc::{Scheduler, SchedulerClient};

const USAGE: &'static str = "
Client application to inquest

Usage:
    inquisitor cancel --domain=<domain> [--dns] [--http [--url-suffix=<url-suffix>]] [--https] [--ping] [--traceroute]
    inquisitor search --domain=<domain> [--dns] [--http] [--https] [--ping] [--traceroute]
    inquisitor schedule-dns (--domain=<domain> | --file=<file>) [--interval=<interval>] [--timeout=<timeout>]
    inquisitor schedule-http (--domain=<domain> | --file=<file>) [--url-suffix=<url-suffix>] [--follow] [--interval=<interval>] [--timeout=<timeout>]
    inquisitor schedule-https (--domain=<domain> | --file=<file>) [--url-suffix=<url-suffix>] [--follow] [--interval=<interval>] [--timeout=<timeout>]
    inquisitor schedule-ping (--domain=<domain> | --file=<file>) [--interval=<interval>] [--timeout=<timeout>]
    inquisitor schedule-traceroute (--domain=<domain> | --file=<file>) [--interval=<interval>] [--timeout=<timeout>]
    inquisitor (-h | --help)

Options:
    --dns                       Search for probes using the DNS protocol.
    --domain=<domain>           Domain to perform schedule action on
    --file=<file>               File containing a single domain on each line to perform schedule action on
    --follow                    Follow HTTP/HTTPS redirects.
    --http                      Search for probes using the HTTP protocol.
    --https                     Search for probes using the HTTPS protocol.
    --interval=<interval>       Probe interval in seconds [default: 3600].
    --ping                      Search for probes using the PING protcol.
    --timeout=<timeout>         Timeout for operation in seconds [default: 120].
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
    flag_dns: bool,
    flag_domain: Option<String>,
    flag_file: Option<String>,
    flag_follow: bool,
    flag_http: bool,
    flag_https: bool,
    flag_interval: u32,
    flag_ping: bool,
    flag_traceroute: bool,
    flag_timeout: u32,
    flag_url_suffix: Option<String>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                        .and_then(|d| d.decode())
                        .unwrap_or_else(|e| e.exit());

    let client = SchedulerClient::new("localhost", 12289, false).unwrap();

    if args.cmd_cancel {
        let request = inquest::create_cancel_probe_request(&args.flag_domain.unwrap(), args.flag_dns, args.flag_http, args.flag_https, args.flag_ping, args.flag_traceroute, args.flag_url_suffix);
        let response = client.CancelProbe(request);

        println!("response: {:?}", response);
    } else if args.cmd_search {
        let request = inquest::create_search_request(&args.flag_domain.unwrap(), args.flag_dns, args.flag_http, args.flag_https, args.flag_ping, args.flag_traceroute);
        let response = client.Search(request);

        println!("response: {:?}", response);
    } else if args.cmd_schedule_dns {
        let probe = inquest::create_dns_probe(&args.flag_domain.unwrap(), args.flag_interval, args.flag_timeout);
        let request = inquest::create_schedule_probe_request(vec!(probe));
        let response = client.ScheduleProbe(request);

        println!("response: {:?}", response);
    } else if args.cmd_schedule_http {
        let response;
        if args.flag_file.is_some() {
            let f = match File::open(args.flag_file.unwrap()) {
                Ok(f) => f,
                Err(e) => panic!("Unable to open fil: {}", e),
            };

            let mut probes = Vec::new();
            let buf_reader = BufReader::new(f);
            for line in buf_reader.lines() {
                probes.push(inquest::create_http_probe(&line.unwrap(), args.flag_interval, args.flag_timeout, args.flag_url_suffix.clone(), args.flag_follow));
            }

            let request = inquest::create_schedule_probe_request(probes);
            response = client.ScheduleProbe(request);
        } else {
            let probe = inquest::create_http_probe(&args.flag_domain.unwrap(), args.flag_interval, args.flag_timeout, args.flag_url_suffix, args.flag_follow);
            let request = inquest::create_schedule_probe_request(vec!(probe));
            response = client.ScheduleProbe(request);
        }

        println!("response: {:?}", response);
    } else if args.cmd_schedule_https {
        unimplemented!();
    } else if args.cmd_schedule_ping {
        unimplemented!();
    } else if args.cmd_schedule_traceroute {
        unimplemented!();
    }
}
