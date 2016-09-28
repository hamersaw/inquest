#[macro_use]
extern crate chan;
extern crate inquest;
extern crate hyper;
extern crate threadpool;
extern crate time;
extern crate toml;

use std::fs::File;
use std::io::Read;

use inquest::inquest_pb_grpc::{ProbeCache, ProbeCacheClient};
use inquest::prober::{Prober, ThreadPoolProberImpl};
use inquest::writer::{FileWriter, PrintWriter, Writer};
use toml::Parser;
use toml::Value::Table;

fn main() {
    //read arguments
    let mut args = std::env::args();
    if args.len() != 2 {
        panic!("Usage: {} <configuration-filename>", args.nth(0).unwrap());
    }

    //read toml configuration file
    let mut input = String::new();
    let filename = args.nth(1).unwrap();
    File::open(&filename).and_then(|mut f| {
        f.read_to_string(&mut input)
    }).unwrap();

    //parse into toml table
    let mut parser = Parser::new(&input);
    let toml = match parser.parse() {
        Some(toml) => toml,
        None => {
            for err in &parser.errors {
                println!("unable to parse configuration server:{} {:?} - {:?} : '{}'", filename, parser.to_linecol(err.lo), parser.to_linecol(err.hi), err.desc);
            }
            return
        }
    };

    //parse toml values
    let toml_table = Table(toml);
    let prober_hostname = toml_table.lookup("prober_hostname")
                        .expect("unable to find field 'prober_hostname'")
                        .as_str().expect("unable to parse prober_hostname into &str");
    let probe_threads = toml_table.lookup("probe_threads")
                        .expect("unable to find field 'probe_threads'")
                        .as_integer().expect("unable to parse probe_threads into integer") as usize;
    let probe_priority = toml_table.lookup("probe_priority")
                        .expect("unable to find field 'probe_priority'")
                        .as_integer().expect("unable to parse probe_priority into integer") as i32;
    let writer_str = toml_table.lookup("writer.type")
                        .expect("unable to find field 'writer.type'")
                        .as_str().expect("unable to parse writer.type into &str");
    let host = toml_table.lookup("server.host")
                        .expect("unable to find field 'server.host'")
                        .as_str().expect("unable to parse configuration_server.host into &str");
    let port = toml_table.lookup("server.port")
                        .expect("unable to find field 'server.port'")
                        .as_integer().expect("unable to parse configuration_server.port into integer") as u16;
    let probe_poll_seconds = toml_table.lookup("server.probe_poll_seconds")
                        .expect("unable to find field 'server.probe_poll_seconds'")
                        .as_integer().expect("unable to parse server.probe_poll_seconds into integer") as u32;

    //create prober
    let writer = match writer_str {
        "PrintWriter" => Box::new(PrintWriter::new()) as Box<Writer + Send>,
        "FileWriter" => {
            let directory = toml_table.lookup("writer.directory")
                                .expect("unable to find field 'writer.directory'")
                                .as_str().expect("unable to parse writer.directory into &str");

            let max_filesize = toml_table.lookup("writer.max_filesize")
                                .expect("unable to find field 'writer.max_filesize'")
                                .as_integer().expect("unable to parse writer.max_filesize into integer") as u32;

            Box::new(FileWriter::new(directory, max_filesize)) as Box<Writer + Send>
        }
        _ => panic!("unknown writer type '{}'", writer_str),
    };
    
    let prober = ThreadPoolProberImpl::new(writer, prober_hostname, probe_threads);

    //open client and start scheduling probes
    let client = ProbeCacheClient::new(host, port).unwrap();

    let tick = chan::tick_ms(probe_poll_seconds * 1000);
    loop {
        chan_select! {
            tick.recv() => {
                let request = inquest::create_gather_probes_request(Some(probe_priority), prober.get_probe_ids());
                let response = client.GatherProbes(request).unwrap();

                //cancel probes
                for probe_id in response.get_cancel_probe_id() {
                    let _ = prober.cancel_probe(probe_id);
                }

                //schedule new probes
                for probe in response.get_probe() {
                    let _ = prober.schedule_probe(probe);
                }
            }
        }
    }
}
