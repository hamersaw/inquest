#[macro_use]
extern crate chan;
extern crate chrono;
extern crate curl;
extern crate grpc;
extern crate futures;
extern crate futures_cpupool;
extern crate protobuf;
extern crate resolv;
extern crate threadpool;
extern crate toml;

pub mod pb;
pub mod writer;

use std::collections::HashMap;
use std::hash::{Hash, Hasher, SipHasher};
use std::time::{Duration, Instant};

use pb::proddle::{CancelProbeRequest, GetBucketKeysRequest, GetProbesRequest, SearchRequest, SendProbeResultsRequest, ScheduleProbeRequest};
use pb::proddle::{CancelProbeReply, GetBucketKeysReply, GetProbesReply, SearchReply, SendProbeResultsReply, ScheduleProbeReply};
use pb::proddle::{BucketHash, BucketProbes, Probe, Protocol, ProbeResult};

use curl::easy::Easy;
use chrono::offset::utc::UTC;
use protobuf::RepeatedField;
use resolv::{Resolver, Class, RecordType};
use resolv::record::A;

/*
 * CancelProbe Messages
 */
pub fn create_cancel_probe_request(domain: &str, dns: bool, http: bool, https: bool, ping: bool, traceroute: bool, url_suffix: Option<String>) -> CancelProbeRequest {
    let mut request = CancelProbeRequest::new();
    request.set_domain(domain.to_owned());

    let empty = !dns && !http && !https && !ping && !traceroute;
    let mut repeated_protocol = Vec::new();
    if empty || dns {
        repeated_protocol.push(Protocol::DNS);
    }
    
    if empty || http {
        repeated_protocol.push(Protocol::HTTP);
    }
    
    if empty || https {
        repeated_protocol.push(Protocol::HTTPS);
    }
    
    if empty || ping {
        repeated_protocol.push(Protocol::PING);
    }

    if empty || traceroute {
        repeated_protocol.push(Protocol::TRACEROUTE);
    }

    request.set_protocol(repeated_protocol);

    if url_suffix.is_some() {
        request.set_url_suffix(url_suffix.unwrap());
    }

    request
}

pub fn create_cancel_probe_reply() -> CancelProbeReply {
    CancelProbeReply::new()
}

/*
 * Search Messages
 */
pub fn create_search_request(domain: &str, dns: bool, http: bool, https: bool, ping: bool, traceroute: bool) -> SearchRequest {
    let mut request = SearchRequest::new();
    request.set_domain(domain.to_owned());
    
    let empty = !dns && !http && !https && !ping && !traceroute;
    let mut repeated_protocol = Vec::new();
    if empty || dns {
        repeated_protocol.push(Protocol::DNS);
    }
    
    if empty || http {
        repeated_protocol.push(Protocol::HTTP);
    }
    
    if empty || https {
        repeated_protocol.push(Protocol::HTTPS);
    }
    
    if empty || ping {
        repeated_protocol.push(Protocol::PING);
    }

    if empty || traceroute {
        repeated_protocol.push(Protocol::TRACEROUTE);
    }

    request.set_protocol(repeated_protocol);
    request
}

pub fn create_search_reply(probes: Vec<Probe>) -> SearchReply {
    let mut reply = SearchReply::new();
    
    let mut repeated_probe = RepeatedField::new();
    for probe in probes {
        repeated_probe.push(probe);
    }
    reply.set_probe(repeated_probe);

    reply
}

/*
 * ScheduleProbe Messages
 */
pub fn create_dns_probe(domain: &str, probe_interval_seconds: u32, timeout_seconds: u32) -> Probe {
    let mut probe = Probe::new();
    probe.set_probe_interval_seconds(probe_interval_seconds);
    probe.set_timeout_seconds(timeout_seconds);

    probe.set_protocol(Protocol::DNS);
    probe.set_domain(domain.to_owned());

    probe
}

pub fn create_http_probe(domain: &str, probe_interval_seconds: u32, timeout_seconds: u32, url_suffix: Option<String>, follow: bool) -> Probe {
    let mut probe = Probe::new();
    probe.set_probe_interval_seconds(probe_interval_seconds);
    probe.set_timeout_seconds(timeout_seconds);

    probe.set_protocol(Protocol::HTTP);
    probe.set_domain(domain.to_owned());
    
    if url_suffix.is_some() {
        probe.set_url_suffix(url_suffix.unwrap());
    }
    probe.set_follow_redirect(follow);

    probe
}

pub fn create_schedule_probe_request(probes: Vec<Probe>) -> ScheduleProbeRequest {
    let mut repeated_probe = RepeatedField::new(); 
    for probe in probes {
        repeated_probe.push(probe);
    }

    let mut schedule_probe_request = ScheduleProbeRequest::new();
    schedule_probe_request.set_probe(repeated_probe);
    schedule_probe_request
}

pub fn create_schedule_probe_reply() -> ScheduleProbeReply {
    ScheduleProbeReply::new()
}

/*
 * ProbeCache Messages
 */
pub fn create_get_bucket_keys_request() -> GetBucketKeysRequest {
    GetBucketKeysRequest::new()
}

pub fn create_get_bucket_keys_reply(bucket_keys: Vec<u64>) -> GetBucketKeysReply {
    let mut reply = GetBucketKeysReply::new();
    reply.set_bucket_key(bucket_keys);
    reply
}

pub fn create_get_probes_request(bucket_hashes: HashMap<u64, u64>) -> GetProbesRequest {
    let mut repeated_bucket_hash = RepeatedField::new();
    for (key, value) in bucket_hashes {
        let mut bucket_hash = BucketHash::new();
        bucket_hash.set_bucket_key(key);
        bucket_hash.set_hash(value);
        repeated_bucket_hash.push(bucket_hash);
    }

    let mut request = GetProbesRequest::new();
    request.set_bucket_hash(repeated_bucket_hash);
    request
}

pub fn create_get_probes_reply(bucket_probes: HashMap<u64, Vec<Probe>>) -> GetProbesReply {
    let mut repeated_bucket_probe = RepeatedField::new();
    for (key, value) in bucket_probes {
        let mut bucket_probe = BucketProbes::new();
        bucket_probe.set_bucket_key(key);
        
        let mut repeated_probe = RepeatedField::new();
        for probe in value {
            repeated_probe.push(probe);
        }
        bucket_probe.set_probe(repeated_probe);

        repeated_bucket_probe.push(bucket_probe);
    }

    let mut reply = GetProbesReply::new();
    reply.set_bucket_probes(repeated_bucket_probe);
    reply
}

pub fn create_send_probe_results_request(probe_results: &Vec<ProbeResult>) -> SendProbeResultsRequest {
    let mut request = SendProbeResultsRequest::new();

    let mut repeated_probe_result = RepeatedField::new();
    for probe in probe_results {
        repeated_probe_result.push(probe.clone());
    }

    request.set_probe_result(repeated_probe_result);
    request
}

pub fn create_send_probe_results_reply() -> SendProbeResultsReply {
    SendProbeResultsReply::new()
}
/*
 * Execute Probe Functions
 */
pub fn execute_probe(probe: &Probe) -> Result<ProbeResult, &str> {
    let mut probe_result = ProbeResult::new();
    probe_result.set_probe_id(probe.get_probe_id().to_owned());
    probe_result.set_timestamp_sec(UTC::now().timestamp() as u64);

    let result = match probe.get_protocol() {
        Protocol::DNS => execute_dns_probe(probe, &mut probe_result),
        Protocol::HTTP => execute_http_probe(probe, &mut probe_result),
        _ => Err("unsupported probe type".to_owned()),
    };

    if result.is_err() {
        println!("{:?}", result);
    }

    Ok(probe_result)
}

fn execute_dns_probe(probe: &Probe, probe_result: &mut ProbeResult) -> Result<(), String> {
    probe_result.set_protocol(Protocol::DNS);

    //DNS resolution
    let mut resolver = Resolver::new().unwrap();
    match resolver.query(&probe.get_domain().as_bytes(), Class::IN, RecordType::A) {
        Ok(mut response) => {
            let mut repeated_dns_answer = RepeatedField::new();
            for answer in response.answers::<A>() {
                repeated_dns_answer.push(answer.data.address.octets().to_vec());
            }
            probe_result.set_dns_answer(repeated_dns_answer);

            probe_result.set_success(true);
        },
        Err(e) => {
            probe_result.set_success(false);
            probe_result.set_error_message(format!("{:?}", e));
        },
    };

    Ok(())
}

fn execute_http_probe(probe: &Probe, probe_result: &mut ProbeResult) -> Result<(), String> {
    //create curl handle
    let mut buffer = Vec::new();
    let mut handle = Easy::new();    
    {
        //set handle parameters
        handle.url(&format!("http://www.{}/{}", probe.get_domain(), probe.get_url_suffix())).unwrap();
        handle.follow_location(probe.get_follow_redirect()).unwrap();
        handle.timeout(Duration::from_secs(probe.get_timeout_seconds() as u64)).unwrap();
        handle.useragent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2840.100 Safari/537.36").unwrap();

        /*let mut list = List::new();
        list.append(format!("Host: {}", host).as_str()).unwrap();
        handle.http_headers(list).unwrap();*/

        //populate write callback function
        let mut transfer = handle.transfer();
        transfer.write_function(|data| {
            buffer.extend_from_slice(data);
            Ok(data.len())
        }).unwrap();
        
        //submit request
        //let start_time = time::now_utc();
        let instant = Instant::now();
        match transfer.perform() {
            Ok(_) => {},
            Err(e) => {
                probe_result.set_success(false);
                probe_result.set_error_message(format!("{}", e));
                return Ok(());
            },
        }

        //let execution_time = time::now_utc().sub(start_time);
        //probe_result.set_application_layer_latency_nanosec(execution_time.num_nanoseconds().unwrap());
        let duration = instant.elapsed();
        probe_result.set_application_layer_latency_nanosec((duration.as_secs() * 1000000000) + (duration.subsec_nanos() as u64));
    }

    probe_result.set_application_bytes_received(buffer.len() as u32);
    match handle.response_code() {
        Ok(response_code) => {
            probe_result.set_success(true);
            probe_result.set_http_status_code(response_code as u32);
        },
        Err(e) => {
            probe_result.set_success(false);
            probe_result.set_error_message(format!("{}", e));
        }
    }

    Ok(())
}

/*
 * Compute Hashes
 */
pub fn compute_probe_hash(probe: &Probe) -> u64 {
    match probe.get_protocol() {
        Protocol::DNS => compute_dns_hash(probe.get_domain()),
        Protocol::HTTP => compute_http_hash(probe.get_domain(), probe.get_url_suffix()),
        _ => 0,
    }
}

pub fn compute_dns_hash(domain: &str) -> u64 {
    let mut hasher = SipHasher::new();
    "DNS".hash(&mut hasher);
    domain.hash(&mut hasher);
    hasher.finish()
}

pub fn compute_http_hash(domain: &str, url_suffix: &str) -> u64 {
    let mut hasher = SipHasher::new();
    "HTTP".hash(&mut hasher);
    domain.hash(&mut hasher);
    url_suffix.hash(&mut hasher);
    hasher.finish()
}

pub fn compute_domain_hash(domain: &str) -> u64 {
    let mut hasher = SipHasher::new();
    domain.hash(&mut hasher);
    hasher.finish()
}
