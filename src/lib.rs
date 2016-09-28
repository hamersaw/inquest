#[macro_use]
extern crate chan;
extern crate grpc;
extern crate futures;
extern crate futures_cpupool;
extern crate hyper;
extern crate protobuf;
extern crate resolv;
extern crate time;
extern crate threadpool;
extern crate toml;

pub mod inquest_pb;
pub mod inquest_pb_grpc;
pub mod prober;
pub mod writer;

use std::io::Read;
use std::ops::Sub;

use inquest_pb::{CancelProbeRequest, DescribeProbeRequest, GatherProbesRequest, ListProbeIdsRequest, ScheduleProbeRequest};
use inquest_pb::{CancelProbeReply, DescribeProbeReply, GatherProbesReply, ListProbeIdsReply, ScheduleProbeReply};
use inquest_pb::{HostProbeResult, Probe, Probe_Protocol, ProbeResult};
use hyper::Client;
use protobuf::RepeatedField;
use resolv::{Resolver, Class, RecordType};
use resolv::record::A;

pub fn create_cancel_probe_request(probe_id: &str) -> CancelProbeRequest {
    let mut request = CancelProbeRequest::new();
    request.set_probe_id(probe_id.to_owned());
    request
}

pub fn create_cancel_probe_reply() -> CancelProbeReply {
    CancelProbeReply::new()
}

pub fn create_describe_probe_request(probe_id: &str) -> DescribeProbeRequest {
    let mut request = DescribeProbeRequest::new();
    request.set_probe_id(probe_id.to_owned());
    request
}

pub fn create_describe_probe_reply(probe: &Probe) -> DescribeProbeReply {
    let mut reply = DescribeProbeReply::new();
    reply.set_probe(probe.clone());
    reply
}

pub fn create_gather_probes_request(probe_priority: Option<i32>, probe_ids: Vec<String>) -> GatherProbesRequest {
    let mut repeated_probe_id: RepeatedField<String> = RepeatedField::new();
    for probe_id in probe_ids {
        repeated_probe_id.push(probe_id);
    }

    let mut request = GatherProbesRequest::new();
    request.set_scheduled_probe_id(repeated_probe_id);
    if probe_priority.is_some() {
        request.set_probe_priority(probe_priority.unwrap());
    }


    request
}

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

pub fn create_list_probe_ids_request(probe_priority: Option<i32>) -> ListProbeIdsRequest {
    let mut request = ListProbeIdsRequest::new();
    if probe_priority.is_some() {
        request.set_probe_priority(probe_priority.unwrap());
    }

    request
}

pub fn create_list_probe_ids_reply(probe_ids: Vec<String>) -> ListProbeIdsReply {
    let mut repeated_probe_id: RepeatedField<String> = RepeatedField::new();
    for probe_id in probe_ids {
        repeated_probe_id.push(probe_id);
    }

    let mut reply = ListProbeIdsReply::new();
    reply.set_probe_id(repeated_probe_id);
    reply
}

pub fn create_schedule_probe_request(probe_id: &str, http: bool, https: bool, host: &str, url_suffix: Option<String>, probe_interval_seconds: Option<i32>, probe_priority: Option<i32>) -> ScheduleProbeRequest {
    let mut probe = Probe::new();
    probe.set_probe_id(probe_id.to_owned());
    probe.set_protocol(
            if http {
                Probe_Protocol::HTTP
            } else if https {
                Probe_Protocol::HTTPS
            } else {
                Probe_Protocol::HTTP
            }
        );

    probe.set_host(host.to_owned());
    probe.set_url_suffix(
            match url_suffix {
                Some(url_suffix) => url_suffix,
                None => "index.html".to_owned(),
            }
        );

    if probe_interval_seconds.is_some() {
        probe.set_probe_interval_seconds(probe_interval_seconds.unwrap());
    }

    if probe_priority.is_some() {
        probe.set_probe_priority(probe_priority.unwrap());
    }

    let mut request = ScheduleProbeRequest::new();
    request.set_probe(probe);
    request
}

pub fn create_schedule_probe_reply() -> ScheduleProbeReply {
    ScheduleProbeReply::new()
}

pub fn execute_probe(probe: &Probe) -> Result<ProbeResult, &str> {
    let mut probe_result = ProbeResult::new();
    probe_result.set_probe_id(probe.get_probe_id().to_owned());
    //TODO populate prober_hostname
    probe_result.set_success(true);

    //DNS resolution
    let mut resolver = Resolver::new().unwrap();
    let mut response = resolver.query(&probe.get_host().to_owned().into_bytes(), Class::IN, RecordType::A).unwrap(); //TODO fix &str -> String -> &[u8]

    let mut repeated_host_probe_result: RepeatedField<HostProbeResult> = RepeatedField::new();
    for answer in response.answers::<A>() {
        let mut host_probe_result = HostProbeResult::new();
        host_probe_result.set_timestamp_sec(time::get_time().sec);
        //TODO set ip_address
        host_probe_result.set_success(true);

        let _ = match probe.get_protocol() {
            Probe_Protocol::HTTP => execute_http_probe(&format!("http://{}/{}", answer.data.address, probe.get_url_suffix()), &mut host_probe_result),
            Probe_Protocol::HTTPS => execute_http_probe(&format!("https://{}/{}", answer.data.address, probe.get_url_suffix()), &mut host_probe_result),
            Probe_Protocol::PING => execute_ping_probe(&mut host_probe_result),
        };

        if !host_probe_result.get_success() {
            probe_result.set_success(false);
        }

        repeated_host_probe_result.push(host_probe_result);
    }

    probe_result.set_host_probe_result(repeated_host_probe_result);
    Ok(probe_result)
}

fn execute_http_probe(url: &str, host_probe_result: &mut HostProbeResult) -> Result<(), String> {
    //submit request
    let client = Client::new();
    let start_time = time::now_utc();
    let response = client.get(url).send();

    //calculate execution time
    let execution_time = time::now_utc().sub(start_time);
    host_probe_result.set_application_layer_latency_nanosec(execution_time.num_nanoseconds().unwrap());

    //parse response
    match response {
        Ok(response) => {
            {
                //populate http status codes and message
                let status_raw = response.status_raw();
                host_probe_result.set_http_status_message(format!("{}", status_raw.1)); //TODO fix status_raw.1 -> String using format!
                let http_status_code = status_raw.0 as i32;
                host_probe_result.set_http_status_code(http_status_code);

                //check if status code is 200
                if http_status_code != 200 {
                    host_probe_result.set_success(false);
                    host_probe_result.set_error_message("HTTP Status Code != 200".to_owned());
                }
            }

            {
                //populate application byte counts
                let byte_count = response.bytes().count();
                host_probe_result.set_application_bytes_received(byte_count as i32);
            }
        },
        Err(e) => {
            host_probe_result.set_success(false);
            host_probe_result.set_error_message(format!("{}", e));
        },
    }

    Ok(())
}

fn execute_ping_probe(host_probe_result: &mut HostProbeResult) -> Result<(), String> {
    unimplemented!();
}
