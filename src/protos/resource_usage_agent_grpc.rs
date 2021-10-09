// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_RESOURCE_USAGE_AGENT_REPORT_CPU_TIME: ::grpcio::Method<super::resource_usage_agent::CpuTimeRecord, super::resource_usage_agent::EmptyResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ClientStreaming,
    name: "/resource_usage_agent.ResourceUsageAgent/ReportCPUTime",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct ResourceUsageAgentClient {
    client: ::grpcio::Client,
}

impl ResourceUsageAgentClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ResourceUsageAgentClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn report_cpu_time_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::resource_usage_agent::CpuTimeRecord>, ::grpcio::ClientCStreamReceiver<super::resource_usage_agent::EmptyResponse>)> {
        self.client.client_streaming(&METHOD_RESOURCE_USAGE_AGENT_REPORT_CPU_TIME, opt)
    }

    pub fn report_cpu_time(&self) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::resource_usage_agent::CpuTimeRecord>, ::grpcio::ClientCStreamReceiver<super::resource_usage_agent::EmptyResponse>)> {
        self.report_cpu_time_opt(::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait ResourceUsageAgent {
    fn report_cpu_time(&mut self, ctx: ::grpcio::RpcContext, _stream: ::grpcio::RequestStream<super::resource_usage_agent::CpuTimeRecord>, sink: ::grpcio::ClientStreamingSink<super::resource_usage_agent::EmptyResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_resource_usage_agent<S: ResourceUsageAgent + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s;
    builder = builder.add_client_streaming_handler(&METHOD_RESOURCE_USAGE_AGENT_REPORT_CPU_TIME, move |ctx, req, resp| {
        instance.report_cpu_time(ctx, req, resp)
    });
    builder.build()
}
