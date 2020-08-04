use std::env;

use grpcio::{CallOption, MetadataBuilder};
use protobuf::{RepeatedField, ProtobufEnum};

use std::process::exit;

use crate::grpc::status_code::StatusCode;
use crate::grpc::service_grpc::V2Client;
use crate::grpc::service::PostModelOutputsRequest;
use crate::grpc::resources::{Image, Data, Input};
use clarifai_grpc::clarifai::insecure_grpc;
use std::fs::File;
use std::io::Read;

mod grpc {
    pub mod service;
    pub mod resources;
    pub mod service_grpc;
    pub mod status;
    pub mod status_code;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let api_key = env::var("CLARIFAI_API_KEY")
        .expect("Please set an environmental variable CLARIFAI_API_KEY");
    let auth = "Key ".to_owned() + &api_key;
    let general_model_id = "aaa03c23b3724a16a56b629203edc62c";

    let ch = insecure_grpc();
    let client = V2Client::new(ch);

    let mut builder = MetadataBuilder::with_capacity(3);
    builder
        .add_str("Authorization", &auth)
        .unwrap()
    ;
    let metadata = builder.build();
    let call_opt = CallOption::default().headers(metadata);

    let mut req = PostModelOutputsRequest::default();
    req.set_model_id(general_model_id.parse().unwrap());

    let mut image = Image::new();
    image.set_base64(read_file(file_name));
    let mut data = Data::new();
    data.set_image(image);
    let mut input = Input::new();
    input.set_data(data);
    let mut inputs: Vec<Input> = Vec::new();

    inputs.push(input);
    req.set_inputs(RepeatedField::from(inputs));
    let response = client.post_model_outputs_opt(&req, call_opt).expect("rpc");

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
}

fn read_file(file_name: &String) -> Vec<u8> {
    let mut file = File::open(file_name).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    buffer
}
