pub mod grpc {
    pub mod resources;
    pub mod service;
    pub mod service_grpc;
    pub mod status;
    pub mod status_code;
    mod annotations;
    mod matrix;
    mod extensions;
    mod extension;
    mod scope;
    mod http;
    mod types;
}

pub mod clarifai_channel {
    use grpcio::{Channel, ChannelBuilder, ChannelCredentialsBuilder, EnvBuilder};
    use std::env;
    use std::sync::Arc;

    pub fn grpc() -> Channel {
        let environment = Arc::new(EnvBuilder::new().build());

        let grpc_base_url =
            env::var("CLARIFAI_GRPC_BASE").unwrap_or("api.clarifai.com".to_string());
        
        let mut channel_builder = ChannelBuilder::new(environment);
        channel_builder = channel_builder.set_credentials(ChannelCredentialsBuilder::new().build());

        return channel_builder.connect(
            grpc_base_url.as_str()
        );
    }
}
