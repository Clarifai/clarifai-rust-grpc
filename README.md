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


# Installation

WIP

## Getting started

Construct the V2Stub object using which you'll access all the Clarifai API functionality:

```rust
use grpcio::{CallOption, MetadataBuilder};
use protobuf::{RepeatedField};

use crate::grpc::resources::{Data, Image, Input};
use crate::grpc::service::PostModelOutputsRequest;
use crate::grpc::service_grpc::V2Client;
use crate::grpc::status_code::StatusCode;

use clarifai_grpc::clarifai_channel;

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
let mut request = PostModelOutputsRequest::default();
// This is the ID of the publicly available General model.
request.set_model_id("aaa03c23b3724a16a56b629203edc62c".to_string());
let mut image = Image::new();
image.set_url("https://samples.clarifai.com/dog2.jpeg".parse().unwrap());
let mut data = Data::new();
data.set_image(image);
let mut input = Input::new();
input.set_data(data);
let mut inputs: Vec<Input> = Vec::new();
inputs.push(input);
request.set_inputs(RepeatedField::from(inputs));
let response = client
    .post_model_outputs_opt(&request, call_opt)
    .expect("Failure");

let status = response.get_status();
if status.get_code() != StatusCode::SUCCESS {
    println!("Failure response:");
    println!("\t{}", status.get_code().value());
    println!("\t{}", status.get_description());
    println!("\t{}", status.get_details());
    exit(1);
}

println!("Predicted concepts:");
for concept in response.get_outputs()[0].get_data().get_concepts() {
    println!("\t{}: {}", concept.get_name(), concept.get_value());
}
```
