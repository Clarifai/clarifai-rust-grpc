
pub mod grpc {
    pub mod service;
    pub mod resources;
    pub mod service_grpc;
    pub mod status;
    pub mod status_code;
}

pub mod clarifai {
    use std::sync::Arc;
    use grpcio::{EnvBuilder, ChannelBuilder, Channel};

    pub fn insecure_grpc() -> Channel {
        let env = Arc::new(EnvBuilder::new().build());
        return ChannelBuilder::new(env).connect("api-grpc.clarifai.com:18080");
    }
}
