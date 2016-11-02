extern crate grpc;
extern crate inquest;
extern crate toml;

use std::collections::{BinaryHeap, HashMap, BTreeMap};
use std::fs::File;
use std::hash::{Hash, Hasher, SipHasher};
use std::io::Read;
use std::sync::{Arc, Mutex, RwLock};

use grpc::error::GrpcError;
use grpc::result::GrpcResult;

use inquest::inquest_pb::{CancelProbeRequest, GetBucketKeysRequest, GetProbesRequest, SearchRequest, SendProbeResultsRequest, ScheduleProbeRequest};
use inquest::inquest_pb::{CancelProbeReply, GetBucketKeysReply, GetProbesReply, SearchReply, SendProbeResultsReply, ScheduleProbeReply};
use inquest::inquest_pb::{Probe, Protocol};
use inquest::inquest_pb_grpc::{ProbeCache, ProbeCacheServer, Scheduler, SchedulerServer};
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
    let writer_str = toml_table.lookup("writer.type")
                        .expect("unable to find field 'writer.type'")
                        .as_str().expect("unable to parse writer.type into &str");
    let bucket_count = toml_table.lookup("bucket_count")
                        .expect("unable to find field 'bucket_count'")
                        .as_integer().expect("unable to parse bucket_count into integer") as u64;

    //create prober
    let writer = match writer_str {
        "PrintWriter" => Arc::new(Mutex::new(Box::new(PrintWriter::new()) as Box<Writer + Send>)),
        "FileWriter" => {
            let directory = toml_table.lookup("writer.directory")
                                .expect("unable to find field 'writer.directory'")
                                .as_str().expect("unable to parse writer.directory into &str");

            let max_filesize = toml_table.lookup("writer.max_filesize")
                                .expect("unable to find field 'writer.max_filesize'")
                                .as_integer().expect("unable to parse writer.max_filesize into integer") as u32;

            Arc::new(Mutex::new(Box::new(FileWriter::new(directory, max_filesize)) as Box<Writer + Send>))
        }
        _ => panic!("unknown writer type '{}'", writer_str),
    };

    //intialize server variables
    let probe_map = Arc::new(RwLock::new(BTreeMap::new()));

    {
        //add buckets to probe_map
        let bucket_count = bucket_count;
        let mut counter = 0;
        let delta = u64::max_value() / bucket_count;
        let mut probe_map = probe_map.write().unwrap();
        for _ in 0..bucket_count {
            probe_map.insert(counter, HashMap::new());
            counter += delta;
        }
    }

    let _probe_cache_server = ProbeCacheServer::new(52890, ProbeCacheImpl::new(probe_map.clone(), writer));
    let _scheduler_server = SchedulerServer::new(12289, SchedulerImpl::new(probe_map.clone()));

    loop {
        std::thread::park();
    }
}

struct ProbeCacheImpl {
    probe_map: Arc<RwLock<BTreeMap<u64, HashMap<String, HashMap<Protocol, Vec<Probe>>>>>>, //map<domain_hash, map<domain, vec<probe>>>
    writer: Arc<Mutex<Box<Writer + Send>>>,
}

impl ProbeCacheImpl {
    fn new(probe_map: Arc<RwLock<BTreeMap<u64, HashMap<String, HashMap<Protocol, Vec<Probe>>>>>>, writer: Arc<Mutex<Box<Writer + Send>>>) -> ProbeCacheImpl {
        ProbeCacheImpl {
            probe_map: probe_map,
            writer: writer,
        }
    }
}

impl ProbeCache for ProbeCacheImpl {
    fn GetBucketKeys(&self, _: GetBucketKeysRequest) -> GrpcResult<GetBucketKeysReply> {
        let bucket_keys;
        {
            let probe_map = self.probe_map.read().unwrap();
            bucket_keys = probe_map.keys().cloned().collect();
        }

        Ok(inquest::create_get_bucket_keys_reply(bucket_keys))
    }

    fn GetProbes(&self, request: GetProbesRequest) -> GrpcResult<GetProbesReply> {
        //compute local bucket hashes
        let mut bucket_hashes = HashMap::new();
        {
            let probe_map = self.probe_map.read().unwrap();
            for (bucket_key, domain_map) in probe_map.iter() {
                let mut hasher = SipHasher::new();

                //add all probe ids to binary heap
                let mut probe_ids = BinaryHeap::new();
                for protocol_map in domain_map.values() {
                    for probes in protocol_map.values() {
                        for probe in probes {
                            probe_ids.push(probe.get_probe_id());
                        }
                    }
                }

                //loo over probe_ids in order
                loop {
                    match probe_ids.pop() {
                        Some(probe_id) => probe_id.hash(&mut hasher),
                        None => break,
                    }
                }

                bucket_hashes.insert(bucket_key.to_owned(), hasher.finish());
            }
        }

        //compare hashes
        let mut bucket_probes = HashMap::new();
        for bucket_hash in request.get_bucket_hash() {
            match bucket_hashes.get(&bucket_hash.get_bucket_key()) {
                Some(local_bucket_hash) => {
                    //check if bucket hashes differ
                    if &bucket_hash.get_hash() != local_bucket_hash {
                        let reply_probes = bucket_probes.entry(bucket_hash.get_bucket_key()).or_insert(Vec::new());

                        //add all local probes
                        let probe_map = self.probe_map.read().unwrap();
                        for domain_map in probe_map.get(&bucket_hash.get_bucket_key()) {
                            for protocol_map in domain_map.values() {
                                for probes in protocol_map.values() {
                                    for probe in probes {
                                        reply_probes.push(probe.clone());
                                    }
                                }
                            }
                        }
                    }
                },
                None => continue,
            }
        }

        Ok(inquest::create_get_probes_reply(bucket_probes))
    }

    fn SendProbeResults(&self, request: SendProbeResultsRequest) -> GrpcResult<SendProbeResultsReply> {
        let mut writer = self.writer.lock().unwrap();
        for probe_result in request.get_probe_result() {
            let _ = writer.write_probe_result(probe_result);
        }

        Ok(inquest::create_send_probe_results_reply())
    }
}

struct SchedulerImpl {
    probe_map: Arc<RwLock<BTreeMap<u64, HashMap<String, HashMap<Protocol, Vec<Probe>>>>>>, //map<domain_hash, map<domain, vec<probe>>>
}

impl SchedulerImpl {
    fn new(probe_map: Arc<RwLock<BTreeMap<u64, HashMap<String, HashMap<Protocol, Vec<Probe>>>>>>) -> SchedulerImpl {
        SchedulerImpl {
            probe_map: probe_map,
        }
    }
}

impl Scheduler for SchedulerImpl {
    fn CancelProbe(&self, request: CancelProbeRequest) -> GrpcResult<CancelProbeReply> {
        let key = inquest::compute_domain_hash(request.get_domain());
        let mut probe_map = self.probe_map.write().unwrap();

        //determine correct bucket key
        let mut bucket_key = 0;
        for map_key in probe_map.keys() {
            if *map_key > key {
                break;
            }

            bucket_key = *map_key;
        }

        let mut domain_map = probe_map.get_mut(&bucket_key).unwrap();
        let remove_domain;
        {
            let mut protocol_map = match domain_map.get_mut(request.get_domain()) {
                Some(protocol_map) => protocol_map,
                None => return Err(GrpcError::Other("domain doesn't exist")),
            };

            //loop over protocols
            for protocol in request.get_protocol() {
                let remove_protocol;
                {
                    let mut probes = match protocol_map.get_mut(protocol) {
                        Some(probes) => probes,
                        None => continue,
                    };

                    match protocol {
                        &Protocol::HTTP => {
                            if request.has_url_suffix() {
                                let mut index = -1;
                                for (i, p) in probes.iter().enumerate() {
                                    if p.get_url_suffix() == request.get_url_suffix() {
                                        index = i as i16;
                                        break;
                                    }
                                }

                                if index != -1 {
                                    probes.remove(index as usize);
                                }
                            } else {
                                probes.clear();
                            }
                        },
                        _ => probes.clear(),
                    }

                    remove_protocol = probes.len() == 0;
                }

                //remove protocol if there are no probes scheduled
                if remove_protocol {
                    protocol_map.remove(protocol);
                }
            }

            remove_domain = protocol_map.len() == 0;
        }

        //remove domain if there are no probes scheduled
        if remove_domain {
            domain_map.remove(request.get_domain());
        }

        Ok(inquest::create_cancel_probe_reply())
    }

    fn Search(&self, request: SearchRequest) -> GrpcResult<SearchReply> {
        let key = inquest::compute_domain_hash(request.get_domain());
        let probe_map = self.probe_map.read().unwrap();

        //determine correct bucket key
        let mut bucket_key = 0;
        for map_key in probe_map.keys() {
            if *map_key > key {
                break;
            }

            bucket_key = *map_key;
        }

        //find map containing protocols pertaining to the given domain
        let domain_map = probe_map.get(&bucket_key).unwrap();
        let protocol_map = match domain_map.get(request.get_domain()) {
            Some(protocol_map) => protocol_map,
            None => return Err(GrpcError::Other("domain does not exist")),
        };

        //loop over protocol arguments and return respective probes
        let mut search_probes: Vec<Probe> = Vec::new();
        for protocol in request.get_protocol() {
            match protocol_map.get(protocol) {
                Some(probes) => {
                    for p in probes {
                        search_probes.push(p.clone());
                    }
                },
                None => continue,
            }
        }

        Ok(inquest::create_search_reply(search_probes))
    }

    fn ScheduleProbe(&self, request: ScheduleProbeRequest) -> GrpcResult<ScheduleProbeReply> {
        for probe in request.get_probe() {
            let key = inquest::compute_domain_hash(probe.get_domain());
            let probe_id = inquest::compute_probe_hash(probe);

            let mut probe_map = self.probe_map.write().unwrap();

            //determine correct bucket key
            let mut bucket_key = 0;
            for map_key in probe_map.keys() {
                if *map_key > key {
                    break;
                }

                bucket_key = *map_key;
            }

            //see if probe already exists
            let mut domain_map = probe_map.get_mut(&bucket_key).unwrap();
            let mut protocol_map = domain_map.entry(probe.get_domain().to_owned()).or_insert(HashMap::new());
            let mut probes = protocol_map.entry(probe.get_protocol()).or_insert(Vec::new());
            
            let mut found = false;
            for p in probes.iter() {
                if p.get_probe_id() == probe_id {
                    found = true;
                    break;
                }
            }

            //add probe if needed
            if !found {
                let mut probe = probe.clone();
                probe.set_probe_id(probe_id);
                probes.push(probe);
            }
        }

        Ok(inquest::create_schedule_probe_reply())
    }
}
