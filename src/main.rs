use std::sync::Arc;

use grpcio::{CallOption, ChannelBuilder, EnvBuilder, MetadataBuilder};
use protobuf::RepeatedField;

use crate::resources::{Data, Image, Input};
use crate::service::PostModelOutputsRequest;
use crate::service_grpc::V2Client;

mod status;
mod status_code;
mod service_grpc;
mod resources;
mod service;

fn main() {
    let auth = "Key MY_CLARIFAI_API_KEY";
    let general_model_id = "aaa03c23b3724a16a56b629203edc62c";

    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("api-grpc.clarifai.com:18080");
    let client = V2Client::new(ch);

    let mut builder = MetadataBuilder::with_capacity(3);
    builder
        .add_str("Authorization", auth)
        .unwrap()
    ;
    let metadata = builder.build();
    let call_opt = CallOption::default().headers(metadata);

    let mut req = PostModelOutputsRequest::default();
    req.set_model_id(general_model_id.parse().unwrap());

    let mut image = Image::new();
    image.set_url("https://samples.clarifai.com/dog2.jpeg".parse().unwrap());
    let mut data = Data::new();
    data.set_image(image);
    let mut input = Input::new();
    input.set_data(data);
    let mut inputs: Vec<Input> = Vec::new();

    inputs.push(input);
    req.set_inputs(RepeatedField::from(inputs));
    let response = client.post_model_outputs_opt(&req, call_opt).expect("rpc");

    println!("Status:");
    println!("{}", response.get_status().get_description());
    println!("{}", response.get_status().get_details());

    println!("Predicted concepts:");
    for concept in response.get_outputs()[0].get_data().get_concepts() {
        println!("\t{}: {}", concept.get_name(), concept.get_value());
    }
}
