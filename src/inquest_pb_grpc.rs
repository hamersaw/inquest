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

    fn Search(&self, p: super::inquest_pb::SearchRequest) -> ::grpc::result::GrpcResult<super::inquest_pb::SearchReply>;

    fn ScheduleProbe(&self, p: super::inquest_pb::ScheduleProbeRequest) -> ::grpc::result::GrpcResult<super::inquest_pb::ScheduleProbeReply>;
}

pub trait SchedulerAsync {
    fn CancelProbe(&self, p: super::inquest_pb::CancelProbeRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::inquest_pb::CancelProbeReply>;

    fn Search(&self, p: super::inquest_pb::SearchRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::inquest_pb::SearchReply>;

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

    fn Search(&self, p: super::inquest_pb::SearchRequest) -> ::grpc::result::GrpcResult<super::inquest_pb::SearchReply> {
        ::futures::Future::wait(self.async_client.Search(p))
    }

    fn ScheduleProbe(&self, p: super::inquest_pb::ScheduleProbeRequest) -> ::grpc::result::GrpcResult<super::inquest_pb::ScheduleProbeReply> {
        ::futures::Future::wait(self.async_client.ScheduleProbe(p))
    }
}

// async client

pub struct SchedulerAsyncClient {
    grpc_client: ::grpc::client::GrpcClient,
    method_CancelProbe: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::inquest_pb::CancelProbeRequest, super::inquest_pb::CancelProbeReply>>,
    method_Search: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::inquest_pb::SearchRequest, super::inquest_pb::SearchReply>>,
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
                method_Search: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/Scheduler/Search".to_string(),
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

    fn Search(&self, p: super::inquest_pb::SearchRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::inquest_pb::SearchReply> {
        self.grpc_client.call_unary(p, self.method_Search.clone())
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

    fn Search(&self, p: super::inquest_pb::SearchRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::inquest_pb::SearchReply> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.Search(p)
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
                        name: "/Scheduler/Search".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.Search(p))
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
    fn GetProbes(&self, p: super::inquest_pb::GetProbesRequest) -> ::grpc::result::GrpcResult<super::inquest_pb::GetProbesReply>;

    fn GetBucketKeys(&self, p: super::inquest_pb::GetBucketKeysRequest) -> ::grpc::result::GrpcResult<super::inquest_pb::GetBucketKeysReply>;
}

pub trait ProbeCacheAsync {
    fn GetProbes(&self, p: super::inquest_pb::GetProbesRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::inquest_pb::GetProbesReply>;

    fn GetBucketKeys(&self, p: super::inquest_pb::GetBucketKeysRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::inquest_pb::GetBucketKeysReply>;
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
    fn GetProbes(&self, p: super::inquest_pb::GetProbesRequest) -> ::grpc::result::GrpcResult<super::inquest_pb::GetProbesReply> {
        ::futures::Future::wait(self.async_client.GetProbes(p))
    }

    fn GetBucketKeys(&self, p: super::inquest_pb::GetBucketKeysRequest) -> ::grpc::result::GrpcResult<super::inquest_pb::GetBucketKeysReply> {
        ::futures::Future::wait(self.async_client.GetBucketKeys(p))
    }
}

// async client

pub struct ProbeCacheAsyncClient {
    grpc_client: ::grpc::client::GrpcClient,
    method_GetProbes: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::inquest_pb::GetProbesRequest, super::inquest_pb::GetProbesReply>>,
    method_GetBucketKeys: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::inquest_pb::GetBucketKeysRequest, super::inquest_pb::GetBucketKeysReply>>,
}

impl ProbeCacheAsyncClient {
    pub fn new(host: &str, port: u16, tls: bool) -> ::grpc::result::GrpcResult<Self> {
        ::grpc::client::GrpcClient::new(host, port, tls).map(|c| {
            ProbeCacheAsyncClient {
                grpc_client: c,
                method_GetProbes: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/ProbeCache/GetProbes".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_GetBucketKeys: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/ProbeCache/GetBucketKeys".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
            }
        })
    }
}

impl ProbeCacheAsync for ProbeCacheAsyncClient {
    fn GetProbes(&self, p: super::inquest_pb::GetProbesRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::inquest_pb::GetProbesReply> {
        self.grpc_client.call_unary(p, self.method_GetProbes.clone())
    }

    fn GetBucketKeys(&self, p: super::inquest_pb::GetBucketKeysRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::inquest_pb::GetBucketKeysReply> {
        self.grpc_client.call_unary(p, self.method_GetBucketKeys.clone())
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
    fn GetProbes(&self, p: super::inquest_pb::GetProbesRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::inquest_pb::GetProbesReply> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.GetProbes(p)
        })
    }

    fn GetBucketKeys(&self, p: super::inquest_pb::GetBucketKeysRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::inquest_pb::GetBucketKeysReply> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.GetBucketKeys(p)
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
                        name: "/ProbeCache/GetProbes".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.GetProbes(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/ProbeCache/GetBucketKeys".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.GetBucketKeys(p))
                    },
                ),
            ],
        );
        ProbeCacheAsyncServer {
            grpc_server: ::grpc::server::GrpcServer::new(port, service_definition),
        }
    }
}
