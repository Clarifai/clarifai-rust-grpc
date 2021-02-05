// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_V2_LIST_CONCEPT_RELATIONS: ::grpcio::Method<super::service::ListConceptRelationsRequest, super::service::MultiConceptRelationResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListConceptRelations",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_CONCEPT_RELATIONS: ::grpcio::Method<super::service::PostConceptRelationsRequest, super::service::MultiConceptRelationResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostConceptRelations",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_DELETE_CONCEPT_RELATIONS: ::grpcio::Method<super::service::DeleteConceptRelationsRequest, super::status::BaseResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/DeleteConceptRelations",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_GET_CONCEPT_COUNTS: ::grpcio::Method<super::service::GetConceptCountsRequest, super::service::MultiConceptCountResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetConceptCounts",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_GET_CONCEPT: ::grpcio::Method<super::service::GetConceptRequest, super::service::SingleConceptResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetConcept",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_LIST_CONCEPTS: ::grpcio::Method<super::service::ListConceptsRequest, super::service::MultiConceptResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListConcepts",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_CONCEPTS_SEARCHES: ::grpcio::Method<super::service::PostConceptsSearchesRequest, super::service::MultiConceptResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostConceptsSearches",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_CONCEPTS: ::grpcio::Method<super::service::PostConceptsRequest, super::service::MultiConceptResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostConcepts",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_PATCH_CONCEPTS: ::grpcio::Method<super::service::PatchConceptsRequest, super::service::MultiConceptResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PatchConcepts",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_GET_CONCEPT_LANGUAGE: ::grpcio::Method<super::service::GetConceptLanguageRequest, super::service::SingleConceptLanguageResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetConceptLanguage",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_LIST_CONCEPT_LANGUAGES: ::grpcio::Method<super::service::ListConceptLanguagesRequest, super::service::MultiConceptLanguageResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListConceptLanguages",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_CONCEPT_LANGUAGES: ::grpcio::Method<super::service::PostConceptLanguagesRequest, super::service::MultiConceptLanguageResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostConceptLanguages",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_PATCH_CONCEPT_LANGUAGES: ::grpcio::Method<super::service::PatchConceptLanguagesRequest, super::service::MultiConceptLanguageResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PatchConceptLanguages",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_LIST_KNOWLEDGE_GRAPHS: ::grpcio::Method<super::service::ListKnowledgeGraphsRequest, super::service::MultiKnowledgeGraphResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListKnowledgeGraphs",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_KNOWLEDGE_GRAPHS: ::grpcio::Method<super::service::PostKnowledgeGraphsRequest, super::service::MultiKnowledgeGraphResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostKnowledgeGraphs",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_CONCEPT_MAPPING_JOBS: ::grpcio::Method<super::service::PostConceptMappingJobsRequest, super::service::MultiConceptMappingJobResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostConceptMappingJobs",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_GET_ANNOTATION: ::grpcio::Method<super::service::GetAnnotationRequest, super::service::SingleAnnotationResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetAnnotation",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_LIST_ANNOTATIONS: ::grpcio::Method<super::service::ListAnnotationsRequest, super::service::MultiAnnotationResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListAnnotations",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_ANNOTATIONS: ::grpcio::Method<super::service::PostAnnotationsRequest, super::service::MultiAnnotationResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostAnnotations",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_PATCH_ANNOTATIONS: ::grpcio::Method<super::service::PatchAnnotationsRequest, super::service::MultiAnnotationResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PatchAnnotations",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_PATCH_ANNOTATIONS_STATUS: ::grpcio::Method<super::service::PatchAnnotationsStatusRequest, super::service::PatchAnnotationsStatusResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PatchAnnotationsStatus",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_DELETE_ANNOTATION: ::grpcio::Method<super::service::DeleteAnnotationRequest, super::status::BaseResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/DeleteAnnotation",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_DELETE_ANNOTATIONS: ::grpcio::Method<super::service::DeleteAnnotationsRequest, super::status::BaseResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/DeleteAnnotations",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_ANNOTATIONS_SEARCHES: ::grpcio::Method<super::service::PostAnnotationsSearchesRequest, super::service::MultiSearchResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostAnnotationsSearches",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_GET_INPUT_COUNT: ::grpcio::Method<super::service::GetInputCountRequest, super::service::SingleInputCountResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetInputCount",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_STREAM_INPUTS: ::grpcio::Method<super::service::StreamInputsRequest, super::service::MultiInputResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/StreamInputs",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_GET_INPUT_SAMPLES: ::grpcio::Method<super::service::GetInputSamplesRequest, super::service::MultiInputAnnotationResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetInputSamples",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_GET_INPUT: ::grpcio::Method<super::service::GetInputRequest, super::service::SingleInputResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetInput",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_LIST_INPUTS: ::grpcio::Method<super::service::ListInputsRequest, super::service::MultiInputResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListInputs",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_INPUTS: ::grpcio::Method<super::service::PostInputsRequest, super::service::MultiInputResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostInputs",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_PATCH_INPUTS: ::grpcio::Method<super::service::PatchInputsRequest, super::service::MultiInputResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PatchInputs",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_DELETE_INPUT: ::grpcio::Method<super::service::DeleteInputRequest, super::status::BaseResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/DeleteInput",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_DELETE_INPUTS: ::grpcio::Method<super::service::DeleteInputsRequest, super::status::BaseResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/DeleteInputs",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_INPUTS_SEARCHES: ::grpcio::Method<super::service::PostInputsSearchesRequest, super::service::MultiSearchResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostInputsSearches",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_MODEL_OUTPUTS: ::grpcio::Method<super::service::PostModelOutputsRequest, super::service::MultiOutputResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostModelOutputs",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_GET_MODEL_TYPE: ::grpcio::Method<super::service::GetModelTypeRequest, super::service::SingleModelTypeResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetModelType",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_LIST_MODEL_TYPES: ::grpcio::Method<super::service::ListModelTypesRequest, super::service::MultiModelTypeResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListModelTypes",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_GET_MODEL: ::grpcio::Method<super::service::GetModelRequest, super::service::SingleModelResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetModel",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_GET_MODEL_OUTPUT_INFO: ::grpcio::Method<super::service::GetModelRequest, super::service::SingleModelResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetModelOutputInfo",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_LIST_MODELS: ::grpcio::Method<super::service::ListModelsRequest, super::service::MultiModelResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListModels",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_MODELS_SEARCHES: ::grpcio::Method<super::service::PostModelsSearchesRequest, super::service::MultiModelResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostModelsSearches",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_MODELS: ::grpcio::Method<super::service::PostModelsRequest, super::service::SingleModelResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostModels",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_PATCH_MODELS: ::grpcio::Method<super::service::PatchModelsRequest, super::service::MultiModelResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PatchModels",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_DELETE_MODEL: ::grpcio::Method<super::service::DeleteModelRequest, super::status::BaseResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/DeleteModel",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_DELETE_MODELS: ::grpcio::Method<super::service::DeleteModelsRequest, super::status::BaseResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/DeleteModels",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_LIST_MODEL_INPUTS: ::grpcio::Method<super::service::ListModelInputsRequest, super::service::MultiInputResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListModelInputs",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_GET_MODEL_VERSION: ::grpcio::Method<super::service::GetModelVersionRequest, super::service::SingleModelVersionResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetModelVersion",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_LIST_MODEL_VERSIONS: ::grpcio::Method<super::service::ListModelVersionsRequest, super::service::MultiModelVersionResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListModelVersions",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_MODEL_VERSIONS: ::grpcio::Method<super::service::PostModelVersionsRequest, super::service::SingleModelResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostModelVersions",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_DELETE_MODEL_VERSION: ::grpcio::Method<super::service::DeleteModelVersionRequest, super::status::BaseResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/DeleteModelVersion",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_GET_MODEL_VERSION_METRICS: ::grpcio::Method<super::service::GetModelVersionMetricsRequest, super::service::SingleModelVersionResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetModelVersionMetrics",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_MODEL_VERSION_METRICS: ::grpcio::Method<super::service::PostModelVersionMetricsRequest, super::service::SingleModelVersionResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostModelVersionMetrics",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_GET_WORKFLOW: ::grpcio::Method<super::service::GetWorkflowRequest, super::service::SingleWorkflowResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetWorkflow",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_LIST_WORKFLOWS: ::grpcio::Method<super::service::ListWorkflowsRequest, super::service::MultiWorkflowResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListWorkflows",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_WORKFLOWS: ::grpcio::Method<super::service::PostWorkflowsRequest, super::service::MultiWorkflowResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostWorkflows",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_PATCH_WORKFLOWS: ::grpcio::Method<super::service::PatchWorkflowsRequest, super::service::MultiWorkflowResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PatchWorkflows",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_DELETE_WORKFLOW: ::grpcio::Method<super::service::DeleteWorkflowRequest, super::status::BaseResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/DeleteWorkflow",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_DELETE_WORKFLOWS: ::grpcio::Method<super::service::DeleteWorkflowsRequest, super::status::BaseResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/DeleteWorkflows",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_WORKFLOW_RESULTS: ::grpcio::Method<super::service::PostWorkflowResultsRequest, super::service::PostWorkflowResultsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostWorkflowResults",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_WORKFLOW_RESULTS_SIMILARITY: ::grpcio::Method<super::service::PostWorkflowResultsSimilarityRequest, super::service::PostWorkflowResultsSimilarityResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostWorkflowResultsSimilarity",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_GET_KEY: ::grpcio::Method<super::service::GetKeyRequest, super::service::SingleKeyResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetKey",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_LIST_KEYS: ::grpcio::Method<super::service::ListKeysRequest, super::service::MultiKeyResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListKeys",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_LIST_APP_KEYS: ::grpcio::Method<super::service::ListAppKeysRequest, super::service::MultiKeyResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListAppKeys",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_DELETE_KEY: ::grpcio::Method<super::service::DeleteKeyRequest, super::status::BaseResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/DeleteKey",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_KEYS: ::grpcio::Method<super::service::PostKeysRequest, super::service::MultiKeyResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostKeys",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_PATCH_KEYS: ::grpcio::Method<super::service::PatchKeysRequest, super::service::MultiKeyResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PatchKeys",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_MY_SCOPES: ::grpcio::Method<super::service::MyScopesRequest, super::service::MultiScopeResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/MyScopes",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_LIST_SCOPES: ::grpcio::Method<super::service::ListScopesRequest, super::service::MultiScopeDepsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListScopes",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_GET_APP: ::grpcio::Method<super::service::GetAppRequest, super::service::SingleAppResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetApp",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_LIST_APPS: ::grpcio::Method<super::service::ListAppsRequest, super::service::MultiAppResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListApps",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_DELETE_APP: ::grpcio::Method<super::service::DeleteAppRequest, super::status::BaseResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/DeleteApp",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_APPS: ::grpcio::Method<super::service::PostAppsRequest, super::service::MultiAppResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostApps",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_PATCH_APPS: ::grpcio::Method<super::service::PatchAppsRequest, super::service::MultiAppResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PatchApps",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_APPS_SEARCHES: ::grpcio::Method<super::service::PostAppsSearchesRequest, super::service::MultiAppResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostAppsSearches",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_VALIDATE_PASSWORD: ::grpcio::Method<super::service::PostValidatePasswordRequest, super::service::SinglePasswordValidationResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostValidatePassword",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_GET_SEARCH: ::grpcio::Method<super::service::GetSearchRequest, super::service::SingleSearchResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetSearch",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_LIST_SEARCHES: ::grpcio::Method<super::service::ListSearchesRequest, super::service::MultiSearchResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListSearches",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_SEARCHES: ::grpcio::Method<super::service::PostSearchesRequest, super::service::MultiSearchResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostSearches",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_SEARCHES_BY_ID: ::grpcio::Method<super::service::PostSearchesByIDRequest, super::service::MultiSearchResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostSearchesByID",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_ANNOTATION_SEARCH_METRICS: ::grpcio::Method<super::service::PostAnnotationSearchMetricsRequest, super::service::MultiAnnotationSearchMetricsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostAnnotationSearchMetrics",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_GET_ANNOTATION_SEARCH_METRICS: ::grpcio::Method<super::service::GetAnnotationSearchMetricsRequest, super::service::MultiAnnotationSearchMetricsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetAnnotationSearchMetrics",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_LIST_ANNOTATION_SEARCH_METRICS: ::grpcio::Method<super::service::ListAnnotationSearchMetricsRequest, super::service::MultiAnnotationSearchMetricsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListAnnotationSearchMetrics",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_DELETE_ANNOTATION_SEARCH_METRICS: ::grpcio::Method<super::service::DeleteAnnotationSearchMetricsRequest, super::status::BaseResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/DeleteAnnotationSearchMetrics",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_DELETE_SEARCH: ::grpcio::Method<super::service::DeleteSearchRequest, super::status::BaseResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/DeleteSearch",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_LIST_STATUS_CODES: ::grpcio::Method<super::service::ListStatusCodesRequest, super::service::MultiStatusCodeResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListStatusCodes",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_GET_STATUS_CODE: ::grpcio::Method<super::service::GetStatusCodeRequest, super::service::SingleStatusCodeResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetStatusCode",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_LIST_COLLABORATORS: ::grpcio::Method<super::service::ListCollaboratorsRequest, super::service::MultiCollaboratorsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListCollaborators",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_COLLABORATORS: ::grpcio::Method<super::service::PostCollaboratorsRequest, super::service::MultiCollaboratorsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostCollaborators",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_PATCH_COLLABORATORS: ::grpcio::Method<super::service::PatchCollaboratorsRequest, super::service::MultiCollaboratorsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PatchCollaborators",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_DELETE_COLLABORATORS: ::grpcio::Method<super::service::DeleteCollaboratorsRequest, super::status::BaseResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/DeleteCollaborators",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_LIST_COLLABORATIONS: ::grpcio::Method<super::service::ListCollaborationsRequest, super::service::MultiCollaborationsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListCollaborations",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_APP_DUPLICATIONS: ::grpcio::Method<super::service::PostAppDuplicationsRequest, super::service::MultiAppDuplicationsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostAppDuplications",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_LIST_APP_DUPLICATIONS: ::grpcio::Method<super::service::ListAppDuplicationsRequest, super::service::MultiAppDuplicationsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListAppDuplications",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_GET_APP_DUPLICATION: ::grpcio::Method<super::service::GetAppDuplicationRequest, super::service::SingleAppDuplicationResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetAppDuplication",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_TASKS: ::grpcio::Method<super::service::PostTasksRequest, super::service::MultiTaskResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostTasks",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_GET_TASK_ANNOTATION_COUNT: ::grpcio::Method<super::service::GetTaskCountRequest, super::service::SingleTaskCountResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetTaskAnnotationCount",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_GET_TASK_INPUT_COUNT: ::grpcio::Method<super::service::GetTaskCountRequest, super::service::SingleTaskCountResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetTaskInputCount",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_GET_TASK: ::grpcio::Method<super::service::GetTaskRequest, super::service::SingleTaskResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetTask",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_LIST_TASKS: ::grpcio::Method<super::service::ListTasksRequest, super::service::MultiTaskResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListTasks",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_PATCH_TASKS: ::grpcio::Method<super::service::PatchTasksRequest, super::service::MultiTaskResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PatchTasks",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_DELETE_TASKS: ::grpcio::Method<super::service::DeleteTasksRequest, super::status::BaseResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/DeleteTasks",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_COLLECTORS: ::grpcio::Method<super::service::PostCollectorsRequest, super::service::MultiCollectorResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostCollectors",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_GET_COLLECTOR: ::grpcio::Method<super::service::GetCollectorRequest, super::service::SingleCollectorResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetCollector",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_LIST_COLLECTORS: ::grpcio::Method<super::service::ListCollectorsRequest, super::service::MultiCollectorResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListCollectors",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_PATCH_COLLECTORS: ::grpcio::Method<super::service::PatchCollectorsRequest, super::service::MultiCollectorResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PatchCollectors",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_DELETE_COLLECTORS: ::grpcio::Method<super::service::DeleteCollectorsRequest, super::status::BaseResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/DeleteCollectors",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_STAT_VALUES: ::grpcio::Method<super::service::PostStatValuesRequest, super::service::MultiStatValueResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostStatValues",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_STAT_VALUES_AGGREGATE: ::grpcio::Method<super::service::PostStatValuesAggregateRequest, super::service::MultiStatValueAggregateResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostStatValuesAggregate",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct V2Client {
    client: ::grpcio::Client,
}

impl V2Client {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        V2Client {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn list_concept_relations_opt(&self, req: &super::service::ListConceptRelationsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiConceptRelationResponse> {
        self.client.unary_call(&METHOD_V2_LIST_CONCEPT_RELATIONS, req, opt)
    }

    pub fn list_concept_relations(&self, req: &super::service::ListConceptRelationsRequest) -> ::grpcio::Result<super::service::MultiConceptRelationResponse> {
        self.list_concept_relations_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_concept_relations_async_opt(&self, req: &super::service::ListConceptRelationsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiConceptRelationResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_CONCEPT_RELATIONS, req, opt)
    }

    pub fn list_concept_relations_async(&self, req: &super::service::ListConceptRelationsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiConceptRelationResponse>> {
        self.list_concept_relations_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_concept_relations_opt(&self, req: &super::service::PostConceptRelationsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiConceptRelationResponse> {
        self.client.unary_call(&METHOD_V2_POST_CONCEPT_RELATIONS, req, opt)
    }

    pub fn post_concept_relations(&self, req: &super::service::PostConceptRelationsRequest) -> ::grpcio::Result<super::service::MultiConceptRelationResponse> {
        self.post_concept_relations_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_concept_relations_async_opt(&self, req: &super::service::PostConceptRelationsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiConceptRelationResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_CONCEPT_RELATIONS, req, opt)
    }

    pub fn post_concept_relations_async(&self, req: &super::service::PostConceptRelationsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiConceptRelationResponse>> {
        self.post_concept_relations_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_concept_relations_opt(&self, req: &super::service::DeleteConceptRelationsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::status::BaseResponse> {
        self.client.unary_call(&METHOD_V2_DELETE_CONCEPT_RELATIONS, req, opt)
    }

    pub fn delete_concept_relations(&self, req: &super::service::DeleteConceptRelationsRequest) -> ::grpcio::Result<super::status::BaseResponse> {
        self.delete_concept_relations_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_concept_relations_async_opt(&self, req: &super::service::DeleteConceptRelationsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.client.unary_call_async(&METHOD_V2_DELETE_CONCEPT_RELATIONS, req, opt)
    }

    pub fn delete_concept_relations_async(&self, req: &super::service::DeleteConceptRelationsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.delete_concept_relations_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_concept_counts_opt(&self, req: &super::service::GetConceptCountsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiConceptCountResponse> {
        self.client.unary_call(&METHOD_V2_GET_CONCEPT_COUNTS, req, opt)
    }

    pub fn get_concept_counts(&self, req: &super::service::GetConceptCountsRequest) -> ::grpcio::Result<super::service::MultiConceptCountResponse> {
        self.get_concept_counts_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_concept_counts_async_opt(&self, req: &super::service::GetConceptCountsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiConceptCountResponse>> {
        self.client.unary_call_async(&METHOD_V2_GET_CONCEPT_COUNTS, req, opt)
    }

    pub fn get_concept_counts_async(&self, req: &super::service::GetConceptCountsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiConceptCountResponse>> {
        self.get_concept_counts_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_concept_opt(&self, req: &super::service::GetConceptRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleConceptResponse> {
        self.client.unary_call(&METHOD_V2_GET_CONCEPT, req, opt)
    }

    pub fn get_concept(&self, req: &super::service::GetConceptRequest) -> ::grpcio::Result<super::service::SingleConceptResponse> {
        self.get_concept_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_concept_async_opt(&self, req: &super::service::GetConceptRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleConceptResponse>> {
        self.client.unary_call_async(&METHOD_V2_GET_CONCEPT, req, opt)
    }

    pub fn get_concept_async(&self, req: &super::service::GetConceptRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleConceptResponse>> {
        self.get_concept_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_concepts_opt(&self, req: &super::service::ListConceptsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiConceptResponse> {
        self.client.unary_call(&METHOD_V2_LIST_CONCEPTS, req, opt)
    }

    pub fn list_concepts(&self, req: &super::service::ListConceptsRequest) -> ::grpcio::Result<super::service::MultiConceptResponse> {
        self.list_concepts_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_concepts_async_opt(&self, req: &super::service::ListConceptsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiConceptResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_CONCEPTS, req, opt)
    }

    pub fn list_concepts_async(&self, req: &super::service::ListConceptsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiConceptResponse>> {
        self.list_concepts_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_concepts_searches_opt(&self, req: &super::service::PostConceptsSearchesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiConceptResponse> {
        self.client.unary_call(&METHOD_V2_POST_CONCEPTS_SEARCHES, req, opt)
    }

    pub fn post_concepts_searches(&self, req: &super::service::PostConceptsSearchesRequest) -> ::grpcio::Result<super::service::MultiConceptResponse> {
        self.post_concepts_searches_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_concepts_searches_async_opt(&self, req: &super::service::PostConceptsSearchesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiConceptResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_CONCEPTS_SEARCHES, req, opt)
    }

    pub fn post_concepts_searches_async(&self, req: &super::service::PostConceptsSearchesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiConceptResponse>> {
        self.post_concepts_searches_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_concepts_opt(&self, req: &super::service::PostConceptsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiConceptResponse> {
        self.client.unary_call(&METHOD_V2_POST_CONCEPTS, req, opt)
    }

    pub fn post_concepts(&self, req: &super::service::PostConceptsRequest) -> ::grpcio::Result<super::service::MultiConceptResponse> {
        self.post_concepts_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_concepts_async_opt(&self, req: &super::service::PostConceptsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiConceptResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_CONCEPTS, req, opt)
    }

    pub fn post_concepts_async(&self, req: &super::service::PostConceptsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiConceptResponse>> {
        self.post_concepts_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_concepts_opt(&self, req: &super::service::PatchConceptsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiConceptResponse> {
        self.client.unary_call(&METHOD_V2_PATCH_CONCEPTS, req, opt)
    }

    pub fn patch_concepts(&self, req: &super::service::PatchConceptsRequest) -> ::grpcio::Result<super::service::MultiConceptResponse> {
        self.patch_concepts_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_concepts_async_opt(&self, req: &super::service::PatchConceptsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiConceptResponse>> {
        self.client.unary_call_async(&METHOD_V2_PATCH_CONCEPTS, req, opt)
    }

    pub fn patch_concepts_async(&self, req: &super::service::PatchConceptsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiConceptResponse>> {
        self.patch_concepts_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_concept_language_opt(&self, req: &super::service::GetConceptLanguageRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleConceptLanguageResponse> {
        self.client.unary_call(&METHOD_V2_GET_CONCEPT_LANGUAGE, req, opt)
    }

    pub fn get_concept_language(&self, req: &super::service::GetConceptLanguageRequest) -> ::grpcio::Result<super::service::SingleConceptLanguageResponse> {
        self.get_concept_language_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_concept_language_async_opt(&self, req: &super::service::GetConceptLanguageRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleConceptLanguageResponse>> {
        self.client.unary_call_async(&METHOD_V2_GET_CONCEPT_LANGUAGE, req, opt)
    }

    pub fn get_concept_language_async(&self, req: &super::service::GetConceptLanguageRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleConceptLanguageResponse>> {
        self.get_concept_language_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_concept_languages_opt(&self, req: &super::service::ListConceptLanguagesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiConceptLanguageResponse> {
        self.client.unary_call(&METHOD_V2_LIST_CONCEPT_LANGUAGES, req, opt)
    }

    pub fn list_concept_languages(&self, req: &super::service::ListConceptLanguagesRequest) -> ::grpcio::Result<super::service::MultiConceptLanguageResponse> {
        self.list_concept_languages_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_concept_languages_async_opt(&self, req: &super::service::ListConceptLanguagesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiConceptLanguageResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_CONCEPT_LANGUAGES, req, opt)
    }

    pub fn list_concept_languages_async(&self, req: &super::service::ListConceptLanguagesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiConceptLanguageResponse>> {
        self.list_concept_languages_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_concept_languages_opt(&self, req: &super::service::PostConceptLanguagesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiConceptLanguageResponse> {
        self.client.unary_call(&METHOD_V2_POST_CONCEPT_LANGUAGES, req, opt)
    }

    pub fn post_concept_languages(&self, req: &super::service::PostConceptLanguagesRequest) -> ::grpcio::Result<super::service::MultiConceptLanguageResponse> {
        self.post_concept_languages_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_concept_languages_async_opt(&self, req: &super::service::PostConceptLanguagesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiConceptLanguageResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_CONCEPT_LANGUAGES, req, opt)
    }

    pub fn post_concept_languages_async(&self, req: &super::service::PostConceptLanguagesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiConceptLanguageResponse>> {
        self.post_concept_languages_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_concept_languages_opt(&self, req: &super::service::PatchConceptLanguagesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiConceptLanguageResponse> {
        self.client.unary_call(&METHOD_V2_PATCH_CONCEPT_LANGUAGES, req, opt)
    }

    pub fn patch_concept_languages(&self, req: &super::service::PatchConceptLanguagesRequest) -> ::grpcio::Result<super::service::MultiConceptLanguageResponse> {
        self.patch_concept_languages_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_concept_languages_async_opt(&self, req: &super::service::PatchConceptLanguagesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiConceptLanguageResponse>> {
        self.client.unary_call_async(&METHOD_V2_PATCH_CONCEPT_LANGUAGES, req, opt)
    }

    pub fn patch_concept_languages_async(&self, req: &super::service::PatchConceptLanguagesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiConceptLanguageResponse>> {
        self.patch_concept_languages_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_knowledge_graphs_opt(&self, req: &super::service::ListKnowledgeGraphsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiKnowledgeGraphResponse> {
        self.client.unary_call(&METHOD_V2_LIST_KNOWLEDGE_GRAPHS, req, opt)
    }

    pub fn list_knowledge_graphs(&self, req: &super::service::ListKnowledgeGraphsRequest) -> ::grpcio::Result<super::service::MultiKnowledgeGraphResponse> {
        self.list_knowledge_graphs_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_knowledge_graphs_async_opt(&self, req: &super::service::ListKnowledgeGraphsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiKnowledgeGraphResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_KNOWLEDGE_GRAPHS, req, opt)
    }

    pub fn list_knowledge_graphs_async(&self, req: &super::service::ListKnowledgeGraphsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiKnowledgeGraphResponse>> {
        self.list_knowledge_graphs_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_knowledge_graphs_opt(&self, req: &super::service::PostKnowledgeGraphsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiKnowledgeGraphResponse> {
        self.client.unary_call(&METHOD_V2_POST_KNOWLEDGE_GRAPHS, req, opt)
    }

    pub fn post_knowledge_graphs(&self, req: &super::service::PostKnowledgeGraphsRequest) -> ::grpcio::Result<super::service::MultiKnowledgeGraphResponse> {
        self.post_knowledge_graphs_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_knowledge_graphs_async_opt(&self, req: &super::service::PostKnowledgeGraphsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiKnowledgeGraphResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_KNOWLEDGE_GRAPHS, req, opt)
    }

    pub fn post_knowledge_graphs_async(&self, req: &super::service::PostKnowledgeGraphsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiKnowledgeGraphResponse>> {
        self.post_knowledge_graphs_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_concept_mapping_jobs_opt(&self, req: &super::service::PostConceptMappingJobsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiConceptMappingJobResponse> {
        self.client.unary_call(&METHOD_V2_POST_CONCEPT_MAPPING_JOBS, req, opt)
    }

    pub fn post_concept_mapping_jobs(&self, req: &super::service::PostConceptMappingJobsRequest) -> ::grpcio::Result<super::service::MultiConceptMappingJobResponse> {
        self.post_concept_mapping_jobs_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_concept_mapping_jobs_async_opt(&self, req: &super::service::PostConceptMappingJobsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiConceptMappingJobResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_CONCEPT_MAPPING_JOBS, req, opt)
    }

    pub fn post_concept_mapping_jobs_async(&self, req: &super::service::PostConceptMappingJobsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiConceptMappingJobResponse>> {
        self.post_concept_mapping_jobs_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_annotation_opt(&self, req: &super::service::GetAnnotationRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleAnnotationResponse> {
        self.client.unary_call(&METHOD_V2_GET_ANNOTATION, req, opt)
    }

    pub fn get_annotation(&self, req: &super::service::GetAnnotationRequest) -> ::grpcio::Result<super::service::SingleAnnotationResponse> {
        self.get_annotation_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_annotation_async_opt(&self, req: &super::service::GetAnnotationRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleAnnotationResponse>> {
        self.client.unary_call_async(&METHOD_V2_GET_ANNOTATION, req, opt)
    }

    pub fn get_annotation_async(&self, req: &super::service::GetAnnotationRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleAnnotationResponse>> {
        self.get_annotation_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_annotations_opt(&self, req: &super::service::ListAnnotationsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiAnnotationResponse> {
        self.client.unary_call(&METHOD_V2_LIST_ANNOTATIONS, req, opt)
    }

    pub fn list_annotations(&self, req: &super::service::ListAnnotationsRequest) -> ::grpcio::Result<super::service::MultiAnnotationResponse> {
        self.list_annotations_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_annotations_async_opt(&self, req: &super::service::ListAnnotationsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiAnnotationResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_ANNOTATIONS, req, opt)
    }

    pub fn list_annotations_async(&self, req: &super::service::ListAnnotationsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiAnnotationResponse>> {
        self.list_annotations_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_annotations_opt(&self, req: &super::service::PostAnnotationsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiAnnotationResponse> {
        self.client.unary_call(&METHOD_V2_POST_ANNOTATIONS, req, opt)
    }

    pub fn post_annotations(&self, req: &super::service::PostAnnotationsRequest) -> ::grpcio::Result<super::service::MultiAnnotationResponse> {
        self.post_annotations_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_annotations_async_opt(&self, req: &super::service::PostAnnotationsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiAnnotationResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_ANNOTATIONS, req, opt)
    }

    pub fn post_annotations_async(&self, req: &super::service::PostAnnotationsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiAnnotationResponse>> {
        self.post_annotations_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_annotations_opt(&self, req: &super::service::PatchAnnotationsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiAnnotationResponse> {
        self.client.unary_call(&METHOD_V2_PATCH_ANNOTATIONS, req, opt)
    }

    pub fn patch_annotations(&self, req: &super::service::PatchAnnotationsRequest) -> ::grpcio::Result<super::service::MultiAnnotationResponse> {
        self.patch_annotations_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_annotations_async_opt(&self, req: &super::service::PatchAnnotationsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiAnnotationResponse>> {
        self.client.unary_call_async(&METHOD_V2_PATCH_ANNOTATIONS, req, opt)
    }

    pub fn patch_annotations_async(&self, req: &super::service::PatchAnnotationsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiAnnotationResponse>> {
        self.patch_annotations_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_annotations_status_opt(&self, req: &super::service::PatchAnnotationsStatusRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::PatchAnnotationsStatusResponse> {
        self.client.unary_call(&METHOD_V2_PATCH_ANNOTATIONS_STATUS, req, opt)
    }

    pub fn patch_annotations_status(&self, req: &super::service::PatchAnnotationsStatusRequest) -> ::grpcio::Result<super::service::PatchAnnotationsStatusResponse> {
        self.patch_annotations_status_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_annotations_status_async_opt(&self, req: &super::service::PatchAnnotationsStatusRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::PatchAnnotationsStatusResponse>> {
        self.client.unary_call_async(&METHOD_V2_PATCH_ANNOTATIONS_STATUS, req, opt)
    }

    pub fn patch_annotations_status_async(&self, req: &super::service::PatchAnnotationsStatusRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::PatchAnnotationsStatusResponse>> {
        self.patch_annotations_status_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_annotation_opt(&self, req: &super::service::DeleteAnnotationRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::status::BaseResponse> {
        self.client.unary_call(&METHOD_V2_DELETE_ANNOTATION, req, opt)
    }

    pub fn delete_annotation(&self, req: &super::service::DeleteAnnotationRequest) -> ::grpcio::Result<super::status::BaseResponse> {
        self.delete_annotation_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_annotation_async_opt(&self, req: &super::service::DeleteAnnotationRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.client.unary_call_async(&METHOD_V2_DELETE_ANNOTATION, req, opt)
    }

    pub fn delete_annotation_async(&self, req: &super::service::DeleteAnnotationRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.delete_annotation_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_annotations_opt(&self, req: &super::service::DeleteAnnotationsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::status::BaseResponse> {
        self.client.unary_call(&METHOD_V2_DELETE_ANNOTATIONS, req, opt)
    }

    pub fn delete_annotations(&self, req: &super::service::DeleteAnnotationsRequest) -> ::grpcio::Result<super::status::BaseResponse> {
        self.delete_annotations_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_annotations_async_opt(&self, req: &super::service::DeleteAnnotationsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.client.unary_call_async(&METHOD_V2_DELETE_ANNOTATIONS, req, opt)
    }

    pub fn delete_annotations_async(&self, req: &super::service::DeleteAnnotationsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.delete_annotations_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_annotations_searches_opt(&self, req: &super::service::PostAnnotationsSearchesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiSearchResponse> {
        self.client.unary_call(&METHOD_V2_POST_ANNOTATIONS_SEARCHES, req, opt)
    }

    pub fn post_annotations_searches(&self, req: &super::service::PostAnnotationsSearchesRequest) -> ::grpcio::Result<super::service::MultiSearchResponse> {
        self.post_annotations_searches_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_annotations_searches_async_opt(&self, req: &super::service::PostAnnotationsSearchesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiSearchResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_ANNOTATIONS_SEARCHES, req, opt)
    }

    pub fn post_annotations_searches_async(&self, req: &super::service::PostAnnotationsSearchesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiSearchResponse>> {
        self.post_annotations_searches_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_input_count_opt(&self, req: &super::service::GetInputCountRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleInputCountResponse> {
        self.client.unary_call(&METHOD_V2_GET_INPUT_COUNT, req, opt)
    }

    pub fn get_input_count(&self, req: &super::service::GetInputCountRequest) -> ::grpcio::Result<super::service::SingleInputCountResponse> {
        self.get_input_count_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_input_count_async_opt(&self, req: &super::service::GetInputCountRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleInputCountResponse>> {
        self.client.unary_call_async(&METHOD_V2_GET_INPUT_COUNT, req, opt)
    }

    pub fn get_input_count_async(&self, req: &super::service::GetInputCountRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleInputCountResponse>> {
        self.get_input_count_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn stream_inputs_opt(&self, req: &super::service::StreamInputsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiInputResponse> {
        self.client.unary_call(&METHOD_V2_STREAM_INPUTS, req, opt)
    }

    pub fn stream_inputs(&self, req: &super::service::StreamInputsRequest) -> ::grpcio::Result<super::service::MultiInputResponse> {
        self.stream_inputs_opt(req, ::grpcio::CallOption::default())
    }

    pub fn stream_inputs_async_opt(&self, req: &super::service::StreamInputsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiInputResponse>> {
        self.client.unary_call_async(&METHOD_V2_STREAM_INPUTS, req, opt)
    }

    pub fn stream_inputs_async(&self, req: &super::service::StreamInputsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiInputResponse>> {
        self.stream_inputs_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_input_samples_opt(&self, req: &super::service::GetInputSamplesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiInputAnnotationResponse> {
        self.client.unary_call(&METHOD_V2_GET_INPUT_SAMPLES, req, opt)
    }

    pub fn get_input_samples(&self, req: &super::service::GetInputSamplesRequest) -> ::grpcio::Result<super::service::MultiInputAnnotationResponse> {
        self.get_input_samples_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_input_samples_async_opt(&self, req: &super::service::GetInputSamplesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiInputAnnotationResponse>> {
        self.client.unary_call_async(&METHOD_V2_GET_INPUT_SAMPLES, req, opt)
    }

    pub fn get_input_samples_async(&self, req: &super::service::GetInputSamplesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiInputAnnotationResponse>> {
        self.get_input_samples_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_input_opt(&self, req: &super::service::GetInputRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleInputResponse> {
        self.client.unary_call(&METHOD_V2_GET_INPUT, req, opt)
    }

    pub fn get_input(&self, req: &super::service::GetInputRequest) -> ::grpcio::Result<super::service::SingleInputResponse> {
        self.get_input_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_input_async_opt(&self, req: &super::service::GetInputRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleInputResponse>> {
        self.client.unary_call_async(&METHOD_V2_GET_INPUT, req, opt)
    }

    pub fn get_input_async(&self, req: &super::service::GetInputRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleInputResponse>> {
        self.get_input_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_inputs_opt(&self, req: &super::service::ListInputsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiInputResponse> {
        self.client.unary_call(&METHOD_V2_LIST_INPUTS, req, opt)
    }

    pub fn list_inputs(&self, req: &super::service::ListInputsRequest) -> ::grpcio::Result<super::service::MultiInputResponse> {
        self.list_inputs_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_inputs_async_opt(&self, req: &super::service::ListInputsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiInputResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_INPUTS, req, opt)
    }

    pub fn list_inputs_async(&self, req: &super::service::ListInputsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiInputResponse>> {
        self.list_inputs_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_inputs_opt(&self, req: &super::service::PostInputsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiInputResponse> {
        self.client.unary_call(&METHOD_V2_POST_INPUTS, req, opt)
    }

    pub fn post_inputs(&self, req: &super::service::PostInputsRequest) -> ::grpcio::Result<super::service::MultiInputResponse> {
        self.post_inputs_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_inputs_async_opt(&self, req: &super::service::PostInputsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiInputResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_INPUTS, req, opt)
    }

    pub fn post_inputs_async(&self, req: &super::service::PostInputsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiInputResponse>> {
        self.post_inputs_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_inputs_opt(&self, req: &super::service::PatchInputsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiInputResponse> {
        self.client.unary_call(&METHOD_V2_PATCH_INPUTS, req, opt)
    }

    pub fn patch_inputs(&self, req: &super::service::PatchInputsRequest) -> ::grpcio::Result<super::service::MultiInputResponse> {
        self.patch_inputs_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_inputs_async_opt(&self, req: &super::service::PatchInputsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiInputResponse>> {
        self.client.unary_call_async(&METHOD_V2_PATCH_INPUTS, req, opt)
    }

    pub fn patch_inputs_async(&self, req: &super::service::PatchInputsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiInputResponse>> {
        self.patch_inputs_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_input_opt(&self, req: &super::service::DeleteInputRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::status::BaseResponse> {
        self.client.unary_call(&METHOD_V2_DELETE_INPUT, req, opt)
    }

    pub fn delete_input(&self, req: &super::service::DeleteInputRequest) -> ::grpcio::Result<super::status::BaseResponse> {
        self.delete_input_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_input_async_opt(&self, req: &super::service::DeleteInputRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.client.unary_call_async(&METHOD_V2_DELETE_INPUT, req, opt)
    }

    pub fn delete_input_async(&self, req: &super::service::DeleteInputRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.delete_input_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_inputs_opt(&self, req: &super::service::DeleteInputsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::status::BaseResponse> {
        self.client.unary_call(&METHOD_V2_DELETE_INPUTS, req, opt)
    }

    pub fn delete_inputs(&self, req: &super::service::DeleteInputsRequest) -> ::grpcio::Result<super::status::BaseResponse> {
        self.delete_inputs_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_inputs_async_opt(&self, req: &super::service::DeleteInputsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.client.unary_call_async(&METHOD_V2_DELETE_INPUTS, req, opt)
    }

    pub fn delete_inputs_async(&self, req: &super::service::DeleteInputsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.delete_inputs_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_inputs_searches_opt(&self, req: &super::service::PostInputsSearchesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiSearchResponse> {
        self.client.unary_call(&METHOD_V2_POST_INPUTS_SEARCHES, req, opt)
    }

    pub fn post_inputs_searches(&self, req: &super::service::PostInputsSearchesRequest) -> ::grpcio::Result<super::service::MultiSearchResponse> {
        self.post_inputs_searches_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_inputs_searches_async_opt(&self, req: &super::service::PostInputsSearchesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiSearchResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_INPUTS_SEARCHES, req, opt)
    }

    pub fn post_inputs_searches_async(&self, req: &super::service::PostInputsSearchesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiSearchResponse>> {
        self.post_inputs_searches_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_model_outputs_opt(&self, req: &super::service::PostModelOutputsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiOutputResponse> {
        self.client.unary_call(&METHOD_V2_POST_MODEL_OUTPUTS, req, opt)
    }

    pub fn post_model_outputs(&self, req: &super::service::PostModelOutputsRequest) -> ::grpcio::Result<super::service::MultiOutputResponse> {
        self.post_model_outputs_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_model_outputs_async_opt(&self, req: &super::service::PostModelOutputsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiOutputResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_MODEL_OUTPUTS, req, opt)
    }

    pub fn post_model_outputs_async(&self, req: &super::service::PostModelOutputsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiOutputResponse>> {
        self.post_model_outputs_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_model_type_opt(&self, req: &super::service::GetModelTypeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleModelTypeResponse> {
        self.client.unary_call(&METHOD_V2_GET_MODEL_TYPE, req, opt)
    }

    pub fn get_model_type(&self, req: &super::service::GetModelTypeRequest) -> ::grpcio::Result<super::service::SingleModelTypeResponse> {
        self.get_model_type_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_model_type_async_opt(&self, req: &super::service::GetModelTypeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleModelTypeResponse>> {
        self.client.unary_call_async(&METHOD_V2_GET_MODEL_TYPE, req, opt)
    }

    pub fn get_model_type_async(&self, req: &super::service::GetModelTypeRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleModelTypeResponse>> {
        self.get_model_type_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_model_types_opt(&self, req: &super::service::ListModelTypesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiModelTypeResponse> {
        self.client.unary_call(&METHOD_V2_LIST_MODEL_TYPES, req, opt)
    }

    pub fn list_model_types(&self, req: &super::service::ListModelTypesRequest) -> ::grpcio::Result<super::service::MultiModelTypeResponse> {
        self.list_model_types_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_model_types_async_opt(&self, req: &super::service::ListModelTypesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiModelTypeResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_MODEL_TYPES, req, opt)
    }

    pub fn list_model_types_async(&self, req: &super::service::ListModelTypesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiModelTypeResponse>> {
        self.list_model_types_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_model_opt(&self, req: &super::service::GetModelRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleModelResponse> {
        self.client.unary_call(&METHOD_V2_GET_MODEL, req, opt)
    }

    pub fn get_model(&self, req: &super::service::GetModelRequest) -> ::grpcio::Result<super::service::SingleModelResponse> {
        self.get_model_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_model_async_opt(&self, req: &super::service::GetModelRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleModelResponse>> {
        self.client.unary_call_async(&METHOD_V2_GET_MODEL, req, opt)
    }

    pub fn get_model_async(&self, req: &super::service::GetModelRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleModelResponse>> {
        self.get_model_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_model_output_info_opt(&self, req: &super::service::GetModelRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleModelResponse> {
        self.client.unary_call(&METHOD_V2_GET_MODEL_OUTPUT_INFO, req, opt)
    }

    pub fn get_model_output_info(&self, req: &super::service::GetModelRequest) -> ::grpcio::Result<super::service::SingleModelResponse> {
        self.get_model_output_info_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_model_output_info_async_opt(&self, req: &super::service::GetModelRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleModelResponse>> {
        self.client.unary_call_async(&METHOD_V2_GET_MODEL_OUTPUT_INFO, req, opt)
    }

    pub fn get_model_output_info_async(&self, req: &super::service::GetModelRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleModelResponse>> {
        self.get_model_output_info_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_models_opt(&self, req: &super::service::ListModelsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiModelResponse> {
        self.client.unary_call(&METHOD_V2_LIST_MODELS, req, opt)
    }

    pub fn list_models(&self, req: &super::service::ListModelsRequest) -> ::grpcio::Result<super::service::MultiModelResponse> {
        self.list_models_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_models_async_opt(&self, req: &super::service::ListModelsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiModelResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_MODELS, req, opt)
    }

    pub fn list_models_async(&self, req: &super::service::ListModelsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiModelResponse>> {
        self.list_models_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_models_searches_opt(&self, req: &super::service::PostModelsSearchesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiModelResponse> {
        self.client.unary_call(&METHOD_V2_POST_MODELS_SEARCHES, req, opt)
    }

    pub fn post_models_searches(&self, req: &super::service::PostModelsSearchesRequest) -> ::grpcio::Result<super::service::MultiModelResponse> {
        self.post_models_searches_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_models_searches_async_opt(&self, req: &super::service::PostModelsSearchesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiModelResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_MODELS_SEARCHES, req, opt)
    }

    pub fn post_models_searches_async(&self, req: &super::service::PostModelsSearchesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiModelResponse>> {
        self.post_models_searches_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_models_opt(&self, req: &super::service::PostModelsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleModelResponse> {
        self.client.unary_call(&METHOD_V2_POST_MODELS, req, opt)
    }

    pub fn post_models(&self, req: &super::service::PostModelsRequest) -> ::grpcio::Result<super::service::SingleModelResponse> {
        self.post_models_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_models_async_opt(&self, req: &super::service::PostModelsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleModelResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_MODELS, req, opt)
    }

    pub fn post_models_async(&self, req: &super::service::PostModelsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleModelResponse>> {
        self.post_models_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_models_opt(&self, req: &super::service::PatchModelsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiModelResponse> {
        self.client.unary_call(&METHOD_V2_PATCH_MODELS, req, opt)
    }

    pub fn patch_models(&self, req: &super::service::PatchModelsRequest) -> ::grpcio::Result<super::service::MultiModelResponse> {
        self.patch_models_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_models_async_opt(&self, req: &super::service::PatchModelsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiModelResponse>> {
        self.client.unary_call_async(&METHOD_V2_PATCH_MODELS, req, opt)
    }

    pub fn patch_models_async(&self, req: &super::service::PatchModelsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiModelResponse>> {
        self.patch_models_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_model_opt(&self, req: &super::service::DeleteModelRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::status::BaseResponse> {
        self.client.unary_call(&METHOD_V2_DELETE_MODEL, req, opt)
    }

    pub fn delete_model(&self, req: &super::service::DeleteModelRequest) -> ::grpcio::Result<super::status::BaseResponse> {
        self.delete_model_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_model_async_opt(&self, req: &super::service::DeleteModelRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.client.unary_call_async(&METHOD_V2_DELETE_MODEL, req, opt)
    }

    pub fn delete_model_async(&self, req: &super::service::DeleteModelRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.delete_model_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_models_opt(&self, req: &super::service::DeleteModelsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::status::BaseResponse> {
        self.client.unary_call(&METHOD_V2_DELETE_MODELS, req, opt)
    }

    pub fn delete_models(&self, req: &super::service::DeleteModelsRequest) -> ::grpcio::Result<super::status::BaseResponse> {
        self.delete_models_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_models_async_opt(&self, req: &super::service::DeleteModelsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.client.unary_call_async(&METHOD_V2_DELETE_MODELS, req, opt)
    }

    pub fn delete_models_async(&self, req: &super::service::DeleteModelsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.delete_models_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_model_inputs_opt(&self, req: &super::service::ListModelInputsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiInputResponse> {
        self.client.unary_call(&METHOD_V2_LIST_MODEL_INPUTS, req, opt)
    }

    pub fn list_model_inputs(&self, req: &super::service::ListModelInputsRequest) -> ::grpcio::Result<super::service::MultiInputResponse> {
        self.list_model_inputs_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_model_inputs_async_opt(&self, req: &super::service::ListModelInputsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiInputResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_MODEL_INPUTS, req, opt)
    }

    pub fn list_model_inputs_async(&self, req: &super::service::ListModelInputsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiInputResponse>> {
        self.list_model_inputs_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_model_version_opt(&self, req: &super::service::GetModelVersionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleModelVersionResponse> {
        self.client.unary_call(&METHOD_V2_GET_MODEL_VERSION, req, opt)
    }

    pub fn get_model_version(&self, req: &super::service::GetModelVersionRequest) -> ::grpcio::Result<super::service::SingleModelVersionResponse> {
        self.get_model_version_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_model_version_async_opt(&self, req: &super::service::GetModelVersionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleModelVersionResponse>> {
        self.client.unary_call_async(&METHOD_V2_GET_MODEL_VERSION, req, opt)
    }

    pub fn get_model_version_async(&self, req: &super::service::GetModelVersionRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleModelVersionResponse>> {
        self.get_model_version_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_model_versions_opt(&self, req: &super::service::ListModelVersionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiModelVersionResponse> {
        self.client.unary_call(&METHOD_V2_LIST_MODEL_VERSIONS, req, opt)
    }

    pub fn list_model_versions(&self, req: &super::service::ListModelVersionsRequest) -> ::grpcio::Result<super::service::MultiModelVersionResponse> {
        self.list_model_versions_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_model_versions_async_opt(&self, req: &super::service::ListModelVersionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiModelVersionResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_MODEL_VERSIONS, req, opt)
    }

    pub fn list_model_versions_async(&self, req: &super::service::ListModelVersionsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiModelVersionResponse>> {
        self.list_model_versions_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_model_versions_opt(&self, req: &super::service::PostModelVersionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleModelResponse> {
        self.client.unary_call(&METHOD_V2_POST_MODEL_VERSIONS, req, opt)
    }

    pub fn post_model_versions(&self, req: &super::service::PostModelVersionsRequest) -> ::grpcio::Result<super::service::SingleModelResponse> {
        self.post_model_versions_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_model_versions_async_opt(&self, req: &super::service::PostModelVersionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleModelResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_MODEL_VERSIONS, req, opt)
    }

    pub fn post_model_versions_async(&self, req: &super::service::PostModelVersionsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleModelResponse>> {
        self.post_model_versions_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_model_version_opt(&self, req: &super::service::DeleteModelVersionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::status::BaseResponse> {
        self.client.unary_call(&METHOD_V2_DELETE_MODEL_VERSION, req, opt)
    }

    pub fn delete_model_version(&self, req: &super::service::DeleteModelVersionRequest) -> ::grpcio::Result<super::status::BaseResponse> {
        self.delete_model_version_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_model_version_async_opt(&self, req: &super::service::DeleteModelVersionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.client.unary_call_async(&METHOD_V2_DELETE_MODEL_VERSION, req, opt)
    }

    pub fn delete_model_version_async(&self, req: &super::service::DeleteModelVersionRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.delete_model_version_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_model_version_metrics_opt(&self, req: &super::service::GetModelVersionMetricsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleModelVersionResponse> {
        self.client.unary_call(&METHOD_V2_GET_MODEL_VERSION_METRICS, req, opt)
    }

    pub fn get_model_version_metrics(&self, req: &super::service::GetModelVersionMetricsRequest) -> ::grpcio::Result<super::service::SingleModelVersionResponse> {
        self.get_model_version_metrics_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_model_version_metrics_async_opt(&self, req: &super::service::GetModelVersionMetricsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleModelVersionResponse>> {
        self.client.unary_call_async(&METHOD_V2_GET_MODEL_VERSION_METRICS, req, opt)
    }

    pub fn get_model_version_metrics_async(&self, req: &super::service::GetModelVersionMetricsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleModelVersionResponse>> {
        self.get_model_version_metrics_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_model_version_metrics_opt(&self, req: &super::service::PostModelVersionMetricsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleModelVersionResponse> {
        self.client.unary_call(&METHOD_V2_POST_MODEL_VERSION_METRICS, req, opt)
    }

    pub fn post_model_version_metrics(&self, req: &super::service::PostModelVersionMetricsRequest) -> ::grpcio::Result<super::service::SingleModelVersionResponse> {
        self.post_model_version_metrics_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_model_version_metrics_async_opt(&self, req: &super::service::PostModelVersionMetricsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleModelVersionResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_MODEL_VERSION_METRICS, req, opt)
    }

    pub fn post_model_version_metrics_async(&self, req: &super::service::PostModelVersionMetricsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleModelVersionResponse>> {
        self.post_model_version_metrics_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_workflow_opt(&self, req: &super::service::GetWorkflowRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleWorkflowResponse> {
        self.client.unary_call(&METHOD_V2_GET_WORKFLOW, req, opt)
    }

    pub fn get_workflow(&self, req: &super::service::GetWorkflowRequest) -> ::grpcio::Result<super::service::SingleWorkflowResponse> {
        self.get_workflow_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_workflow_async_opt(&self, req: &super::service::GetWorkflowRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleWorkflowResponse>> {
        self.client.unary_call_async(&METHOD_V2_GET_WORKFLOW, req, opt)
    }

    pub fn get_workflow_async(&self, req: &super::service::GetWorkflowRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleWorkflowResponse>> {
        self.get_workflow_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_workflows_opt(&self, req: &super::service::ListWorkflowsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiWorkflowResponse> {
        self.client.unary_call(&METHOD_V2_LIST_WORKFLOWS, req, opt)
    }

    pub fn list_workflows(&self, req: &super::service::ListWorkflowsRequest) -> ::grpcio::Result<super::service::MultiWorkflowResponse> {
        self.list_workflows_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_workflows_async_opt(&self, req: &super::service::ListWorkflowsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiWorkflowResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_WORKFLOWS, req, opt)
    }

    pub fn list_workflows_async(&self, req: &super::service::ListWorkflowsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiWorkflowResponse>> {
        self.list_workflows_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_workflows_opt(&self, req: &super::service::PostWorkflowsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiWorkflowResponse> {
        self.client.unary_call(&METHOD_V2_POST_WORKFLOWS, req, opt)
    }

    pub fn post_workflows(&self, req: &super::service::PostWorkflowsRequest) -> ::grpcio::Result<super::service::MultiWorkflowResponse> {
        self.post_workflows_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_workflows_async_opt(&self, req: &super::service::PostWorkflowsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiWorkflowResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_WORKFLOWS, req, opt)
    }

    pub fn post_workflows_async(&self, req: &super::service::PostWorkflowsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiWorkflowResponse>> {
        self.post_workflows_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_workflows_opt(&self, req: &super::service::PatchWorkflowsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiWorkflowResponse> {
        self.client.unary_call(&METHOD_V2_PATCH_WORKFLOWS, req, opt)
    }

    pub fn patch_workflows(&self, req: &super::service::PatchWorkflowsRequest) -> ::grpcio::Result<super::service::MultiWorkflowResponse> {
        self.patch_workflows_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_workflows_async_opt(&self, req: &super::service::PatchWorkflowsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiWorkflowResponse>> {
        self.client.unary_call_async(&METHOD_V2_PATCH_WORKFLOWS, req, opt)
    }

    pub fn patch_workflows_async(&self, req: &super::service::PatchWorkflowsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiWorkflowResponse>> {
        self.patch_workflows_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_workflow_opt(&self, req: &super::service::DeleteWorkflowRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::status::BaseResponse> {
        self.client.unary_call(&METHOD_V2_DELETE_WORKFLOW, req, opt)
    }

    pub fn delete_workflow(&self, req: &super::service::DeleteWorkflowRequest) -> ::grpcio::Result<super::status::BaseResponse> {
        self.delete_workflow_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_workflow_async_opt(&self, req: &super::service::DeleteWorkflowRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.client.unary_call_async(&METHOD_V2_DELETE_WORKFLOW, req, opt)
    }

    pub fn delete_workflow_async(&self, req: &super::service::DeleteWorkflowRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.delete_workflow_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_workflows_opt(&self, req: &super::service::DeleteWorkflowsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::status::BaseResponse> {
        self.client.unary_call(&METHOD_V2_DELETE_WORKFLOWS, req, opt)
    }

    pub fn delete_workflows(&self, req: &super::service::DeleteWorkflowsRequest) -> ::grpcio::Result<super::status::BaseResponse> {
        self.delete_workflows_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_workflows_async_opt(&self, req: &super::service::DeleteWorkflowsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.client.unary_call_async(&METHOD_V2_DELETE_WORKFLOWS, req, opt)
    }

    pub fn delete_workflows_async(&self, req: &super::service::DeleteWorkflowsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.delete_workflows_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_workflow_results_opt(&self, req: &super::service::PostWorkflowResultsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::PostWorkflowResultsResponse> {
        self.client.unary_call(&METHOD_V2_POST_WORKFLOW_RESULTS, req, opt)
    }

    pub fn post_workflow_results(&self, req: &super::service::PostWorkflowResultsRequest) -> ::grpcio::Result<super::service::PostWorkflowResultsResponse> {
        self.post_workflow_results_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_workflow_results_async_opt(&self, req: &super::service::PostWorkflowResultsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::PostWorkflowResultsResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_WORKFLOW_RESULTS, req, opt)
    }

    pub fn post_workflow_results_async(&self, req: &super::service::PostWorkflowResultsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::PostWorkflowResultsResponse>> {
        self.post_workflow_results_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_workflow_results_similarity_opt(&self, req: &super::service::PostWorkflowResultsSimilarityRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::PostWorkflowResultsSimilarityResponse> {
        self.client.unary_call(&METHOD_V2_POST_WORKFLOW_RESULTS_SIMILARITY, req, opt)
    }

    pub fn post_workflow_results_similarity(&self, req: &super::service::PostWorkflowResultsSimilarityRequest) -> ::grpcio::Result<super::service::PostWorkflowResultsSimilarityResponse> {
        self.post_workflow_results_similarity_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_workflow_results_similarity_async_opt(&self, req: &super::service::PostWorkflowResultsSimilarityRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::PostWorkflowResultsSimilarityResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_WORKFLOW_RESULTS_SIMILARITY, req, opt)
    }

    pub fn post_workflow_results_similarity_async(&self, req: &super::service::PostWorkflowResultsSimilarityRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::PostWorkflowResultsSimilarityResponse>> {
        self.post_workflow_results_similarity_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_key_opt(&self, req: &super::service::GetKeyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleKeyResponse> {
        self.client.unary_call(&METHOD_V2_GET_KEY, req, opt)
    }

    pub fn get_key(&self, req: &super::service::GetKeyRequest) -> ::grpcio::Result<super::service::SingleKeyResponse> {
        self.get_key_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_key_async_opt(&self, req: &super::service::GetKeyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleKeyResponse>> {
        self.client.unary_call_async(&METHOD_V2_GET_KEY, req, opt)
    }

    pub fn get_key_async(&self, req: &super::service::GetKeyRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleKeyResponse>> {
        self.get_key_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_keys_opt(&self, req: &super::service::ListKeysRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiKeyResponse> {
        self.client.unary_call(&METHOD_V2_LIST_KEYS, req, opt)
    }

    pub fn list_keys(&self, req: &super::service::ListKeysRequest) -> ::grpcio::Result<super::service::MultiKeyResponse> {
        self.list_keys_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_keys_async_opt(&self, req: &super::service::ListKeysRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiKeyResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_KEYS, req, opt)
    }

    pub fn list_keys_async(&self, req: &super::service::ListKeysRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiKeyResponse>> {
        self.list_keys_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_app_keys_opt(&self, req: &super::service::ListAppKeysRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiKeyResponse> {
        self.client.unary_call(&METHOD_V2_LIST_APP_KEYS, req, opt)
    }

    pub fn list_app_keys(&self, req: &super::service::ListAppKeysRequest) -> ::grpcio::Result<super::service::MultiKeyResponse> {
        self.list_app_keys_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_app_keys_async_opt(&self, req: &super::service::ListAppKeysRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiKeyResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_APP_KEYS, req, opt)
    }

    pub fn list_app_keys_async(&self, req: &super::service::ListAppKeysRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiKeyResponse>> {
        self.list_app_keys_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_key_opt(&self, req: &super::service::DeleteKeyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::status::BaseResponse> {
        self.client.unary_call(&METHOD_V2_DELETE_KEY, req, opt)
    }

    pub fn delete_key(&self, req: &super::service::DeleteKeyRequest) -> ::grpcio::Result<super::status::BaseResponse> {
        self.delete_key_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_key_async_opt(&self, req: &super::service::DeleteKeyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.client.unary_call_async(&METHOD_V2_DELETE_KEY, req, opt)
    }

    pub fn delete_key_async(&self, req: &super::service::DeleteKeyRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.delete_key_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_keys_opt(&self, req: &super::service::PostKeysRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiKeyResponse> {
        self.client.unary_call(&METHOD_V2_POST_KEYS, req, opt)
    }

    pub fn post_keys(&self, req: &super::service::PostKeysRequest) -> ::grpcio::Result<super::service::MultiKeyResponse> {
        self.post_keys_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_keys_async_opt(&self, req: &super::service::PostKeysRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiKeyResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_KEYS, req, opt)
    }

    pub fn post_keys_async(&self, req: &super::service::PostKeysRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiKeyResponse>> {
        self.post_keys_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_keys_opt(&self, req: &super::service::PatchKeysRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiKeyResponse> {
        self.client.unary_call(&METHOD_V2_PATCH_KEYS, req, opt)
    }

    pub fn patch_keys(&self, req: &super::service::PatchKeysRequest) -> ::grpcio::Result<super::service::MultiKeyResponse> {
        self.patch_keys_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_keys_async_opt(&self, req: &super::service::PatchKeysRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiKeyResponse>> {
        self.client.unary_call_async(&METHOD_V2_PATCH_KEYS, req, opt)
    }

    pub fn patch_keys_async(&self, req: &super::service::PatchKeysRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiKeyResponse>> {
        self.patch_keys_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn my_scopes_opt(&self, req: &super::service::MyScopesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiScopeResponse> {
        self.client.unary_call(&METHOD_V2_MY_SCOPES, req, opt)
    }

    pub fn my_scopes(&self, req: &super::service::MyScopesRequest) -> ::grpcio::Result<super::service::MultiScopeResponse> {
        self.my_scopes_opt(req, ::grpcio::CallOption::default())
    }

    pub fn my_scopes_async_opt(&self, req: &super::service::MyScopesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiScopeResponse>> {
        self.client.unary_call_async(&METHOD_V2_MY_SCOPES, req, opt)
    }

    pub fn my_scopes_async(&self, req: &super::service::MyScopesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiScopeResponse>> {
        self.my_scopes_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_scopes_opt(&self, req: &super::service::ListScopesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiScopeDepsResponse> {
        self.client.unary_call(&METHOD_V2_LIST_SCOPES, req, opt)
    }

    pub fn list_scopes(&self, req: &super::service::ListScopesRequest) -> ::grpcio::Result<super::service::MultiScopeDepsResponse> {
        self.list_scopes_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_scopes_async_opt(&self, req: &super::service::ListScopesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiScopeDepsResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_SCOPES, req, opt)
    }

    pub fn list_scopes_async(&self, req: &super::service::ListScopesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiScopeDepsResponse>> {
        self.list_scopes_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_app_opt(&self, req: &super::service::GetAppRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleAppResponse> {
        self.client.unary_call(&METHOD_V2_GET_APP, req, opt)
    }

    pub fn get_app(&self, req: &super::service::GetAppRequest) -> ::grpcio::Result<super::service::SingleAppResponse> {
        self.get_app_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_app_async_opt(&self, req: &super::service::GetAppRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleAppResponse>> {
        self.client.unary_call_async(&METHOD_V2_GET_APP, req, opt)
    }

    pub fn get_app_async(&self, req: &super::service::GetAppRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleAppResponse>> {
        self.get_app_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_apps_opt(&self, req: &super::service::ListAppsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiAppResponse> {
        self.client.unary_call(&METHOD_V2_LIST_APPS, req, opt)
    }

    pub fn list_apps(&self, req: &super::service::ListAppsRequest) -> ::grpcio::Result<super::service::MultiAppResponse> {
        self.list_apps_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_apps_async_opt(&self, req: &super::service::ListAppsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiAppResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_APPS, req, opt)
    }

    pub fn list_apps_async(&self, req: &super::service::ListAppsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiAppResponse>> {
        self.list_apps_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_app_opt(&self, req: &super::service::DeleteAppRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::status::BaseResponse> {
        self.client.unary_call(&METHOD_V2_DELETE_APP, req, opt)
    }

    pub fn delete_app(&self, req: &super::service::DeleteAppRequest) -> ::grpcio::Result<super::status::BaseResponse> {
        self.delete_app_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_app_async_opt(&self, req: &super::service::DeleteAppRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.client.unary_call_async(&METHOD_V2_DELETE_APP, req, opt)
    }

    pub fn delete_app_async(&self, req: &super::service::DeleteAppRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.delete_app_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_apps_opt(&self, req: &super::service::PostAppsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiAppResponse> {
        self.client.unary_call(&METHOD_V2_POST_APPS, req, opt)
    }

    pub fn post_apps(&self, req: &super::service::PostAppsRequest) -> ::grpcio::Result<super::service::MultiAppResponse> {
        self.post_apps_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_apps_async_opt(&self, req: &super::service::PostAppsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiAppResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_APPS, req, opt)
    }

    pub fn post_apps_async(&self, req: &super::service::PostAppsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiAppResponse>> {
        self.post_apps_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_apps_opt(&self, req: &super::service::PatchAppsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiAppResponse> {
        self.client.unary_call(&METHOD_V2_PATCH_APPS, req, opt)
    }

    pub fn patch_apps(&self, req: &super::service::PatchAppsRequest) -> ::grpcio::Result<super::service::MultiAppResponse> {
        self.patch_apps_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_apps_async_opt(&self, req: &super::service::PatchAppsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiAppResponse>> {
        self.client.unary_call_async(&METHOD_V2_PATCH_APPS, req, opt)
    }

    pub fn patch_apps_async(&self, req: &super::service::PatchAppsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiAppResponse>> {
        self.patch_apps_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_apps_searches_opt(&self, req: &super::service::PostAppsSearchesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiAppResponse> {
        self.client.unary_call(&METHOD_V2_POST_APPS_SEARCHES, req, opt)
    }

    pub fn post_apps_searches(&self, req: &super::service::PostAppsSearchesRequest) -> ::grpcio::Result<super::service::MultiAppResponse> {
        self.post_apps_searches_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_apps_searches_async_opt(&self, req: &super::service::PostAppsSearchesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiAppResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_APPS_SEARCHES, req, opt)
    }

    pub fn post_apps_searches_async(&self, req: &super::service::PostAppsSearchesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiAppResponse>> {
        self.post_apps_searches_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_validate_password_opt(&self, req: &super::service::PostValidatePasswordRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SinglePasswordValidationResponse> {
        self.client.unary_call(&METHOD_V2_POST_VALIDATE_PASSWORD, req, opt)
    }

    pub fn post_validate_password(&self, req: &super::service::PostValidatePasswordRequest) -> ::grpcio::Result<super::service::SinglePasswordValidationResponse> {
        self.post_validate_password_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_validate_password_async_opt(&self, req: &super::service::PostValidatePasswordRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SinglePasswordValidationResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_VALIDATE_PASSWORD, req, opt)
    }

    pub fn post_validate_password_async(&self, req: &super::service::PostValidatePasswordRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SinglePasswordValidationResponse>> {
        self.post_validate_password_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_search_opt(&self, req: &super::service::GetSearchRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleSearchResponse> {
        self.client.unary_call(&METHOD_V2_GET_SEARCH, req, opt)
    }

    pub fn get_search(&self, req: &super::service::GetSearchRequest) -> ::grpcio::Result<super::service::SingleSearchResponse> {
        self.get_search_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_search_async_opt(&self, req: &super::service::GetSearchRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleSearchResponse>> {
        self.client.unary_call_async(&METHOD_V2_GET_SEARCH, req, opt)
    }

    pub fn get_search_async(&self, req: &super::service::GetSearchRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleSearchResponse>> {
        self.get_search_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_searches_opt(&self, req: &super::service::ListSearchesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiSearchResponse> {
        self.client.unary_call(&METHOD_V2_LIST_SEARCHES, req, opt)
    }

    pub fn list_searches(&self, req: &super::service::ListSearchesRequest) -> ::grpcio::Result<super::service::MultiSearchResponse> {
        self.list_searches_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_searches_async_opt(&self, req: &super::service::ListSearchesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiSearchResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_SEARCHES, req, opt)
    }

    pub fn list_searches_async(&self, req: &super::service::ListSearchesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiSearchResponse>> {
        self.list_searches_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_searches_opt(&self, req: &super::service::PostSearchesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiSearchResponse> {
        self.client.unary_call(&METHOD_V2_POST_SEARCHES, req, opt)
    }

    pub fn post_searches(&self, req: &super::service::PostSearchesRequest) -> ::grpcio::Result<super::service::MultiSearchResponse> {
        self.post_searches_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_searches_async_opt(&self, req: &super::service::PostSearchesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiSearchResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_SEARCHES, req, opt)
    }

    pub fn post_searches_async(&self, req: &super::service::PostSearchesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiSearchResponse>> {
        self.post_searches_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_searches_by_id_opt(&self, req: &super::service::PostSearchesByIDRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiSearchResponse> {
        self.client.unary_call(&METHOD_V2_POST_SEARCHES_BY_ID, req, opt)
    }

    pub fn post_searches_by_id(&self, req: &super::service::PostSearchesByIDRequest) -> ::grpcio::Result<super::service::MultiSearchResponse> {
        self.post_searches_by_id_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_searches_by_id_async_opt(&self, req: &super::service::PostSearchesByIDRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiSearchResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_SEARCHES_BY_ID, req, opt)
    }

    pub fn post_searches_by_id_async(&self, req: &super::service::PostSearchesByIDRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiSearchResponse>> {
        self.post_searches_by_id_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_annotation_search_metrics_opt(&self, req: &super::service::PostAnnotationSearchMetricsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiAnnotationSearchMetricsResponse> {
        self.client.unary_call(&METHOD_V2_POST_ANNOTATION_SEARCH_METRICS, req, opt)
    }

    pub fn post_annotation_search_metrics(&self, req: &super::service::PostAnnotationSearchMetricsRequest) -> ::grpcio::Result<super::service::MultiAnnotationSearchMetricsResponse> {
        self.post_annotation_search_metrics_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_annotation_search_metrics_async_opt(&self, req: &super::service::PostAnnotationSearchMetricsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiAnnotationSearchMetricsResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_ANNOTATION_SEARCH_METRICS, req, opt)
    }

    pub fn post_annotation_search_metrics_async(&self, req: &super::service::PostAnnotationSearchMetricsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiAnnotationSearchMetricsResponse>> {
        self.post_annotation_search_metrics_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_annotation_search_metrics_opt(&self, req: &super::service::GetAnnotationSearchMetricsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiAnnotationSearchMetricsResponse> {
        self.client.unary_call(&METHOD_V2_GET_ANNOTATION_SEARCH_METRICS, req, opt)
    }

    pub fn get_annotation_search_metrics(&self, req: &super::service::GetAnnotationSearchMetricsRequest) -> ::grpcio::Result<super::service::MultiAnnotationSearchMetricsResponse> {
        self.get_annotation_search_metrics_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_annotation_search_metrics_async_opt(&self, req: &super::service::GetAnnotationSearchMetricsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiAnnotationSearchMetricsResponse>> {
        self.client.unary_call_async(&METHOD_V2_GET_ANNOTATION_SEARCH_METRICS, req, opt)
    }

    pub fn get_annotation_search_metrics_async(&self, req: &super::service::GetAnnotationSearchMetricsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiAnnotationSearchMetricsResponse>> {
        self.get_annotation_search_metrics_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_annotation_search_metrics_opt(&self, req: &super::service::ListAnnotationSearchMetricsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiAnnotationSearchMetricsResponse> {
        self.client.unary_call(&METHOD_V2_LIST_ANNOTATION_SEARCH_METRICS, req, opt)
    }

    pub fn list_annotation_search_metrics(&self, req: &super::service::ListAnnotationSearchMetricsRequest) -> ::grpcio::Result<super::service::MultiAnnotationSearchMetricsResponse> {
        self.list_annotation_search_metrics_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_annotation_search_metrics_async_opt(&self, req: &super::service::ListAnnotationSearchMetricsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiAnnotationSearchMetricsResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_ANNOTATION_SEARCH_METRICS, req, opt)
    }

    pub fn list_annotation_search_metrics_async(&self, req: &super::service::ListAnnotationSearchMetricsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiAnnotationSearchMetricsResponse>> {
        self.list_annotation_search_metrics_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_annotation_search_metrics_opt(&self, req: &super::service::DeleteAnnotationSearchMetricsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::status::BaseResponse> {
        self.client.unary_call(&METHOD_V2_DELETE_ANNOTATION_SEARCH_METRICS, req, opt)
    }

    pub fn delete_annotation_search_metrics(&self, req: &super::service::DeleteAnnotationSearchMetricsRequest) -> ::grpcio::Result<super::status::BaseResponse> {
        self.delete_annotation_search_metrics_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_annotation_search_metrics_async_opt(&self, req: &super::service::DeleteAnnotationSearchMetricsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.client.unary_call_async(&METHOD_V2_DELETE_ANNOTATION_SEARCH_METRICS, req, opt)
    }

    pub fn delete_annotation_search_metrics_async(&self, req: &super::service::DeleteAnnotationSearchMetricsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.delete_annotation_search_metrics_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_search_opt(&self, req: &super::service::DeleteSearchRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::status::BaseResponse> {
        self.client.unary_call(&METHOD_V2_DELETE_SEARCH, req, opt)
    }

    pub fn delete_search(&self, req: &super::service::DeleteSearchRequest) -> ::grpcio::Result<super::status::BaseResponse> {
        self.delete_search_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_search_async_opt(&self, req: &super::service::DeleteSearchRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.client.unary_call_async(&METHOD_V2_DELETE_SEARCH, req, opt)
    }

    pub fn delete_search_async(&self, req: &super::service::DeleteSearchRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.delete_search_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_status_codes_opt(&self, req: &super::service::ListStatusCodesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiStatusCodeResponse> {
        self.client.unary_call(&METHOD_V2_LIST_STATUS_CODES, req, opt)
    }

    pub fn list_status_codes(&self, req: &super::service::ListStatusCodesRequest) -> ::grpcio::Result<super::service::MultiStatusCodeResponse> {
        self.list_status_codes_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_status_codes_async_opt(&self, req: &super::service::ListStatusCodesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiStatusCodeResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_STATUS_CODES, req, opt)
    }

    pub fn list_status_codes_async(&self, req: &super::service::ListStatusCodesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiStatusCodeResponse>> {
        self.list_status_codes_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_status_code_opt(&self, req: &super::service::GetStatusCodeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleStatusCodeResponse> {
        self.client.unary_call(&METHOD_V2_GET_STATUS_CODE, req, opt)
    }

    pub fn get_status_code(&self, req: &super::service::GetStatusCodeRequest) -> ::grpcio::Result<super::service::SingleStatusCodeResponse> {
        self.get_status_code_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_status_code_async_opt(&self, req: &super::service::GetStatusCodeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleStatusCodeResponse>> {
        self.client.unary_call_async(&METHOD_V2_GET_STATUS_CODE, req, opt)
    }

    pub fn get_status_code_async(&self, req: &super::service::GetStatusCodeRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleStatusCodeResponse>> {
        self.get_status_code_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_collaborators_opt(&self, req: &super::service::ListCollaboratorsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiCollaboratorsResponse> {
        self.client.unary_call(&METHOD_V2_LIST_COLLABORATORS, req, opt)
    }

    pub fn list_collaborators(&self, req: &super::service::ListCollaboratorsRequest) -> ::grpcio::Result<super::service::MultiCollaboratorsResponse> {
        self.list_collaborators_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_collaborators_async_opt(&self, req: &super::service::ListCollaboratorsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiCollaboratorsResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_COLLABORATORS, req, opt)
    }

    pub fn list_collaborators_async(&self, req: &super::service::ListCollaboratorsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiCollaboratorsResponse>> {
        self.list_collaborators_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_collaborators_opt(&self, req: &super::service::PostCollaboratorsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiCollaboratorsResponse> {
        self.client.unary_call(&METHOD_V2_POST_COLLABORATORS, req, opt)
    }

    pub fn post_collaborators(&self, req: &super::service::PostCollaboratorsRequest) -> ::grpcio::Result<super::service::MultiCollaboratorsResponse> {
        self.post_collaborators_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_collaborators_async_opt(&self, req: &super::service::PostCollaboratorsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiCollaboratorsResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_COLLABORATORS, req, opt)
    }

    pub fn post_collaborators_async(&self, req: &super::service::PostCollaboratorsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiCollaboratorsResponse>> {
        self.post_collaborators_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_collaborators_opt(&self, req: &super::service::PatchCollaboratorsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiCollaboratorsResponse> {
        self.client.unary_call(&METHOD_V2_PATCH_COLLABORATORS, req, opt)
    }

    pub fn patch_collaborators(&self, req: &super::service::PatchCollaboratorsRequest) -> ::grpcio::Result<super::service::MultiCollaboratorsResponse> {
        self.patch_collaborators_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_collaborators_async_opt(&self, req: &super::service::PatchCollaboratorsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiCollaboratorsResponse>> {
        self.client.unary_call_async(&METHOD_V2_PATCH_COLLABORATORS, req, opt)
    }

    pub fn patch_collaborators_async(&self, req: &super::service::PatchCollaboratorsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiCollaboratorsResponse>> {
        self.patch_collaborators_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_collaborators_opt(&self, req: &super::service::DeleteCollaboratorsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::status::BaseResponse> {
        self.client.unary_call(&METHOD_V2_DELETE_COLLABORATORS, req, opt)
    }

    pub fn delete_collaborators(&self, req: &super::service::DeleteCollaboratorsRequest) -> ::grpcio::Result<super::status::BaseResponse> {
        self.delete_collaborators_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_collaborators_async_opt(&self, req: &super::service::DeleteCollaboratorsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.client.unary_call_async(&METHOD_V2_DELETE_COLLABORATORS, req, opt)
    }

    pub fn delete_collaborators_async(&self, req: &super::service::DeleteCollaboratorsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.delete_collaborators_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_collaborations_opt(&self, req: &super::service::ListCollaborationsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiCollaborationsResponse> {
        self.client.unary_call(&METHOD_V2_LIST_COLLABORATIONS, req, opt)
    }

    pub fn list_collaborations(&self, req: &super::service::ListCollaborationsRequest) -> ::grpcio::Result<super::service::MultiCollaborationsResponse> {
        self.list_collaborations_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_collaborations_async_opt(&self, req: &super::service::ListCollaborationsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiCollaborationsResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_COLLABORATIONS, req, opt)
    }

    pub fn list_collaborations_async(&self, req: &super::service::ListCollaborationsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiCollaborationsResponse>> {
        self.list_collaborations_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_app_duplications_opt(&self, req: &super::service::PostAppDuplicationsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiAppDuplicationsResponse> {
        self.client.unary_call(&METHOD_V2_POST_APP_DUPLICATIONS, req, opt)
    }

    pub fn post_app_duplications(&self, req: &super::service::PostAppDuplicationsRequest) -> ::grpcio::Result<super::service::MultiAppDuplicationsResponse> {
        self.post_app_duplications_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_app_duplications_async_opt(&self, req: &super::service::PostAppDuplicationsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiAppDuplicationsResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_APP_DUPLICATIONS, req, opt)
    }

    pub fn post_app_duplications_async(&self, req: &super::service::PostAppDuplicationsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiAppDuplicationsResponse>> {
        self.post_app_duplications_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_app_duplications_opt(&self, req: &super::service::ListAppDuplicationsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiAppDuplicationsResponse> {
        self.client.unary_call(&METHOD_V2_LIST_APP_DUPLICATIONS, req, opt)
    }

    pub fn list_app_duplications(&self, req: &super::service::ListAppDuplicationsRequest) -> ::grpcio::Result<super::service::MultiAppDuplicationsResponse> {
        self.list_app_duplications_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_app_duplications_async_opt(&self, req: &super::service::ListAppDuplicationsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiAppDuplicationsResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_APP_DUPLICATIONS, req, opt)
    }

    pub fn list_app_duplications_async(&self, req: &super::service::ListAppDuplicationsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiAppDuplicationsResponse>> {
        self.list_app_duplications_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_app_duplication_opt(&self, req: &super::service::GetAppDuplicationRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleAppDuplicationResponse> {
        self.client.unary_call(&METHOD_V2_GET_APP_DUPLICATION, req, opt)
    }

    pub fn get_app_duplication(&self, req: &super::service::GetAppDuplicationRequest) -> ::grpcio::Result<super::service::SingleAppDuplicationResponse> {
        self.get_app_duplication_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_app_duplication_async_opt(&self, req: &super::service::GetAppDuplicationRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleAppDuplicationResponse>> {
        self.client.unary_call_async(&METHOD_V2_GET_APP_DUPLICATION, req, opt)
    }

    pub fn get_app_duplication_async(&self, req: &super::service::GetAppDuplicationRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleAppDuplicationResponse>> {
        self.get_app_duplication_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_tasks_opt(&self, req: &super::service::PostTasksRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiTaskResponse> {
        self.client.unary_call(&METHOD_V2_POST_TASKS, req, opt)
    }

    pub fn post_tasks(&self, req: &super::service::PostTasksRequest) -> ::grpcio::Result<super::service::MultiTaskResponse> {
        self.post_tasks_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_tasks_async_opt(&self, req: &super::service::PostTasksRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiTaskResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_TASKS, req, opt)
    }

    pub fn post_tasks_async(&self, req: &super::service::PostTasksRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiTaskResponse>> {
        self.post_tasks_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_task_annotation_count_opt(&self, req: &super::service::GetTaskCountRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleTaskCountResponse> {
        self.client.unary_call(&METHOD_V2_GET_TASK_ANNOTATION_COUNT, req, opt)
    }

    pub fn get_task_annotation_count(&self, req: &super::service::GetTaskCountRequest) -> ::grpcio::Result<super::service::SingleTaskCountResponse> {
        self.get_task_annotation_count_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_task_annotation_count_async_opt(&self, req: &super::service::GetTaskCountRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleTaskCountResponse>> {
        self.client.unary_call_async(&METHOD_V2_GET_TASK_ANNOTATION_COUNT, req, opt)
    }

    pub fn get_task_annotation_count_async(&self, req: &super::service::GetTaskCountRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleTaskCountResponse>> {
        self.get_task_annotation_count_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_task_input_count_opt(&self, req: &super::service::GetTaskCountRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleTaskCountResponse> {
        self.client.unary_call(&METHOD_V2_GET_TASK_INPUT_COUNT, req, opt)
    }

    pub fn get_task_input_count(&self, req: &super::service::GetTaskCountRequest) -> ::grpcio::Result<super::service::SingleTaskCountResponse> {
        self.get_task_input_count_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_task_input_count_async_opt(&self, req: &super::service::GetTaskCountRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleTaskCountResponse>> {
        self.client.unary_call_async(&METHOD_V2_GET_TASK_INPUT_COUNT, req, opt)
    }

    pub fn get_task_input_count_async(&self, req: &super::service::GetTaskCountRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleTaskCountResponse>> {
        self.get_task_input_count_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_task_opt(&self, req: &super::service::GetTaskRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleTaskResponse> {
        self.client.unary_call(&METHOD_V2_GET_TASK, req, opt)
    }

    pub fn get_task(&self, req: &super::service::GetTaskRequest) -> ::grpcio::Result<super::service::SingleTaskResponse> {
        self.get_task_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_task_async_opt(&self, req: &super::service::GetTaskRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleTaskResponse>> {
        self.client.unary_call_async(&METHOD_V2_GET_TASK, req, opt)
    }

    pub fn get_task_async(&self, req: &super::service::GetTaskRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleTaskResponse>> {
        self.get_task_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_tasks_opt(&self, req: &super::service::ListTasksRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiTaskResponse> {
        self.client.unary_call(&METHOD_V2_LIST_TASKS, req, opt)
    }

    pub fn list_tasks(&self, req: &super::service::ListTasksRequest) -> ::grpcio::Result<super::service::MultiTaskResponse> {
        self.list_tasks_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_tasks_async_opt(&self, req: &super::service::ListTasksRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiTaskResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_TASKS, req, opt)
    }

    pub fn list_tasks_async(&self, req: &super::service::ListTasksRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiTaskResponse>> {
        self.list_tasks_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_tasks_opt(&self, req: &super::service::PatchTasksRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiTaskResponse> {
        self.client.unary_call(&METHOD_V2_PATCH_TASKS, req, opt)
    }

    pub fn patch_tasks(&self, req: &super::service::PatchTasksRequest) -> ::grpcio::Result<super::service::MultiTaskResponse> {
        self.patch_tasks_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_tasks_async_opt(&self, req: &super::service::PatchTasksRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiTaskResponse>> {
        self.client.unary_call_async(&METHOD_V2_PATCH_TASKS, req, opt)
    }

    pub fn patch_tasks_async(&self, req: &super::service::PatchTasksRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiTaskResponse>> {
        self.patch_tasks_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_tasks_opt(&self, req: &super::service::DeleteTasksRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::status::BaseResponse> {
        self.client.unary_call(&METHOD_V2_DELETE_TASKS, req, opt)
    }

    pub fn delete_tasks(&self, req: &super::service::DeleteTasksRequest) -> ::grpcio::Result<super::status::BaseResponse> {
        self.delete_tasks_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_tasks_async_opt(&self, req: &super::service::DeleteTasksRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.client.unary_call_async(&METHOD_V2_DELETE_TASKS, req, opt)
    }

    pub fn delete_tasks_async(&self, req: &super::service::DeleteTasksRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.delete_tasks_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_collectors_opt(&self, req: &super::service::PostCollectorsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiCollectorResponse> {
        self.client.unary_call(&METHOD_V2_POST_COLLECTORS, req, opt)
    }

    pub fn post_collectors(&self, req: &super::service::PostCollectorsRequest) -> ::grpcio::Result<super::service::MultiCollectorResponse> {
        self.post_collectors_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_collectors_async_opt(&self, req: &super::service::PostCollectorsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiCollectorResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_COLLECTORS, req, opt)
    }

    pub fn post_collectors_async(&self, req: &super::service::PostCollectorsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiCollectorResponse>> {
        self.post_collectors_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_collector_opt(&self, req: &super::service::GetCollectorRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleCollectorResponse> {
        self.client.unary_call(&METHOD_V2_GET_COLLECTOR, req, opt)
    }

    pub fn get_collector(&self, req: &super::service::GetCollectorRequest) -> ::grpcio::Result<super::service::SingleCollectorResponse> {
        self.get_collector_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_collector_async_opt(&self, req: &super::service::GetCollectorRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleCollectorResponse>> {
        self.client.unary_call_async(&METHOD_V2_GET_COLLECTOR, req, opt)
    }

    pub fn get_collector_async(&self, req: &super::service::GetCollectorRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleCollectorResponse>> {
        self.get_collector_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_collectors_opt(&self, req: &super::service::ListCollectorsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiCollectorResponse> {
        self.client.unary_call(&METHOD_V2_LIST_COLLECTORS, req, opt)
    }

    pub fn list_collectors(&self, req: &super::service::ListCollectorsRequest) -> ::grpcio::Result<super::service::MultiCollectorResponse> {
        self.list_collectors_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_collectors_async_opt(&self, req: &super::service::ListCollectorsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiCollectorResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_COLLECTORS, req, opt)
    }

    pub fn list_collectors_async(&self, req: &super::service::ListCollectorsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiCollectorResponse>> {
        self.list_collectors_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_collectors_opt(&self, req: &super::service::PatchCollectorsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiCollectorResponse> {
        self.client.unary_call(&METHOD_V2_PATCH_COLLECTORS, req, opt)
    }

    pub fn patch_collectors(&self, req: &super::service::PatchCollectorsRequest) -> ::grpcio::Result<super::service::MultiCollectorResponse> {
        self.patch_collectors_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_collectors_async_opt(&self, req: &super::service::PatchCollectorsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiCollectorResponse>> {
        self.client.unary_call_async(&METHOD_V2_PATCH_COLLECTORS, req, opt)
    }

    pub fn patch_collectors_async(&self, req: &super::service::PatchCollectorsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiCollectorResponse>> {
        self.patch_collectors_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_collectors_opt(&self, req: &super::service::DeleteCollectorsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::status::BaseResponse> {
        self.client.unary_call(&METHOD_V2_DELETE_COLLECTORS, req, opt)
    }

    pub fn delete_collectors(&self, req: &super::service::DeleteCollectorsRequest) -> ::grpcio::Result<super::status::BaseResponse> {
        self.delete_collectors_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_collectors_async_opt(&self, req: &super::service::DeleteCollectorsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.client.unary_call_async(&METHOD_V2_DELETE_COLLECTORS, req, opt)
    }

    pub fn delete_collectors_async(&self, req: &super::service::DeleteCollectorsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.delete_collectors_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_stat_values_opt(&self, req: &super::service::PostStatValuesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiStatValueResponse> {
        self.client.unary_call(&METHOD_V2_POST_STAT_VALUES, req, opt)
    }

    pub fn post_stat_values(&self, req: &super::service::PostStatValuesRequest) -> ::grpcio::Result<super::service::MultiStatValueResponse> {
        self.post_stat_values_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_stat_values_async_opt(&self, req: &super::service::PostStatValuesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiStatValueResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_STAT_VALUES, req, opt)
    }

    pub fn post_stat_values_async(&self, req: &super::service::PostStatValuesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiStatValueResponse>> {
        self.post_stat_values_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_stat_values_aggregate_opt(&self, req: &super::service::PostStatValuesAggregateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiStatValueAggregateResponse> {
        self.client.unary_call(&METHOD_V2_POST_STAT_VALUES_AGGREGATE, req, opt)
    }

    pub fn post_stat_values_aggregate(&self, req: &super::service::PostStatValuesAggregateRequest) -> ::grpcio::Result<super::service::MultiStatValueAggregateResponse> {
        self.post_stat_values_aggregate_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_stat_values_aggregate_async_opt(&self, req: &super::service::PostStatValuesAggregateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiStatValueAggregateResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_STAT_VALUES_AGGREGATE, req, opt)
    }

    pub fn post_stat_values_aggregate_async(&self, req: &super::service::PostStatValuesAggregateRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiStatValueAggregateResponse>> {
        self.post_stat_values_aggregate_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait V2 {
    fn list_concept_relations(&mut self, ctx: ::grpcio::RpcContext, req: super::service::ListConceptRelationsRequest, sink: ::grpcio::UnarySink<super::service::MultiConceptRelationResponse>);
    fn post_concept_relations(&mut self, ctx: ::grpcio::RpcContext, req: super::service::PostConceptRelationsRequest, sink: ::grpcio::UnarySink<super::service::MultiConceptRelationResponse>);
    fn delete_concept_relations(&mut self, ctx: ::grpcio::RpcContext, req: super::service::DeleteConceptRelationsRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>);
    fn get_concept_counts(&mut self, ctx: ::grpcio::RpcContext, req: super::service::GetConceptCountsRequest, sink: ::grpcio::UnarySink<super::service::MultiConceptCountResponse>);
    fn get_concept(&mut self, ctx: ::grpcio::RpcContext, req: super::service::GetConceptRequest, sink: ::grpcio::UnarySink<super::service::SingleConceptResponse>);
    fn list_concepts(&mut self, ctx: ::grpcio::RpcContext, req: super::service::ListConceptsRequest, sink: ::grpcio::UnarySink<super::service::MultiConceptResponse>);
    fn post_concepts_searches(&mut self, ctx: ::grpcio::RpcContext, req: super::service::PostConceptsSearchesRequest, sink: ::grpcio::UnarySink<super::service::MultiConceptResponse>);
    fn post_concepts(&mut self, ctx: ::grpcio::RpcContext, req: super::service::PostConceptsRequest, sink: ::grpcio::UnarySink<super::service::MultiConceptResponse>);
    fn patch_concepts(&mut self, ctx: ::grpcio::RpcContext, req: super::service::PatchConceptsRequest, sink: ::grpcio::UnarySink<super::service::MultiConceptResponse>);
    fn get_concept_language(&mut self, ctx: ::grpcio::RpcContext, req: super::service::GetConceptLanguageRequest, sink: ::grpcio::UnarySink<super::service::SingleConceptLanguageResponse>);
    fn list_concept_languages(&mut self, ctx: ::grpcio::RpcContext, req: super::service::ListConceptLanguagesRequest, sink: ::grpcio::UnarySink<super::service::MultiConceptLanguageResponse>);
    fn post_concept_languages(&mut self, ctx: ::grpcio::RpcContext, req: super::service::PostConceptLanguagesRequest, sink: ::grpcio::UnarySink<super::service::MultiConceptLanguageResponse>);
    fn patch_concept_languages(&mut self, ctx: ::grpcio::RpcContext, req: super::service::PatchConceptLanguagesRequest, sink: ::grpcio::UnarySink<super::service::MultiConceptLanguageResponse>);
    fn list_knowledge_graphs(&mut self, ctx: ::grpcio::RpcContext, req: super::service::ListKnowledgeGraphsRequest, sink: ::grpcio::UnarySink<super::service::MultiKnowledgeGraphResponse>);
    fn post_knowledge_graphs(&mut self, ctx: ::grpcio::RpcContext, req: super::service::PostKnowledgeGraphsRequest, sink: ::grpcio::UnarySink<super::service::MultiKnowledgeGraphResponse>);
    fn post_concept_mapping_jobs(&mut self, ctx: ::grpcio::RpcContext, req: super::service::PostConceptMappingJobsRequest, sink: ::grpcio::UnarySink<super::service::MultiConceptMappingJobResponse>);
    fn get_annotation(&mut self, ctx: ::grpcio::RpcContext, req: super::service::GetAnnotationRequest, sink: ::grpcio::UnarySink<super::service::SingleAnnotationResponse>);
    fn list_annotations(&mut self, ctx: ::grpcio::RpcContext, req: super::service::ListAnnotationsRequest, sink: ::grpcio::UnarySink<super::service::MultiAnnotationResponse>);
    fn post_annotations(&mut self, ctx: ::grpcio::RpcContext, req: super::service::PostAnnotationsRequest, sink: ::grpcio::UnarySink<super::service::MultiAnnotationResponse>);
    fn patch_annotations(&mut self, ctx: ::grpcio::RpcContext, req: super::service::PatchAnnotationsRequest, sink: ::grpcio::UnarySink<super::service::MultiAnnotationResponse>);
    fn patch_annotations_status(&mut self, ctx: ::grpcio::RpcContext, req: super::service::PatchAnnotationsStatusRequest, sink: ::grpcio::UnarySink<super::service::PatchAnnotationsStatusResponse>);
    fn delete_annotation(&mut self, ctx: ::grpcio::RpcContext, req: super::service::DeleteAnnotationRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>);
    fn delete_annotations(&mut self, ctx: ::grpcio::RpcContext, req: super::service::DeleteAnnotationsRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>);
    fn post_annotations_searches(&mut self, ctx: ::grpcio::RpcContext, req: super::service::PostAnnotationsSearchesRequest, sink: ::grpcio::UnarySink<super::service::MultiSearchResponse>);
    fn get_input_count(&mut self, ctx: ::grpcio::RpcContext, req: super::service::GetInputCountRequest, sink: ::grpcio::UnarySink<super::service::SingleInputCountResponse>);
    fn stream_inputs(&mut self, ctx: ::grpcio::RpcContext, req: super::service::StreamInputsRequest, sink: ::grpcio::UnarySink<super::service::MultiInputResponse>);
    fn get_input_samples(&mut self, ctx: ::grpcio::RpcContext, req: super::service::GetInputSamplesRequest, sink: ::grpcio::UnarySink<super::service::MultiInputAnnotationResponse>);
    fn get_input(&mut self, ctx: ::grpcio::RpcContext, req: super::service::GetInputRequest, sink: ::grpcio::UnarySink<super::service::SingleInputResponse>);
    fn list_inputs(&mut self, ctx: ::grpcio::RpcContext, req: super::service::ListInputsRequest, sink: ::grpcio::UnarySink<super::service::MultiInputResponse>);
    fn post_inputs(&mut self, ctx: ::grpcio::RpcContext, req: super::service::PostInputsRequest, sink: ::grpcio::UnarySink<super::service::MultiInputResponse>);
    fn patch_inputs(&mut self, ctx: ::grpcio::RpcContext, req: super::service::PatchInputsRequest, sink: ::grpcio::UnarySink<super::service::MultiInputResponse>);
    fn delete_input(&mut self, ctx: ::grpcio::RpcContext, req: super::service::DeleteInputRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>);
    fn delete_inputs(&mut self, ctx: ::grpcio::RpcContext, req: super::service::DeleteInputsRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>);
    fn post_inputs_searches(&mut self, ctx: ::grpcio::RpcContext, req: super::service::PostInputsSearchesRequest, sink: ::grpcio::UnarySink<super::service::MultiSearchResponse>);
    fn post_model_outputs(&mut self, ctx: ::grpcio::RpcContext, req: super::service::PostModelOutputsRequest, sink: ::grpcio::UnarySink<super::service::MultiOutputResponse>);
    fn get_model_type(&mut self, ctx: ::grpcio::RpcContext, req: super::service::GetModelTypeRequest, sink: ::grpcio::UnarySink<super::service::SingleModelTypeResponse>);
    fn list_model_types(&mut self, ctx: ::grpcio::RpcContext, req: super::service::ListModelTypesRequest, sink: ::grpcio::UnarySink<super::service::MultiModelTypeResponse>);
    fn get_model(&mut self, ctx: ::grpcio::RpcContext, req: super::service::GetModelRequest, sink: ::grpcio::UnarySink<super::service::SingleModelResponse>);
    fn get_model_output_info(&mut self, ctx: ::grpcio::RpcContext, req: super::service::GetModelRequest, sink: ::grpcio::UnarySink<super::service::SingleModelResponse>);
    fn list_models(&mut self, ctx: ::grpcio::RpcContext, req: super::service::ListModelsRequest, sink: ::grpcio::UnarySink<super::service::MultiModelResponse>);
    fn post_models_searches(&mut self, ctx: ::grpcio::RpcContext, req: super::service::PostModelsSearchesRequest, sink: ::grpcio::UnarySink<super::service::MultiModelResponse>);
    fn post_models(&mut self, ctx: ::grpcio::RpcContext, req: super::service::PostModelsRequest, sink: ::grpcio::UnarySink<super::service::SingleModelResponse>);
    fn patch_models(&mut self, ctx: ::grpcio::RpcContext, req: super::service::PatchModelsRequest, sink: ::grpcio::UnarySink<super::service::MultiModelResponse>);
    fn delete_model(&mut self, ctx: ::grpcio::RpcContext, req: super::service::DeleteModelRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>);
    fn delete_models(&mut self, ctx: ::grpcio::RpcContext, req: super::service::DeleteModelsRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>);
    fn list_model_inputs(&mut self, ctx: ::grpcio::RpcContext, req: super::service::ListModelInputsRequest, sink: ::grpcio::UnarySink<super::service::MultiInputResponse>);
    fn get_model_version(&mut self, ctx: ::grpcio::RpcContext, req: super::service::GetModelVersionRequest, sink: ::grpcio::UnarySink<super::service::SingleModelVersionResponse>);
    fn list_model_versions(&mut self, ctx: ::grpcio::RpcContext, req: super::service::ListModelVersionsRequest, sink: ::grpcio::UnarySink<super::service::MultiModelVersionResponse>);
    fn post_model_versions(&mut self, ctx: ::grpcio::RpcContext, req: super::service::PostModelVersionsRequest, sink: ::grpcio::UnarySink<super::service::SingleModelResponse>);
    fn delete_model_version(&mut self, ctx: ::grpcio::RpcContext, req: super::service::DeleteModelVersionRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>);
    fn get_model_version_metrics(&mut self, ctx: ::grpcio::RpcContext, req: super::service::GetModelVersionMetricsRequest, sink: ::grpcio::UnarySink<super::service::SingleModelVersionResponse>);
    fn post_model_version_metrics(&mut self, ctx: ::grpcio::RpcContext, req: super::service::PostModelVersionMetricsRequest, sink: ::grpcio::UnarySink<super::service::SingleModelVersionResponse>);
    fn get_workflow(&mut self, ctx: ::grpcio::RpcContext, req: super::service::GetWorkflowRequest, sink: ::grpcio::UnarySink<super::service::SingleWorkflowResponse>);
    fn list_workflows(&mut self, ctx: ::grpcio::RpcContext, req: super::service::ListWorkflowsRequest, sink: ::grpcio::UnarySink<super::service::MultiWorkflowResponse>);
    fn post_workflows(&mut self, ctx: ::grpcio::RpcContext, req: super::service::PostWorkflowsRequest, sink: ::grpcio::UnarySink<super::service::MultiWorkflowResponse>);
    fn patch_workflows(&mut self, ctx: ::grpcio::RpcContext, req: super::service::PatchWorkflowsRequest, sink: ::grpcio::UnarySink<super::service::MultiWorkflowResponse>);
    fn delete_workflow(&mut self, ctx: ::grpcio::RpcContext, req: super::service::DeleteWorkflowRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>);
    fn delete_workflows(&mut self, ctx: ::grpcio::RpcContext, req: super::service::DeleteWorkflowsRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>);
    fn post_workflow_results(&mut self, ctx: ::grpcio::RpcContext, req: super::service::PostWorkflowResultsRequest, sink: ::grpcio::UnarySink<super::service::PostWorkflowResultsResponse>);
    fn post_workflow_results_similarity(&mut self, ctx: ::grpcio::RpcContext, req: super::service::PostWorkflowResultsSimilarityRequest, sink: ::grpcio::UnarySink<super::service::PostWorkflowResultsSimilarityResponse>);
    fn get_key(&mut self, ctx: ::grpcio::RpcContext, req: super::service::GetKeyRequest, sink: ::grpcio::UnarySink<super::service::SingleKeyResponse>);
    fn list_keys(&mut self, ctx: ::grpcio::RpcContext, req: super::service::ListKeysRequest, sink: ::grpcio::UnarySink<super::service::MultiKeyResponse>);
    fn list_app_keys(&mut self, ctx: ::grpcio::RpcContext, req: super::service::ListAppKeysRequest, sink: ::grpcio::UnarySink<super::service::MultiKeyResponse>);
    fn delete_key(&mut self, ctx: ::grpcio::RpcContext, req: super::service::DeleteKeyRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>);
    fn post_keys(&mut self, ctx: ::grpcio::RpcContext, req: super::service::PostKeysRequest, sink: ::grpcio::UnarySink<super::service::MultiKeyResponse>);
    fn patch_keys(&mut self, ctx: ::grpcio::RpcContext, req: super::service::PatchKeysRequest, sink: ::grpcio::UnarySink<super::service::MultiKeyResponse>);
    fn my_scopes(&mut self, ctx: ::grpcio::RpcContext, req: super::service::MyScopesRequest, sink: ::grpcio::UnarySink<super::service::MultiScopeResponse>);
    fn list_scopes(&mut self, ctx: ::grpcio::RpcContext, req: super::service::ListScopesRequest, sink: ::grpcio::UnarySink<super::service::MultiScopeDepsResponse>);
    fn get_app(&mut self, ctx: ::grpcio::RpcContext, req: super::service::GetAppRequest, sink: ::grpcio::UnarySink<super::service::SingleAppResponse>);
    fn list_apps(&mut self, ctx: ::grpcio::RpcContext, req: super::service::ListAppsRequest, sink: ::grpcio::UnarySink<super::service::MultiAppResponse>);
    fn delete_app(&mut self, ctx: ::grpcio::RpcContext, req: super::service::DeleteAppRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>);
    fn post_apps(&mut self, ctx: ::grpcio::RpcContext, req: super::service::PostAppsRequest, sink: ::grpcio::UnarySink<super::service::MultiAppResponse>);
    fn patch_apps(&mut self, ctx: ::grpcio::RpcContext, req: super::service::PatchAppsRequest, sink: ::grpcio::UnarySink<super::service::MultiAppResponse>);
    fn post_apps_searches(&mut self, ctx: ::grpcio::RpcContext, req: super::service::PostAppsSearchesRequest, sink: ::grpcio::UnarySink<super::service::MultiAppResponse>);
    fn post_validate_password(&mut self, ctx: ::grpcio::RpcContext, req: super::service::PostValidatePasswordRequest, sink: ::grpcio::UnarySink<super::service::SinglePasswordValidationResponse>);
    fn get_search(&mut self, ctx: ::grpcio::RpcContext, req: super::service::GetSearchRequest, sink: ::grpcio::UnarySink<super::service::SingleSearchResponse>);
    fn list_searches(&mut self, ctx: ::grpcio::RpcContext, req: super::service::ListSearchesRequest, sink: ::grpcio::UnarySink<super::service::MultiSearchResponse>);
    fn post_searches(&mut self, ctx: ::grpcio::RpcContext, req: super::service::PostSearchesRequest, sink: ::grpcio::UnarySink<super::service::MultiSearchResponse>);
    fn post_searches_by_id(&mut self, ctx: ::grpcio::RpcContext, req: super::service::PostSearchesByIDRequest, sink: ::grpcio::UnarySink<super::service::MultiSearchResponse>);
    fn post_annotation_search_metrics(&mut self, ctx: ::grpcio::RpcContext, req: super::service::PostAnnotationSearchMetricsRequest, sink: ::grpcio::UnarySink<super::service::MultiAnnotationSearchMetricsResponse>);
    fn get_annotation_search_metrics(&mut self, ctx: ::grpcio::RpcContext, req: super::service::GetAnnotationSearchMetricsRequest, sink: ::grpcio::UnarySink<super::service::MultiAnnotationSearchMetricsResponse>);
    fn list_annotation_search_metrics(&mut self, ctx: ::grpcio::RpcContext, req: super::service::ListAnnotationSearchMetricsRequest, sink: ::grpcio::UnarySink<super::service::MultiAnnotationSearchMetricsResponse>);
    fn delete_annotation_search_metrics(&mut self, ctx: ::grpcio::RpcContext, req: super::service::DeleteAnnotationSearchMetricsRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>);
    fn delete_search(&mut self, ctx: ::grpcio::RpcContext, req: super::service::DeleteSearchRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>);
    fn list_status_codes(&mut self, ctx: ::grpcio::RpcContext, req: super::service::ListStatusCodesRequest, sink: ::grpcio::UnarySink<super::service::MultiStatusCodeResponse>);
    fn get_status_code(&mut self, ctx: ::grpcio::RpcContext, req: super::service::GetStatusCodeRequest, sink: ::grpcio::UnarySink<super::service::SingleStatusCodeResponse>);
    fn list_collaborators(&mut self, ctx: ::grpcio::RpcContext, req: super::service::ListCollaboratorsRequest, sink: ::grpcio::UnarySink<super::service::MultiCollaboratorsResponse>);
    fn post_collaborators(&mut self, ctx: ::grpcio::RpcContext, req: super::service::PostCollaboratorsRequest, sink: ::grpcio::UnarySink<super::service::MultiCollaboratorsResponse>);
    fn patch_collaborators(&mut self, ctx: ::grpcio::RpcContext, req: super::service::PatchCollaboratorsRequest, sink: ::grpcio::UnarySink<super::service::MultiCollaboratorsResponse>);
    fn delete_collaborators(&mut self, ctx: ::grpcio::RpcContext, req: super::service::DeleteCollaboratorsRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>);
    fn list_collaborations(&mut self, ctx: ::grpcio::RpcContext, req: super::service::ListCollaborationsRequest, sink: ::grpcio::UnarySink<super::service::MultiCollaborationsResponse>);
    fn post_app_duplications(&mut self, ctx: ::grpcio::RpcContext, req: super::service::PostAppDuplicationsRequest, sink: ::grpcio::UnarySink<super::service::MultiAppDuplicationsResponse>);
    fn list_app_duplications(&mut self, ctx: ::grpcio::RpcContext, req: super::service::ListAppDuplicationsRequest, sink: ::grpcio::UnarySink<super::service::MultiAppDuplicationsResponse>);
    fn get_app_duplication(&mut self, ctx: ::grpcio::RpcContext, req: super::service::GetAppDuplicationRequest, sink: ::grpcio::UnarySink<super::service::SingleAppDuplicationResponse>);
    fn post_tasks(&mut self, ctx: ::grpcio::RpcContext, req: super::service::PostTasksRequest, sink: ::grpcio::UnarySink<super::service::MultiTaskResponse>);
    fn get_task_annotation_count(&mut self, ctx: ::grpcio::RpcContext, req: super::service::GetTaskCountRequest, sink: ::grpcio::UnarySink<super::service::SingleTaskCountResponse>);
    fn get_task_input_count(&mut self, ctx: ::grpcio::RpcContext, req: super::service::GetTaskCountRequest, sink: ::grpcio::UnarySink<super::service::SingleTaskCountResponse>);
    fn get_task(&mut self, ctx: ::grpcio::RpcContext, req: super::service::GetTaskRequest, sink: ::grpcio::UnarySink<super::service::SingleTaskResponse>);
    fn list_tasks(&mut self, ctx: ::grpcio::RpcContext, req: super::service::ListTasksRequest, sink: ::grpcio::UnarySink<super::service::MultiTaskResponse>);
    fn patch_tasks(&mut self, ctx: ::grpcio::RpcContext, req: super::service::PatchTasksRequest, sink: ::grpcio::UnarySink<super::service::MultiTaskResponse>);
    fn delete_tasks(&mut self, ctx: ::grpcio::RpcContext, req: super::service::DeleteTasksRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>);
    fn post_collectors(&mut self, ctx: ::grpcio::RpcContext, req: super::service::PostCollectorsRequest, sink: ::grpcio::UnarySink<super::service::MultiCollectorResponse>);
    fn get_collector(&mut self, ctx: ::grpcio::RpcContext, req: super::service::GetCollectorRequest, sink: ::grpcio::UnarySink<super::service::SingleCollectorResponse>);
    fn list_collectors(&mut self, ctx: ::grpcio::RpcContext, req: super::service::ListCollectorsRequest, sink: ::grpcio::UnarySink<super::service::MultiCollectorResponse>);
    fn patch_collectors(&mut self, ctx: ::grpcio::RpcContext, req: super::service::PatchCollectorsRequest, sink: ::grpcio::UnarySink<super::service::MultiCollectorResponse>);
    fn delete_collectors(&mut self, ctx: ::grpcio::RpcContext, req: super::service::DeleteCollectorsRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>);
    fn post_stat_values(&mut self, ctx: ::grpcio::RpcContext, req: super::service::PostStatValuesRequest, sink: ::grpcio::UnarySink<super::service::MultiStatValueResponse>);
    fn post_stat_values_aggregate(&mut self, ctx: ::grpcio::RpcContext, req: super::service::PostStatValuesAggregateRequest, sink: ::grpcio::UnarySink<super::service::MultiStatValueAggregateResponse>);
}

pub fn create_v2<S: V2 + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_LIST_CONCEPT_RELATIONS, move |ctx, req, resp| {
        instance.list_concept_relations(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_CONCEPT_RELATIONS, move |ctx, req, resp| {
        instance.post_concept_relations(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_DELETE_CONCEPT_RELATIONS, move |ctx, req, resp| {
        instance.delete_concept_relations(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_GET_CONCEPT_COUNTS, move |ctx, req, resp| {
        instance.get_concept_counts(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_GET_CONCEPT, move |ctx, req, resp| {
        instance.get_concept(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_LIST_CONCEPTS, move |ctx, req, resp| {
        instance.list_concepts(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_CONCEPTS_SEARCHES, move |ctx, req, resp| {
        instance.post_concepts_searches(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_CONCEPTS, move |ctx, req, resp| {
        instance.post_concepts(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_PATCH_CONCEPTS, move |ctx, req, resp| {
        instance.patch_concepts(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_GET_CONCEPT_LANGUAGE, move |ctx, req, resp| {
        instance.get_concept_language(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_LIST_CONCEPT_LANGUAGES, move |ctx, req, resp| {
        instance.list_concept_languages(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_CONCEPT_LANGUAGES, move |ctx, req, resp| {
        instance.post_concept_languages(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_PATCH_CONCEPT_LANGUAGES, move |ctx, req, resp| {
        instance.patch_concept_languages(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_LIST_KNOWLEDGE_GRAPHS, move |ctx, req, resp| {
        instance.list_knowledge_graphs(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_KNOWLEDGE_GRAPHS, move |ctx, req, resp| {
        instance.post_knowledge_graphs(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_CONCEPT_MAPPING_JOBS, move |ctx, req, resp| {
        instance.post_concept_mapping_jobs(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_GET_ANNOTATION, move |ctx, req, resp| {
        instance.get_annotation(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_LIST_ANNOTATIONS, move |ctx, req, resp| {
        instance.list_annotations(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_ANNOTATIONS, move |ctx, req, resp| {
        instance.post_annotations(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_PATCH_ANNOTATIONS, move |ctx, req, resp| {
        instance.patch_annotations(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_PATCH_ANNOTATIONS_STATUS, move |ctx, req, resp| {
        instance.patch_annotations_status(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_DELETE_ANNOTATION, move |ctx, req, resp| {
        instance.delete_annotation(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_DELETE_ANNOTATIONS, move |ctx, req, resp| {
        instance.delete_annotations(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_ANNOTATIONS_SEARCHES, move |ctx, req, resp| {
        instance.post_annotations_searches(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_GET_INPUT_COUNT, move |ctx, req, resp| {
        instance.get_input_count(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_STREAM_INPUTS, move |ctx, req, resp| {
        instance.stream_inputs(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_GET_INPUT_SAMPLES, move |ctx, req, resp| {
        instance.get_input_samples(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_GET_INPUT, move |ctx, req, resp| {
        instance.get_input(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_LIST_INPUTS, move |ctx, req, resp| {
        instance.list_inputs(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_INPUTS, move |ctx, req, resp| {
        instance.post_inputs(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_PATCH_INPUTS, move |ctx, req, resp| {
        instance.patch_inputs(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_DELETE_INPUT, move |ctx, req, resp| {
        instance.delete_input(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_DELETE_INPUTS, move |ctx, req, resp| {
        instance.delete_inputs(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_INPUTS_SEARCHES, move |ctx, req, resp| {
        instance.post_inputs_searches(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_MODEL_OUTPUTS, move |ctx, req, resp| {
        instance.post_model_outputs(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_GET_MODEL_TYPE, move |ctx, req, resp| {
        instance.get_model_type(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_LIST_MODEL_TYPES, move |ctx, req, resp| {
        instance.list_model_types(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_GET_MODEL, move |ctx, req, resp| {
        instance.get_model(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_GET_MODEL_OUTPUT_INFO, move |ctx, req, resp| {
        instance.get_model_output_info(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_LIST_MODELS, move |ctx, req, resp| {
        instance.list_models(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_MODELS_SEARCHES, move |ctx, req, resp| {
        instance.post_models_searches(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_MODELS, move |ctx, req, resp| {
        instance.post_models(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_PATCH_MODELS, move |ctx, req, resp| {
        instance.patch_models(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_DELETE_MODEL, move |ctx, req, resp| {
        instance.delete_model(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_DELETE_MODELS, move |ctx, req, resp| {
        instance.delete_models(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_LIST_MODEL_INPUTS, move |ctx, req, resp| {
        instance.list_model_inputs(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_GET_MODEL_VERSION, move |ctx, req, resp| {
        instance.get_model_version(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_LIST_MODEL_VERSIONS, move |ctx, req, resp| {
        instance.list_model_versions(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_MODEL_VERSIONS, move |ctx, req, resp| {
        instance.post_model_versions(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_DELETE_MODEL_VERSION, move |ctx, req, resp| {
        instance.delete_model_version(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_GET_MODEL_VERSION_METRICS, move |ctx, req, resp| {
        instance.get_model_version_metrics(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_MODEL_VERSION_METRICS, move |ctx, req, resp| {
        instance.post_model_version_metrics(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_GET_WORKFLOW, move |ctx, req, resp| {
        instance.get_workflow(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_LIST_WORKFLOWS, move |ctx, req, resp| {
        instance.list_workflows(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_WORKFLOWS, move |ctx, req, resp| {
        instance.post_workflows(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_PATCH_WORKFLOWS, move |ctx, req, resp| {
        instance.patch_workflows(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_DELETE_WORKFLOW, move |ctx, req, resp| {
        instance.delete_workflow(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_DELETE_WORKFLOWS, move |ctx, req, resp| {
        instance.delete_workflows(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_WORKFLOW_RESULTS, move |ctx, req, resp| {
        instance.post_workflow_results(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_WORKFLOW_RESULTS_SIMILARITY, move |ctx, req, resp| {
        instance.post_workflow_results_similarity(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_GET_KEY, move |ctx, req, resp| {
        instance.get_key(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_LIST_KEYS, move |ctx, req, resp| {
        instance.list_keys(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_LIST_APP_KEYS, move |ctx, req, resp| {
        instance.list_app_keys(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_DELETE_KEY, move |ctx, req, resp| {
        instance.delete_key(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_KEYS, move |ctx, req, resp| {
        instance.post_keys(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_PATCH_KEYS, move |ctx, req, resp| {
        instance.patch_keys(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_MY_SCOPES, move |ctx, req, resp| {
        instance.my_scopes(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_LIST_SCOPES, move |ctx, req, resp| {
        instance.list_scopes(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_GET_APP, move |ctx, req, resp| {
        instance.get_app(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_LIST_APPS, move |ctx, req, resp| {
        instance.list_apps(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_DELETE_APP, move |ctx, req, resp| {
        instance.delete_app(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_APPS, move |ctx, req, resp| {
        instance.post_apps(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_PATCH_APPS, move |ctx, req, resp| {
        instance.patch_apps(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_APPS_SEARCHES, move |ctx, req, resp| {
        instance.post_apps_searches(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_VALIDATE_PASSWORD, move |ctx, req, resp| {
        instance.post_validate_password(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_GET_SEARCH, move |ctx, req, resp| {
        instance.get_search(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_LIST_SEARCHES, move |ctx, req, resp| {
        instance.list_searches(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_SEARCHES, move |ctx, req, resp| {
        instance.post_searches(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_SEARCHES_BY_ID, move |ctx, req, resp| {
        instance.post_searches_by_id(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_ANNOTATION_SEARCH_METRICS, move |ctx, req, resp| {
        instance.post_annotation_search_metrics(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_GET_ANNOTATION_SEARCH_METRICS, move |ctx, req, resp| {
        instance.get_annotation_search_metrics(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_LIST_ANNOTATION_SEARCH_METRICS, move |ctx, req, resp| {
        instance.list_annotation_search_metrics(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_DELETE_ANNOTATION_SEARCH_METRICS, move |ctx, req, resp| {
        instance.delete_annotation_search_metrics(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_DELETE_SEARCH, move |ctx, req, resp| {
        instance.delete_search(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_LIST_STATUS_CODES, move |ctx, req, resp| {
        instance.list_status_codes(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_GET_STATUS_CODE, move |ctx, req, resp| {
        instance.get_status_code(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_LIST_COLLABORATORS, move |ctx, req, resp| {
        instance.list_collaborators(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_COLLABORATORS, move |ctx, req, resp| {
        instance.post_collaborators(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_PATCH_COLLABORATORS, move |ctx, req, resp| {
        instance.patch_collaborators(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_DELETE_COLLABORATORS, move |ctx, req, resp| {
        instance.delete_collaborators(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_LIST_COLLABORATIONS, move |ctx, req, resp| {
        instance.list_collaborations(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_APP_DUPLICATIONS, move |ctx, req, resp| {
        instance.post_app_duplications(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_LIST_APP_DUPLICATIONS, move |ctx, req, resp| {
        instance.list_app_duplications(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_GET_APP_DUPLICATION, move |ctx, req, resp| {
        instance.get_app_duplication(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_TASKS, move |ctx, req, resp| {
        instance.post_tasks(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_GET_TASK_ANNOTATION_COUNT, move |ctx, req, resp| {
        instance.get_task_annotation_count(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_GET_TASK_INPUT_COUNT, move |ctx, req, resp| {
        instance.get_task_input_count(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_GET_TASK, move |ctx, req, resp| {
        instance.get_task(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_LIST_TASKS, move |ctx, req, resp| {
        instance.list_tasks(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_PATCH_TASKS, move |ctx, req, resp| {
        instance.patch_tasks(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_DELETE_TASKS, move |ctx, req, resp| {
        instance.delete_tasks(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_COLLECTORS, move |ctx, req, resp| {
        instance.post_collectors(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_GET_COLLECTOR, move |ctx, req, resp| {
        instance.get_collector(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_LIST_COLLECTORS, move |ctx, req, resp| {
        instance.list_collectors(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_PATCH_COLLECTORS, move |ctx, req, resp| {
        instance.patch_collectors(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_DELETE_COLLECTORS, move |ctx, req, resp| {
        instance.delete_collectors(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_STAT_VALUES, move |ctx, req, resp| {
        instance.post_stat_values(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_V2_POST_STAT_VALUES_AGGREGATE, move |ctx, req, resp| {
        instance.post_stat_values_aggregate(ctx, req, resp)
    });
    builder.build()
}
