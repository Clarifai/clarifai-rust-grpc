pub mod grpc {
    pub mod resources;
    pub mod service;
    pub mod service_grpc;
    pub mod status;
    pub mod status_code;
}

pub mod clarifai_channel {
    use grpcio::{Channel, ChannelBuilder, ChannelCredentialsBuilder, EnvBuilder};
    use std::sync::Arc;

    pub fn grpc() -> Channel {
        let env = Arc::new(EnvBuilder::new().build());
        return ChannelBuilder::new(env)
            .secure_connect("api.clarifai.com", ChannelCredentialsBuilder::new().build());
    }

    pub fn insecure_grpc() -> Channel {
        let env = Arc::new(EnvBuilder::new().build());
        return ChannelBuilder::new(env).connect("api-grpc.clarifai.com:18080");
    }
}
