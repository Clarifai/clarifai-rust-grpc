pub mod grpc {
    pub mod resources;
    pub mod service;
    pub mod service_grpc;
    pub mod status;
    pub mod status_code;
    pub mod matrix;
    pub mod extension;
    pub mod extensions;
    pub mod scope;
    pub mod types;
}

pub mod clarifai_channel {
    use grpcio::{Channel, ChannelBuilder, ChannelCredentialsBuilder, EnvBuilder};
    use std::env;
    use std::sync::Arc;

    pub fn grpc() -> Channel {
        let environment = Arc::new(EnvBuilder::new().build());

        let grpc_base_url =
            env::var("CLARIFAI_GRPC_BASE").unwrap_or("api.clarifai.com".to_string());

        return ChannelBuilder::new(environment).secure_connect(
            grpc_base_url.as_str(),
            ChannelCredentialsBuilder::new().build(),
        );
    }
}
