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
    test_url_suffix: ::protobuf::SingularField<::std::string::String>,
    start_time_seconds: ::std::option::Option<i64>,
    probe_interval_seconds: ::std::option::Option<i32>,
    probe_interval_post_failure_seconds: ::std::option::Option<i32>,
    declare_failure_after_x_attempts: ::std::option::Option<i32>,
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
                    test_url_suffix: ::protobuf::SingularField::none(),
                    start_time_seconds: ::std::option::Option::None,
                    probe_interval_seconds: ::std::option::Option::None,
                    probe_interval_post_failure_seconds: ::std::option::Option::None,
                    declare_failure_after_x_attempts: ::std::option::Option::None,
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

    // optional string test_url_suffix = 7;

    pub fn clear_test_url_suffix(&mut self) {
        self.test_url_suffix.clear();
    }

    pub fn has_test_url_suffix(&self) -> bool {
        self.test_url_suffix.is_some()
    }

    // Param is passed by value, moved
    pub fn set_test_url_suffix(&mut self, v: ::std::string::String) {
        self.test_url_suffix = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_test_url_suffix(&mut self) -> &mut ::std::string::String {
        if self.test_url_suffix.is_none() {
            self.test_url_suffix.set_default();
        };
        self.test_url_suffix.as_mut().unwrap()
    }

    // Take field
    pub fn take_test_url_suffix(&mut self) -> ::std::string::String {
        self.test_url_suffix.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_test_url_suffix(&self) -> &str {
        match self.test_url_suffix.as_ref() {
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

    // optional int32 declare_failure_after_x_attempts = 11;

    pub fn clear_declare_failure_after_x_attempts(&mut self) {
        self.declare_failure_after_x_attempts = ::std::option::Option::None;
    }

    pub fn has_declare_failure_after_x_attempts(&self) -> bool {
        self.declare_failure_after_x_attempts.is_some()
    }

    // Param is passed by value, moved
    pub fn set_declare_failure_after_x_attempts(&mut self, v: i32) {
        self.declare_failure_after_x_attempts = ::std::option::Option::Some(v);
    }

    pub fn get_declare_failure_after_x_attempts(&self) -> i32 {
        self.declare_failure_after_x_attempts.unwrap_or(0)
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
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.test_url_suffix));
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
                    self.declare_failure_after_x_attempts = ::std::option::Option::Some(tmp);
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
        for value in &self.test_url_suffix {
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
        for value in &self.declare_failure_after_x_attempts {
            my_size += ::protobuf::rt::value_size(11, *value, ::protobuf::wire_format::WireTypeVarint);
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
        if let Some(v) = self.test_url_suffix.as_ref() {
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
        if let Some(v) = self.declare_failure_after_x_attempts {
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
                    "test_url_suffix",
                    Probe::has_test_url_suffix,
                    Probe::get_test_url_suffix,
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
                    "declare_failure_after_x_attempts",
                    Probe::has_declare_failure_after_x_attempts,
                    Probe::get_declare_failure_after_x_attempts,
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
        self.clear_test_url_suffix();
        self.clear_start_time_seconds();
        self.clear_probe_interval_seconds();
        self.clear_probe_interval_post_failure_seconds();
        self.clear_declare_failure_after_x_attempts();
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
        self.test_url_suffix == other.test_url_suffix &&
        self.start_time_seconds == other.start_time_seconds &&
        self.probe_interval_seconds == other.probe_interval_seconds &&
        self.probe_interval_post_failure_seconds == other.probe_interval_post_failure_seconds &&
        self.declare_failure_after_x_attempts == other.declare_failure_after_x_attempts &&
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
}

impl ::protobuf::ProtobufEnum for Probe_Protocol {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Probe_Protocol> {
        match value {
            0 => ::std::option::Option::Some(Probe_Protocol::HTTP),
            1 => ::std::option::Option::Some(Probe_Protocol::HTTPS),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Probe_Protocol] = &[
            Probe_Protocol::HTTP,
            Probe_Protocol::HTTPS,
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
    probe: ::protobuf::SingularPtrField<Probe>,
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
                    probe: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .Probe probe = 2;

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

impl ::protobuf::Message for ScheduleProbeRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                2 => {
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
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "probe",
                    ScheduleProbeRequest::has_probe,
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
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GatherProbesRequest {
    fn eq(&self, other: &GatherProbesRequest) -> bool {
        self.probe_priority == other.probe_priority &&
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
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GatherProbesReply {
    fn eq(&self, other: &GatherProbesReply) -> bool {
        self.probe == other.probe &&
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
    0x73, 0x74, 0x5f, 0x70, 0x62, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x88, 0x04, 0x0a, 0x05,
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
    0x6f, 0x72, 0x74, 0x12, 0x26, 0x0a, 0x0f, 0x74, 0x65, 0x73, 0x74, 0x5f, 0x75, 0x72, 0x6c, 0x5f,
    0x73, 0x75, 0x66, 0x66, 0x69, 0x78, 0x18, 0x07, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0d, 0x74, 0x65,
    0x73, 0x74, 0x55, 0x72, 0x6c, 0x53, 0x75, 0x66, 0x66, 0x69, 0x78, 0x12, 0x2c, 0x0a, 0x12, 0x73,
    0x74, 0x61, 0x72, 0x74, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x5f, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64,
    0x73, 0x18, 0x08, 0x20, 0x01, 0x28, 0x03, 0x52, 0x10, 0x73, 0x74, 0x61, 0x72, 0x74, 0x54, 0x69,
    0x6d, 0x65, 0x53, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x12, 0x34, 0x0a, 0x16, 0x70, 0x72, 0x6f,
    0x62, 0x65, 0x5f, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x76, 0x61, 0x6c, 0x5f, 0x73, 0x65, 0x63, 0x6f,
    0x6e, 0x64, 0x73, 0x18, 0x09, 0x20, 0x01, 0x28, 0x05, 0x52, 0x14, 0x70, 0x72, 0x6f, 0x62, 0x65,
    0x49, 0x6e, 0x74, 0x65, 0x72, 0x76, 0x61, 0x6c, 0x53, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x12,
    0x4c, 0x0a, 0x23, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x5f, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x76, 0x61,
    0x6c, 0x5f, 0x70, 0x6f, 0x73, 0x74, 0x5f, 0x66, 0x61, 0x69, 0x6c, 0x75, 0x72, 0x65, 0x5f, 0x73,
    0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x05, 0x52, 0x1f, 0x70, 0x72,
    0x6f, 0x62, 0x65, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x76, 0x61, 0x6c, 0x50, 0x6f, 0x73, 0x74, 0x46,
    0x61, 0x69, 0x6c, 0x75, 0x72, 0x65, 0x53, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x12, 0x46, 0x0a,
    0x20, 0x64, 0x65, 0x63, 0x6c, 0x61, 0x72, 0x65, 0x5f, 0x66, 0x61, 0x69, 0x6c, 0x75, 0x72, 0x65,
    0x5f, 0x61, 0x66, 0x74, 0x65, 0x72, 0x5f, 0x78, 0x5f, 0x61, 0x74, 0x74, 0x65, 0x6d, 0x70, 0x74,
    0x73, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x05, 0x52, 0x1c, 0x64, 0x65, 0x63, 0x6c, 0x61, 0x72, 0x65,
    0x46, 0x61, 0x69, 0x6c, 0x75, 0x72, 0x65, 0x41, 0x66, 0x74, 0x65, 0x72, 0x58, 0x41, 0x74, 0x74,
    0x65, 0x6d, 0x70, 0x74, 0x73, 0x22, 0x1f, 0x0a, 0x08, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f,
    0x6c, 0x12, 0x08, 0x0a, 0x04, 0x48, 0x54, 0x54, 0x50, 0x10, 0x00, 0x12, 0x09, 0x0a, 0x05, 0x48,
    0x54, 0x54, 0x50, 0x53, 0x10, 0x01, 0x22, 0x31, 0x0a, 0x14, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69,
    0x62, 0x65, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x19,
    0x0a, 0x08, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x07, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x49, 0x64, 0x22, 0x32, 0x0a, 0x12, 0x44, 0x65, 0x73,
    0x63, 0x72, 0x69, 0x62, 0x65, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x12,
    0x1c, 0x0a, 0x05, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x06,
    0x2e, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x52, 0x05, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x22, 0x3c, 0x0a,
    0x13, 0x4c, 0x69, 0x73, 0x74, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x49, 0x64, 0x73, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x12, 0x25, 0x0a, 0x0e, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x5f, 0x70, 0x72,
    0x69, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0d, 0x70, 0x72,
    0x6f, 0x62, 0x65, 0x50, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x22, 0x2e, 0x0a, 0x11, 0x4c,
    0x69, 0x73, 0x74, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x49, 0x64, 0x73, 0x52, 0x65, 0x70, 0x6c, 0x79,
    0x12, 0x19, 0x0a, 0x08, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x03,
    0x28, 0x09, 0x52, 0x07, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x49, 0x64, 0x22, 0x34, 0x0a, 0x14, 0x53,
    0x63, 0x68, 0x65, 0x64, 0x75, 0x6c, 0x65, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x12, 0x1c, 0x0a, 0x05, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x06, 0x2e, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x52, 0x05, 0x70, 0x72, 0x6f, 0x62,
    0x65, 0x22, 0x14, 0x0a, 0x12, 0x53, 0x63, 0x68, 0x65, 0x64, 0x75, 0x6c, 0x65, 0x50, 0x72, 0x6f,
    0x62, 0x65, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x22, 0x3c, 0x0a, 0x13, 0x47, 0x61, 0x74, 0x68, 0x65,
    0x72, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x25,
    0x0a, 0x0e, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x5f, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0d, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x50, 0x72, 0x69,
    0x6f, 0x72, 0x69, 0x74, 0x79, 0x22, 0x31, 0x0a, 0x11, 0x47, 0x61, 0x74, 0x68, 0x65, 0x72, 0x50,
    0x72, 0x6f, 0x62, 0x65, 0x73, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x12, 0x1c, 0x0a, 0x05, 0x70, 0x72,
    0x6f, 0x62, 0x65, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x06, 0x2e, 0x50, 0x72, 0x6f, 0x62,
    0x65, 0x52, 0x05, 0x70, 0x72, 0x6f, 0x62, 0x65, 0x32, 0xc5, 0x01, 0x0a, 0x09, 0x53, 0x63, 0x68,
    0x65, 0x64, 0x75, 0x6c, 0x65, 0x72, 0x12, 0x3d, 0x0a, 0x0d, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69,
    0x62, 0x65, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x12, 0x15, 0x2e, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69,
    0x62, 0x65, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x13,
    0x2e, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x52, 0x65,
    0x70, 0x6c, 0x79, 0x22, 0x00, 0x12, 0x3a, 0x0a, 0x0c, 0x4c, 0x69, 0x73, 0x74, 0x50, 0x72, 0x6f,
    0x62, 0x65, 0x49, 0x64, 0x73, 0x12, 0x14, 0x2e, 0x4c, 0x69, 0x73, 0x74, 0x50, 0x72, 0x6f, 0x62,
    0x65, 0x49, 0x64, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x12, 0x2e, 0x4c, 0x69,
    0x73, 0x74, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x49, 0x64, 0x73, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x22,
    0x00, 0x12, 0x3d, 0x0a, 0x0d, 0x53, 0x63, 0x68, 0x65, 0x64, 0x75, 0x6c, 0x65, 0x50, 0x72, 0x6f,
    0x62, 0x65, 0x12, 0x15, 0x2e, 0x53, 0x63, 0x68, 0x65, 0x64, 0x75, 0x6c, 0x65, 0x50, 0x72, 0x6f,
    0x62, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x13, 0x2e, 0x53, 0x63, 0x68, 0x65,
    0x64, 0x75, 0x6c, 0x65, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x22, 0x00,
    0x32, 0x44, 0x0a, 0x06, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x72, 0x12, 0x3a, 0x0a, 0x0c, 0x47, 0x61,
    0x74, 0x68, 0x65, 0x72, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x73, 0x12, 0x14, 0x2e, 0x47, 0x61, 0x74,
    0x68, 0x65, 0x72, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x1a, 0x12, 0x2e, 0x47, 0x61, 0x74, 0x68, 0x65, 0x72, 0x50, 0x72, 0x6f, 0x62, 0x65, 0x73, 0x52,
    0x65, 0x70, 0x6c, 0x79, 0x22, 0x00, 0x4a, 0xae, 0x0f, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x47,
    0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x10, 0x0a, 0x1e, 0x0a, 0x02, 0x04,
    0x00, 0x12, 0x04, 0x05, 0x00, 0x18, 0x01, 0x1a, 0x12, 0x2a, 0x0a, 0x20, 0x70, 0x72, 0x6f, 0x62,
    0x65, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x00, 0x01, 0x12, 0x03, 0x05, 0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x06, 0x04, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x06,
    0x04, 0x05, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x06, 0x04,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x06, 0x0b, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x06, 0x16, 0x17, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x07, 0x04, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x04, 0x12, 0x04, 0x07, 0x04, 0x06, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x07, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x07, 0x0a, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x07, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x08, 0x04, 0x1d,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0x08, 0x04, 0x07, 0x1d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x08, 0x04, 0x09, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x08, 0x0a, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x08, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x04,
    0x00, 0x12, 0x04, 0x0a, 0x04, 0x0d, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x04, 0x00, 0x01,
    0x12, 0x03, 0x0a, 0x09, 0x11, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x0b, 0x08, 0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x0b, 0x08, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12,
    0x03, 0x0b, 0x0f, 0x10, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03,
    0x0c, 0x08, 0x12, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x0c, 0x08, 0x0d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03,
    0x0c, 0x10, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0f, 0x04, 0x1a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x04, 0x0f, 0x04, 0x0d, 0x05, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x06, 0x12, 0x03, 0x0f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0f, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0f, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x04, 0x12, 0x03, 0x10, 0x04, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12,
    0x04, 0x10, 0x04, 0x0f, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03,
    0x10, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x10, 0x0b,
    0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x10, 0x12, 0x13, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x11, 0x04, 0x13, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x04, 0x11, 0x04, 0x10, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x11, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x05, 0x01, 0x12, 0x03, 0x11, 0x0a, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03,
    0x12, 0x03, 0x11, 0x11, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x12,
    0x04, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x04, 0x12, 0x04, 0x12, 0x04, 0x11,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x05, 0x12, 0x03, 0x12, 0x04, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x12, 0x0b, 0x1a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x12, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x07, 0x12, 0x03, 0x14, 0x04, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07,
    0x04, 0x12, 0x04, 0x14, 0x04, 0x12, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x05,
    0x12, 0x03, 0x14, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03,
    0x14, 0x0a, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x03, 0x12, 0x03, 0x14, 0x1f,
    0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x08, 0x12, 0x03, 0x15, 0x04, 0x25, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x04, 0x12, 0x04, 0x15, 0x04, 0x14, 0x21, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x08, 0x05, 0x12, 0x03, 0x15, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x15, 0x0a, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x08, 0x03, 0x12, 0x03, 0x15, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x09, 0x12,
    0x03, 0x16, 0x04, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x04, 0x12, 0x04, 0x16,
    0x04, 0x15, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x05, 0x12, 0x03, 0x16, 0x04,
    0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x01, 0x12, 0x03, 0x16, 0x0a, 0x2d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x03, 0x12, 0x03, 0x16, 0x30, 0x32, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x0a, 0x12, 0x03, 0x17, 0x04, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x0a, 0x04, 0x12, 0x04, 0x17, 0x04, 0x16, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x0a, 0x05, 0x12, 0x03, 0x17, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x01,
    0x12, 0x03, 0x17, 0x0a, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x03, 0x12, 0x03,
    0x17, 0x2d, 0x2f, 0x0a, 0x2c, 0x0a, 0x02, 0x06, 0x00, 0x12, 0x04, 0x1d, 0x00, 0x21, 0x01, 0x1a,
    0x20, 0x2a, 0x0a, 0x20, 0x73, 0x63, 0x68, 0x65, 0x64, 0x75, 0x6c, 0x65, 0x72, 0x20, 0x73, 0x65,
    0x72, 0x76, 0x69, 0x63, 0x65, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x69, 0x74, 0x69, 0x6f, 0x6e,
    0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00, 0x01, 0x12, 0x03, 0x1d, 0x08, 0x11, 0x0a, 0x0b, 0x0a,
    0x04, 0x06, 0x00, 0x02, 0x00, 0x12, 0x03, 0x1e, 0x04, 0x4b, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x1e, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00,
    0x02, 0x12, 0x03, 0x1e, 0x16, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x1e, 0x35, 0x47, 0x0a, 0x0b, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x01, 0x12, 0x03, 0x1f, 0x04,
    0x48, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1f, 0x08, 0x14, 0x0a,
    0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x1f, 0x15, 0x28, 0x0a, 0x0c, 0x0a,
    0x05, 0x06, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1f, 0x33, 0x44, 0x0a, 0x0b, 0x0a, 0x04, 0x06,
    0x00, 0x02, 0x02, 0x12, 0x03, 0x20, 0x04, 0x4c, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x20, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x02, 0x12,
    0x03, 0x20, 0x16, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x20,
    0x36, 0x48, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x23, 0x00, 0x25, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x23, 0x08, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x00, 0x12, 0x03, 0x24, 0x04, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04,
    0x12, 0x04, 0x24, 0x04, 0x23, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x24, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x24,
    0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x24, 0x16, 0x17,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x27, 0x00, 0x29, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x02, 0x01, 0x12, 0x03, 0x27, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00,
    0x12, 0x03, 0x28, 0x04, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x04,
    0x28, 0x04, 0x27, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x06, 0x12, 0x03, 0x28,
    0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x28, 0x0a, 0x0f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x28, 0x12, 0x13, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x2b, 0x00, 0x2d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03,
    0x01, 0x12, 0x03, 0x2b, 0x08, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03,
    0x2c, 0x04, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x04, 0x2c, 0x04,
    0x2b, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2c, 0x04, 0x09,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2c, 0x0a, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2c, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x04, 0x12, 0x04, 0x2f, 0x00, 0x31, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12,
    0x03, 0x2f, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x30, 0x04,
    0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x30, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x30, 0x0d, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x30, 0x14, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x30, 0x1f, 0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12,
    0x04, 0x33, 0x00, 0x35, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x33, 0x08,
    0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x34, 0x04, 0x14, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x04, 0x34, 0x04, 0x33, 0x1e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x00, 0x06, 0x12, 0x03, 0x34, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x34, 0x0a, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x34, 0x12, 0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x37,
    0x00, 0x38, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x37, 0x08, 0x1a, 0x0a,
    0x28, 0x0a, 0x02, 0x06, 0x01, 0x12, 0x04, 0x3d, 0x00, 0x3f, 0x01, 0x1a, 0x1c, 0x0a, 0x20, 0x70,
    0x72, 0x6f, 0x62, 0x65, 0x72, 0x20, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x20, 0x64, 0x65,
    0x66, 0x69, 0x6e, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x01, 0x01,
    0x12, 0x03, 0x3d, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x06, 0x01, 0x02, 0x00, 0x12, 0x03, 0x3e,
    0x04, 0x48, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3e, 0x08, 0x14,
    0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x01, 0x02, 0x00, 0x02, 0x12, 0x03, 0x3e, 0x15, 0x28, 0x0a, 0x0c,
    0x0a, 0x05, 0x06, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3e, 0x33, 0x44, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x07, 0x12, 0x04, 0x41, 0x00, 0x43, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12,
    0x03, 0x41, 0x08, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x42, 0x04,
    0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x04, 0x42, 0x04, 0x41, 0x1d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x05, 0x12, 0x03, 0x42, 0x04, 0x09, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x42, 0x0a, 0x18, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x42, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08,
    0x12, 0x04, 0x45, 0x00, 0x47, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x45,
    0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x46, 0x04, 0x1d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x03, 0x46, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x00, 0x06, 0x12, 0x03, 0x46, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x46, 0x13, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x46, 0x1b, 0x1c, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
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
