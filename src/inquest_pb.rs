// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct Probe {
    // message fields
    probe_id: ::std::option::Option<u64>,
    probe_interval_seconds: ::std::option::Option<i32>,
    probe_interval_post_failure_seconds: ::std::option::Option<i32>,
    attempts_to_declare_failure: ::std::option::Option<i32>,
    protocol: ::std::option::Option<Protocol>,
    domain: ::protobuf::SingularField<::std::string::String>,
    port: ::std::option::Option<i32>,
    url_suffix: ::protobuf::SingularField<::std::string::String>,
    follow_redirect: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Probe {}

impl Probe {
    pub fn new() -> Probe {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Probe {
        static mut instance: ::protobuf::lazy::Lazy<Probe> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Probe,
        };
        unsafe {
            instance.get(|| {
                Probe {
                    probe_id: ::std::option::Option::None,
                    probe_interval_seconds: ::std::option::Option::None,
                    probe_interval_post_failure_seconds: ::std::option::Option::None,
                    attempts_to_declare_failure: ::std::option::Option::None,
                    protocol: ::std::option::Option::None,
                    domain: ::protobuf::SingularField::none(),
                    port: ::std::option::Option::None,
                    url_suffix: ::protobuf::SingularField::none(),
                    follow_redirect: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 probe_id = 1;

    pub fn clear_probe_id(&mut self) {
        self.probe_id = ::std::option::Option::None;
    }

    pub fn has_probe_id(&self) -> bool {
        self.probe_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_probe_id(&mut self, v: u64) {
        self.probe_id = ::std::option::Option::Some(v);
    }

    pub fn get_probe_id(&self) -> u64 {
        self.probe_id.unwrap_or(0)
    }

    // optional int32 probe_interval_seconds = 2;

    pub fn clear_probe_interval_seconds(&mut self) {
        self.probe_interval_seconds = ::std::option::Option::None;
    }

    pub fn has_probe_interval_seconds(&self) -> bool {
        self.probe_interval_seconds.is_some()
    }

    // Param is passed by value, moved
    pub fn set_probe_interval_seconds(&mut self, v: i32) {
        self.probe_interval_seconds = ::std::option::Option::Some(v);
    }

    pub fn get_probe_interval_seconds(&self) -> i32 {
        self.probe_interval_seconds.unwrap_or(0)
    }

    // optional int32 probe_interval_post_failure_seconds = 3;

    pub fn clear_probe_interval_post_failure_seconds(&mut self) {
        self.probe_interval_post_failure_seconds = ::std::option::Option::None;
    }

    pub fn has_probe_interval_post_failure_seconds(&self) -> bool {
        self.probe_interval_post_failure_seconds.is_some()
    }

    // Param is passed by value, moved
    pub fn set_probe_interval_post_failure_seconds(&mut self, v: i32) {
        self.probe_interval_post_failure_seconds = ::std::option::Option::Some(v);
    }

    pub fn get_probe_interval_post_failure_seconds(&self) -> i32 {
        self.probe_interval_post_failure_seconds.unwrap_or(0)
    }

    // optional int32 attempts_to_declare_failure = 4;

    pub fn clear_attempts_to_declare_failure(&mut self) {
        self.attempts_to_declare_failure = ::std::option::Option::None;
    }

    pub fn has_attempts_to_declare_failure(&self) -> bool {
        self.attempts_to_declare_failure.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attempts_to_declare_failure(&mut self, v: i32) {
        self.attempts_to_declare_failure = ::std::option::Option::Some(v);
    }

    pub fn get_attempts_to_declare_failure(&self) -> i32 {
        self.attempts_to_declare_failure.unwrap_or(0)
    }

    // optional .Protocol protocol = 5;

    pub fn clear_protocol(&mut self) {
        self.protocol = ::std::option::Option::None;
    }

    pub fn has_protocol(&self) -> bool {
        self.protocol.is_some()
    }

    // Param is passed by value, moved
    pub fn set_protocol(&mut self, v: Protocol) {
        self.protocol = ::std::option::Option::Some(v);
    }

    pub fn get_protocol(&self) -> Protocol {
        self.protocol.unwrap_or(Protocol::DNS)
    }

    // optional string domain = 6;

    pub fn clear_domain(&mut self) {
        self.domain.clear();
    }

    pub fn has_domain(&self) -> bool {
        self.domain.is_some()
    }

    // Param is passed by value, moved
    pub fn set_domain(&mut self, v: ::std::string::String) {
        self.domain = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_domain(&mut self) -> &mut ::std::string::String {
        if self.domain.is_none() {
            self.domain.set_default();
        };
        self.domain.as_mut().unwrap()
    }

    // Take field
    pub fn take_domain(&mut self) -> ::std::string::String {
        self.domain.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_domain(&self) -> &str {
        match self.domain.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional int32 port = 7;

    pub fn clear_port(&mut self) {
        self.port = ::std::option::Option::None;
    }

    pub fn has_port(&self) -> bool {
        self.port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_port(&mut self, v: i32) {
        self.port = ::std::option::Option::Some(v);
    }

    pub fn get_port(&self) -> i32 {
        self.port.unwrap_or(0)
    }

    // optional string url_suffix = 8;

    pub fn clear_url_suffix(&mut self) {
        self.url_suffix.clear();
    }

    pub fn has_url_suffix(&self) -> bool {
        self.url_suffix.is_some()
    }

    // Param is passed by value, moved
    pub fn set_url_suffix(&mut self, v: ::std::string::String) {
        self.url_suffix = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_url_suffix(&mut self) -> &mut ::std::string::String {
        if self.url_suffix.is_none() {
            self.url_suffix.set_default();
        };
        self.url_suffix.as_mut().unwrap()
    }

    // Take field
    pub fn take_url_suffix(&mut self) -> ::std::string::String {
        self.url_suffix.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_url_suffix(&self) -> &str {
        match self.url_suffix.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional bool follow_redirect = 9;

    pub fn clear_follow_redirect(&mut self) {
        self.follow_redirect = ::std::option::Option::None;
    }

    pub fn has_follow_redirect(&self) -> bool {
        self.follow_redirect.is_some()
    }

    // Param is passed by value, moved
    pub fn set_follow_redirect(&mut self, v: bool) {
        self.follow_redirect = ::std::option::Option::Some(v);
    }

    pub fn get_follow_redirect(&self) -> bool {
        self.follow_redirect.unwrap_or(false)
    }
}

impl ::protobuf::Message for Probe {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.probe_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.probe_interval_seconds = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.probe_interval_post_failure_seconds = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.attempts_to_declare_failure = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.protocol = ::std::option::Option::Some(tmp);
                },
                6 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.domain));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.port = ::std::option::Option::Some(tmp);
                },
                8 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.url_suffix));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.follow_redirect = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.probe_id {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.probe_interval_seconds {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.probe_interval_post_failure_seconds {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.attempts_to_declare_failure {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.protocol {
            my_size += ::protobuf::rt::enum_size(5, *value);
        };
        for value in &self.domain {
            my_size += ::protobuf::rt::string_size(6, &value);
        };
        for value in &self.port {
            my_size += ::protobuf::rt::value_size(7, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.url_suffix {
            my_size += ::protobuf::rt::string_size(8, &value);
        };
        if self.follow_redirect.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.probe_id {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.probe_interval_seconds {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.probe_interval_post_failure_seconds {
            try!(os.write_int32(3, v));
        };
        if let Some(v) = self.attempts_to_declare_failure {
            try!(os.write_int32(4, v));
        };
        if let Some(v) = self.protocol {
            try!(os.write_enum(5, v.value()));
        };
        if let Some(v) = self.domain.as_ref() {
            try!(os.write_string(6, &v));
        };
        if let Some(v) = self.port {
            try!(os.write_int32(7, v));
        };
        if let Some(v) = self.url_suffix.as_ref() {
            try!(os.write_string(8, &v));
        };
        if let Some(v) = self.follow_redirect {
            try!(os.write_bool(9, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Probe>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Probe {
    fn new() -> Probe {
        Probe::new()
    }

    fn descriptor_static(_: ::std::option::Option<Probe>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "probe_id",
                    Probe::has_probe_id,
                    Probe::get_probe_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "probe_interval_seconds",
                    Probe::has_probe_interval_seconds,
                    Probe::get_probe_interval_seconds,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "probe_interval_post_failure_seconds",
                    Probe::has_probe_interval_post_failure_seconds,
                    Probe::get_probe_interval_post_failure_seconds,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "attempts_to_declare_failure",
                    Probe::has_attempts_to_declare_failure,
                    Probe::get_attempts_to_declare_failure,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "protocol",
                    Probe::has_protocol,
                    Probe::get_protocol,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "domain",
                    Probe::has_domain,
                    Probe::get_domain,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "port",
                    Probe::has_port,
                    Probe::get_port,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "url_suffix",
                    Probe::has_url_suffix,
                    Probe::get_url_suffix,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "follow_redirect",
                    Probe::has_follow_redirect,
                    Probe::get_follow_redirect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Probe>(
                    "Probe",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Probe {
    fn clear(&mut self) {
        self.clear_probe_id();
        self.clear_probe_interval_seconds();
        self.clear_probe_interval_post_failure_seconds();
        self.clear_attempts_to_declare_failure();
        self.clear_protocol();
        self.clear_domain();
        self.clear_port();
        self.clear_url_suffix();
        self.clear_follow_redirect();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Probe {
    fn eq(&self, other: &Probe) -> bool {
        self.probe_id == other.probe_id &&
        self.probe_interval_seconds == other.probe_interval_seconds &&
        self.probe_interval_post_failure_seconds == other.probe_interval_post_failure_seconds &&
        self.attempts_to_declare_failure == other.attempts_to_declare_failure &&
        self.protocol == other.protocol &&
        self.domain == other.domain &&
        self.port == other.port &&
        self.url_suffix == other.url_suffix &&
        self.follow_redirect == other.follow_redirect &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Probe {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ProbeResult {
    // message fields
    probe_id: ::std::option::Option<u64>,
    prober_hostname: ::protobuf::SingularField<::std::string::String>,
    protocol: ::std::option::Option<Protocol>,
    timestamp_sec: ::std::option::Option<i64>,
    success: ::std::option::Option<bool>,
    error_message: ::protobuf::SingularField<::std::string::String>,
    application_layer_latency_nanosec: ::std::option::Option<i64>,
    dns_answer: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    http_status_code: ::std::option::Option<i32>,
    http_status_message: ::protobuf::SingularField<::std::string::String>,
    application_bytes_received: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ProbeResult {}

impl ProbeResult {
    pub fn new() -> ProbeResult {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ProbeResult {
        static mut instance: ::protobuf::lazy::Lazy<ProbeResult> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ProbeResult,
        };
        unsafe {
            instance.get(|| {
                ProbeResult {
                    probe_id: ::std::option::Option::None,
                    prober_hostname: ::protobuf::SingularField::none(),
                    protocol: ::std::option::Option::None,
                    timestamp_sec: ::std::option::Option::None,
                    success: ::std::option::Option::None,
                    error_message: ::protobuf::SingularField::none(),
                    application_layer_latency_nanosec: ::std::option::Option::None,
                    dns_answer: ::protobuf::RepeatedField::new(),
                    http_status_code: ::std::option::Option::None,
                    http_status_message: ::protobuf::SingularField::none(),
                    application_bytes_received: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 probe_id = 1;

    pub fn clear_probe_id(&mut self) {
        self.probe_id = ::std::option::Option::None;
    }

    pub fn has_probe_id(&self) -> bool {
        self.probe_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_probe_id(&mut self, v: u64) {
        self.probe_id = ::std::option::Option::Some(v);
    }

    pub fn get_probe_id(&self) -> u64 {
        self.probe_id.unwrap_or(0)
    }

    // optional string prober_hostname = 2;

    pub fn clear_prober_hostname(&mut self) {
        self.prober_hostname.clear();
    }

    pub fn has_prober_hostname(&self) -> bool {
        self.prober_hostname.is_some()
    }

    // Param is passed by value, moved
    pub fn set_prober_hostname(&mut self, v: ::std::string::String) {
        self.prober_hostname = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_prober_hostname(&mut self) -> &mut ::std::string::String {
        if self.prober_hostname.is_none() {
            self.prober_hostname.set_default();
        };
        self.prober_hostname.as_mut().unwrap()
    }

    // Take field
    pub fn take_prober_hostname(&mut self) -> ::std::string::String {
        self.prober_hostname.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_prober_hostname(&self) -> &str {
        match self.prober_hostname.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .Protocol protocol = 3;

    pub fn clear_protocol(&mut self) {
        self.protocol = ::std::option::Option::None;
    }

    pub fn has_protocol(&self) -> bool {
        self.protocol.is_some()
    }

    // Param is passed by value, moved
    pub fn set_protocol(&mut self, v: Protocol) {
        self.protocol = ::std::option::Option::Some(v);
    }

    pub fn get_protocol(&self) -> Protocol {
        self.protocol.unwrap_or(Protocol::DNS)
    }

    // optional int64 timestamp_sec = 4;

    pub fn clear_timestamp_sec(&mut self) {
        self.timestamp_sec = ::std::option::Option::None;
    }

    pub fn has_timestamp_sec(&self) -> bool {
        self.timestamp_sec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp_sec(&mut self, v: i64) {
        self.timestamp_sec = ::std::option::Option::Some(v);
    }

    pub fn get_timestamp_sec(&self) -> i64 {
        self.timestamp_sec.unwrap_or(0)
    }

    // optional bool success = 5;

    pub fn clear_success(&mut self) {
        self.success = ::std::option::Option::None;
    }

    pub fn has_success(&self) -> bool {
        self.success.is_some()
    }

    // Param is passed by value, moved
    pub fn set_success(&mut self, v: bool) {
        self.success = ::std::option::Option::Some(v);
    }

    pub fn get_success(&self) -> bool {
        self.success.unwrap_or(false)
    }

    // optional string error_message = 6;

    pub fn clear_error_message(&mut self) {
        self.error_message.clear();
    }

    pub fn has_error_message(&self) -> bool {
        self.error_message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error_message(&mut self, v: ::std::string::String) {
        self.error_message = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error_message(&mut self) -> &mut ::std::string::String {
        if self.error_message.is_none() {
            self.error_message.set_default();
        };
        self.error_message.as_mut().unwrap()
    }

    // Take field
    pub fn take_error_message(&mut self) -> ::std::string::String {
        self.error_message.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_error_message(&self) -> &str {
        match self.error_message.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional int64 application_layer_latency_nanosec = 7;

    pub fn clear_application_layer_latency_nanosec(&mut self) {
        self.application_layer_latency_nanosec = ::std::option::Option::None;
    }

    pub fn has_application_layer_latency_nanosec(&self) -> bool {
        self.application_layer_latency_nanosec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_application_layer_latency_nanosec(&mut self, v: i64) {
        self.application_layer_latency_nanosec = ::std::option::Option::Some(v);
    }

    pub fn get_application_layer_latency_nanosec(&self) -> i64 {
        self.application_layer_latency_nanosec.unwrap_or(0)
    }

    // repeated bytes dns_answer = 8;

    pub fn clear_dns_answer(&mut self) {
        self.dns_answer.clear();
    }

    // Param is passed by value, moved
    pub fn set_dns_answer(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.dns_answer = v;
    }

    // Mutable pointer to the field.
    pub fn mut_dns_answer(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.dns_answer
    }

    // Take field
    pub fn take_dns_answer(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.dns_answer, ::protobuf::RepeatedField::new())
    }

    pub fn get_dns_answer(&self) -> &[::std::vec::Vec<u8>] {
        &self.dns_answer
    }

    // optional int32 http_status_code = 9;

    pub fn clear_http_status_code(&mut self) {
        self.http_status_code = ::std::option::Option::None;
    }

    pub fn has_http_status_code(&self) -> bool {
        self.http_status_code.is_some()
    }

    // Param is passed by value, moved
    pub fn set_http_status_code(&mut self, v: i32) {
        self.http_status_code = ::std::option::Option::Some(v);
    }

    pub fn get_http_status_code(&self) -> i32 {
        self.http_status_code.unwrap_or(0)
    }

    // optional string http_status_message = 10;

    pub fn clear_http_status_message(&mut self) {
        self.http_status_message.clear();
    }

    pub fn has_http_status_message(&self) -> bool {
        self.http_status_message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_http_status_message(&mut self, v: ::std::string::String) {
        self.http_status_message = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_http_status_message(&mut self) -> &mut ::std::string::String {
        if self.http_status_message.is_none() {
            self.http_status_message.set_default();
        };
        self.http_status_message.as_mut().unwrap()
    }

    // Take field
    pub fn take_http_status_message(&mut self) -> ::std::string::String {
        self.http_status_message.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_http_status_message(&self) -> &str {
        match self.http_status_message.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional int32 application_bytes_received = 11;

    pub fn clear_application_bytes_received(&mut self) {
        self.application_bytes_received = ::std::option::Option::None;
    }

    pub fn has_application_bytes_received(&self) -> bool {
        self.application_bytes_received.is_some()
    }

    // Param is passed by value, moved
    pub fn set_application_bytes_received(&mut self, v: i32) {
        self.application_bytes_received = ::std::option::Option::Some(v);
    }

    pub fn get_application_bytes_received(&self) -> i32 {
        self.application_bytes_received.unwrap_or(0)
    }
}

impl ::protobuf::Message for ProbeResult {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.probe_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.prober_hostname));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.protocol = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.timestamp_sec = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.success = ::std::option::Option::Some(tmp);
                },
                6 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.error_message));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.application_layer_latency_nanosec = ::std::option::Option::Some(tmp);
                },
                8 => {
                    try!(::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.dns_answer));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.http_status_code = ::std::option::Option::Some(tmp);
                },
                10 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.http_status_message));
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.application_bytes_received = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.probe_id {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.prober_hostname {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.protocol {
            my_size += ::protobuf::rt::enum_size(3, *value);
        };
        for value in &self.timestamp_sec {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.success.is_some() {
            my_size += 2;
        };
        for value in &self.error_message {
            my_size += ::protobuf::rt::string_size(6, &value);
        };
        for value in &self.application_layer_latency_nanosec {
            my_size += ::protobuf::rt::value_size(7, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.dns_answer {
            my_size += ::protobuf::rt::bytes_size(8, &value);
        };
        for value in &self.http_status_code {
            my_size += ::protobuf::rt::value_size(9, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.http_status_message {
            my_size += ::protobuf::rt::string_size(10, &value);
        };
        for value in &self.application_bytes_received {
            my_size += ::protobuf::rt::value_size(11, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.probe_id {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.prober_hostname.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.protocol {
            try!(os.write_enum(3, v.value()));
        };
        if let Some(v) = self.timestamp_sec {
            try!(os.write_int64(4, v));
        };
        if let Some(v) = self.success {
            try!(os.write_bool(5, v));
        };
        if let Some(v) = self.error_message.as_ref() {
            try!(os.write_string(6, &v));
        };
        if let Some(v) = self.application_layer_latency_nanosec {
            try!(os.write_int64(7, v));
        };
        for v in &self.dns_answer {
            try!(os.write_bytes(8, &v));
        };
        if let Some(v) = self.http_status_code {
            try!(os.write_int32(9, v));
        };
        if let Some(v) = self.http_status_message.as_ref() {
            try!(os.write_string(10, &v));
        };
        if let Some(v) = self.application_bytes_received {
            try!(os.write_int32(11, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ProbeResult>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ProbeResult {
    fn new() -> ProbeResult {
        ProbeResult::new()
    }

    fn descriptor_static(_: ::std::option::Option<ProbeResult>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "probe_id",
                    ProbeResult::has_probe_id,
                    ProbeResult::get_probe_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "prober_hostname",
                    ProbeResult::has_prober_hostname,
                    ProbeResult::get_prober_hostname,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "protocol",
                    ProbeResult::has_protocol,
                    ProbeResult::get_protocol,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "timestamp_sec",
                    ProbeResult::has_timestamp_sec,
                    ProbeResult::get_timestamp_sec,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "success",
                    ProbeResult::has_success,
                    ProbeResult::get_success,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "error_message",
                    ProbeResult::has_error_message,
                    ProbeResult::get_error_message,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "application_layer_latency_nanosec",
                    ProbeResult::has_application_layer_latency_nanosec,
                    ProbeResult::get_application_layer_latency_nanosec,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_bytes_accessor(
                    "dns_answer",
                    ProbeResult::get_dns_answer,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "http_status_code",
                    ProbeResult::has_http_status_code,
                    ProbeResult::get_http_status_code,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "http_status_message",
                    ProbeResult::has_http_status_message,
                    ProbeResult::get_http_status_message,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "application_bytes_received",
                    ProbeResult::has_application_bytes_received,
                    ProbeResult::get_application_bytes_received,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ProbeResult>(
                    "ProbeResult",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ProbeResult {
    fn clear(&mut self) {
        self.clear_probe_id();
        self.clear_prober_hostname();
        self.clear_protocol();
        self.clear_timestamp_sec();
        self.clear_success();
        self.clear_error_message();
        self.clear_application_layer_latency_nanosec();
        self.clear_dns_answer();
        self.clear_http_status_code();
        self.clear_http_status_message();
        self.clear_application_bytes_received();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ProbeResult {
    fn eq(&self, other: &ProbeResult) -> bool {
        self.probe_id == other.probe_id &&
        self.prober_hostname == other.prober_hostname &&
        self.protocol == other.protocol &&
        self.timestamp_sec == other.timestamp_sec &&
        self.success == other.success &&
        self.error_message == other.error_message &&
        self.application_layer_latency_nanosec == other.application_layer_latency_nanosec &&
        self.dns_answer == other.dns_answer &&
        self.http_status_code == other.http_status_code &&
        self.http_status_message == other.http_status_message &&
        self.application_bytes_received == other.application_bytes_received &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ProbeResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CancelProbeRequest {
    // message fields
    domain: ::protobuf::SingularField<::std::string::String>,
    protocol: ::std::vec::Vec<Protocol>,
    url_suffix: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CancelProbeRequest {}

impl CancelProbeRequest {
    pub fn new() -> CancelProbeRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CancelProbeRequest {
        static mut instance: ::protobuf::lazy::Lazy<CancelProbeRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CancelProbeRequest,
        };
        unsafe {
            instance.get(|| {
                CancelProbeRequest {
                    domain: ::protobuf::SingularField::none(),
                    protocol: ::std::vec::Vec::new(),
                    url_suffix: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string domain = 1;

    pub fn clear_domain(&mut self) {
        self.domain.clear();
    }

    pub fn has_domain(&self) -> bool {
        self.domain.is_some()
    }

    // Param is passed by value, moved
    pub fn set_domain(&mut self, v: ::std::string::String) {
        self.domain = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_domain(&mut self) -> &mut ::std::string::String {
        if self.domain.is_none() {
            self.domain.set_default();
        };
        self.domain.as_mut().unwrap()
    }

    // Take field
    pub fn take_domain(&mut self) -> ::std::string::String {
        self.domain.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_domain(&self) -> &str {
        match self.domain.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // repeated .Protocol protocol = 2;

    pub fn clear_protocol(&mut self) {
        self.protocol.clear();
    }

    // Param is passed by value, moved
    pub fn set_protocol(&mut self, v: ::std::vec::Vec<Protocol>) {
        self.protocol = v;
    }

    // Mutable pointer to the field.
    pub fn mut_protocol(&mut self) -> &mut ::std::vec::Vec<Protocol> {
        &mut self.protocol
    }

    // Take field
    pub fn take_protocol(&mut self) -> ::std::vec::Vec<Protocol> {
        ::std::mem::replace(&mut self.protocol, ::std::vec::Vec::new())
    }

    pub fn get_protocol(&self) -> &[Protocol] {
        &self.protocol
    }

    // optional string url_suffix = 3;

    pub fn clear_url_suffix(&mut self) {
        self.url_suffix.clear();
    }

    pub fn has_url_suffix(&self) -> bool {
        self.url_suffix.is_some()
    }

    // Param is passed by value, moved
    pub fn set_url_suffix(&mut self, v: ::std::string::String) {
        self.url_suffix = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_url_suffix(&mut self) -> &mut ::std::string::String {
        if self.url_suffix.is_none() {
            self.url_suffix.set_default();
        };
        self.url_suffix.as_mut().unwrap()
    }

    // Take field
    pub fn take_url_suffix(&mut self) -> ::std::string::String {
        self.url_suffix.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_url_suffix(&self) -> &str {
        match self.url_suffix.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for CancelProbeRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.domain));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.protocol));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.url_suffix));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.domain {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.protocol {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in &self.url_suffix {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.domain.as_ref() {
            try!(os.write_string(1, &v));
        };
        for v in &self.protocol {
            try!(os.write_enum(2, v.value()));
        };
        if let Some(v) = self.url_suffix.as_ref() {
            try!(os.write_string(3, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<CancelProbeRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CancelProbeRequest {
    fn new() -> CancelProbeRequest {
        CancelProbeRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CancelProbeRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "domain",
                    CancelProbeRequest::has_domain,
                    CancelProbeRequest::get_domain,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_enum_accessor(
                    "protocol",
                    CancelProbeRequest::get_protocol,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "url_suffix",
                    CancelProbeRequest::has_url_suffix,
                    CancelProbeRequest::get_url_suffix,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CancelProbeRequest>(
                    "CancelProbeRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CancelProbeRequest {
    fn clear(&mut self) {
        self.clear_domain();
        self.clear_protocol();
        self.clear_url_suffix();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CancelProbeRequest {
    fn eq(&self, other: &CancelProbeRequest) -> bool {
        self.domain == other.domain &&
        self.protocol == other.protocol &&
        self.url_suffix == other.url_suffix &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CancelProbeRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CancelProbeReply {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CancelProbeReply {}

impl CancelProbeReply {
    pub fn new() -> CancelProbeReply {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CancelProbeReply {
        static mut instance: ::protobuf::lazy::Lazy<CancelProbeReply> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CancelProbeReply,
        };
        unsafe {
            instance.get(|| {
                CancelProbeReply {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for CancelProbeReply {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<CancelProbeReply>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CancelProbeReply {
    fn new() -> CancelProbeReply {
        CancelProbeReply::new()
    }

    fn descriptor_static(_: ::std::option::Option<CancelProbeReply>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CancelProbeReply>(
                    "CancelProbeReply",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CancelProbeReply {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CancelProbeReply {
    fn eq(&self, other: &CancelProbeReply) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CancelProbeReply {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SearchRequest {
    // message fields
    protocol: ::std::vec::Vec<Protocol>,
    domain: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SearchRequest {}

impl SearchRequest {
    pub fn new() -> SearchRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SearchRequest {
        static mut instance: ::protobuf::lazy::Lazy<SearchRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SearchRequest,
        };
        unsafe {
            instance.get(|| {
                SearchRequest {
                    protocol: ::std::vec::Vec::new(),
                    domain: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .Protocol protocol = 1;

    pub fn clear_protocol(&mut self) {
        self.protocol.clear();
    }

    // Param is passed by value, moved
    pub fn set_protocol(&mut self, v: ::std::vec::Vec<Protocol>) {
        self.protocol = v;
    }

    // Mutable pointer to the field.
    pub fn mut_protocol(&mut self) -> &mut ::std::vec::Vec<Protocol> {
        &mut self.protocol
    }

    // Take field
    pub fn take_protocol(&mut self) -> ::std::vec::Vec<Protocol> {
        ::std::mem::replace(&mut self.protocol, ::std::vec::Vec::new())
    }

    pub fn get_protocol(&self) -> &[Protocol] {
        &self.protocol
    }

    // optional string domain = 2;

    pub fn clear_domain(&mut self) {
        self.domain.clear();
    }

    pub fn has_domain(&self) -> bool {
        self.domain.is_some()
    }

    // Param is passed by value, moved
    pub fn set_domain(&mut self, v: ::std::string::String) {
        self.domain = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_domain(&mut self) -> &mut ::std::string::String {
        if self.domain.is_none() {
            self.domain.set_default();
        };
        self.domain.as_mut().unwrap()
    }

    // Take field
    pub fn take_domain(&mut self) -> ::std::string::String {
        self.domain.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_domain(&self) -> &str {
        match self.domain.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for SearchRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.protocol));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.domain));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.protocol {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.domain {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.protocol {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.domain.as_ref() {
            try!(os.write_string(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<SearchRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SearchRequest {
    fn new() -> SearchRequest {
        SearchRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<SearchRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_enum_accessor(
                    "protocol",
                    SearchRequest::get_protocol,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "domain",
                    SearchRequest::has_domain,
                    SearchRequest::get_domain,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SearchRequest>(
                    "SearchRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SearchRequest {
    fn clear(&mut self) {
        self.clear_protocol();
        self.clear_domain();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SearchRequest {
    fn eq(&self, other: &SearchRequest) -> bool {
        self.protocol == other.protocol &&
        self.domain == other.domain &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SearchRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SearchReply {
    // message fields
    probe: ::protobuf::RepeatedField<Probe>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SearchReply {}

impl SearchReply {
    pub fn new() -> SearchReply {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SearchReply {
        static mut instance: ::protobuf::lazy::Lazy<SearchReply> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SearchReply,
        };
        unsafe {
            instance.get(|| {
                SearchReply {
                    probe: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .Probe probe = 1;

    pub fn clear_probe(&mut self) {
        self.probe.clear();
    }

    // Param is passed by value, moved
    pub fn set_probe(&mut self, v: ::protobuf::RepeatedField<Probe>) {
        self.probe = v;
    }

    // Mutable pointer to the field.
    pub fn mut_probe(&mut self) -> &mut ::protobuf::RepeatedField<Probe> {
        &mut self.probe
    }

    // Take field
    pub fn take_probe(&mut self) -> ::protobuf::RepeatedField<Probe> {
        ::std::mem::replace(&mut self.probe, ::protobuf::RepeatedField::new())
    }

    pub fn get_probe(&self) -> &[Probe] {
        &self.probe
    }
}

impl ::protobuf::Message for SearchReply {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.probe));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.probe {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.probe {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<SearchReply>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SearchReply {
    fn new() -> SearchReply {
        SearchReply::new()
    }

    fn descriptor_static(_: ::std::option::Option<SearchReply>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "probe",
                    SearchReply::get_probe,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SearchReply>(
                    "SearchReply",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SearchReply {
    fn clear(&mut self) {
        self.clear_probe();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SearchReply {
    fn eq(&self, other: &SearchReply) -> bool {
        self.probe == other.probe &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SearchReply {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ScheduleProbeRequest {
    // message fields
    probe: ::protobuf::RepeatedField<Probe>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ScheduleProbeRequest {}

impl ScheduleProbeRequest {
    pub fn new() -> ScheduleProbeRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ScheduleProbeRequest {
        static mut instance: ::protobuf::lazy::Lazy<ScheduleProbeRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ScheduleProbeRequest,
        };
        unsafe {
            instance.get(|| {
                ScheduleProbeRequest {
                    probe: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .Probe probe = 2;

    pub fn clear_probe(&mut self) {
        self.probe.clear();
    }

    // Param is passed by value, moved
    pub fn set_probe(&mut self, v: ::protobuf::RepeatedField<Probe>) {
        self.probe = v;
    }

    // Mutable pointer to the field.
    pub fn mut_probe(&mut self) -> &mut ::protobuf::RepeatedField<Probe> {
        &mut self.probe
    }

    // Take field
    pub fn take_probe(&mut self) -> ::protobuf::RepeatedField<Probe> {
        ::std::mem::replace(&mut self.probe, ::protobuf::RepeatedField::new())
    }

    pub fn get_probe(&self) -> &[Probe] {
        &self.probe
    }
}

impl ::protobuf::Message for ScheduleProbeRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.probe));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.probe {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.probe {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ScheduleProbeRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ScheduleProbeRequest {
    fn new() -> ScheduleProbeRequest {
        ScheduleProbeRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ScheduleProbeRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "probe",
                    ScheduleProbeRequest::get_probe,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ScheduleProbeRequest>(
                    "ScheduleProbeRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ScheduleProbeRequest {
    fn clear(&mut self) {
        self.clear_probe();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ScheduleProbeRequest {
    fn eq(&self, other: &ScheduleProbeRequest) -> bool {
        self.probe == other.probe &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ScheduleProbeRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ScheduleProbeReply {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ScheduleProbeReply {}

impl ScheduleProbeReply {
    pub fn new() -> ScheduleProbeReply {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ScheduleProbeReply {
        static mut instance: ::protobuf::lazy::Lazy<ScheduleProbeReply> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ScheduleProbeReply,
        };
        unsafe {
            instance.get(|| {
                ScheduleProbeReply {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for ScheduleProbeReply {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ScheduleProbeReply>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ScheduleProbeReply {
    fn new() -> ScheduleProbeReply {
        ScheduleProbeReply::new()
    }

    fn descriptor_static(_: ::std::option::Option<ScheduleProbeReply>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ScheduleProbeReply>(
                    "ScheduleProbeReply",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ScheduleProbeReply {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ScheduleProbeReply {
    fn eq(&self, other: &ScheduleProbeReply) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ScheduleProbeReply {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct BucketHash {
    // message fields
    bucket_key: ::std::option::Option<u64>,
    hash: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BucketHash {}

impl BucketHash {
    pub fn new() -> BucketHash {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BucketHash {
        static mut instance: ::protobuf::lazy::Lazy<BucketHash> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BucketHash,
        };
        unsafe {
            instance.get(|| {
                BucketHash {
                    bucket_key: ::std::option::Option::None,
                    hash: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 bucket_key = 1;

    pub fn clear_bucket_key(&mut self) {
        self.bucket_key = ::std::option::Option::None;
    }

    pub fn has_bucket_key(&self) -> bool {
        self.bucket_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bucket_key(&mut self, v: u64) {
        self.bucket_key = ::std::option::Option::Some(v);
    }

    pub fn get_bucket_key(&self) -> u64 {
        self.bucket_key.unwrap_or(0)
    }

    // optional uint64 hash = 2;

    pub fn clear_hash(&mut self) {
        self.hash = ::std::option::Option::None;
    }

    pub fn has_hash(&self) -> bool {
        self.hash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hash(&mut self, v: u64) {
        self.hash = ::std::option::Option::Some(v);
    }

    pub fn get_hash(&self) -> u64 {
        self.hash.unwrap_or(0)
    }
}

impl ::protobuf::Message for BucketHash {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.bucket_key = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.hash = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.bucket_key {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.hash {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.bucket_key {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.hash {
            try!(os.write_uint64(2, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<BucketHash>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BucketHash {
    fn new() -> BucketHash {
        BucketHash::new()
    }

    fn descriptor_static(_: ::std::option::Option<BucketHash>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "bucket_key",
                    BucketHash::has_bucket_key,
                    BucketHash::get_bucket_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "hash",
                    BucketHash::has_hash,
                    BucketHash::get_hash,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BucketHash>(
                    "BucketHash",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BucketHash {
    fn clear(&mut self) {
        self.clear_bucket_key();
        self.clear_hash();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for BucketHash {
    fn eq(&self, other: &BucketHash) -> bool {
        self.bucket_key == other.bucket_key &&
        self.hash == other.hash &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for BucketHash {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct BucketProbes {
    // message fields
    bucket_key: ::std::option::Option<u64>,
    probe: ::protobuf::RepeatedField<Probe>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BucketProbes {}

impl BucketProbes {
    pub fn new() -> BucketProbes {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BucketProbes {
        static mut instance: ::protobuf::lazy::Lazy<BucketProbes> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BucketProbes,
        };
        unsafe {
            instance.get(|| {
                BucketProbes {
                    bucket_key: ::std::option::Option::None,
                    probe: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 bucket_key = 1;

    pub fn clear_bucket_key(&mut self) {
        self.bucket_key = ::std::option::Option::None;
    }

    pub fn has_bucket_key(&self) -> bool {
        self.bucket_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bucket_key(&mut self, v: u64) {
        self.bucket_key = ::std::option::Option::Some(v);
    }

    pub fn get_bucket_key(&self) -> u64 {
        self.bucket_key.unwrap_or(0)
    }

    // repeated .Probe probe = 2;

    pub fn clear_probe(&mut self) {
        self.probe.clear();
    }

    // Param is passed by value, moved
    pub fn set_probe(&mut self, v: ::protobuf::RepeatedField<Probe>) {
        self.probe = v;
    }

    // Mutable pointer to the field.
    pub fn mut_probe(&mut self) -> &mut ::protobuf::RepeatedField<Probe> {
        &mut self.probe
    }

    // Take field
    pub fn take_probe(&mut self) -> ::protobuf::RepeatedField<Probe> {
        ::std::mem::replace(&mut self.probe, ::protobuf::RepeatedField::new())
    }

    pub fn get_probe(&self) -> &[Probe] {
        &self.probe
    }
}

impl ::protobuf::Message for BucketProbes {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.bucket_key = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.probe));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.bucket_key {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.probe {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.bucket_key {
            try!(os.write_uint64(1, v));
        };
        for v in &self.probe {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<BucketProbes>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BucketProbes {
    fn new() -> BucketProbes {
        BucketProbes::new()
    }

    fn descriptor_static(_: ::std::option::Option<BucketProbes>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "bucket_key",
                    BucketProbes::has_bucket_key,
                    BucketProbes::get_bucket_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "probe",
                    BucketProbes::get_probe,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BucketProbes>(
                    "BucketProbes",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BucketProbes {
    fn clear(&mut self) {
        self.clear_bucket_key();
        self.clear_probe();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for BucketProbes {
    fn eq(&self, other: &BucketProbes) -> bool {
        self.bucket_key == other.bucket_key &&
        self.probe == other.probe &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for BucketProbes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetProbesRequest {
    // message fields
    bucket_hash: ::protobuf::RepeatedField<BucketHash>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetProbesRequest {}

impl GetProbesRequest {
    pub fn new() -> GetProbesRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetProbesRequest {
        static mut instance: ::protobuf::lazy::Lazy<GetProbesRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetProbesRequest,
        };
        unsafe {
            instance.get(|| {
                GetProbesRequest {
                    bucket_hash: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .BucketHash bucket_hash = 1;

    pub fn clear_bucket_hash(&mut self) {
        self.bucket_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_bucket_hash(&mut self, v: ::protobuf::RepeatedField<BucketHash>) {
        self.bucket_hash = v;
    }

    // Mutable pointer to the field.
    pub fn mut_bucket_hash(&mut self) -> &mut ::protobuf::RepeatedField<BucketHash> {
        &mut self.bucket_hash
    }

    // Take field
    pub fn take_bucket_hash(&mut self) -> ::protobuf::RepeatedField<BucketHash> {
        ::std::mem::replace(&mut self.bucket_hash, ::protobuf::RepeatedField::new())
    }

    pub fn get_bucket_hash(&self) -> &[BucketHash] {
        &self.bucket_hash
    }
}

impl ::protobuf::Message for GetProbesRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.bucket_hash));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.bucket_hash {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.bucket_hash {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<GetProbesRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetProbesRequest {
    fn new() -> GetProbesRequest {
        GetProbesRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetProbesRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "bucket_hash",
                    GetProbesRequest::get_bucket_hash,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetProbesRequest>(
                    "GetProbesRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetProbesRequest {
    fn clear(&mut self) {
        self.clear_bucket_hash();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetProbesRequest {
    fn eq(&self, other: &GetProbesRequest) -> bool {
        self.bucket_hash == other.bucket_hash &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetProbesRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetProbesReply {
    // message fields
    bucket_probes: ::protobuf::RepeatedField<BucketProbes>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetProbesReply {}

impl GetProbesReply {
    pub fn new() -> GetProbesReply {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetProbesReply {
        static mut instance: ::protobuf::lazy::Lazy<GetProbesReply> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetProbesReply,
        };
        unsafe {
            instance.get(|| {
                GetProbesReply {
                    bucket_probes: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .BucketProbes bucket_probes = 1;

    pub fn clear_bucket_probes(&mut self) {
        self.bucket_probes.clear();
    }

    // Param is passed by value, moved
    pub fn set_bucket_probes(&mut self, v: ::protobuf::RepeatedField<BucketProbes>) {
        self.bucket_probes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_bucket_probes(&mut self) -> &mut ::protobuf::RepeatedField<BucketProbes> {
        &mut self.bucket_probes
    }

    // Take field
    pub fn take_bucket_probes(&mut self) -> ::protobuf::RepeatedField<BucketProbes> {
        ::std::mem::replace(&mut self.bucket_probes, ::protobuf::RepeatedField::new())
    }

    pub fn get_bucket_probes(&self) -> &[BucketProbes] {
        &self.bucket_probes
    }
}

impl ::protobuf::Message for GetProbesReply {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.bucket_probes));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.bucket_probes {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.bucket_probes {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<GetProbesReply>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetProbesReply {
    fn new() -> GetProbesReply {
        GetProbesReply::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetProbesReply>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "bucket_probes",
                    GetProbesReply::get_bucket_probes,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetProbesReply>(
                    "GetProbesReply",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetProbesReply {
    fn clear(&mut self) {
        self.clear_bucket_probes();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetProbesReply {
    fn eq(&self, other: &GetProbesReply) -> bool {
        self.bucket_probes == other.bucket_probes &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetProbesReply {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetBucketKeysRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetBucketKeysRequest {}

impl GetBucketKeysRequest {
    pub fn new() -> GetBucketKeysRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetBucketKeysRequest {
        static mut instance: ::protobuf::lazy::Lazy<GetBucketKeysRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetBucketKeysRequest,
        };
        unsafe {
            instance.get(|| {
                GetBucketKeysRequest {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for GetBucketKeysRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<GetBucketKeysRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetBucketKeysRequest {
    fn new() -> GetBucketKeysRequest {
        GetBucketKeysRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetBucketKeysRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<GetBucketKeysRequest>(
                    "GetBucketKeysRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetBucketKeysRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetBucketKeysRequest {
    fn eq(&self, other: &GetBucketKeysRequest) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetBucketKeysRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetBucketKeysReply {
    // message fields
    bucket_key: ::std::vec::Vec<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetBucketKeysReply {}

impl GetBucketKeysReply {
    pub fn new() -> GetBucketKeysReply {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetBucketKeysReply {
        static mut instance: ::protobuf::lazy::Lazy<GetBucketKeysReply> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetBucketKeysReply,
        };
        unsafe {
            instance.get(|| {
                GetBucketKeysReply {
                    bucket_key: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated uint64 bucket_key = 1;

    pub fn clear_bucket_key(&mut self) {
        self.bucket_key.clear();
    }

    // Param is passed by value, moved
    pub fn set_bucket_key(&mut self, v: ::std::vec::Vec<u64>) {
        self.bucket_key = v;
    }

    // Mutable pointer to the field.
    pub fn mut_bucket_key(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.bucket_key
    }

    // Take field
    pub fn take_bucket_key(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.bucket_key, ::std::vec::Vec::new())
    }

    pub fn get_bucket_key(&self) -> &[u64] {
        &self.bucket_key
    }
}

impl ::protobuf::Message for GetBucketKeysReply {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.bucket_key));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.bucket_key {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.bucket_key {
            try!(os.write_uint64(1, *v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<GetBucketKeysReply>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetBucketKeysReply {
    fn new() -> GetBucketKeysReply {
        GetBucketKeysReply::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetBucketKeysReply>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_u64_accessor(
                    "bucket_key",
                    GetBucketKeysReply::get_bucket_key,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetBucketKeysReply>(
                    "GetBucketKeysReply",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetBucketKeysReply {
    fn clear(&mut self) {
        self.clear_bucket_key();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetBucketKeysReply {
    fn eq(&self, other: &GetBucketKeysReply) -> bool {
        self.bucket_key == other.bucket_key &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetBucketKeysReply {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Protocol {
    DNS = 0,
    HTTP = 1,
    HTTPS = 2,
    PING = 3,
    TRACEROUTE = 4,
}

impl ::protobuf::ProtobufEnum for Protocol {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Protocol> {
        match value {
            0 => ::std::option::Option::Some(Protocol::DNS),
            1 => ::std::option::Option::Some(Protocol::HTTP),
            2 => ::std::option::Option::Some(Protocol::HTTPS),
            3 => ::std::option::Option::Some(Protocol::PING),
            4 => ::std::option::Option::Some(Protocol::TRACEROUTE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Protocol] = &[
            Protocol::DNS,
            Protocol::HTTP,
            Protocol::HTTPS,
            Protocol::PING,
            Protocol::TRACEROUTE,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Protocol>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Protocol", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Protocol {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x19, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x69, 0x6e, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x5f, 0x70, 0x62, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x80, 0x03, 0x0a, 0x05,
    0x50, 0x72, 0x6f, 0x62, 0x65, 0x12, 0x19, 0x0a, 0x08, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x5f, 0x69,
    0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x07, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x49, 0x64,
    0x12, 0x34, 0x0a, 0x16, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x5f, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x76,
    0x61, 0x6c, 0x5f, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05,
    0x52, 0x14, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x76, 0x61, 0x6c, 0x53,
    0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x12, 0x4c, 0x0a, 0x23, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x5f,
    0x69, 0x6e, 0x74, 0x65, 0x72, 0x76, 0x61, 0x6c, 0x5f, 0x70, 0x6f, 0x73, 0x74, 0x5f, 0x66, 0x61,
    0x69, 0x6c, 0x75, 0x72, 0x65, 0x5f, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x05, 0x52, 0x1f, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x76,
    0x61, 0x6c, 0x50, 0x6f, 0x73, 0x74, 0x46, 0x61, 0x69, 0x6c, 0x75, 0x72, 0x65, 0x53, 0x65, 0x63,
    0x6f, 0x6e, 0x64, 0x73, 0x12, 0x3d, 0x0a, 0x1b, 0x61, 0x74, 0x74, 0x65, 0x6d, 0x70, 0x74, 0x73,
    0x5f, 0x74, 0x6f, 0x5f, 0x64, 0x65, 0x63, 0x6c, 0x61, 0x72, 0x65, 0x5f, 0x66, 0x61, 0x69, 0x6c,
    0x75, 0x72, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x05, 0x52, 0x18, 0x61, 0x74, 0x74, 0x65, 0x6d,
    0x70, 0x74, 0x73, 0x54, 0x6f, 0x44, 0x65, 0x63, 0x6c, 0x61, 0x72, 0x65, 0x46, 0x61, 0x69, 0x6c,
    0x75, 0x72, 0x65, 0x12, 0x25, 0x0a, 0x08, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x18,
    0x05, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x09, 0x2e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c,
    0x52, 0x08, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x12, 0x16, 0x0a, 0x06, 0x64, 0x6f,
    0x6d, 0x61, 0x69, 0x6e, 0x18, 0x06, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x64, 0x6f, 0x6d, 0x61,
    0x69, 0x6e, 0x12, 0x12, 0x0a, 0x04, 0x70, 0x6f, 0x72, 0x74, 0x18, 0x07, 0x20, 0x01, 0x28, 0x05,
    0x52, 0x04, 0x70, 0x6f, 0x72, 0x74, 0x12, 0x1d, 0x0a, 0x0a, 0x75, 0x72, 0x6c, 0x5f, 0x73, 0x75,
    0x66, 0x66, 0x69, 0x78, 0x18, 0x08, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x75, 0x72, 0x6c, 0x53,
    0x75, 0x66, 0x66, 0x69, 0x78, 0x12, 0x27, 0x0a, 0x0f, 0x66, 0x6f, 0x6c, 0x6c, 0x6f, 0x77, 0x5f,
    0x72, 0x65, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x18, 0x09, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0e,
    0x66, 0x6f, 0x6c, 0x6c, 0x6f, 0x77, 0x52, 0x65, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x22, 0xde,
    0x03, 0x0a, 0x0b, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x19,
    0x0a, 0x08, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04,
    0x52, 0x07, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x49, 0x64, 0x12, 0x27, 0x0a, 0x0f, 0x70, 0x72, 0x6f,
    0x62, 0x65, 0x72, 0x5f, 0x68, 0x6f, 0x73, 0x74, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x0e, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x72, 0x48, 0x6f, 0x73, 0x74, 0x6e, 0x61,
    0x6d, 0x65, 0x12, 0x25, 0x0a, 0x08, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0e, 0x32, 0x09, 0x2e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x52,
    0x08, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x12, 0x23, 0x0a, 0x0d, 0x74, 0x69, 0x6d,
    0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x5f, 0x73, 0x65, 0x63, 0x18, 0x04, 0x20, 0x01, 0x28, 0x03,
    0x52, 0x0c, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x53, 0x65, 0x63, 0x12, 0x18,
    0x0a, 0x07, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x18, 0x05, 0x20, 0x01, 0x28, 0x08, 0x52,
    0x07, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x12, 0x23, 0x0a, 0x0d, 0x65, 0x72, 0x72, 0x6f,
    0x72, 0x5f, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x0c, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x49, 0x0a,
    0x21, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x6c, 0x61, 0x79,
    0x65, 0x72, 0x5f, 0x6c, 0x61, 0x74, 0x65, 0x6e, 0x63, 0x79, 0x5f, 0x6e, 0x61, 0x6e, 0x6f, 0x73,
    0x65, 0x63, 0x18, 0x07, 0x20, 0x01, 0x28, 0x03, 0x52, 0x1e, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x63,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x4c, 0x61, 0x79, 0x65, 0x72, 0x4c, 0x61, 0x74, 0x65, 0x6e, 0x63,
    0x79, 0x4e, 0x61, 0x6e, 0x6f, 0x73, 0x65, 0x63, 0x12, 0x1d, 0x0a, 0x0a, 0x64, 0x6e, 0x73, 0x5f,
    0x61, 0x6e, 0x73, 0x77, 0x65, 0x72, 0x18, 0x08, 0x20, 0x03, 0x28, 0x0c, 0x52, 0x09, 0x64, 0x6e,
    0x73, 0x41, 0x6e, 0x73, 0x77, 0x65, 0x72, 0x12, 0x28, 0x0a, 0x10, 0x68, 0x74, 0x74, 0x70, 0x5f,
    0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x5f, 0x63, 0x6f, 0x64, 0x65, 0x18, 0x09, 0x20, 0x01, 0x28,
    0x05, 0x52, 0x0e, 0x68, 0x74, 0x74, 0x70, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x43, 0x6f, 0x64,
    0x65, 0x12, 0x2e, 0x0a, 0x13, 0x68, 0x74, 0x74, 0x70, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73,
    0x5f, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x09, 0x52, 0x11,
    0x68, 0x74, 0x74, 0x70, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x12, 0x3c, 0x0a, 0x1a, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x5f, 0x62, 0x79, 0x74, 0x65, 0x73, 0x5f, 0x72, 0x65, 0x63, 0x65, 0x69, 0x76, 0x65, 0x64, 0x18,
    0x0b, 0x20, 0x01, 0x28, 0x05, 0x52, 0x18, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x42, 0x79, 0x74, 0x65, 0x73, 0x52, 0x65, 0x63, 0x65, 0x69, 0x76, 0x65, 0x64, 0x22,
    0x72, 0x0a, 0x12, 0x43, 0x61, 0x6e, 0x63, 0x65, 0x6c, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x16, 0x0a, 0x06, 0x64, 0x6f, 0x6d, 0x61, 0x69, 0x6e, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x64, 0x6f, 0x6d, 0x61, 0x69, 0x6e, 0x12, 0x25, 0x0a,
    0x08, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0e, 0x32,
    0x09, 0x2e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x52, 0x08, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x63, 0x6f, 0x6c, 0x12, 0x1d, 0x0a, 0x0a, 0x75, 0x72, 0x6c, 0x5f, 0x73, 0x75, 0x66, 0x66,
    0x69, 0x78, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x75, 0x72, 0x6c, 0x53, 0x75, 0x66,
    0x66, 0x69, 0x78, 0x22, 0x12, 0x0a, 0x10, 0x43, 0x61, 0x6e, 0x63, 0x65, 0x6c, 0x50, 0x72, 0x6f,
    0x62, 0x65, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x22, 0x4e, 0x0a, 0x0d, 0x53, 0x65, 0x61, 0x72, 0x63,
    0x68, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x25, 0x0a, 0x08, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x63, 0x6f, 0x6c, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0e, 0x32, 0x09, 0x2e, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x52, 0x08, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x12,
    0x16, 0x0a, 0x06, 0x64, 0x6f, 0x6d, 0x61, 0x69, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x06, 0x64, 0x6f, 0x6d, 0x61, 0x69, 0x6e, 0x22, 0x2b, 0x0a, 0x0b, 0x53, 0x65, 0x61, 0x72, 0x63,
    0x68, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x12, 0x1c, 0x0a, 0x05, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x18,
    0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x52, 0x05, 0x70,
    0x72, 0x6f, 0x62, 0x65, 0x22, 0x34, 0x0a, 0x14, 0x53, 0x63, 0x68, 0x65, 0x64, 0x75, 0x6c, 0x65,
    0x50, 0x72, 0x6f, 0x62, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x1c, 0x0a, 0x05,
    0x70, 0x72, 0x6f, 0x62, 0x65, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x50, 0x72,
    0x6f, 0x62, 0x65, 0x52, 0x05, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x22, 0x14, 0x0a, 0x12, 0x53, 0x63,
    0x68, 0x65, 0x64, 0x75, 0x6c, 0x65, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x52, 0x65, 0x70, 0x6c, 0x79,
    0x22, 0x3f, 0x0a, 0x0a, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x48, 0x61, 0x73, 0x68, 0x12, 0x1d,
    0x0a, 0x0a, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x04, 0x52, 0x09, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x4b, 0x65, 0x79, 0x12, 0x12, 0x0a,
    0x04, 0x68, 0x61, 0x73, 0x68, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x04, 0x68, 0x61, 0x73,
    0x68, 0x22, 0x4b, 0x0a, 0x0c, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x50, 0x72, 0x6f, 0x62, 0x65,
    0x73, 0x12, 0x1d, 0x0a, 0x0a, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x5f, 0x6b, 0x65, 0x79, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x09, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x4b, 0x65, 0x79,
    0x12, 0x1c, 0x0a, 0x05, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x06, 0x2e, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x52, 0x05, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x22, 0x40,
    0x0a, 0x10, 0x47, 0x65, 0x74, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x12, 0x2c, 0x0a, 0x0b, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x5f, 0x68, 0x61, 0x73,
    0x68, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74,
    0x48, 0x61, 0x73, 0x68, 0x52, 0x0a, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x48, 0x61, 0x73, 0x68,
    0x22, 0x44, 0x0a, 0x0e, 0x47, 0x65, 0x74, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x73, 0x52, 0x65, 0x70,
    0x6c, 0x79, 0x12, 0x32, 0x0a, 0x0d, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x5f, 0x70, 0x72, 0x6f,
    0x62, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x42, 0x75, 0x63, 0x6b,
    0x65, 0x74, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x73, 0x52, 0x0c, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74,
    0x50, 0x72, 0x6f, 0x62, 0x65, 0x73, 0x22, 0x16, 0x0a, 0x14, 0x47, 0x65, 0x74, 0x42, 0x75, 0x63,
    0x6b, 0x65, 0x74, 0x4b, 0x65, 0x79, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x22, 0x33,
    0x0a, 0x12, 0x47, 0x65, 0x74, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x4b, 0x65, 0x79, 0x73, 0x52,
    0x65, 0x70, 0x6c, 0x79, 0x12, 0x1d, 0x0a, 0x0a, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x5f, 0x6b,
    0x65, 0x79, 0x18, 0x01, 0x20, 0x03, 0x28, 0x04, 0x52, 0x09, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74,
    0x4b, 0x65, 0x79, 0x2a, 0x42, 0x0a, 0x08, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x12,
    0x07, 0x0a, 0x03, 0x44, 0x4e, 0x53, 0x10, 0x00, 0x12, 0x08, 0x0a, 0x04, 0x48, 0x54, 0x54, 0x50,
    0x10, 0x01, 0x12, 0x09, 0x0a, 0x05, 0x48, 0x54, 0x54, 0x50, 0x53, 0x10, 0x02, 0x12, 0x08, 0x0a,
    0x04, 0x50, 0x49, 0x4e, 0x47, 0x10, 0x03, 0x12, 0x0e, 0x0a, 0x0a, 0x54, 0x52, 0x41, 0x43, 0x45,
    0x52, 0x4f, 0x55, 0x54, 0x45, 0x10, 0x04, 0x32, 0xad, 0x01, 0x0a, 0x09, 0x53, 0x63, 0x68, 0x65,
    0x64, 0x75, 0x6c, 0x65, 0x72, 0x12, 0x37, 0x0a, 0x0b, 0x43, 0x61, 0x6e, 0x63, 0x65, 0x6c, 0x50,
    0x72, 0x6f, 0x62, 0x65, 0x12, 0x13, 0x2e, 0x43, 0x61, 0x6e, 0x63, 0x65, 0x6c, 0x50, 0x72, 0x6f,
    0x62, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x11, 0x2e, 0x43, 0x61, 0x6e, 0x63,
    0x65, 0x6c, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x22, 0x00, 0x12, 0x28,
    0x0a, 0x06, 0x53, 0x65, 0x61, 0x72, 0x63, 0x68, 0x12, 0x0e, 0x2e, 0x53, 0x65, 0x61, 0x72, 0x63,
    0x68, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x0c, 0x2e, 0x53, 0x65, 0x61, 0x72, 0x63,
    0x68, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x22, 0x00, 0x12, 0x3d, 0x0a, 0x0d, 0x53, 0x63, 0x68, 0x65,
    0x64, 0x75, 0x6c, 0x65, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x12, 0x15, 0x2e, 0x53, 0x63, 0x68, 0x65,
    0x64, 0x75, 0x6c, 0x65, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x1a, 0x13, 0x2e, 0x53, 0x63, 0x68, 0x65, 0x64, 0x75, 0x6c, 0x65, 0x50, 0x72, 0x6f, 0x62, 0x65,
    0x52, 0x65, 0x70, 0x6c, 0x79, 0x22, 0x00, 0x32, 0x7e, 0x0a, 0x0a, 0x50, 0x72, 0x6f, 0x62, 0x65,
    0x43, 0x61, 0x63, 0x68, 0x65, 0x12, 0x31, 0x0a, 0x09, 0x47, 0x65, 0x74, 0x50, 0x72, 0x6f, 0x62,
    0x65, 0x73, 0x12, 0x11, 0x2e, 0x47, 0x65, 0x74, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x73, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x0f, 0x2e, 0x47, 0x65, 0x74, 0x50, 0x72, 0x6f, 0x62, 0x65,
    0x73, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x22, 0x00, 0x12, 0x3d, 0x0a, 0x0d, 0x47, 0x65, 0x74, 0x42,
    0x75, 0x63, 0x6b, 0x65, 0x74, 0x4b, 0x65, 0x79, 0x73, 0x12, 0x15, 0x2e, 0x47, 0x65, 0x74, 0x42,
    0x75, 0x63, 0x6b, 0x65, 0x74, 0x4b, 0x65, 0x79, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x1a, 0x13, 0x2e, 0x47, 0x65, 0x74, 0x42, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x4b, 0x65, 0x79, 0x73,
    0x52, 0x65, 0x70, 0x6c, 0x79, 0x22, 0x00, 0x4a, 0x80, 0x1b, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00,
    0x7c, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x10, 0x0a, 0x1e, 0x0a, 0x02,
    0x05, 0x00, 0x12, 0x04, 0x05, 0x00, 0x0b, 0x01, 0x1a, 0x12, 0x2a, 0x0a, 0x20, 0x70, 0x72, 0x6f,
    0x62, 0x65, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x0a, 0x0a, 0x0a, 0x0a, 0x03,
    0x05, 0x00, 0x01, 0x12, 0x03, 0x05, 0x05, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x06, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x06, 0x04, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x06, 0x0a,
    0x0b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x07, 0x04, 0x0d, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x07, 0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x07, 0x0b, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x02, 0x12, 0x03, 0x08, 0x04, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x08, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03,
    0x08, 0x0c, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x03, 0x12, 0x03, 0x09, 0x04, 0x0d,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x09, 0x04, 0x08, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x09, 0x0b, 0x0c, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0a, 0x04, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x04, 0x01, 0x12, 0x03, 0x0a, 0x04, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x04, 0x02,
    0x12, 0x03, 0x0a, 0x11, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x0d, 0x00, 0x20,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x08, 0x0d, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0e, 0x04, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x0e, 0x04, 0x0d, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x0e, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x0e, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x0e, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x04, 0x25,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x0f, 0x04, 0x0e, 0x18, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0f, 0x04, 0x09, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0f, 0x0a, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0f, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x02, 0x12, 0x03, 0x10, 0x04, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12,
    0x04, 0x10, 0x04, 0x0f, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x10, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x10, 0x0a,
    0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x10, 0x30, 0x31, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x11, 0x04, 0x2a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x04, 0x11, 0x04, 0x10, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x11, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x11, 0x0a, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x11, 0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x13,
    0x04, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x04, 0x13, 0x04, 0x11,
    0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x06, 0x12, 0x03, 0x13, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x13, 0x0d, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x13, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x05, 0x12, 0x03, 0x14, 0x04, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05,
    0x04, 0x12, 0x04, 0x14, 0x04, 0x13, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x05,
    0x12, 0x03, 0x14, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03,
    0x14, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x14, 0x14,
    0x15, 0x0a, 0x30, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x19, 0x04, 0x13, 0x1a, 0x14,
    0x48, 0x54, 0x54, 0x50, 0x2f, 0x48, 0x54, 0x54, 0x50, 0x53, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69,
    0x66, 0x69, 0x63, 0x0a, 0x32, 0x0d, 0x44, 0x4e, 0x53, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66,
    0x69, 0x63, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x04, 0x12, 0x04, 0x19, 0x04,
    0x14, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x05, 0x12, 0x03, 0x19, 0x04, 0x09,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x19, 0x0a, 0x0e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x19, 0x11, 0x12, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x1a, 0x04, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x07, 0x04, 0x12, 0x04, 0x1a, 0x04, 0x19, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07,
    0x05, 0x12, 0x03, 0x1a, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12,
    0x03, 0x1a, 0x0b, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x03, 0x12, 0x03, 0x1a,
    0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x08, 0x12, 0x03, 0x1b, 0x04, 0x1d, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x04, 0x12, 0x04, 0x1b, 0x04, 0x1a, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x05, 0x12, 0x03, 0x1b, 0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x1b, 0x09, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x08, 0x03, 0x12, 0x03, 0x1b, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04,
    0x22, 0x00, 0x37, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x22, 0x08, 0x13,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x23, 0x04, 0x18, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0x23, 0x04, 0x22, 0x15, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x23, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x23, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x23, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03,
    0x24, 0x04, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x04, 0x24, 0x04,
    0x23, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x24, 0x04, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x24, 0x0b, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x24, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x25, 0x04, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x02, 0x04, 0x12, 0x04, 0x25, 0x04, 0x24, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02,
    0x06, 0x12, 0x03, 0x25, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x25, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x25,
    0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x27, 0x04, 0x1c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x04, 0x12, 0x04, 0x27, 0x04, 0x25, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x05, 0x12, 0x03, 0x27, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x27, 0x0a, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x03, 0x03, 0x12, 0x03, 0x27, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x04,
    0x12, 0x03, 0x28, 0x04, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x04, 0x12, 0x04,
    0x28, 0x04, 0x27, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x05, 0x12, 0x03, 0x28,
    0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x28, 0x09, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x03, 0x12, 0x03, 0x28, 0x13, 0x14, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x05, 0x12, 0x03, 0x29, 0x04, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x05, 0x04, 0x12, 0x04, 0x29, 0x04, 0x28, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x05, 0x05, 0x12, 0x03, 0x29, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05,
    0x01, 0x12, 0x03, 0x29, 0x0b, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x03, 0x12,
    0x03, 0x29, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x06, 0x12, 0x03, 0x2a, 0x04,
    0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x04, 0x12, 0x04, 0x2a, 0x04, 0x29, 0x1d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x05, 0x12, 0x03, 0x2a, 0x04, 0x09, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x01, 0x12, 0x03, 0x2a, 0x0a, 0x2b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x06, 0x03, 0x12, 0x03, 0x2a, 0x2e, 0x2f, 0x0a, 0x1a, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x07, 0x12, 0x03, 0x2d, 0x04, 0x22, 0x1a, 0x0d, 0x44, 0x4e, 0x53, 0x20, 0x73, 0x70, 0x65,
    0x63, 0x69, 0x66, 0x69, 0x63, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x04, 0x12,
    0x03, 0x2d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x05, 0x12, 0x03, 0x2d,
    0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x01, 0x12, 0x03, 0x2d, 0x13, 0x1d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x03, 0x12, 0x03, 0x2d, 0x20, 0x21, 0x0a, 0x21,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x08, 0x12, 0x03, 0x30, 0x04, 0x1f, 0x1a, 0x14, 0x48, 0x54, 0x54,
    0x50, 0x2f, 0x48, 0x54, 0x54, 0x50, 0x53, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x63,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x08, 0x04, 0x12, 0x04, 0x30, 0x04, 0x2d, 0x22,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x08, 0x05, 0x12, 0x03, 0x30, 0x04, 0x09, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x08, 0x01, 0x12, 0x03, 0x30, 0x0a, 0x1a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x08, 0x03, 0x12, 0x03, 0x30, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x09, 0x12, 0x03, 0x31, 0x04, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x04,
    0x12, 0x04, 0x31, 0x04, 0x30, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x05, 0x12,
    0x03, 0x31, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x01, 0x12, 0x03, 0x31,
    0x0b, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x03, 0x12, 0x03, 0x31, 0x21, 0x23,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x0a, 0x12, 0x03, 0x32, 0x04, 0x2a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x0a, 0x04, 0x12, 0x04, 0x32, 0x04, 0x31, 0x24, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x32, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x0a, 0x01, 0x12, 0x03, 0x32, 0x0a, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0a,
    0x03, 0x12, 0x03, 0x32, 0x27, 0x29, 0x0a, 0x2c, 0x0a, 0x02, 0x06, 0x00, 0x12, 0x04, 0x3c, 0x00,
    0x40, 0x01, 0x1a, 0x20, 0x2a, 0x0a, 0x20, 0x73, 0x63, 0x68, 0x65, 0x64, 0x75, 0x6c, 0x65, 0x72,
    0x20, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x69, 0x74,
    0x69, 0x6f, 0x6e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00, 0x01, 0x12, 0x03, 0x3c, 0x08, 0x11,
    0x0a, 0x0b, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12, 0x03, 0x3d, 0x04, 0x45, 0x0a, 0x0c, 0x0a,
    0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3d, 0x08, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x06,
    0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x3d, 0x14, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x3d, 0x31, 0x41, 0x0a, 0x0b, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x01, 0x12,
    0x03, 0x3e, 0x04, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3e,
    0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x3e, 0x0f, 0x1c,
    0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x3e, 0x27, 0x32, 0x0a, 0x0b,
    0x0a, 0x04, 0x06, 0x00, 0x02, 0x02, 0x12, 0x03, 0x3f, 0x04, 0x4c, 0x0a, 0x0c, 0x0a, 0x05, 0x06,
    0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x3f, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02,
    0x02, 0x02, 0x12, 0x03, 0x3f, 0x16, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x3f, 0x36, 0x48, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x42, 0x00, 0x48,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x42, 0x08, 0x1a, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x43, 0x04, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x43, 0x04, 0x42, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x43, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x43, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x43, 0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x44, 0x04, 0x23,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x44, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x06, 0x12, 0x03, 0x44, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x44, 0x16, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x44, 0x21, 0x22, 0x0a, 0x1b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02,
    0x12, 0x03, 0x47, 0x04, 0x1a, 0x1a, 0x0e, 0x48, 0x54, 0x54, 0x50, 0x20, 0x73, 0x70, 0x65, 0x63,
    0x69, 0x66, 0x69, 0x63, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12, 0x04,
    0x47, 0x04, 0x44, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12, 0x03, 0x47,
    0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x47, 0x0b, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x47, 0x18, 0x19, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x4a, 0x00, 0x4b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03,
    0x01, 0x12, 0x03, 0x4a, 0x08, 0x18, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x4d, 0x00,
    0x50, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x4d, 0x08, 0x15, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x4e, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x4e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x4e, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x4e, 0x16, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x4e, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x4f, 0x04, 0x16,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x04, 0x4f, 0x04, 0x4e, 0x23, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03, 0x4f, 0x04, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x4f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x4f, 0x14, 0x15, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12,
    0x04, 0x52, 0x00, 0x54, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x52, 0x08,
    0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x53, 0x04, 0x1d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x53, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x00, 0x06, 0x12, 0x03, 0x53, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x53, 0x13, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x53, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x56, 0x00,
    0x58, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x56, 0x08, 0x1c, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x57, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x57, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x57, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x57, 0x13, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x57, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x5a, 0x00, 0x5b, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x5a, 0x08, 0x1a, 0x0a, 0x2d, 0x0a, 0x02, 0x06,
    0x01, 0x12, 0x04, 0x60, 0x00, 0x63, 0x01, 0x1a, 0x21, 0x0a, 0x20, 0x70, 0x72, 0x6f, 0x62, 0x65,
    0x20, 0x63, 0x61, 0x63, 0x68, 0x65, 0x20, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x20, 0x64,
    0x65, 0x66, 0x69, 0x6e, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x01,
    0x01, 0x12, 0x03, 0x60, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x06, 0x01, 0x02, 0x00, 0x12, 0x03,
    0x61, 0x04, 0x3f, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x61, 0x08,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x01, 0x02, 0x00, 0x02, 0x12, 0x03, 0x61, 0x12, 0x22, 0x0a,
    0x0c, 0x0a, 0x05, 0x06, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x61, 0x2d, 0x3b, 0x0a, 0x0b, 0x0a,
    0x04, 0x06, 0x01, 0x02, 0x01, 0x12, 0x03, 0x62, 0x04, 0x4b, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x01,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x62, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x01, 0x02, 0x01,
    0x02, 0x12, 0x03, 0x62, 0x16, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x01, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x62, 0x35, 0x47, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x65, 0x00, 0x68, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x65, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x66, 0x04, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x00, 0x04, 0x12, 0x04, 0x66, 0x04, 0x65, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x66, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x66, 0x0b, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x66,
    0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x01, 0x12, 0x03, 0x67, 0x04, 0x14, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x04, 0x12, 0x04, 0x67, 0x04, 0x66, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x05, 0x12, 0x03, 0x67, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x03, 0x67, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x67, 0x12, 0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04,
    0x6a, 0x00, 0x6d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x03, 0x6a, 0x08, 0x14,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12, 0x03, 0x6b, 0x04, 0x1a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x04, 0x6b, 0x04, 0x6a, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x00, 0x05, 0x12, 0x03, 0x6b, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x6b, 0x0b, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x6b, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x01, 0x12, 0x03,
    0x6c, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x04, 0x12, 0x03, 0x6c, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x06, 0x12, 0x03, 0x6c, 0x0d, 0x12, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x01, 0x12, 0x03, 0x6c, 0x13, 0x18, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x01, 0x03, 0x12, 0x03, 0x6c, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x0a, 0x12, 0x04, 0x6f, 0x00, 0x71, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x03,
    0x6f, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12, 0x03, 0x70, 0x04, 0x28,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x03, 0x70, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x06, 0x12, 0x03, 0x70, 0x0d, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x03, 0x70, 0x18, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x70, 0x26, 0x27, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x04,
    0x73, 0x00, 0x75, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x03, 0x73, 0x08, 0x16,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x03, 0x74, 0x04, 0x2c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x03, 0x74, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x00, 0x06, 0x12, 0x03, 0x74, 0x0d, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x74, 0x1a, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x74, 0x2a, 0x2b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x04, 0x77, 0x00, 0x78,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12, 0x03, 0x77, 0x08, 0x1c, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x0d, 0x12, 0x04, 0x7a, 0x00, 0x7c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0d, 0x01,
    0x12, 0x03, 0x7a, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x00, 0x12, 0x03, 0x7b,
    0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x04, 0x12, 0x03, 0x7b, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x05, 0x12, 0x03, 0x7b, 0x0d, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x01, 0x12, 0x03, 0x7b, 0x14, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x00, 0x03, 0x12, 0x03, 0x7b, 0x21, 0x22, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x33,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
