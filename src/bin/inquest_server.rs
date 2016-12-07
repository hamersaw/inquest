extern crate docopt;
extern crate grpc;
extern crate inquest;
extern crate rustc_serialize;

use std::collections::{BinaryHeap, HashMap, BTreeMap};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex, RwLock};

use docopt::Docopt;
use grpc::error::GrpcError;
use grpc::result::GrpcResult;
use inquest::pb::proddle::{CancelProbeRequest, GetBucketKeysRequest, GetProbesRequest, SearchRequest, SendProbeResultsRequest, ScheduleProbeRequest};
use inquest::pb::proddle::{CancelProbeReply, GetBucketKeysReply, GetProbesReply, SearchReply, SendProbeResultsReply, ScheduleProbeReply};
use inquest::pb::proddle::{Probe, Protocol};
use inquest::pb::proddle_grpc::{ProbeCache, ProbeCacheServer, Scheduler, SchedulerServer};
use inquest::writer::{FileWriter, PrintWriter, Writer};

const USAGE: &'static str = "
Inquest Server

Usage:
    inquest_server (-h | --help)
    inquest_server (--print | --file <directory> [--max-filesize=<max-filesize>]) [--bucket-count=<bucket-count>]

Options:
    --directory=<directory>             Directory to write result files.
    -h --help                           Display this screen.
    --bucket-count=<bucket-count>       Number of buckets [default: 1000];
    --max-filesize=<max-filesize>       Maxiumum filesize for results files (in MB) [default: 5];
    --print                             Print results to stdout;
    --file                              Write results out to files.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_directory: String,
    flag_bucket_count: u64,
    flag_max_filesize: u32,
    flag_print: bool,
    flag_file: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                        .and_then(|d| d.decode())
                        .unwrap_or_else(|e| e.exit());

    //initialize writer
    let writer = if args.flag_print {
        Arc::new(Mutex::new(Box::new(PrintWriter::new()) as Box<Writer + Send>))
    } else if args.flag_file {
        Arc::new(Mutex::new(Box::new(FileWriter::new(&args.arg_directory, 1024 * 1024 * args.flag_max_filesize)) as Box<Writer + Send>))
    } else {
        panic!("Unable to start inquest_server without writer type. Please specify '--print' or '--file' in arguments.");    
    };

    //intialize server variables
    let probe_map = Arc::new(RwLock::new(BTreeMap::new()));

    {
        //add buckets to probe_map
        let mut counter = 0;
        let delta = u64::max_value() / args.flag_bucket_count;
        let mut probe_map = probe_map.write().unwrap();
        for _ in 0..args.flag_bucket_count {
            probe_map.insert(counter, HashMap::new());
            counter += delta;
        }
    }

    let _probe_cache_server = ProbeCacheServer::new(52890, ProbeCacheImpl::new(probe_map.clone(), writer.clone()));
    let _scheduler_server = SchedulerServer::new(12289, SchedulerImpl::new(probe_map.clone(), writer.clone()));

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
                let mut hasher = DefaultHasher::new();

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
                while let Some(probe_id) = probe_ids.pop() {
                    probe_id.hash(&mut hasher);
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
    writer: Arc<Mutex<Box<Writer + Send>>>,
}

impl SchedulerImpl {
    fn new(probe_map: Arc<RwLock<BTreeMap<u64, HashMap<String, HashMap<Protocol, Vec<Probe>>>>>>, writer: Arc<Mutex<Box<Writer + Send>>>) -> SchedulerImpl {
        SchedulerImpl {
            probe_map: probe_map,
            writer: writer,
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

            if !found {
                //add probe
                let mut probe = probe.clone();
                probe.set_probe_id(probe_id);

                {
                    let mut writer = self.writer.lock().unwrap();
                    let _ = writer.write_probe(&probe);
                }
                probes.push(probe);
            }
        }

        Ok(inquest::create_schedule_probe_reply())
    }
}
