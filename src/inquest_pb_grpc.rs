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

pub trait Inquest {
    fn DescribeProbe(&self, p: super::inquest_pb::DescribeProbeRequest) -> ::grpc::result::GrpcResult<super::inquest_pb::DescribeProbeReply>;

    fn ListProbeIds(&self, p: super::inquest_pb::ListProbeIdsRequest) -> ::grpc::result::GrpcResult<super::inquest_pb::ListProbeIdsReply>;

    fn ScheduleProbe(&self, p: super::inquest_pb::ScheduleProbeRequest) -> ::grpc::result::GrpcResult<super::inquest_pb::ScheduleProbeReply>;
}

pub trait InquestAsync {
    fn DescribeProbe(&self, p: super::inquest_pb::DescribeProbeRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::inquest_pb::DescribeProbeReply>;

    fn ListProbeIds(&self, p: super::inquest_pb::ListProbeIdsRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::inquest_pb::ListProbeIdsReply>;

    fn ScheduleProbe(&self, p: super::inquest_pb::ScheduleProbeRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::inquest_pb::ScheduleProbeReply>;
}

// sync client

pub struct InquestClient {
    async_client: InquestAsyncClient,
}

impl InquestClient {
    pub fn new(host: &str, port: u16) -> ::grpc::result::GrpcResult<Self> {
        InquestAsyncClient::new(host, port).map(|c| {
            InquestClient {
                async_client: c,
            }
        })
    }
}

impl Inquest for InquestClient {
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

pub struct InquestAsyncClient {
    grpc_client: ::grpc::client::GrpcClient,
    method_DescribeProbe: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::inquest_pb::DescribeProbeRequest, super::inquest_pb::DescribeProbeReply>>,
    method_ListProbeIds: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::inquest_pb::ListProbeIdsRequest, super::inquest_pb::ListProbeIdsReply>>,
    method_ScheduleProbe: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::inquest_pb::ScheduleProbeRequest, super::inquest_pb::ScheduleProbeReply>>,
}

impl InquestAsyncClient {
    pub fn new(host: &str, port: u16) -> ::grpc::result::GrpcResult<Self> {
        ::grpc::client::GrpcClient::new(host, port).map(|c| {
            InquestAsyncClient {
                grpc_client: c,
                method_DescribeProbe: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/.Inquest/DescribeProbe".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_ListProbeIds: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/.Inquest/ListProbeIds".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_ScheduleProbe: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/.Inquest/ScheduleProbe".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
            }
        })
    }
}

impl InquestAsync for InquestAsyncClient {
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

pub struct InquestServer {
    async_server: InquestAsyncServer,
}

struct InquestServerHandlerToAsync {
    handler: ::std::sync::Arc<Inquest + Send + Sync>,
    cpupool: ::futures_cpupool::CpuPool,
}

impl InquestAsync for InquestServerHandlerToAsync {
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

impl InquestServer {
    pub fn new<H : Inquest + Send + Sync + 'static>(port: u16, h: H) -> Self {
        let h = InquestServerHandlerToAsync {
            cpupool: ::futures_cpupool::CpuPool::new_num_cpus(),
            handler: ::std::sync::Arc::new(h),
        };
        InquestServer {
            async_server: InquestAsyncServer::new(port, h),
        }
    }
}

// async server

pub struct InquestAsyncServer {
    grpc_server: ::grpc::server::GrpcServer,
}

impl InquestAsyncServer {
    pub fn new<H : InquestAsync + 'static + Sync + Send + 'static>(port: u16, h: H) -> Self {
        let handler_arc = ::std::sync::Arc::new(h);
        let service_definition = ::grpc::server::ServerServiceDefinition::new(
            vec![
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/.Inquest/DescribeProbe".to_string(),
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
                        name: "/.Inquest/ListProbeIds".to_string(),
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
                        name: "/.Inquest/ScheduleProbe".to_string(),
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
        InquestAsyncServer {
            grpc_server: ::grpc::server::GrpcServer::new(port, service_definition),
        }
    }
}
