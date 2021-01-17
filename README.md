![Clarifai logo](docs/logo.png)

# Clarifai Rust gRPC Client

This is the official Clarifai gRPC Rust client for interacting with our powerful recognition
[API](https://docs.clarifai.com).
Clarifai provides a platform for data scientists, developers, researchers and enterprises to master the entire
artificial intelligence lifecycle. Gather valuable business insights from images, video and text using computer vision
and natural language processing.

* Try the Clarifai demo at: https://clarifai.com/demo
* Sign up for a free account at: https://portal.clarifai.com/signup
* Read the documentation at: https://docs.clarifai.com/

[![crates.io](https://img.shields.io/crates/v/clarifai_grpc.svg)](https://crates.io/crates/clarifai_grpc)
[![Run tests](https://github.com/Clarifai/clarifai-rust-grpc/workflows/Run%20tests/badge.svg)](https://github.com/Clarifai/clarifai-rust-grpc/actions)

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
use clarifai_grpc::grpc::resources;
use clarifai_grpc::grpc::service;
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

> On Windows and macOS gRPC requires explicitly setting the root of trust for SSL. One way to do this is by setting the `GRPC_DEFAULT_SSL_ROOTS_FILE_PATH` environmental variable. To do this on macOS use:
>
> ```
> curl -Lo roots.pem https://raw.githubusercontent.com/grpc/grpc/master/etc/roots.pem
> export GRPC_DEFAULT_SSL_ROOTS_FILE_PATH="$PWD/roots.pem"
> ```
>
> On Windows use:
>
> ```
> @powershell -NoProfile -ExecutionPolicy unrestricted -Command ^
>     (new-object System.Net.WebClient).Downloadfile( ^
>         'https://raw.githubusercontent.com/grpc/grpc/master/etc/roots.pem', ^
>         'roots.pem')
> set GRPC_DEFAULT_SSL_ROOTS_FILE_PATH=%cd%\roots.pem
> ```
> See more [here](https://github.com/grpc/grpc/issues/16571).

Predict concepts in an image:

```rust
// This is a publicly available model.
const GENERAL_MODEL_ID: &str = "aaa03c23b3724a16a56b629203edc62c";

let request = service::PostModelOutputsRequest {
    model_id: GENERAL_MODEL_ID.to_string(),
    inputs: RepeatedField::from(vec![resources::Input {
        data: SingularPtrField::some(resources::Data {
            image: SingularPtrField::some(resources::Image {
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
