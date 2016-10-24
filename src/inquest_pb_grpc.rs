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
    fn CancelProbe(&self, p: super::inquest_pb::CancelProbeRequest) -> ::grpc::result::GrpcResult<super::inquest_pb::CancelProbeReply>;

    fn DescribeProbe(&self, p: super::inquest_pb::DescribeProbeRequest) -> ::grpc::result::GrpcResult<super::inquest_pb::DescribeProbeReply>;

    fn ListProbeIds(&self, p: super::inquest_pb::ListProbeIdsRequest) -> ::grpc::result::GrpcResult<super::inquest_pb::ListProbeIdsReply>;

    fn ScheduleProbe(&self, p: super::inquest_pb::ScheduleProbeRequest) -> ::grpc::result::GrpcResult<super::inquest_pb::ScheduleProbeReply>;
}

pub trait SchedulerAsync {
    fn CancelProbe(&self, p: super::inquest_pb::CancelProbeRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::inquest_pb::CancelProbeReply>;

    fn DescribeProbe(&self, p: super::inquest_pb::DescribeProbeRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::inquest_pb::DescribeProbeReply>;

    fn ListProbeIds(&self, p: super::inquest_pb::ListProbeIdsRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::inquest_pb::ListProbeIdsReply>;

    fn ScheduleProbe(&self, p: super::inquest_pb::ScheduleProbeRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::inquest_pb::ScheduleProbeReply>;
}

// sync client

pub struct SchedulerClient {
    async_client: SchedulerAsyncClient,
}

impl SchedulerClient {
    pub fn new(host: &str, port: u16, tls: bool) -> ::grpc::result::GrpcResult<Self> {
        SchedulerAsyncClient::new(host, port, tls).map(|c| {
            SchedulerClient {
                async_client: c,
            }
        })
    }
}

impl Scheduler for SchedulerClient {
    fn CancelProbe(&self, p: super::inquest_pb::CancelProbeRequest) -> ::grpc::result::GrpcResult<super::inquest_pb::CancelProbeReply> {
        ::futures::Future::wait(self.async_client.CancelProbe(p))
    }

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
    method_CancelProbe: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::inquest_pb::CancelProbeRequest, super::inquest_pb::CancelProbeReply>>,
    method_DescribeProbe: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::inquest_pb::DescribeProbeRequest, super::inquest_pb::DescribeProbeReply>>,
    method_ListProbeIds: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::inquest_pb::ListProbeIdsRequest, super::inquest_pb::ListProbeIdsReply>>,
    method_ScheduleProbe: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::inquest_pb::ScheduleProbeRequest, super::inquest_pb::ScheduleProbeReply>>,
}

impl SchedulerAsyncClient {
    pub fn new(host: &str, port: u16, tls: bool) -> ::grpc::result::GrpcResult<Self> {
        ::grpc::client::GrpcClient::new(host, port, tls).map(|c| {
            SchedulerAsyncClient {
                grpc_client: c,
                method_CancelProbe: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/Scheduler/CancelProbe".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_DescribeProbe: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/Scheduler/DescribeProbe".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_ListProbeIds: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/Scheduler/ListProbeIds".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_ScheduleProbe: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/Scheduler/ScheduleProbe".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
            }
        })
    }
}

impl SchedulerAsync for SchedulerAsyncClient {
    fn CancelProbe(&self, p: super::inquest_pb::CancelProbeRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::inquest_pb::CancelProbeReply> {
        self.grpc_client.call_unary(p, self.method_CancelProbe.clone())
    }

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
    fn CancelProbe(&self, p: super::inquest_pb::CancelProbeRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::inquest_pb::CancelProbeReply> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.CancelProbe(p)
        })
    }

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
                        name: "/Scheduler/CancelProbe".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.CancelProbe(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/Scheduler/DescribeProbe".to_string(),
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
                        name: "/Scheduler/ListProbeIds".to_string(),
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
                        name: "/Scheduler/ScheduleProbe".to_string(),
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

pub trait ProbeCache {
    fn GatherProbes(&self, p: super::inquest_pb::GatherProbesRequest) -> ::grpc::result::GrpcResult<super::inquest_pb::GatherProbesReply>;
}

pub trait ProbeCacheAsync {
    fn GatherProbes(&self, p: super::inquest_pb::GatherProbesRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::inquest_pb::GatherProbesReply>;
}

// sync client

pub struct ProbeCacheClient {
    async_client: ProbeCacheAsyncClient,
}

impl ProbeCacheClient {
    pub fn new(host: &str, port: u16, tls: bool) -> ::grpc::result::GrpcResult<Self> {
        ProbeCacheAsyncClient::new(host, port, tls).map(|c| {
            ProbeCacheClient {
                async_client: c,
            }
        })
    }
}

impl ProbeCache for ProbeCacheClient {
    fn GatherProbes(&self, p: super::inquest_pb::GatherProbesRequest) -> ::grpc::result::GrpcResult<super::inquest_pb::GatherProbesReply> {
        ::futures::Future::wait(self.async_client.GatherProbes(p))
    }
}

// async client

pub struct ProbeCacheAsyncClient {
    grpc_client: ::grpc::client::GrpcClient,
    method_GatherProbes: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::inquest_pb::GatherProbesRequest, super::inquest_pb::GatherProbesReply>>,
}

impl ProbeCacheAsyncClient {
    pub fn new(host: &str, port: u16, tls: bool) -> ::grpc::result::GrpcResult<Self> {
        ::grpc::client::GrpcClient::new(host, port, tls).map(|c| {
            ProbeCacheAsyncClient {
                grpc_client: c,
                method_GatherProbes: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/ProbeCache/GatherProbes".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
            }
        })
    }
}

impl ProbeCacheAsync for ProbeCacheAsyncClient {
    fn GatherProbes(&self, p: super::inquest_pb::GatherProbesRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::inquest_pb::GatherProbesReply> {
        self.grpc_client.call_unary(p, self.method_GatherProbes.clone())
    }
}

// sync server

pub struct ProbeCacheServer {
    async_server: ProbeCacheAsyncServer,
}

struct ProbeCacheServerHandlerToAsync {
    handler: ::std::sync::Arc<ProbeCache + Send + Sync>,
    cpupool: ::futures_cpupool::CpuPool,
}

impl ProbeCacheAsync for ProbeCacheServerHandlerToAsync {
    fn GatherProbes(&self, p: super::inquest_pb::GatherProbesRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::inquest_pb::GatherProbesReply> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.GatherProbes(p)
        })
    }
}

impl ProbeCacheServer {
    pub fn new<H : ProbeCache + Send + Sync + 'static>(port: u16, h: H) -> Self {
        let h = ProbeCacheServerHandlerToAsync {
            cpupool: ::futures_cpupool::CpuPool::new_num_cpus(),
            handler: ::std::sync::Arc::new(h),
        };
        ProbeCacheServer {
            async_server: ProbeCacheAsyncServer::new(port, h),
        }
    }
}

// async server

pub struct ProbeCacheAsyncServer {
    grpc_server: ::grpc::server::GrpcServer,
}

impl ProbeCacheAsyncServer {
    pub fn new<H : ProbeCacheAsync + 'static + Sync + Send + 'static>(port: u16, h: H) -> Self {
        let handler_arc = ::std::sync::Arc::new(h);
        let service_definition = ::grpc::server::ServerServiceDefinition::new(
            vec![
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/ProbeCache/GatherProbes".to_string(),
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
        ProbeCacheAsyncServer {
            grpc_server: ::grpc::server::GrpcServer::new(port, service_definition),
        }
    }
}
