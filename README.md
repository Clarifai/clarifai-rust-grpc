![Clarifai logo](docs/logo.png)

# Clarifai Rust gRPC Client

This is the official Clarifai gRPC Rust client for interacting with our powerful recognition
[API](https://docs.clarifai.com).
The Clarifai API offers image and video recognition as a service. Whether you have one image or
billions, you are only steps away from using artificial intelligence to recognize your visual
content.

* Try the Clarifai demo at: https://clarifai.com/demo
* Sign up for a free account at: https://portal.clarifai.com/signup
* Read the documentation at: https://docs.clarifai.com/

[![crates.io](https://img.shields.io/crates/v/clarifai_grpc.svg)](https://crates.io/crates/clarifai_grpc)
![Run tests](https://github.com/Clarifai/clarifai-rust-grpc/workflows/Run%20tests/badge.svg)

# Installation

Add these dependencies to `Cargo.toml`: `clarifai_grpc`, `protobuf` and `grpcio`.

```
[dependencies]
clarifai_grpc = "*"
grpcio = "0.6.0"
protobuf = "2.0"
```

## Getting started

Construct the `V2Client` object using which you'll access all the Clarifai API functionality,
and a `CallOption` object that will be used for authentication.

```rust
use grpcio::{CallOption, MetadataBuilder};
use protobuf::{RepeatedField, SingularPtrField};

use clarifai_grpc::clarifai_channel;
use clarifai_grpc::grpc::resources::{Data, Image, Input};
use clarifai_grpc::grpc::service::PostModelOutputsRequest;
use clarifai_grpc::grpc::service_grpc::V2Client;
use clarifai_grpc::grpc::status_code::StatusCode;

let client = V2Client::new(clarifai_channel::grpc());

// Setup authentication.
let auth = "Key YOUR_CLARIFAI_API_KEY_OR_PAT".to_string();

let mut builder = MetadataBuilder::with_capacity(1);
builder.add_str("Authorization", &auth).unwrap();
let metadata = builder.build();
let call_opt = CallOption::default().headers(metadata);
```

Predict concepts in an image:

```rust
// This is a publicly available model.
const GENERAL_MODEL_ID: &str = "aaa03c23b3724a16a56b629203edc62c";

let request = PostModelOutputsRequest {
    model_id: GENERAL_MODEL_ID.to_string(),
    inputs: RepeatedField::from(vec![Input {
        data: SingularPtrField::some(Data {
            image: SingularPtrField::some(Image {
                url: "https://samples.clarifai.com/dog2.jpeg".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    }]),
    ..Default::default()
};
let response = client
    .post_model_outputs_opt(&request, call_opt)
    .expect("Failure");

let status = response.get_status();
if status.get_code() != StatusCode::SUCCESS {
    println!("Failure response:");
    println!("\t{:?}", status.get_code());
    println!("\t{}", status.get_description());
    println!("\t{}", status.get_details());
    std::process::exit(1);
}

println!("Predicted concepts:");
for concept in response.get_outputs()[0].get_data().get_concepts() {
    println!("\t{}: {}", concept.get_name(), concept.get_value());
}
```
