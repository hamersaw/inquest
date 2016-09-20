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


// interface

pub trait Scheduler {
    fn DescribeProbe(&self, p: super::inquest_pb::DescribeProbeRequest) -> ::grpc::result::GrpcResult<super::inquest_pb::DescribeProbeReply>;

    fn ListProbeIds(&self, p: super::inquest_pb::ListProbeIdsRequest) -> ::grpc::result::GrpcResult<super::inquest_pb::ListProbeIdsReply>;

    fn ScheduleProbe(&self, p: super::inquest_pb::ScheduleProbeRequest) -> ::grpc::result::GrpcResult<super::inquest_pb::ScheduleProbeReply>;
}

pub trait SchedulerAsync {
    fn DescribeProbe(&self, p: super::inquest_pb::DescribeProbeRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::inquest_pb::DescribeProbeReply>;

    fn ListProbeIds(&self, p: super::inquest_pb::ListProbeIdsRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::inquest_pb::ListProbeIdsReply>;

    fn ScheduleProbe(&self, p: super::inquest_pb::ScheduleProbeRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::inquest_pb::ScheduleProbeReply>;
}

// sync client

pub struct SchedulerClient {
    async_client: SchedulerAsyncClient,
}

impl SchedulerClient {
    pub fn new(host: &str, port: u16) -> ::grpc::result::GrpcResult<Self> {
        SchedulerAsyncClient::new(host, port).map(|c| {
            SchedulerClient {
                async_client: c,
            }
        })
    }
}

impl Scheduler for SchedulerClient {
    fn DescribeProbe(&self, p: super::inquest_pb::DescribeProbeRequest) -> ::grpc::result::GrpcResult<super::inquest_pb::DescribeProbeReply> {
        ::futures::Future::wait(self.async_client.DescribeProbe(p))
    }

    fn ListProbeIds(&self, p: super::inquest_pb::ListProbeIdsRequest) -> ::grpc::result::GrpcResult<super::inquest_pb::ListProbeIdsReply> {
        ::futures::Future::wait(self.async_client.ListProbeIds(p))
    }

    fn ScheduleProbe(&self, p: super::inquest_pb::ScheduleProbeRequest) -> ::grpc::result::GrpcResult<super::inquest_pb::ScheduleProbeReply> {
        ::futures::Future::wait(self.async_client.ScheduleProbe(p))
    }
}

// async client

pub struct SchedulerAsyncClient {
    grpc_client: ::grpc::client::GrpcClient,
    method_DescribeProbe: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::inquest_pb::DescribeProbeRequest, super::inquest_pb::DescribeProbeReply>>,
    method_ListProbeIds: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::inquest_pb::ListProbeIdsRequest, super::inquest_pb::ListProbeIdsReply>>,
    method_ScheduleProbe: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::inquest_pb::ScheduleProbeRequest, super::inquest_pb::ScheduleProbeReply>>,
}

impl SchedulerAsyncClient {
    pub fn new(host: &str, port: u16) -> ::grpc::result::GrpcResult<Self> {
        ::grpc::client::GrpcClient::new(host, port).map(|c| {
            SchedulerAsyncClient {
                grpc_client: c,
                method_DescribeProbe: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/.Scheduler/DescribeProbe".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_ListProbeIds: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/.Scheduler/ListProbeIds".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_ScheduleProbe: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/.Scheduler/ScheduleProbe".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
            }
        })
    }
}

impl SchedulerAsync for SchedulerAsyncClient {
    fn DescribeProbe(&self, p: super::inquest_pb::DescribeProbeRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::inquest_pb::DescribeProbeReply> {
        self.grpc_client.call_unary(p, self.method_DescribeProbe.clone())
    }

    fn ListProbeIds(&self, p: super::inquest_pb::ListProbeIdsRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::inquest_pb::ListProbeIdsReply> {
        self.grpc_client.call_unary(p, self.method_ListProbeIds.clone())
    }

    fn ScheduleProbe(&self, p: super::inquest_pb::ScheduleProbeRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::inquest_pb::ScheduleProbeReply> {
        self.grpc_client.call_unary(p, self.method_ScheduleProbe.clone())
    }
}

// sync server

pub struct SchedulerServer {
    async_server: SchedulerAsyncServer,
}

struct SchedulerServerHandlerToAsync {
    handler: ::std::sync::Arc<Scheduler + Send + Sync>,
    cpupool: ::futures_cpupool::CpuPool,
}

impl SchedulerAsync for SchedulerServerHandlerToAsync {
    fn DescribeProbe(&self, p: super::inquest_pb::DescribeProbeRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::inquest_pb::DescribeProbeReply> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.DescribeProbe(p)
        })
    }

    fn ListProbeIds(&self, p: super::inquest_pb::ListProbeIdsRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::inquest_pb::ListProbeIdsReply> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.ListProbeIds(p)
        })
    }

    fn ScheduleProbe(&self, p: super::inquest_pb::ScheduleProbeRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::inquest_pb::ScheduleProbeReply> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.ScheduleProbe(p)
        })
    }
}

impl SchedulerServer {
    pub fn new<H : Scheduler + Send + Sync + 'static>(port: u16, h: H) -> Self {
        let h = SchedulerServerHandlerToAsync {
            cpupool: ::futures_cpupool::CpuPool::new_num_cpus(),
            handler: ::std::sync::Arc::new(h),
        };
        SchedulerServer {
            async_server: SchedulerAsyncServer::new(port, h),
        }
    }
}

// async server

pub struct SchedulerAsyncServer {
    grpc_server: ::grpc::server::GrpcServer,
}

impl SchedulerAsyncServer {
    pub fn new<H : SchedulerAsync + 'static + Sync + Send + 'static>(port: u16, h: H) -> Self {
        let handler_arc = ::std::sync::Arc::new(h);
        let service_definition = ::grpc::server::ServerServiceDefinition::new(
            vec![
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/.Scheduler/DescribeProbe".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.DescribeProbe(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/.Scheduler/ListProbeIds".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.ListProbeIds(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/.Scheduler/ScheduleProbe".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.ScheduleProbe(p))
                    },
                ),
            ],
        );
        SchedulerAsyncServer {
            grpc_server: ::grpc::server::GrpcServer::new(port, service_definition),
        }
    }
}

// interface

pub trait Prober {
    fn GatherProbes(&self, p: super::inquest_pb::GatherProbesRequest) -> ::grpc::result::GrpcResult<super::inquest_pb::GatherProbesReply>;
}

pub trait ProberAsync {
    fn GatherProbes(&self, p: super::inquest_pb::GatherProbesRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::inquest_pb::GatherProbesReply>;
}

// sync client

pub struct ProberClient {
    async_client: ProberAsyncClient,
}

impl ProberClient {
    pub fn new(host: &str, port: u16) -> ::grpc::result::GrpcResult<Self> {
        ProberAsyncClient::new(host, port).map(|c| {
            ProberClient {
                async_client: c,
            }
        })
    }
}

impl Prober for ProberClient {
    fn GatherProbes(&self, p: super::inquest_pb::GatherProbesRequest) -> ::grpc::result::GrpcResult<super::inquest_pb::GatherProbesReply> {
        ::futures::Future::wait(self.async_client.GatherProbes(p))
    }
}

// async client

pub struct ProberAsyncClient {
    grpc_client: ::grpc::client::GrpcClient,
    method_GatherProbes: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::inquest_pb::GatherProbesRequest, super::inquest_pb::GatherProbesReply>>,
}

impl ProberAsyncClient {
    pub fn new(host: &str, port: u16) -> ::grpc::result::GrpcResult<Self> {
        ::grpc::client::GrpcClient::new(host, port).map(|c| {
            ProberAsyncClient {
                grpc_client: c,
                method_GatherProbes: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/.Prober/GatherProbes".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
            }
        })
    }
}

impl ProberAsync for ProberAsyncClient {
    fn GatherProbes(&self, p: super::inquest_pb::GatherProbesRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::inquest_pb::GatherProbesReply> {
        self.grpc_client.call_unary(p, self.method_GatherProbes.clone())
    }
}

// sync server

pub struct ProberServer {
    async_server: ProberAsyncServer,
}

struct ProberServerHandlerToAsync {
    handler: ::std::sync::Arc<Prober + Send + Sync>,
    cpupool: ::futures_cpupool::CpuPool,
}

impl ProberAsync for ProberServerHandlerToAsync {
    fn GatherProbes(&self, p: super::inquest_pb::GatherProbesRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::inquest_pb::GatherProbesReply> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.GatherProbes(p)
        })
    }
}

impl ProberServer {
    pub fn new<H : Prober + Send + Sync + 'static>(port: u16, h: H) -> Self {
        let h = ProberServerHandlerToAsync {
            cpupool: ::futures_cpupool::CpuPool::new_num_cpus(),
            handler: ::std::sync::Arc::new(h),
        };
        ProberServer {
            async_server: ProberAsyncServer::new(port, h),
        }
    }
}

// async server

pub struct ProberAsyncServer {
    grpc_server: ::grpc::server::GrpcServer,
}

impl ProberAsyncServer {
    pub fn new<H : ProberAsync + 'static + Sync + Send + 'static>(port: u16, h: H) -> Self {
        let handler_arc = ::std::sync::Arc::new(h);
        let service_definition = ::grpc::server::ServerServiceDefinition::new(
            vec![
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/.Prober/GatherProbes".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.GatherProbes(p))
                    },
                ),
            ],
        );
        ProberAsyncServer {
            grpc_server: ::grpc::server::GrpcServer::new(port, service_definition),
        }
    }
}
