use std::env;

use grpcio::{CallOption, MetadataBuilder};
use protobuf::{ProtobufEnum, RepeatedField, SingularPtrField};

extern crate clarifai_grpc;

use clarifai_grpc::clarifai_channel;
use clarifai_grpc::grpc::resources::{Concept, Data, Image, Input};
use clarifai_grpc::grpc::service::{
    DeleteInputRequest, GetInputRequest, GetModelRequest, ListModelsRequest, PatchInputsRequest,
    PostInputsRequest, PostModelOutputsRequest,
};
use clarifai_grpc::grpc::service_grpc::V2Client;
use clarifai_grpc::grpc::status::Status;
use clarifai_grpc::grpc::status_code::StatusCode;
use std::thread::sleep;
use std::time::Duration;

const GENERAL_MODEL_ID: &str = "aaa03c23b3724a16a56b629203edc62c";
const TRUCK_IMAGE_URL: &str = "https://samples.clarifai.com/red-truck.png";
const DOG_IMAGE_URL: &str = "https://samples.clarifai.com/dog2.jpeg";
const NON_EXISTING_IMAGE_URL: &str = "https://samples.clarifai.com/non-existing-image.jpeg";

#[test]
fn test_get_model() {
    let request = GetModelRequest {
        model_id: GENERAL_MODEL_ID.to_string(),
        ..Default::default()
    };

    let response = client()
        .get_model_opt(&request, call_opt())
        .expect("Failure");

    assert_success_response(response.get_status());

    assert_eq!("general", response.get_model().name);
}

#[test]
fn test_list_model_with_pagination() {
    let request = ListModelsRequest {
        per_page: 2,
        ..Default::default()
    };

    let response = client()
        .list_models_opt(&request, call_opt())
        .expect("Failure");

    assert_success_response(response.get_status());

    assert_eq!(2, response.get_models().len());
}

#[test]
fn test_post_model_outputs_url() {
    let request = PostModelOutputsRequest {
        model_id: GENERAL_MODEL_ID.to_string(),
        inputs: RepeatedField::from(vec![Input {
            data: SingularPtrField::some(Data {
                image: SingularPtrField::some(Image {
                    url: DOG_IMAGE_URL.to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        }]),
        ..Default::default()
    };

    let response = client()
        .post_model_outputs_opt(&request, call_opt())
        .expect("Failure");

    assert_success_response(response.get_status());

    assert_ne!(0, response.get_outputs()[0].get_data().get_concepts().len());
}

#[test]
fn test_failed_post_model_outputs() {
    let request = PostModelOutputsRequest {
        model_id: GENERAL_MODEL_ID.to_string(),
        inputs: RepeatedField::from(vec![Input {
            data: SingularPtrField::some(Data {
                image: SingularPtrField::some(Image {
                    url: NON_EXISTING_IMAGE_URL.to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        }]),
        ..Default::default()
    };

    let response = client()
        .post_model_outputs_opt(&request, call_opt())
        .expect("Failure");

    assert_eq!(StatusCode::FAILURE, response.get_status().code);
}

#[test]
fn test_mixed_success_post_model_outputs() {
    let request = PostModelOutputsRequest {
        model_id: GENERAL_MODEL_ID.to_string(),
        inputs: RepeatedField::from(vec![
            Input {
                data: SingularPtrField::some(Data {
                    image: SingularPtrField::some(Image {
                        url: DOG_IMAGE_URL.to_string(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
            Input {
                data: SingularPtrField::some(Data {
                    image: SingularPtrField::some(Image {
                        url: NON_EXISTING_IMAGE_URL.to_string(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
        ]),
        ..Default::default()
    };

    let response = client()
        .post_model_outputs_opt(&request, call_opt())
        .expect("Failure");

    assert_eq!(StatusCode::MIXED_STATUS, response.get_status().code);
    assert_eq!(
        StatusCode::SUCCESS,
        response.get_outputs()[0].get_status().code
    );
    assert_eq!(
        StatusCode::INPUT_DOWNLOAD_FAILED,
        response.get_outputs()[1].get_status().code
    );
}

#[test]
fn test_post_patch_and_delete_input() {
    let post_inputs_response = client()
        .post_inputs_opt(
            &PostInputsRequest {
                inputs: RepeatedField::from(vec![Input {
                    data: SingularPtrField::some(Data {
                        image: SingularPtrField::some(Image {
                            url: TRUCK_IMAGE_URL.to_string(),
                            allow_duplicate_url: true,
                            ..Default::default()
                        }),
                        concepts: RepeatedField::from(vec![Concept {
                            id: "red-truck".to_string(),
                            ..Default::default()
                        }]),
                        ..Default::default()
                    }),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            call_opt(),
        )
        .expect("Failure");

    assert_success_response(post_inputs_response.get_status());

    let input_id = post_inputs_response
        .get_inputs()
        .get(0)
        .expect("Failure")
        .get_id();
    loop {
        let get_input_response = client()
            .get_input_opt(
                &GetInputRequest {
                    input_id: input_id.to_string(),
                    ..Default::default()
                },
                call_opt(),
            )
            .expect("Failure");
        assert_success_response(get_input_response.get_status());
        let input_status_code = get_input_response.get_input().get_status().get_code();
        if input_status_code == StatusCode::INPUT_DOWNLOAD_SUCCESS {
            break;
        }
        if input_status_code != StatusCode::INPUT_DOWNLOAD_PENDING
            && input_status_code != StatusCode::INPUT_DOWNLOAD_IN_PROGRESS
        {
            panic!(format!(
                "Waiting for input ID {} failed, status code is {:?}",
                input_id, input_status_code
            ));
        }
        sleep(Duration::from_secs(1));
    }

    let patch_inputs_response = client()
        .patch_inputs_opt(
            &PatchInputsRequest {
                action: "overwrite".to_string(),
                inputs: RepeatedField::from(vec![Input {
                    id: input_id.to_string(),
                    data: SingularPtrField::some(Data {
                        concepts: RepeatedField::from(vec![Concept {
                            id: "very-red-truck".to_string(),
                            ..Default::default()
                        }]),
                        ..Default::default()
                    }),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            call_opt(),
        )
        .expect("Failure");

    assert_success_response(patch_inputs_response.get_status());

    let delete_input_response = client()
        .delete_input_opt(
            &DeleteInputRequest {
                input_id: input_id.to_string(),
                ..Default::default()
            },
            call_opt(),
        )
        .expect("Failure");
    assert_success_response(delete_input_response.get_status());
}

fn client() -> V2Client {
    let client = V2Client::new(clarifai_channel::grpc());
    client
}

fn call_opt() -> CallOption {
    let api_key = env::var("CLARIFAI_API_KEY")
        .expect("Please set an environmental variable CLARIFAI_API_KEY");
    let auth = "Key ".to_string() + &api_key;

    let mut builder = MetadataBuilder::with_capacity(1);
    builder.add_str("Authorization", &auth).unwrap();
    let metadata = builder.build();
    let call_opt = CallOption::default().headers(metadata);
    call_opt
}

fn assert_success_response(status: &Status) {
    if status.get_code() != StatusCode::SUCCESS {
        println!("Failure response:");
        println!("\t{}", status.get_code().value());
        println!("\t{}", status.get_description());
        println!("\t{}", status.get_details());
        assert!(false);
    }
}
