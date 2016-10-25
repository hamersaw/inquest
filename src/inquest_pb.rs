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
    probe_id: ::protobuf::SingularField<::std::string::String>,
    probe_priority: ::std::option::Option<i32>,
    version_number: ::std::option::Option<i64>,
    protocol: ::std::option::Option<Probe_Protocol>,
    host: ::protobuf::SingularField<::std::string::String>,
    port: ::std::option::Option<i32>,
    url_suffix: ::protobuf::SingularField<::std::string::String>,
    start_time_seconds: ::std::option::Option<i64>,
    probe_interval_seconds: ::std::option::Option<i32>,
    probe_interval_post_failure_seconds: ::std::option::Option<i32>,
    attempts_to_declare_failure: ::std::option::Option<i32>,
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
                    probe_id: ::protobuf::SingularField::none(),
                    probe_priority: ::std::option::Option::None,
                    version_number: ::std::option::Option::None,
                    protocol: ::std::option::Option::None,
                    host: ::protobuf::SingularField::none(),
                    port: ::std::option::Option::None,
                    url_suffix: ::protobuf::SingularField::none(),
                    start_time_seconds: ::std::option::Option::None,
                    probe_interval_seconds: ::std::option::Option::None,
                    probe_interval_post_failure_seconds: ::std::option::Option::None,
                    attempts_to_declare_failure: ::std::option::Option::None,
                    follow_redirect: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string probe_id = 1;

    pub fn clear_probe_id(&mut self) {
        self.probe_id.clear();
    }

    pub fn has_probe_id(&self) -> bool {
        self.probe_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_probe_id(&mut self, v: ::std::string::String) {
        self.probe_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_probe_id(&mut self) -> &mut ::std::string::String {
        if self.probe_id.is_none() {
            self.probe_id.set_default();
        };
        self.probe_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_probe_id(&mut self) -> ::std::string::String {
        self.probe_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_probe_id(&self) -> &str {
        match self.probe_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional int32 probe_priority = 2;

    pub fn clear_probe_priority(&mut self) {
        self.probe_priority = ::std::option::Option::None;
    }

    pub fn has_probe_priority(&self) -> bool {
        self.probe_priority.is_some()
    }

    // Param is passed by value, moved
    pub fn set_probe_priority(&mut self, v: i32) {
        self.probe_priority = ::std::option::Option::Some(v);
    }

    pub fn get_probe_priority(&self) -> i32 {
        self.probe_priority.unwrap_or(0)
    }

    // optional int64 version_number = 3;

    pub fn clear_version_number(&mut self) {
        self.version_number = ::std::option::Option::None;
    }

    pub fn has_version_number(&self) -> bool {
        self.version_number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version_number(&mut self, v: i64) {
        self.version_number = ::std::option::Option::Some(v);
    }

    pub fn get_version_number(&self) -> i64 {
        self.version_number.unwrap_or(0)
    }

    // optional .Probe.Protocol protocol = 4;

    pub fn clear_protocol(&mut self) {
        self.protocol = ::std::option::Option::None;
    }

    pub fn has_protocol(&self) -> bool {
        self.protocol.is_some()
    }

    // Param is passed by value, moved
    pub fn set_protocol(&mut self, v: Probe_Protocol) {
        self.protocol = ::std::option::Option::Some(v);
    }

    pub fn get_protocol(&self) -> Probe_Protocol {
        self.protocol.unwrap_or(Probe_Protocol::HTTP)
    }

    // optional string host = 5;

    pub fn clear_host(&mut self) {
        self.host.clear();
    }

    pub fn has_host(&self) -> bool {
        self.host.is_some()
    }

    // Param is passed by value, moved
    pub fn set_host(&mut self, v: ::std::string::String) {
        self.host = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_host(&mut self) -> &mut ::std::string::String {
        if self.host.is_none() {
            self.host.set_default();
        };
        self.host.as_mut().unwrap()
    }

    // Take field
    pub fn take_host(&mut self) -> ::std::string::String {
        self.host.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_host(&self) -> &str {
        match self.host.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional int32 port = 6;

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

    // optional string url_suffix = 7;

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

    // optional int64 start_time_seconds = 8;

    pub fn clear_start_time_seconds(&mut self) {
        self.start_time_seconds = ::std::option::Option::None;
    }

    pub fn has_start_time_seconds(&self) -> bool {
        self.start_time_seconds.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_time_seconds(&mut self, v: i64) {
        self.start_time_seconds = ::std::option::Option::Some(v);
    }

    pub fn get_start_time_seconds(&self) -> i64 {
        self.start_time_seconds.unwrap_or(0)
    }

    // optional int32 probe_interval_seconds = 9;

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

    // optional int32 probe_interval_post_failure_seconds = 10;

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

    // optional int32 attempts_to_declare_failure = 11;

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

    // optional bool follow_redirect = 12;

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
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.probe_id));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.probe_priority = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.version_number = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.protocol = ::std::option::Option::Some(tmp);
                },
                5 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.host));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.port = ::std::option::Option::Some(tmp);
                },
                7 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.url_suffix));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.start_time_seconds = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.probe_interval_seconds = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.probe_interval_post_failure_seconds = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.attempts_to_declare_failure = ::std::option::Option::Some(tmp);
                },
                12 => {
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
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.probe_priority {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.version_number {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.protocol {
            my_size += ::protobuf::rt::enum_size(4, *value);
        };
        for value in &self.host {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        for value in &self.port {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.url_suffix {
            my_size += ::protobuf::rt::string_size(7, &value);
        };
        for value in &self.start_time_seconds {
            my_size += ::protobuf::rt::value_size(8, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.probe_interval_seconds {
            my_size += ::protobuf::rt::value_size(9, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.probe_interval_post_failure_seconds {
            my_size += ::protobuf::rt::value_size(10, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.attempts_to_declare_failure {
            my_size += ::protobuf::rt::value_size(11, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.follow_redirect.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.probe_id.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.probe_priority {
            try!(os.write_int32(2, v));
        };
        if let Some(v) = self.version_number {
            try!(os.write_int64(3, v));
        };
        if let Some(v) = self.protocol {
            try!(os.write_enum(4, v.value()));
        };
        if let Some(v) = self.host.as_ref() {
            try!(os.write_string(5, &v));
        };
        if let Some(v) = self.port {
            try!(os.write_int32(6, v));
        };
        if let Some(v) = self.url_suffix.as_ref() {
            try!(os.write_string(7, &v));
        };
        if let Some(v) = self.start_time_seconds {
            try!(os.write_int64(8, v));
        };
        if let Some(v) = self.probe_interval_seconds {
            try!(os.write_int32(9, v));
        };
        if let Some(v) = self.probe_interval_post_failure_seconds {
            try!(os.write_int32(10, v));
        };
        if let Some(v) = self.attempts_to_declare_failure {
            try!(os.write_int32(11, v));
        };
        if let Some(v) = self.follow_redirect {
            try!(os.write_bool(12, v));
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
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "probe_id",
                    Probe::has_probe_id,
                    Probe::get_probe_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "probe_priority",
                    Probe::has_probe_priority,
                    Probe::get_probe_priority,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "version_number",
                    Probe::has_version_number,
                    Probe::get_version_number,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "protocol",
                    Probe::has_protocol,
                    Probe::get_protocol,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "host",
                    Probe::has_host,
                    Probe::get_host,
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
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "start_time_seconds",
                    Probe::has_start_time_seconds,
                    Probe::get_start_time_seconds,
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
        self.clear_probe_priority();
        self.clear_version_number();
        self.clear_protocol();
        self.clear_host();
        self.clear_port();
        self.clear_url_suffix();
        self.clear_start_time_seconds();
        self.clear_probe_interval_seconds();
        self.clear_probe_interval_post_failure_seconds();
        self.clear_attempts_to_declare_failure();
        self.clear_follow_redirect();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Probe {
    fn eq(&self, other: &Probe) -> bool {
        self.probe_id == other.probe_id &&
        self.probe_priority == other.probe_priority &&
        self.version_number == other.version_number &&
        self.protocol == other.protocol &&
        self.host == other.host &&
        self.port == other.port &&
        self.url_suffix == other.url_suffix &&
        self.start_time_seconds == other.start_time_seconds &&
        self.probe_interval_seconds == other.probe_interval_seconds &&
        self.probe_interval_post_failure_seconds == other.probe_interval_post_failure_seconds &&
        self.attempts_to_declare_failure == other.attempts_to_declare_failure &&
        self.follow_redirect == other.follow_redirect &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Probe {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Probe_Protocol {
    HTTP = 0,
    HTTPS = 1,
    PING = 3,
}

impl ::protobuf::ProtobufEnum for Probe_Protocol {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Probe_Protocol> {
        match value {
            0 => ::std::option::Option::Some(Probe_Protocol::HTTP),
            1 => ::std::option::Option::Some(Probe_Protocol::HTTPS),
            3 => ::std::option::Option::Some(Probe_Protocol::PING),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Probe_Protocol] = &[
            Probe_Protocol::HTTP,
            Probe_Protocol::HTTPS,
            Probe_Protocol::PING,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Probe_Protocol>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Probe_Protocol", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Probe_Protocol {
}

#[derive(Clone,Default)]
pub struct ProbeResult {
    // message fields
    probe_id: ::protobuf::SingularField<::std::string::String>,
    version_number: ::std::option::Option<i64>,
    prober_hostname: ::protobuf::SingularField<::std::string::String>,
    success: ::std::option::Option<bool>,
    error_message: ::protobuf::SingularField<::std::string::String>,
    dns_answer: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    host_probe_result: ::protobuf::RepeatedField<HostProbeResult>,
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
                    probe_id: ::protobuf::SingularField::none(),
                    version_number: ::std::option::Option::None,
                    prober_hostname: ::protobuf::SingularField::none(),
                    success: ::std::option::Option::None,
                    error_message: ::protobuf::SingularField::none(),
                    dns_answer: ::protobuf::RepeatedField::new(),
                    host_probe_result: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string probe_id = 1;

    pub fn clear_probe_id(&mut self) {
        self.probe_id.clear();
    }

    pub fn has_probe_id(&self) -> bool {
        self.probe_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_probe_id(&mut self, v: ::std::string::String) {
        self.probe_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_probe_id(&mut self) -> &mut ::std::string::String {
        if self.probe_id.is_none() {
            self.probe_id.set_default();
        };
        self.probe_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_probe_id(&mut self) -> ::std::string::String {
        self.probe_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_probe_id(&self) -> &str {
        match self.probe_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional int64 version_number = 2;

    pub fn clear_version_number(&mut self) {
        self.version_number = ::std::option::Option::None;
    }

    pub fn has_version_number(&self) -> bool {
        self.version_number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version_number(&mut self, v: i64) {
        self.version_number = ::std::option::Option::Some(v);
    }

    pub fn get_version_number(&self) -> i64 {
        self.version_number.unwrap_or(0)
    }

    // optional string prober_hostname = 3;

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

    // optional bool success = 4;

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

    // optional string error_message = 5;

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

    // repeated bytes dns_answer = 6;

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

    // repeated .HostProbeResult host_probe_result = 7;

    pub fn clear_host_probe_result(&mut self) {
        self.host_probe_result.clear();
    }

    // Param is passed by value, moved
    pub fn set_host_probe_result(&mut self, v: ::protobuf::RepeatedField<HostProbeResult>) {
        self.host_probe_result = v;
    }

    // Mutable pointer to the field.
    pub fn mut_host_probe_result(&mut self) -> &mut ::protobuf::RepeatedField<HostProbeResult> {
        &mut self.host_probe_result
    }

    // Take field
    pub fn take_host_probe_result(&mut self) -> ::protobuf::RepeatedField<HostProbeResult> {
        ::std::mem::replace(&mut self.host_probe_result, ::protobuf::RepeatedField::new())
    }

    pub fn get_host_probe_result(&self) -> &[HostProbeResult] {
        &self.host_probe_result
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
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.probe_id));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.version_number = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.prober_hostname));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.success = ::std::option::Option::Some(tmp);
                },
                5 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.error_message));
                },
                6 => {
                    try!(::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.dns_answer));
                },
                7 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.host_probe_result));
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
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.version_number {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.prober_hostname {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        if self.success.is_some() {
            my_size += 2;
        };
        for value in &self.error_message {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        for value in &self.dns_answer {
            my_size += ::protobuf::rt::bytes_size(6, &value);
        };
        for value in &self.host_probe_result {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.probe_id.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.version_number {
            try!(os.write_int64(2, v));
        };
        if let Some(v) = self.prober_hostname.as_ref() {
            try!(os.write_string(3, &v));
        };
        if let Some(v) = self.success {
            try!(os.write_bool(4, v));
        };
        if let Some(v) = self.error_message.as_ref() {
            try!(os.write_string(5, &v));
        };
        for v in &self.dns_answer {
            try!(os.write_bytes(6, &v));
        };
        for v in &self.host_probe_result {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
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
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "probe_id",
                    ProbeResult::has_probe_id,
                    ProbeResult::get_probe_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "version_number",
                    ProbeResult::has_version_number,
                    ProbeResult::get_version_number,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "prober_hostname",
                    ProbeResult::has_prober_hostname,
                    ProbeResult::get_prober_hostname,
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
                fields.push(::protobuf::reflect::accessor::make_repeated_bytes_accessor(
                    "dns_answer",
                    ProbeResult::get_dns_answer,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "host_probe_result",
                    ProbeResult::get_host_probe_result,
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
        self.clear_version_number();
        self.clear_prober_hostname();
        self.clear_success();
        self.clear_error_message();
        self.clear_dns_answer();
        self.clear_host_probe_result();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ProbeResult {
    fn eq(&self, other: &ProbeResult) -> bool {
        self.probe_id == other.probe_id &&
        self.version_number == other.version_number &&
        self.prober_hostname == other.prober_hostname &&
        self.success == other.success &&
        self.error_message == other.error_message &&
        self.dns_answer == other.dns_answer &&
        self.host_probe_result == other.host_probe_result &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ProbeResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct HostProbeResult {
    // message fields
    timestamp_sec: ::std::option::Option<i64>,
    application_layer_latency_nanosec: ::std::option::Option<i64>,
    success: ::std::option::Option<bool>,
    error_message: ::protobuf::SingularField<::std::string::String>,
    url: ::protobuf::SingularField<::std::string::String>,
    http_status_code: ::std::option::Option<i32>,
    http_status_message: ::protobuf::SingularField<::std::string::String>,
    application_bytes_received: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HostProbeResult {}

impl HostProbeResult {
    pub fn new() -> HostProbeResult {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HostProbeResult {
        static mut instance: ::protobuf::lazy::Lazy<HostProbeResult> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HostProbeResult,
        };
        unsafe {
            instance.get(|| {
                HostProbeResult {
                    timestamp_sec: ::std::option::Option::None,
                    application_layer_latency_nanosec: ::std::option::Option::None,
                    success: ::std::option::Option::None,
                    error_message: ::protobuf::SingularField::none(),
                    url: ::protobuf::SingularField::none(),
                    http_status_code: ::std::option::Option::None,
                    http_status_message: ::protobuf::SingularField::none(),
                    application_bytes_received: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int64 timestamp_sec = 1;

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

    // optional int64 application_layer_latency_nanosec = 2;

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

    // optional bool success = 3;

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

    // optional string error_message = 4;

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

    // optional string url = 5;

    pub fn clear_url(&mut self) {
        self.url.clear();
    }

    pub fn has_url(&self) -> bool {
        self.url.is_some()
    }

    // Param is passed by value, moved
    pub fn set_url(&mut self, v: ::std::string::String) {
        self.url = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_url(&mut self) -> &mut ::std::string::String {
        if self.url.is_none() {
            self.url.set_default();
        };
        self.url.as_mut().unwrap()
    }

    // Take field
    pub fn take_url(&mut self) -> ::std::string::String {
        self.url.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_url(&self) -> &str {
        match self.url.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional int32 http_status_code = 6;

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

    // optional string http_status_message = 7;

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

    // optional int32 application_bytes_received = 8;

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

impl ::protobuf::Message for HostProbeResult {
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
                    let tmp = try!(is.read_int64());
                    self.timestamp_sec = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.application_layer_latency_nanosec = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.success = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.error_message));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.url));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int32());
                    self.http_status_code = ::std::option::Option::Some(tmp);
                },
                7 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.http_status_message));
                },
                8 => {
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
        for value in &self.timestamp_sec {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.application_layer_latency_nanosec {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.success.is_some() {
            my_size += 2;
        };
        for value in &self.error_message {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        for value in &self.url {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        for value in &self.http_status_code {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.http_status_message {
            my_size += ::protobuf::rt::string_size(7, &value);
        };
        for value in &self.application_bytes_received {
            my_size += ::protobuf::rt::value_size(8, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.timestamp_sec {
            try!(os.write_int64(1, v));
        };
        if let Some(v) = self.application_layer_latency_nanosec {
            try!(os.write_int64(2, v));
        };
        if let Some(v) = self.success {
            try!(os.write_bool(3, v));
        };
        if let Some(v) = self.error_message.as_ref() {
            try!(os.write_string(4, &v));
        };
        if let Some(v) = self.url.as_ref() {
            try!(os.write_string(5, &v));
        };
        if let Some(v) = self.http_status_code {
            try!(os.write_int32(6, v));
        };
        if let Some(v) = self.http_status_message.as_ref() {
            try!(os.write_string(7, &v));
        };
        if let Some(v) = self.application_bytes_received {
            try!(os.write_int32(8, v));
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
        ::std::any::TypeId::of::<HostProbeResult>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for HostProbeResult {
    fn new() -> HostProbeResult {
        HostProbeResult::new()
    }

    fn descriptor_static(_: ::std::option::Option<HostProbeResult>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "timestamp_sec",
                    HostProbeResult::has_timestamp_sec,
                    HostProbeResult::get_timestamp_sec,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "application_layer_latency_nanosec",
                    HostProbeResult::has_application_layer_latency_nanosec,
                    HostProbeResult::get_application_layer_latency_nanosec,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "success",
                    HostProbeResult::has_success,
                    HostProbeResult::get_success,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "error_message",
                    HostProbeResult::has_error_message,
                    HostProbeResult::get_error_message,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "url",
                    HostProbeResult::has_url,
                    HostProbeResult::get_url,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "http_status_code",
                    HostProbeResult::has_http_status_code,
                    HostProbeResult::get_http_status_code,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "http_status_message",
                    HostProbeResult::has_http_status_message,
                    HostProbeResult::get_http_status_message,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "application_bytes_received",
                    HostProbeResult::has_application_bytes_received,
                    HostProbeResult::get_application_bytes_received,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HostProbeResult>(
                    "HostProbeResult",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HostProbeResult {
    fn clear(&mut self) {
        self.clear_timestamp_sec();
        self.clear_application_layer_latency_nanosec();
        self.clear_success();
        self.clear_error_message();
        self.clear_url();
        self.clear_http_status_code();
        self.clear_http_status_message();
        self.clear_application_bytes_received();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for HostProbeResult {
    fn eq(&self, other: &HostProbeResult) -> bool {
        self.timestamp_sec == other.timestamp_sec &&
        self.application_layer_latency_nanosec == other.application_layer_latency_nanosec &&
        self.success == other.success &&
        self.error_message == other.error_message &&
        self.url == other.url &&
        self.http_status_code == other.http_status_code &&
        self.http_status_message == other.http_status_message &&
        self.application_bytes_received == other.application_bytes_received &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for HostProbeResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CancelProbeRequest {
    // message fields
    probe_id: ::protobuf::SingularField<::std::string::String>,
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
                    probe_id: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string probe_id = 1;

    pub fn clear_probe_id(&mut self) {
        self.probe_id.clear();
    }

    pub fn has_probe_id(&self) -> bool {
        self.probe_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_probe_id(&mut self, v: ::std::string::String) {
        self.probe_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_probe_id(&mut self) -> &mut ::std::string::String {
        if self.probe_id.is_none() {
            self.probe_id.set_default();
        };
        self.probe_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_probe_id(&mut self) -> ::std::string::String {
        self.probe_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_probe_id(&self) -> &str {
        match self.probe_id.as_ref() {
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
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.probe_id));
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
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.probe_id.as_ref() {
            try!(os.write_string(1, &v));
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
                    "probe_id",
                    CancelProbeRequest::has_probe_id,
                    CancelProbeRequest::get_probe_id,
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
        self.clear_probe_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CancelProbeRequest {
    fn eq(&self, other: &CancelProbeRequest) -> bool {
        self.probe_id == other.probe_id &&
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
pub struct DescribeProbeRequest {
    // message fields
    probe_id: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DescribeProbeRequest {}

impl DescribeProbeRequest {
    pub fn new() -> DescribeProbeRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DescribeProbeRequest {
        static mut instance: ::protobuf::lazy::Lazy<DescribeProbeRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DescribeProbeRequest,
        };
        unsafe {
            instance.get(|| {
                DescribeProbeRequest {
                    probe_id: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string probe_id = 1;

    pub fn clear_probe_id(&mut self) {
        self.probe_id.clear();
    }

    pub fn has_probe_id(&self) -> bool {
        self.probe_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_probe_id(&mut self, v: ::std::string::String) {
        self.probe_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_probe_id(&mut self) -> &mut ::std::string::String {
        if self.probe_id.is_none() {
            self.probe_id.set_default();
        };
        self.probe_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_probe_id(&mut self) -> ::std::string::String {
        self.probe_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_probe_id(&self) -> &str {
        match self.probe_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for DescribeProbeRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.probe_id));
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
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.probe_id.as_ref() {
            try!(os.write_string(1, &v));
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
        ::std::any::TypeId::of::<DescribeProbeRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DescribeProbeRequest {
    fn new() -> DescribeProbeRequest {
        DescribeProbeRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<DescribeProbeRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "probe_id",
                    DescribeProbeRequest::has_probe_id,
                    DescribeProbeRequest::get_probe_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DescribeProbeRequest>(
                    "DescribeProbeRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DescribeProbeRequest {
    fn clear(&mut self) {
        self.clear_probe_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DescribeProbeRequest {
    fn eq(&self, other: &DescribeProbeRequest) -> bool {
        self.probe_id == other.probe_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DescribeProbeRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DescribeProbeReply {
    // message fields
    probe: ::protobuf::SingularPtrField<Probe>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DescribeProbeReply {}

impl DescribeProbeReply {
    pub fn new() -> DescribeProbeReply {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DescribeProbeReply {
        static mut instance: ::protobuf::lazy::Lazy<DescribeProbeReply> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DescribeProbeReply,
        };
        unsafe {
            instance.get(|| {
                DescribeProbeReply {
                    probe: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .Probe probe = 1;

    pub fn clear_probe(&mut self) {
        self.probe.clear();
    }

    pub fn has_probe(&self) -> bool {
        self.probe.is_some()
    }

    // Param is passed by value, moved
    pub fn set_probe(&mut self, v: Probe) {
        self.probe = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_probe(&mut self) -> &mut Probe {
        if self.probe.is_none() {
            self.probe.set_default();
        };
        self.probe.as_mut().unwrap()
    }

    // Take field
    pub fn take_probe(&mut self) -> Probe {
        self.probe.take().unwrap_or_else(|| Probe::new())
    }

    pub fn get_probe(&self) -> &Probe {
        self.probe.as_ref().unwrap_or_else(|| Probe::default_instance())
    }
}

impl ::protobuf::Message for DescribeProbeReply {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.probe));
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
        if let Some(v) = self.probe.as_ref() {
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
        ::std::any::TypeId::of::<DescribeProbeReply>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DescribeProbeReply {
    fn new() -> DescribeProbeReply {
        DescribeProbeReply::new()
    }

    fn descriptor_static(_: ::std::option::Option<DescribeProbeReply>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "probe",
                    DescribeProbeReply::has_probe,
                    DescribeProbeReply::get_probe,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DescribeProbeReply>(
                    "DescribeProbeReply",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DescribeProbeReply {
    fn clear(&mut self) {
        self.clear_probe();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DescribeProbeReply {
    fn eq(&self, other: &DescribeProbeReply) -> bool {
        self.probe == other.probe &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DescribeProbeReply {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ListProbeIdsRequest {
    // message fields
    probe_priority: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListProbeIdsRequest {}

impl ListProbeIdsRequest {
    pub fn new() -> ListProbeIdsRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListProbeIdsRequest {
        static mut instance: ::protobuf::lazy::Lazy<ListProbeIdsRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListProbeIdsRequest,
        };
        unsafe {
            instance.get(|| {
                ListProbeIdsRequest {
                    probe_priority: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 probe_priority = 1;

    pub fn clear_probe_priority(&mut self) {
        self.probe_priority = ::std::option::Option::None;
    }

    pub fn has_probe_priority(&self) -> bool {
        self.probe_priority.is_some()
    }

    // Param is passed by value, moved
    pub fn set_probe_priority(&mut self, v: i32) {
        self.probe_priority = ::std::option::Option::Some(v);
    }

    pub fn get_probe_priority(&self) -> i32 {
        self.probe_priority.unwrap_or(0)
    }
}

impl ::protobuf::Message for ListProbeIdsRequest {
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
                    let tmp = try!(is.read_int32());
                    self.probe_priority = ::std::option::Option::Some(tmp);
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
        for value in &self.probe_priority {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.probe_priority {
            try!(os.write_int32(1, v));
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
        ::std::any::TypeId::of::<ListProbeIdsRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ListProbeIdsRequest {
    fn new() -> ListProbeIdsRequest {
        ListProbeIdsRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListProbeIdsRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "probe_priority",
                    ListProbeIdsRequest::has_probe_priority,
                    ListProbeIdsRequest::get_probe_priority,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListProbeIdsRequest>(
                    "ListProbeIdsRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListProbeIdsRequest {
    fn clear(&mut self) {
        self.clear_probe_priority();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ListProbeIdsRequest {
    fn eq(&self, other: &ListProbeIdsRequest) -> bool {
        self.probe_priority == other.probe_priority &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ListProbeIdsRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ListProbeIdsReply {
    // message fields
    probe_id: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListProbeIdsReply {}

impl ListProbeIdsReply {
    pub fn new() -> ListProbeIdsReply {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListProbeIdsReply {
        static mut instance: ::protobuf::lazy::Lazy<ListProbeIdsReply> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListProbeIdsReply,
        };
        unsafe {
            instance.get(|| {
                ListProbeIdsReply {
                    probe_id: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated string probe_id = 1;

    pub fn clear_probe_id(&mut self) {
        self.probe_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_probe_id(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.probe_id = v;
    }

    // Mutable pointer to the field.
    pub fn mut_probe_id(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.probe_id
    }

    // Take field
    pub fn take_probe_id(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.probe_id, ::protobuf::RepeatedField::new())
    }

    pub fn get_probe_id(&self) -> &[::std::string::String] {
        &self.probe_id
    }
}

impl ::protobuf::Message for ListProbeIdsReply {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.probe_id));
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
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.probe_id {
            try!(os.write_string(1, &v));
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
        ::std::any::TypeId::of::<ListProbeIdsReply>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ListProbeIdsReply {
    fn new() -> ListProbeIdsReply {
        ListProbeIdsReply::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListProbeIdsReply>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "probe_id",
                    ListProbeIdsReply::get_probe_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListProbeIdsReply>(
                    "ListProbeIdsReply",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListProbeIdsReply {
    fn clear(&mut self) {
        self.clear_probe_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ListProbeIdsReply {
    fn eq(&self, other: &ListProbeIdsReply) -> bool {
        self.probe_id == other.probe_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ListProbeIdsReply {
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
pub struct GatherProbesRequest {
    // message fields
    probe_priority: ::std::option::Option<i32>,
    scheduled_probe_id: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GatherProbesRequest {}

impl GatherProbesRequest {
    pub fn new() -> GatherProbesRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GatherProbesRequest {
        static mut instance: ::protobuf::lazy::Lazy<GatherProbesRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GatherProbesRequest,
        };
        unsafe {
            instance.get(|| {
                GatherProbesRequest {
                    probe_priority: ::std::option::Option::None,
                    scheduled_probe_id: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int32 probe_priority = 1;

    pub fn clear_probe_priority(&mut self) {
        self.probe_priority = ::std::option::Option::None;
    }

    pub fn has_probe_priority(&self) -> bool {
        self.probe_priority.is_some()
    }

    // Param is passed by value, moved
    pub fn set_probe_priority(&mut self, v: i32) {
        self.probe_priority = ::std::option::Option::Some(v);
    }

    pub fn get_probe_priority(&self) -> i32 {
        self.probe_priority.unwrap_or(0)
    }

    // repeated string scheduled_probe_id = 2;

    pub fn clear_scheduled_probe_id(&mut self) {
        self.scheduled_probe_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_scheduled_probe_id(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.scheduled_probe_id = v;
    }

    // Mutable pointer to the field.
    pub fn mut_scheduled_probe_id(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.scheduled_probe_id
    }

    // Take field
    pub fn take_scheduled_probe_id(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.scheduled_probe_id, ::protobuf::RepeatedField::new())
    }

    pub fn get_scheduled_probe_id(&self) -> &[::std::string::String] {
        &self.scheduled_probe_id
    }
}

impl ::protobuf::Message for GatherProbesRequest {
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
                    let tmp = try!(is.read_int32());
                    self.probe_priority = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.scheduled_probe_id));
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
        for value in &self.probe_priority {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.scheduled_probe_id {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.probe_priority {
            try!(os.write_int32(1, v));
        };
        for v in &self.scheduled_probe_id {
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
        ::std::any::TypeId::of::<GatherProbesRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GatherProbesRequest {
    fn new() -> GatherProbesRequest {
        GatherProbesRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<GatherProbesRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "probe_priority",
                    GatherProbesRequest::has_probe_priority,
                    GatherProbesRequest::get_probe_priority,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "scheduled_probe_id",
                    GatherProbesRequest::get_scheduled_probe_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GatherProbesRequest>(
                    "GatherProbesRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GatherProbesRequest {
    fn clear(&mut self) {
        self.clear_probe_priority();
        self.clear_scheduled_probe_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GatherProbesRequest {
    fn eq(&self, other: &GatherProbesRequest) -> bool {
        self.probe_priority == other.probe_priority &&
        self.scheduled_probe_id == other.scheduled_probe_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GatherProbesRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GatherProbesReply {
    // message fields
    probe: ::protobuf::RepeatedField<Probe>,
    cancel_probe_id: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GatherProbesReply {}

impl GatherProbesReply {
    pub fn new() -> GatherProbesReply {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GatherProbesReply {
        static mut instance: ::protobuf::lazy::Lazy<GatherProbesReply> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GatherProbesReply,
        };
        unsafe {
            instance.get(|| {
                GatherProbesReply {
                    probe: ::protobuf::RepeatedField::new(),
                    cancel_probe_id: ::protobuf::RepeatedField::new(),
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

    // repeated string cancel_probe_id = 2;

    pub fn clear_cancel_probe_id(&mut self) {
        self.cancel_probe_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_cancel_probe_id(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.cancel_probe_id = v;
    }

    // Mutable pointer to the field.
    pub fn mut_cancel_probe_id(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.cancel_probe_id
    }

    // Take field
    pub fn take_cancel_probe_id(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.cancel_probe_id, ::protobuf::RepeatedField::new())
    }

    pub fn get_cancel_probe_id(&self) -> &[::std::string::String] {
        &self.cancel_probe_id
    }
}

impl ::protobuf::Message for GatherProbesReply {
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
                2 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.cancel_probe_id));
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
        for value in &self.cancel_probe_id {
            my_size += ::protobuf::rt::string_size(2, &value);
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
        for v in &self.cancel_probe_id {
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
        ::std::any::TypeId::of::<GatherProbesReply>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GatherProbesReply {
    fn new() -> GatherProbesReply {
        GatherProbesReply::new()
    }

    fn descriptor_static(_: ::std::option::Option<GatherProbesReply>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "probe",
                    GatherProbesReply::get_probe,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "cancel_probe_id",
                    GatherProbesReply::get_cancel_probe_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GatherProbesReply>(
                    "GatherProbesReply",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GatherProbesReply {
    fn clear(&mut self) {
        self.clear_probe();
        self.clear_cancel_probe_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GatherProbesReply {
    fn eq(&self, other: &GatherProbesReply) -> bool {
        self.probe == other.probe &&
        self.cancel_probe_id == other.cancel_probe_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GatherProbesReply {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x19, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x69, 0x6e, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x5f, 0x70, 0x62, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xa9, 0x04, 0x0a, 0x05,
    0x50, 0x72, 0x6f, 0x62, 0x65, 0x12, 0x19, 0x0a, 0x08, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x5f, 0x69,
    0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x49, 0x64,
    0x12, 0x25, 0x0a, 0x0e, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x5f, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69,
    0x74, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0d, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x50,
    0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x12, 0x25, 0x0a, 0x0e, 0x76, 0x65, 0x72, 0x73, 0x69,
    0x6f, 0x6e, 0x5f, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x03, 0x52,
    0x0d, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x4e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x12, 0x2b,
    0x0a, 0x08, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0e,
    0x32, 0x0f, 0x2e, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x2e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f,
    0x6c, 0x52, 0x08, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x12, 0x12, 0x0a, 0x04, 0x68,
    0x6f, 0x73, 0x74, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x68, 0x6f, 0x73, 0x74, 0x12,
    0x12, 0x0a, 0x04, 0x70, 0x6f, 0x72, 0x74, 0x18, 0x06, 0x20, 0x01, 0x28, 0x05, 0x52, 0x04, 0x70,
    0x6f, 0x72, 0x74, 0x12, 0x1d, 0x0a, 0x0a, 0x75, 0x72, 0x6c, 0x5f, 0x73, 0x75, 0x66, 0x66, 0x69,
    0x78, 0x18, 0x07, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x75, 0x72, 0x6c, 0x53, 0x75, 0x66, 0x66,
    0x69, 0x78, 0x12, 0x2c, 0x0a, 0x12, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x74, 0x69, 0x6d, 0x65,
    0x5f, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x18, 0x08, 0x20, 0x01, 0x28, 0x03, 0x52, 0x10,
    0x73, 0x74, 0x61, 0x72, 0x74, 0x54, 0x69, 0x6d, 0x65, 0x53, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73,
    0x12, 0x34, 0x0a, 0x16, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x5f, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x76,
    0x61, 0x6c, 0x5f, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x18, 0x09, 0x20, 0x01, 0x28, 0x05,
    0x52, 0x14, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x76, 0x61, 0x6c, 0x53,
    0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x12, 0x4c, 0x0a, 0x23, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x5f,
    0x69, 0x6e, 0x74, 0x65, 0x72, 0x76, 0x61, 0x6c, 0x5f, 0x70, 0x6f, 0x73, 0x74, 0x5f, 0x66, 0x61,
    0x69, 0x6c, 0x75, 0x72, 0x65, 0x5f, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x18, 0x0a, 0x20,
    0x01, 0x28, 0x05, 0x52, 0x1f, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x76,
    0x61, 0x6c, 0x50, 0x6f, 0x73, 0x74, 0x46, 0x61, 0x69, 0x6c, 0x75, 0x72, 0x65, 0x53, 0x65, 0x63,
    0x6f, 0x6e, 0x64, 0x73, 0x12, 0x3d, 0x0a, 0x1b, 0x61, 0x74, 0x74, 0x65, 0x6d, 0x70, 0x74, 0x73,
    0x5f, 0x74, 0x6f, 0x5f, 0x64, 0x65, 0x63, 0x6c, 0x61, 0x72, 0x65, 0x5f, 0x66, 0x61, 0x69, 0x6c,
    0x75, 0x72, 0x65, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x05, 0x52, 0x18, 0x61, 0x74, 0x74, 0x65, 0x6d,
    0x70, 0x74, 0x73, 0x54, 0x6f, 0x44, 0x65, 0x63, 0x6c, 0x61, 0x72, 0x65, 0x46, 0x61, 0x69, 0x6c,
    0x75, 0x72, 0x65, 0x12, 0x27, 0x0a, 0x0f, 0x66, 0x6f, 0x6c, 0x6c, 0x6f, 0x77, 0x5f, 0x72, 0x65,
    0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0e, 0x66, 0x6f,
    0x6c, 0x6c, 0x6f, 0x77, 0x52, 0x65, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x22, 0x29, 0x0a, 0x08,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x12, 0x08, 0x0a, 0x04, 0x48, 0x54, 0x54, 0x50,
    0x10, 0x00, 0x12, 0x09, 0x0a, 0x05, 0x48, 0x54, 0x54, 0x50, 0x53, 0x10, 0x01, 0x12, 0x08, 0x0a,
    0x04, 0x50, 0x49, 0x4e, 0x47, 0x10, 0x03, 0x22, 0x94, 0x02, 0x0a, 0x0b, 0x50, 0x72, 0x6f, 0x62,
    0x65, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x19, 0x0a, 0x08, 0x70, 0x72, 0x6f, 0x62, 0x65,
    0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x70, 0x72, 0x6f, 0x62, 0x65,
    0x49, 0x64, 0x12, 0x25, 0x0a, 0x0e, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x5f, 0x6e, 0x75,
    0x6d, 0x62, 0x65, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x03, 0x52, 0x0d, 0x76, 0x65, 0x72, 0x73,
    0x69, 0x6f, 0x6e, 0x4e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x12, 0x27, 0x0a, 0x0f, 0x70, 0x72, 0x6f,
    0x62, 0x65, 0x72, 0x5f, 0x68, 0x6f, 0x73, 0x74, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x0e, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x72, 0x48, 0x6f, 0x73, 0x74, 0x6e, 0x61,
    0x6d, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x18, 0x04, 0x20,
    0x01, 0x28, 0x08, 0x52, 0x07, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x12, 0x23, 0x0a, 0x0d,
    0x65, 0x72, 0x72, 0x6f, 0x72, 0x5f, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x05, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x0c, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x64, 0x6e, 0x73, 0x5f, 0x61, 0x6e, 0x73, 0x77, 0x65, 0x72, 0x18,
    0x06, 0x20, 0x03, 0x28, 0x0c, 0x52, 0x09, 0x64, 0x6e, 0x73, 0x41, 0x6e, 0x73, 0x77, 0x65, 0x72,
    0x12, 0x3c, 0x0a, 0x11, 0x68, 0x6f, 0x73, 0x74, 0x5f, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x5f, 0x72,
    0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x07, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x48, 0x6f,
    0x73, 0x74, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x52, 0x0f, 0x68,
    0x6f, 0x73, 0x74, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x22, 0xea,
    0x02, 0x0a, 0x0f, 0x48, 0x6f, 0x73, 0x74, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x52, 0x65, 0x73, 0x75,
    0x6c, 0x74, 0x12, 0x23, 0x0a, 0x0d, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x5f,
    0x73, 0x65, 0x63, 0x18, 0x01, 0x20, 0x01, 0x28, 0x03, 0x52, 0x0c, 0x74, 0x69, 0x6d, 0x65, 0x73,
    0x74, 0x61, 0x6d, 0x70, 0x53, 0x65, 0x63, 0x12, 0x49, 0x0a, 0x21, 0x61, 0x70, 0x70, 0x6c, 0x69,
    0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x6c, 0x61, 0x74,
    0x65, 0x6e, 0x63, 0x79, 0x5f, 0x6e, 0x61, 0x6e, 0x6f, 0x73, 0x65, 0x63, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x03, 0x52, 0x1e, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x4c,
    0x61, 0x79, 0x65, 0x72, 0x4c, 0x61, 0x74, 0x65, 0x6e, 0x63, 0x79, 0x4e, 0x61, 0x6e, 0x6f, 0x73,
    0x65, 0x63, 0x12, 0x18, 0x0a, 0x07, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x08, 0x52, 0x07, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x12, 0x23, 0x0a, 0x0d,
    0x65, 0x72, 0x72, 0x6f, 0x72, 0x5f, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x04, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x0c, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x12, 0x10, 0x0a, 0x03, 0x75, 0x72, 0x6c, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03,
    0x75, 0x72, 0x6c, 0x12, 0x28, 0x0a, 0x10, 0x68, 0x74, 0x74, 0x70, 0x5f, 0x73, 0x74, 0x61, 0x74,
    0x75, 0x73, 0x5f, 0x63, 0x6f, 0x64, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0e, 0x68,
    0x74, 0x74, 0x70, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x43, 0x6f, 0x64, 0x65, 0x12, 0x2e, 0x0a,
    0x13, 0x68, 0x74, 0x74, 0x70, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x5f, 0x6d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x09, 0x52, 0x11, 0x68, 0x74, 0x74, 0x70,
    0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x3c, 0x0a,
    0x1a, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x62, 0x79, 0x74,
    0x65, 0x73, 0x5f, 0x72, 0x65, 0x63, 0x65, 0x69, 0x76, 0x65, 0x64, 0x18, 0x08, 0x20, 0x01, 0x28,
    0x05, 0x52, 0x18, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x42, 0x79,
    0x74, 0x65, 0x73, 0x52, 0x65, 0x63, 0x65, 0x69, 0x76, 0x65, 0x64, 0x22, 0x2f, 0x0a, 0x12, 0x43,
    0x61, 0x6e, 0x63, 0x65, 0x6c, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x12, 0x19, 0x0a, 0x08, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x07, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x49, 0x64, 0x22, 0x12, 0x0a, 0x10,
    0x43, 0x61, 0x6e, 0x63, 0x65, 0x6c, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x52, 0x65, 0x70, 0x6c, 0x79,
    0x22, 0x31, 0x0a, 0x14, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x50, 0x72, 0x6f, 0x62,
    0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x19, 0x0a, 0x08, 0x70, 0x72, 0x6f, 0x62,
    0x65, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x70, 0x72, 0x6f, 0x62,
    0x65, 0x49, 0x64, 0x22, 0x32, 0x0a, 0x12, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x50,
    0x72, 0x6f, 0x62, 0x65, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x12, 0x1c, 0x0a, 0x05, 0x70, 0x72, 0x6f,
    0x62, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x50, 0x72, 0x6f, 0x62, 0x65,
    0x52, 0x05, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x22, 0x3c, 0x0a, 0x13, 0x4c, 0x69, 0x73, 0x74, 0x50,
    0x72, 0x6f, 0x62, 0x65, 0x49, 0x64, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x25,
    0x0a, 0x0e, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x5f, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0d, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x50, 0x72, 0x69,
    0x6f, 0x72, 0x69, 0x74, 0x79, 0x22, 0x2e, 0x0a, 0x11, 0x4c, 0x69, 0x73, 0x74, 0x50, 0x72, 0x6f,
    0x62, 0x65, 0x49, 0x64, 0x73, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x12, 0x19, 0x0a, 0x08, 0x70, 0x72,
    0x6f, 0x62, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x03, 0x28, 0x09, 0x52, 0x07, 0x70, 0x72,
    0x6f, 0x62, 0x65, 0x49, 0x64, 0x22, 0x34, 0x0a, 0x14, 0x53, 0x63, 0x68, 0x65, 0x64, 0x75, 0x6c,
    0x65, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x1c, 0x0a,
    0x05, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x50,
    0x72, 0x6f, 0x62, 0x65, 0x52, 0x05, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x22, 0x14, 0x0a, 0x12, 0x53,
    0x63, 0x68, 0x65, 0x64, 0x75, 0x6c, 0x65, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x52, 0x65, 0x70, 0x6c,
    0x79, 0x22, 0x6a, 0x0a, 0x13, 0x47, 0x61, 0x74, 0x68, 0x65, 0x72, 0x50, 0x72, 0x6f, 0x62, 0x65,
    0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x25, 0x0a, 0x0e, 0x70, 0x72, 0x6f, 0x62,
    0x65, 0x5f, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05,
    0x52, 0x0d, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x50, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x12,
    0x2c, 0x0a, 0x12, 0x73, 0x63, 0x68, 0x65, 0x64, 0x75, 0x6c, 0x65, 0x64, 0x5f, 0x70, 0x72, 0x6f,
    0x62, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x03, 0x28, 0x09, 0x52, 0x10, 0x73, 0x63, 0x68,
    0x65, 0x64, 0x75, 0x6c, 0x65, 0x64, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x49, 0x64, 0x22, 0x59, 0x0a,
    0x11, 0x47, 0x61, 0x74, 0x68, 0x65, 0x72, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x73, 0x52, 0x65, 0x70,
    0x6c, 0x79, 0x12, 0x1c, 0x0a, 0x05, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x18, 0x01, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x06, 0x2e, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x52, 0x05, 0x70, 0x72, 0x6f, 0x62, 0x65,
    0x12, 0x26, 0x0a, 0x0f, 0x63, 0x61, 0x6e, 0x63, 0x65, 0x6c, 0x5f, 0x70, 0x72, 0x6f, 0x62, 0x65,
    0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x03, 0x28, 0x09, 0x52, 0x0d, 0x63, 0x61, 0x6e, 0x63, 0x65,
    0x6c, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x49, 0x64, 0x32, 0xfe, 0x01, 0x0a, 0x09, 0x53, 0x63, 0x68,
    0x65, 0x64, 0x75, 0x6c, 0x65, 0x72, 0x12, 0x37, 0x0a, 0x0b, 0x43, 0x61, 0x6e, 0x63, 0x65, 0x6c,
    0x50, 0x72, 0x6f, 0x62, 0x65, 0x12, 0x13, 0x2e, 0x43, 0x61, 0x6e, 0x63, 0x65, 0x6c, 0x50, 0x72,
    0x6f, 0x62, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x11, 0x2e, 0x43, 0x61, 0x6e,
    0x63, 0x65, 0x6c, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x22, 0x00, 0x12,
    0x3d, 0x0a, 0x0d, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x50, 0x72, 0x6f, 0x62, 0x65,
    0x12, 0x15, 0x2e, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x50, 0x72, 0x6f, 0x62, 0x65,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x13, 0x2e, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69,
    0x62, 0x65, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x22, 0x00, 0x12, 0x3a,
    0x0a, 0x0c, 0x4c, 0x69, 0x73, 0x74, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x49, 0x64, 0x73, 0x12, 0x14,
    0x2e, 0x4c, 0x69, 0x73, 0x74, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x49, 0x64, 0x73, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x1a, 0x12, 0x2e, 0x4c, 0x69, 0x73, 0x74, 0x50, 0x72, 0x6f, 0x62, 0x65,
    0x49, 0x64, 0x73, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x22, 0x00, 0x12, 0x3d, 0x0a, 0x0d, 0x53, 0x63,
    0x68, 0x65, 0x64, 0x75, 0x6c, 0x65, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x12, 0x15, 0x2e, 0x53, 0x63,
    0x68, 0x65, 0x64, 0x75, 0x6c, 0x65, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x1a, 0x13, 0x2e, 0x53, 0x63, 0x68, 0x65, 0x64, 0x75, 0x6c, 0x65, 0x50, 0x72, 0x6f,
    0x62, 0x65, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x22, 0x00, 0x32, 0x48, 0x0a, 0x0a, 0x50, 0x72, 0x6f,
    0x62, 0x65, 0x43, 0x61, 0x63, 0x68, 0x65, 0x12, 0x3a, 0x0a, 0x0c, 0x47, 0x61, 0x74, 0x68, 0x65,
    0x72, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x73, 0x12, 0x14, 0x2e, 0x47, 0x61, 0x74, 0x68, 0x65, 0x72,
    0x50, 0x72, 0x6f, 0x62, 0x65, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x12, 0x2e,
    0x47, 0x61, 0x74, 0x68, 0x65, 0x72, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x73, 0x52, 0x65, 0x70, 0x6c,
    0x79, 0x22, 0x00, 0x4a, 0xa6, 0x1b, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x6c, 0x01, 0x0a, 0x08,
    0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x10, 0x0a, 0x1e, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04,
    0x05, 0x00, 0x1b, 0x01, 0x1a, 0x12, 0x2a, 0x0a, 0x20, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x20, 0x6d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12,
    0x03, 0x05, 0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x06, 0x04,
    0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x06, 0x04, 0x05, 0x0f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x06, 0x04, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x06, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x06, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x01, 0x12, 0x03, 0x07, 0x04, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04,
    0x12, 0x04, 0x07, 0x04, 0x06, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x07, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x07,
    0x0a, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x07, 0x1b, 0x1c,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x08, 0x04, 0x1d, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0x08, 0x04, 0x07, 0x1d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x08, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x08, 0x0a, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x08, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x04, 0x00, 0x12, 0x04,
    0x0a, 0x04, 0x0e, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x04, 0x00, 0x01, 0x12, 0x03, 0x0a,
    0x09, 0x11, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0b, 0x08,
    0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x08,
    0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0b, 0x0f,
    0x10, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x08, 0x12,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0c, 0x08, 0x0d,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x0c, 0x10, 0x11,
    0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0d, 0x08, 0x11, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x08, 0x0c, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x0d, 0x0f, 0x10, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x10, 0x04, 0x1a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x04, 0x10, 0x04, 0x0e, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x03, 0x06, 0x12, 0x03, 0x10, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x10, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x10, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x11,
    0x04, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x04, 0x11, 0x04, 0x10,
    0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x11, 0x04, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x11, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x11, 0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x05, 0x12, 0x03, 0x12, 0x04, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05,
    0x04, 0x12, 0x04, 0x12, 0x04, 0x11, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x05,
    0x12, 0x03, 0x12, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03,
    0x12, 0x0a, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x12, 0x11,
    0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x13, 0x04, 0x1a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x04, 0x12, 0x04, 0x13, 0x04, 0x12, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x06, 0x05, 0x12, 0x03, 0x13, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x13, 0x0b, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x06, 0x03, 0x12, 0x03, 0x13, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x07, 0x12,
    0x03, 0x15, 0x04, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x04, 0x12, 0x04, 0x15,
    0x04, 0x13, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x05, 0x12, 0x03, 0x15, 0x04,
    0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x15, 0x0a, 0x1c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x03, 0x12, 0x03, 0x15, 0x1f, 0x20, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x08, 0x12, 0x03, 0x16, 0x04, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x08, 0x04, 0x12, 0x04, 0x16, 0x04, 0x15, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x08, 0x05, 0x12, 0x03, 0x16, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x01,
    0x12, 0x03, 0x16, 0x0a, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x03, 0x12, 0x03,
    0x16, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x09, 0x12, 0x03, 0x17, 0x04, 0x33,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x04, 0x12, 0x04, 0x17, 0x04, 0x16, 0x25, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x05, 0x12, 0x03, 0x17, 0x04, 0x09, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x09, 0x01, 0x12, 0x03, 0x17, 0x0a, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x09, 0x03, 0x12, 0x03, 0x17, 0x30, 0x32, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x0a, 0x12, 0x03, 0x18, 0x04, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x04, 0x12,
    0x04, 0x18, 0x04, 0x17, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x05, 0x12, 0x03,
    0x18, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x18, 0x0a,
    0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x18, 0x28, 0x2a, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0b, 0x12, 0x03, 0x1a, 0x04, 0x1e, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x0b, 0x04, 0x12, 0x04, 0x1a, 0x04, 0x18, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x0b, 0x05, 0x12, 0x03, 0x1a, 0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x0b, 0x01, 0x12, 0x03, 0x1a, 0x09, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x03,
    0x12, 0x03, 0x1a, 0x1b, 0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x1d, 0x00, 0x26,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x1d, 0x08, 0x13, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x1e, 0x04, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x1e, 0x04, 0x1d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x1e, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x1e, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x1e, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x1f, 0x04, 0x1d,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x04, 0x1f, 0x04, 0x1e, 0x18, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x1f, 0x04, 0x09, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1f, 0x0a, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1f, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02,
    0x02, 0x12, 0x03, 0x20, 0x04, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12,
    0x04, 0x20, 0x04, 0x1f, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x20, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x20, 0x0b,
    0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x20, 0x1d, 0x1e, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x22, 0x04, 0x15, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x03, 0x04, 0x12, 0x04, 0x22, 0x04, 0x20, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x03, 0x05, 0x12, 0x03, 0x22, 0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x22, 0x09, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x22, 0x13, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x04, 0x12, 0x03, 0x23,
    0x04, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x04, 0x12, 0x04, 0x23, 0x04, 0x22,
    0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x05, 0x12, 0x03, 0x23, 0x04, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x23, 0x0b, 0x18, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x04, 0x03, 0x12, 0x03, 0x23, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x05, 0x12, 0x03, 0x24, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05,
    0x04, 0x12, 0x03, 0x24, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x05, 0x12,
    0x03, 0x24, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x01, 0x12, 0x03, 0x24,
    0x13, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x03, 0x12, 0x03, 0x24, 0x20, 0x21,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x06, 0x12, 0x03, 0x25, 0x04, 0x33, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x06, 0x04, 0x12, 0x03, 0x25, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x06, 0x06, 0x12, 0x03, 0x25, 0x0d, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x06, 0x01, 0x12, 0x03, 0x25, 0x1d, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x03,
    0x12, 0x03, 0x25, 0x31, 0x32, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x28, 0x00, 0x33,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x28, 0x08, 0x17, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x29, 0x04, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x29, 0x04, 0x28, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x29, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x29, 0x0a, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x29, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2a, 0x04, 0x30,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x04, 0x2a, 0x04, 0x29, 0x1c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x2a, 0x04, 0x09, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2a, 0x0a, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2a, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02,
    0x02, 0x12, 0x03, 0x2b, 0x04, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12,
    0x04, 0x2b, 0x04, 0x2a, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x2b, 0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2b, 0x09,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2b, 0x13, 0x14, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2c, 0x04, 0x1d, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x03, 0x04, 0x12, 0x04, 0x2c, 0x04, 0x2b, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x03, 0x05, 0x12, 0x03, 0x2c, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x2c, 0x0b, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x2c, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x2e,
    0x04, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x04, 0x12, 0x04, 0x2e, 0x04, 0x2c,
    0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x05, 0x12, 0x03, 0x2e, 0x04, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03, 0x2e, 0x0b, 0x0e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x04, 0x03, 0x12, 0x03, 0x2e, 0x11, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x02, 0x02, 0x05, 0x12, 0x03, 0x30, 0x04, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05,
    0x04, 0x12, 0x04, 0x30, 0x04, 0x2e, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x05,
    0x12, 0x03, 0x30, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x01, 0x12, 0x03,
    0x30, 0x0a, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x03, 0x12, 0x03, 0x30, 0x1d,
    0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x06, 0x12, 0x03, 0x31, 0x04, 0x23, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x04, 0x12, 0x04, 0x31, 0x04, 0x30, 0x1f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x06, 0x05, 0x12, 0x03, 0x31, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x06, 0x01, 0x12, 0x03, 0x31, 0x0b, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x06, 0x03, 0x12, 0x03, 0x31, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x07, 0x12,
    0x03, 0x32, 0x04, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x04, 0x12, 0x04, 0x32,
    0x04, 0x31, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x05, 0x12, 0x03, 0x32, 0x04,
    0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x01, 0x12, 0x03, 0x32, 0x0a, 0x24, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x03, 0x12, 0x03, 0x32, 0x27, 0x28, 0x0a, 0x2c, 0x0a,
    0x02, 0x06, 0x00, 0x12, 0x04, 0x38, 0x00, 0x3d, 0x01, 0x1a, 0x20, 0x2a, 0x0a, 0x20, 0x73, 0x63,
    0x68, 0x65, 0x64, 0x75, 0x6c, 0x65, 0x72, 0x20, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x20,
    0x64, 0x65, 0x66, 0x69, 0x6e, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x06,
    0x00, 0x01, 0x12, 0x03, 0x38, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x39, 0x04, 0x45, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x39,
    0x08, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x39, 0x14, 0x26,
    0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x39, 0x31, 0x41, 0x0a, 0x0b,
    0x0a, 0x04, 0x06, 0x00, 0x02, 0x01, 0x12, 0x03, 0x3a, 0x04, 0x4b, 0x0a, 0x0c, 0x0a, 0x05, 0x06,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3a, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02,
    0x01, 0x02, 0x12, 0x03, 0x3a, 0x16, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x3a, 0x35, 0x47, 0x0a, 0x0b, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x02, 0x12, 0x03, 0x3b,
    0x04, 0x48, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x3b, 0x08, 0x14,
    0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x3b, 0x15, 0x28, 0x0a, 0x0c,
    0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x3b, 0x33, 0x44, 0x0a, 0x0b, 0x0a, 0x04,
    0x06, 0x00, 0x02, 0x03, 0x12, 0x03, 0x3c, 0x04, 0x4c, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x3c, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x03, 0x02,
    0x12, 0x03, 0x3c, 0x16, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x3c, 0x36, 0x48, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x3f, 0x00, 0x41, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x3f, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x03, 0x02, 0x00, 0x12, 0x03, 0x40, 0x04, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x04, 0x12, 0x04, 0x40, 0x04, 0x3f, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x40, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x40, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x40, 0x16,
    0x17, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x43, 0x00, 0x44, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x43, 0x08, 0x18, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12,
    0x04, 0x46, 0x00, 0x48, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x46, 0x08,
    0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x47, 0x04, 0x18, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x04, 0x47, 0x04, 0x46, 0x1e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x47, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x47, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x47, 0x16, 0x17, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x4a,
    0x00, 0x4c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x4a, 0x08, 0x1a, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x4b, 0x04, 0x14, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x04, 0x4b, 0x04, 0x4a, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x00, 0x06, 0x12, 0x03, 0x4b, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x4b, 0x0a, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x4b, 0x12, 0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x4e, 0x00, 0x50,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x4e, 0x08, 0x1b, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x4f, 0x04, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x4f, 0x04, 0x4e, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x4f, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x4f, 0x0a, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x4f, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x52, 0x00, 0x54, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x52, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x08, 0x02, 0x00, 0x12, 0x03, 0x53, 0x04, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x53, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x53, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x53,
    0x14, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x53, 0x1f, 0x20,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x56, 0x00, 0x58, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x09, 0x01, 0x12, 0x03, 0x56, 0x08, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00,
    0x12, 0x03, 0x57, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x57, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x06, 0x12, 0x03, 0x57, 0x0d,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x57, 0x13, 0x18, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x57, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x0a, 0x12, 0x04, 0x5a, 0x00, 0x5b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0a, 0x01,
    0x12, 0x03, 0x5a, 0x08, 0x1a, 0x0a, 0x2d, 0x0a, 0x02, 0x06, 0x01, 0x12, 0x04, 0x60, 0x00, 0x62,
    0x01, 0x1a, 0x21, 0x0a, 0x20, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x20, 0x63, 0x61, 0x63, 0x68, 0x65,
    0x20, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x69, 0x74,
    0x69, 0x6f, 0x6e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x01, 0x01, 0x12, 0x03, 0x60, 0x08, 0x12,
    0x0a, 0x0b, 0x0a, 0x04, 0x06, 0x01, 0x02, 0x00, 0x12, 0x03, 0x61, 0x04, 0x48, 0x0a, 0x0c, 0x0a,
    0x05, 0x06, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x61, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x06,
    0x01, 0x02, 0x00, 0x02, 0x12, 0x03, 0x61, 0x15, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x01, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x61, 0x33, 0x44, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x04, 0x64,
    0x00, 0x67, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x03, 0x64, 0x08, 0x1b, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x03, 0x65, 0x04, 0x1d, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x04, 0x65, 0x04, 0x64, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x00, 0x05, 0x12, 0x03, 0x65, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x65, 0x0a, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x65, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x01, 0x12, 0x03, 0x66,
    0x04, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x04, 0x12, 0x03, 0x66, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x05, 0x12, 0x03, 0x66, 0x0d, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x01, 0x12, 0x03, 0x66, 0x14, 0x26, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x01, 0x03, 0x12, 0x03, 0x66, 0x29, 0x2a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0c,
    0x12, 0x04, 0x69, 0x00, 0x6c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12, 0x03, 0x69,
    0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x00, 0x12, 0x03, 0x6a, 0x04, 0x1d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x04, 0x12, 0x03, 0x6a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x00, 0x06, 0x12, 0x03, 0x6a, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6a, 0x13, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x6a, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x01, 0x12,
    0x03, 0x6b, 0x04, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x04, 0x12, 0x03, 0x6b,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x05, 0x12, 0x03, 0x6b, 0x0d, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x01, 0x12, 0x03, 0x6b, 0x14, 0x23, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x03, 0x12, 0x03, 0x6b, 0x26, 0x27, 0x62, 0x06, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x33,
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
