extern crate inquest;
extern crate toml;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, RwLock};

use inquest::inquest_pb::Probe;
use inquest::inquest_pb_grpc::{ProbeCache, ProbeCacheClient};
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
    let host = toml_table.lookup("server.host")
                        .expect("unable to find field 'server.host'")
                        .as_str().expect("unable to parse configuration_server.host into &str");
    let port = toml_table.lookup("server.port")
                        .expect("unable to find field 'server.port'")
                        .as_integer().expect("unable to parse configuration_server.port into integer") as u16;
    let probe_priority = toml_table.lookup("probe.probe_priority")
                        .expect("unable to find field 'probe.probe_priority'")
                        .as_integer().expect("unable to parse probe.probe_priority into integer") as i32;

    //create prober
    let prober = ProberImpl::new();

    //open client and start scheduling probes
    let client = ProbeCacheClient::new(host, port).unwrap();
    loop {
        {
            let request = inquest::create_gather_probes_request(Some(probe_priority), prober.get_probe_ids());
            let response = client.GatherProbes(request).unwrap();

            //cancel probes
            for probe_id in response.get_cancel_probe_id() {
                prober.cancel_probe(probe_id);
            }

            //schedule new probes
            for probe in response.get_probe() {
                prober.schedule_probe(probe);
            }
        }

        std::thread::sleep_ms(5000);
    }
}

struct ProberImpl {
    probe_map: Arc<RwLock<HashMap<String, Probe>>>,
}

impl ProberImpl {
    fn new() -> ProberImpl {
        ProberImpl {
            probe_map: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn schedule_probe(&self, probe: &Probe) {
        println!("TODO scheudle_probe: {:?}", probe);
    }

    fn cancel_probe(&self, probe_id: &str) {
        println!("TODO cancel_probe: {}", probe_id);
    }
    
    fn get_probe_ids(&self) -> Vec<String> {
        let probe_map = self.probe_map.read().unwrap();
        probe_map.keys().map(|key| key.to_owned()).collect()
    }
}
