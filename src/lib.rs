#[macro_use]
extern crate chan;
extern crate curl;
extern crate grpc;
extern crate futures;
extern crate futures_cpupool;
extern crate protobuf;
extern crate resolv;
extern crate time;
extern crate threadpool;
extern crate toml;

pub mod inquest_pb;
pub mod inquest_pb_grpc;
pub mod prober;
pub mod writer;

use std::ops::Sub;

use inquest_pb::{CancelProbeRequest, GatherProbesRequest, SearchRequest, ScheduleProbeRequest};
use inquest_pb::{CancelProbeReply, GatherProbesReply, SearchReply, ScheduleProbeReply};
use inquest_pb::{Probe, Protocol, ProbeResult};
use curl::easy::Easy;
use protobuf::RepeatedField;
use resolv::{Resolver, Class, RecordType};
use resolv::record::A;

/*
 * CancelProbe Messages
 */
pub fn create_cancel_probe_request(probe_id: &str) -> CancelProbeRequest {
    let mut request = CancelProbeRequest::new();
    request.set_probe_id(probe_id.to_owned());
    request
}

pub fn create_cancel_probe_reply() -> CancelProbeReply {
    CancelProbeReply::new()
}

pub fn create_gather_probes_request(probe_ids: Vec<String>) -> GatherProbesRequest {
    let mut repeated_probe_id: RepeatedField<String> = RepeatedField::new();
    for probe_id in probe_ids {
        repeated_probe_id.push(probe_id);
    }

    let mut request = GatherProbesRequest::new();
    request.set_scheduled_probe_id(repeated_probe_id);

    request
}

/*
 * GatherProbes Messages
 */
pub fn create_gather_probes_reply(probes: Vec<&Probe>, probe_ids: Vec<&String>) -> GatherProbesReply {
    let mut repeated_probe: RepeatedField<Probe> = RepeatedField::new();
    for probe in probes {
        repeated_probe.push(probe.clone());
    }

    let mut repeated_probe_id: RepeatedField<String> = RepeatedField::new();
    for probe_id in probe_ids {
        repeated_probe_id.push(probe_id.to_owned());
    }

    let mut reply = GatherProbesReply::new();
    reply.set_probe(repeated_probe);
    reply.set_cancel_probe_id(repeated_probe_id);
    reply
}

pub fn create_dns_probe(domain: &str, probe_interval_seconds: i32) -> Probe {
    let mut probe = Probe::new();
    probe.set_probe_interval_seconds(probe_interval_seconds);
    probe.set_probe_interval_post_failure_seconds(probe_interval_seconds);

    probe.set_protocol(Protocol::DNS);
    probe.set_domain(domain.to_owned());

    probe
}

/*
 * Search Messages
 */
pub fn create_search_request(domain: &str, dns: bool, http: bool, https: bool, ping: bool, traceroute: bool) -> SearchRequest {
    let mut request = SearchRequest::new();
    request.set_domain(domain.to_owned());
    
    let mut repeated_protocol = Vec::new();
    if dns {
        repeated_protocol.push(Protocol::DNS);
    }
    
    if http {
        repeated_protocol.push(Protocol::HTTP);
    }
    
    if https {
        repeated_protocol.push(Protocol::HTTPS);
    }
    
    if ping {
        repeated_protocol.push(Protocol::PING);
    }

    if traceroute {
        repeated_protocol.push(Protocol::TRACEROUTE);
    }

    request.set_protocol(repeated_protocol);
    request
}

pub fn create_search_reply(probes: Vec<&Probe>) -> SearchReply {
    let mut reply = SearchReply::new();
    
    let mut repeated_probe = RepeatedField::new();
    for probe in probes {
        repeated_probe.push(probe.clone());
    }
    reply.set_probe(repeated_probe);

    reply
}

/*
 * ScheduleProbe Messages
 */
pub fn create_http_probe(domain: &str, probe_interval_seconds: i32, url_suffix: Option<String>, follow: bool) -> Probe {
    let mut probe = Probe::new();
    probe.set_probe_interval_seconds(probe_interval_seconds);
    probe.set_probe_interval_post_failure_seconds(probe_interval_seconds);

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
 * Execute Probe Functions
 */
pub fn execute_probe(probe: &Probe) -> Result<ProbeResult, &str> {
    let mut probe_result = ProbeResult::new();
    probe_result.set_probe_id(probe.get_probe_id().to_owned());
    probe_result.set_timestamp_sec(time::get_time().sec);

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
        let start_time = time::now_utc();
        match transfer.perform() {
            Ok(_) => {},
            Err(e) => {
                probe_result.set_success(false);
                probe_result.set_error_message(format!("{}", e));
                return Ok(());
            },
        }

        let execution_time = time::now_utc().sub(start_time);
        probe_result.set_application_layer_latency_nanosec(execution_time.num_nanoseconds().unwrap());
    }

    probe_result.set_application_bytes_received(buffer.len() as i32);
    match handle.response_code() {
        Ok(response_code) => {
            probe_result.set_success(true);
            probe_result.set_http_status_code(response_code as i32);
        },
        Err(e) => {
            probe_result.set_success(false);
            probe_result.set_error_message(format!("{}", e));
        }
    }

    Ok(())
}
