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

const METHOD_V2_LIST_MODEL_CONCEPTS: ::grpcio::Method<super::service::ListModelConceptsRequest, super::service::MultiConceptResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListModelConcepts",
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

const METHOD_V2_PATCH_ANNOTATIONS_SEARCHES: ::grpcio::Method<super::service::PatchAnnotationsSearchesRequest, super::service::MultiSearchResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PatchAnnotationsSearches",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_ANNOTATIONS_SEARCHES: ::grpcio::Method<super::service::PostAnnotationsSearchesRequest, super::service::MultiSearchResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostAnnotationsSearches",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_LIST_ANNOTATION_WORKERS: ::grpcio::Method<super::service::ListAnnotationWorkersRequest, super::service::MultiWorkerResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListAnnotationWorkers",
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

const METHOD_V2_GET_INPUT_VIDEO_MANIFEST: ::grpcio::Method<super::service::GetVideoManifestRequest, super::service::GetVideoManifestResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetInputVideoManifest",
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

const METHOD_V2_PATCH_INPUTS_SEARCHES: ::grpcio::Method<super::service::PatchInputsSearchesRequest, super::service::MultiSearchResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PatchInputsSearches",
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

const METHOD_V2_LIST_DATASETS: ::grpcio::Method<super::service::ListDatasetsRequest, super::service::MultiDatasetResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListDatasets",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_GET_DATASET: ::grpcio::Method<super::service::GetDatasetRequest, super::service::SingleDatasetResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetDataset",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_DATASETS: ::grpcio::Method<super::service::PostDatasetsRequest, super::service::MultiDatasetResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostDatasets",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_PATCH_DATASETS: ::grpcio::Method<super::service::PatchDatasetsRequest, super::service::MultiDatasetResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PatchDatasets",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_DELETE_DATASETS: ::grpcio::Method<super::service::DeleteDatasetsRequest, super::status::BaseResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/DeleteDatasets",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_LIST_DATASET_INPUTS: ::grpcio::Method<super::service::ListDatasetInputsRequest, super::service::MultiDatasetInputResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListDatasetInputs",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_GET_DATASET_INPUT: ::grpcio::Method<super::service::GetDatasetInputRequest, super::service::SingleDatasetInputResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetDatasetInput",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_DATASET_INPUTS: ::grpcio::Method<super::service::PostDatasetInputsRequest, super::service::MultiDatasetInputResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostDatasetInputs",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_DELETE_DATASET_INPUTS: ::grpcio::Method<super::service::DeleteDatasetInputsRequest, super::status::BaseResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/DeleteDatasetInputs",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_LIST_DATASET_VERSIONS: ::grpcio::Method<super::service::ListDatasetVersionsRequest, super::service::MultiDatasetVersionResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListDatasetVersions",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_GET_DATASET_VERSION: ::grpcio::Method<super::service::GetDatasetVersionRequest, super::service::SingleDatasetVersionResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetDatasetVersion",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_LIST_DATASET_VERSION_METRICS_GROUPS: ::grpcio::Method<super::service::ListDatasetVersionMetricsGroupsRequest, super::service::MultiDatasetVersionMetricsGroupResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListDatasetVersionMetricsGroups",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_DATASET_VERSIONS: ::grpcio::Method<super::service::PostDatasetVersionsRequest, super::service::MultiDatasetVersionResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostDatasetVersions",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_PATCH_DATASET_VERSIONS: ::grpcio::Method<super::service::PatchDatasetVersionsRequest, super::service::MultiDatasetVersionResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PatchDatasetVersions",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_DELETE_DATASET_VERSIONS: ::grpcio::Method<super::service::DeleteDatasetVersionsRequest, super::status::BaseResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/DeleteDatasetVersions",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_PUT_DATASET_VERSION_EXPORTS: ::grpcio::Method<super::service::PutDatasetVersionExportsRequest, super::service::MultiDatasetVersionExportResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PutDatasetVersionExports",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_GET_MODEL_TYPE: ::grpcio::Method<super::service::GetModelTypeRequest, super::service::SingleModelTypeResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetModelType",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_LIST_OPEN_SOURCE_LICENSES: ::grpcio::Method<super::service::ListOpenSourceLicensesRequest, super::service::ListOpenSourceLicensesResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListOpenSourceLicenses",
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

const METHOD_V2_GET_RESOURCE_COUNTS: ::grpcio::Method<super::service::GetResourceCountsRequest, super::service::GetResourceCountsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetResourceCounts",
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

const METHOD_V2_PATCH_MODEL_IDS: ::grpcio::Method<super::service::PatchModelIdsRequest, super::service::MultiModelResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PatchModelIds",
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

const METHOD_V2_PATCH_MODEL_CHECK_CONSENTS: ::grpcio::Method<super::service::PatchModelCheckConsentsRequest, super::service::MultiModelCheckConsentResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PatchModelCheckConsents",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_PATCH_MODEL_TOOLKITS: ::grpcio::Method<super::service::PatchModelToolkitsRequest, super::service::MultiModelToolkitResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PatchModelToolkits",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_PATCH_MODEL_USE_CASES: ::grpcio::Method<super::service::PatchModelUseCasesRequest, super::service::MultiModelUseCaseResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PatchModelUseCases",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_PATCH_MODEL_LANGUAGES: ::grpcio::Method<super::service::PatchModelLanguagesRequest, super::service::MultiModelLanguageResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PatchModelLanguages",
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

const METHOD_V2_POST_WORKFLOW_VERSIONS_UN_PUBLISH: ::grpcio::Method<super::service::PostWorkflowVersionsUnPublishRequest, super::status::BaseResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostWorkflowVersionsUnPublish",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_WORKFLOW_VERSIONS_PUBLISH: ::grpcio::Method<super::service::PostWorkflowVersionsPublishRequest, super::status::BaseResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostWorkflowVersionsPublish",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_MODEL_VERSIONS_PUBLISH: ::grpcio::Method<super::service::PostModelVersionsPublishRequest, super::status::BaseResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostModelVersionsPublish",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_MODEL_VERSIONS_UN_PUBLISH: ::grpcio::Method<super::service::PostModelVersionsUnPublishRequest, super::status::BaseResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostModelVersionsUnPublish",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_MODEL_VERSIONS: ::grpcio::Method<super::service::PostModelVersionsRequest, super::service::SingleModelResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostModelVersions",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_PATCH_MODEL_VERSIONS: ::grpcio::Method<super::service::PatchModelVersionsRequest, super::service::MultiModelVersionResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PatchModelVersions",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_DELETE_MODEL_VERSION: ::grpcio::Method<super::service::DeleteModelVersionRequest, super::status::BaseResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/DeleteModelVersion",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_MODEL_VERSIONS_UPLOAD: ::grpcio::Method<super::service::PostModelVersionsUploadRequest, super::service::PostModelVersionsUploadResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/clarifai.api.V2/PostModelVersionsUpload",
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

const METHOD_V2_POST_MODEL_VERSION_EVALUATIONS: ::grpcio::Method<super::service::PostModelVersionEvaluationsRequest, super::service::MultiEvalMetricsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostModelVersionEvaluations",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_LIST_MODEL_VERSION_EVALUATIONS: ::grpcio::Method<super::service::ListModelVersionEvaluationsRequest, super::service::MultiEvalMetricsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListModelVersionEvaluations",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_GET_MODEL_VERSION_EVALUATION: ::grpcio::Method<super::service::GetModelVersionEvaluationRequest, super::service::SingleEvalMetricsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetModelVersionEvaluation",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_EVALUATIONS: ::grpcio::Method<super::service::PostEvaluationsRequest, super::service::MultiEvalMetricsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostEvaluations",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_LIST_EVALUATIONS: ::grpcio::Method<super::service::ListEvaluationsRequest, super::service::MultiEvalMetricsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListEvaluations",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_GET_EVALUATION: ::grpcio::Method<super::service::GetEvaluationRequest, super::service::SingleEvalMetricsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetEvaluation",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_LIST_MODEL_REFERENCES: ::grpcio::Method<super::service::ListModelReferencesRequest, super::service::MultiModelReferenceResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListModelReferences",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_GET_MODEL_VERSION_INPUT_EXAMPLE: ::grpcio::Method<super::service::GetModelVersionInputExampleRequest, super::service::SingleModelVersionInputExampleResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetModelVersionInputExample",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_LIST_MODEL_VERSION_INPUT_EXAMPLES: ::grpcio::Method<super::service::ListModelVersionInputExamplesRequest, super::service::MultiModelVersionInputExampleResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListModelVersionInputExamples",
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

const METHOD_V2_PATCH_WORKFLOW_IDS: ::grpcio::Method<super::service::PatchWorkflowIdsRequest, super::service::MultiWorkflowResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PatchWorkflowIds",
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

const METHOD_V2_LIST_WORKFLOW_VERSIONS: ::grpcio::Method<super::service::ListWorkflowVersionsRequest, super::service::MultiWorkflowVersionResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListWorkflowVersions",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_GET_WORKFLOW_VERSION: ::grpcio::Method<super::service::GetWorkflowVersionRequest, super::service::SingleWorkflowVersionResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetWorkflowVersion",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_DELETE_WORKFLOW_VERSIONS: ::grpcio::Method<super::service::DeleteWorkflowVersionsRequest, super::status::BaseResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/DeleteWorkflowVersions",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_PATCH_WORKFLOW_VERSIONS: ::grpcio::Method<super::service::PatchWorkflowVersionsRequest, super::service::MultiWorkflowVersionResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PatchWorkflowVersions",
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

const METHOD_V2_MY_SCOPES_USER: ::grpcio::Method<super::service::MyScopesUserRequest, super::service::MultiScopeUserResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/MyScopesUser",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_MY_SCOPES_ROOT: ::grpcio::Method<super::service::MyScopesRootRequest, super::service::MultiScopeRootResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/MyScopesRoot",
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

const METHOD_V2_PATCH_APPS_IDS: ::grpcio::Method<super::service::PatchAppsIdsRequest, super::service::MultiAppResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PatchAppsIds",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_PATCH_APP: ::grpcio::Method<super::service::PatchAppRequest, super::service::SingleAppResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PatchApp",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_APPS_SEARCHES: ::grpcio::Method<super::service::PostAppsSearchesRequest, super::service::MultiAppResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostAppsSearches",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_GET_USER: ::grpcio::Method<super::service::GetUserRequest, super::service::SingleUserResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetUser",
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

const METHOD_V2_PATCH_SEARCHES: ::grpcio::Method<super::service::PatchSearchesRequest, super::service::MultiSearchResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PatchSearches",
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

const METHOD_V2_LIST_ANNOTATION_FILTERS: ::grpcio::Method<super::service::ListAnnotationFiltersRequest, super::service::MultiAnnotationFilterResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListAnnotationFilters",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_GET_ANNOTATION_FILTER: ::grpcio::Method<super::service::GetAnnotationFilterRequest, super::service::SingleAnnotationFilterResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetAnnotationFilter",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_ANNOTATION_FILTERS: ::grpcio::Method<super::service::PostAnnotationFiltersRequest, super::service::MultiAnnotationFilterResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostAnnotationFilters",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_PATCH_ANNOTATION_FILTERS: ::grpcio::Method<super::service::PatchAnnotationFiltersRequest, super::service::MultiAnnotationFilterResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PatchAnnotationFilters",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_DELETE_ANNOTATION_FILTERS: ::grpcio::Method<super::service::DeleteAnnotationFiltersRequest, super::status::BaseResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/DeleteAnnotationFilters",
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

const METHOD_V2_GET_RESOURCE_PRICE: ::grpcio::Method<super::service::GetResourcePriceRequest, super::service::GetResourcePriceResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetResourcePrice",
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

const METHOD_V2_POST_LABEL_ORDERS: ::grpcio::Method<super::service::PostLabelOrdersRequest, super::service::MultiLabelOrderResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostLabelOrders",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_GET_LABEL_ORDER: ::grpcio::Method<super::service::GetLabelOrderRequest, super::service::SingleLabelOrderResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetLabelOrder",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_LIST_LABEL_ORDERS: ::grpcio::Method<super::service::ListLabelOrdersRequest, super::service::MultiLabelOrderResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListLabelOrders",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_PATCH_LABEL_ORDERS: ::grpcio::Method<super::service::PatchLabelOrdersRequest, super::service::MultiLabelOrderResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PatchLabelOrders",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_DELETE_LABEL_ORDERS: ::grpcio::Method<super::service::DeleteLabelOrdersRequest, super::status::BaseResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/DeleteLabelOrders",
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

const METHOD_V2_POST_TRENDING_METRICS_VIEW: ::grpcio::Method<super::service::PostTrendingMetricsViewRequest, super::status::BaseResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostTrendingMetricsView",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_LIST_TRENDING_METRICS_VIEWS: ::grpcio::Method<super::service::ListTrendingMetricsViewsRequest, super::service::MultiTrendingMetricsViewResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListTrendingMetricsViews",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_GET_MODULE: ::grpcio::Method<super::service::GetModuleRequest, super::service::SingleModuleResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetModule",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_LIST_MODULES: ::grpcio::Method<super::service::ListModulesRequest, super::service::MultiModuleResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListModules",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_MODULES: ::grpcio::Method<super::service::PostModulesRequest, super::service::MultiModuleResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostModules",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_PATCH_MODULES: ::grpcio::Method<super::service::PatchModulesRequest, super::service::MultiModuleResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PatchModules",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_DELETE_MODULES: ::grpcio::Method<super::service::DeleteModulesRequest, super::status::BaseResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/DeleteModules",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_GET_MODULE_VERSION: ::grpcio::Method<super::service::GetModuleVersionRequest, super::service::SingleModuleVersionResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetModuleVersion",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_LIST_MODULE_VERSIONS: ::grpcio::Method<super::service::ListModuleVersionsRequest, super::service::MultiModuleVersionResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListModuleVersions",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_MODULE_VERSIONS: ::grpcio::Method<super::service::PostModuleVersionsRequest, super::service::MultiModuleVersionResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostModuleVersions",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_PATCH_MODULE_VERSIONS: ::grpcio::Method<super::service::PatchModuleVersionsRequest, super::service::MultiModuleVersionResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PatchModuleVersions",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_DELETE_MODULE_VERSIONS: ::grpcio::Method<super::service::DeleteModuleVersionsRequest, super::status::BaseResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/DeleteModuleVersions",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_GET_MODULE_VERSION_USAGE_COUNT: ::grpcio::Method<super::service::GetModuleVersionUsageCountRequest, super::service::SingleModuleVersionUsageCountResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetModuleVersionUsageCount",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_GET_INSTALLED_MODULE_VERSION: ::grpcio::Method<super::service::GetInstalledModuleVersionRequest, super::service::SingleInstalledModuleVersionResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetInstalledModuleVersion",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_LIST_INSTALLED_MODULE_VERSIONS: ::grpcio::Method<super::service::ListInstalledModuleVersionsRequest, super::service::MultiInstalledModuleVersionResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListInstalledModuleVersions",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_INSTALLED_MODULE_VERSIONS: ::grpcio::Method<super::service::PostInstalledModuleVersionsRequest, super::service::MultiInstalledModuleVersionResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostInstalledModuleVersions",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_DELETE_INSTALLED_MODULE_VERSIONS: ::grpcio::Method<super::service::DeleteInstalledModuleVersionsRequest, super::status::BaseResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/DeleteInstalledModuleVersions",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_INSTALLED_MODULE_VERSIONS_KEY: ::grpcio::Method<super::service::PostInstalledModuleVersionsKeyRequest, super::service::SingleKeyResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostInstalledModuleVersionsKey",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_BULK_OPERATIONS: ::grpcio::Method<super::service::PostBulkOperationsRequest, super::service::MultiBulkOperationsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostBulkOperations",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_LIST_BULK_OPERATIONS: ::grpcio::Method<super::service::ListBulkOperationsRequest, super::service::MultiBulkOperationsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListBulkOperations",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_GET_BULK_OPERATION: ::grpcio::Method<super::service::GetBulkOperationRequest, super::service::SingleBulkOperationsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetBulkOperation",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_CANCEL_BULK_OPERATIONS: ::grpcio::Method<super::service::CancelBulkOperationRequest, super::service::MultiBulkOperationsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/CancelBulkOperations",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_DELETE_BULK_OPERATIONS: ::grpcio::Method<super::service::DeleteBulkOperationRequest, super::status::BaseResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/DeleteBulkOperations",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_LIST_NEXT_TASK_ASSIGNMENTS: ::grpcio::Method<super::service::ListNextTaskAssignmentsRequest, super::service::MultiInputResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListNextTaskAssignments",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_PUT_TASK_ASSIGNMENTS: ::grpcio::Method<super::service::PutTaskAssignmentsRequest, super::status::BaseResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PutTaskAssignments",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_LIST_INPUTS_ADD_JOBS: ::grpcio::Method<super::service::ListInputsAddJobsRequest, super::service::MultiInputsAddJobResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListInputsAddJobs",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_GET_INPUTS_ADD_JOB: ::grpcio::Method<super::service::GetInputsAddJobRequest, super::service::SingleInputsAddJobResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetInputsAddJob",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_CANCEL_INPUTS_ADD_JOB: ::grpcio::Method<super::service::CancelInputsAddJobRequest, super::service::SingleInputsAddJobResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/CancelInputsAddJob",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_UPLOADS: ::grpcio::Method<super::service::PostUploadsRequest, super::service::MultiUploadResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostUploads",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_PUT_UPLOAD_CONTENT_PARTS: ::grpcio::Method<super::service::PutUploadContentPartsRequest, super::service::SingleUploadResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PutUploadContentParts",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_GET_UPLOAD: ::grpcio::Method<super::service::GetUploadRequest, super::service::SingleUploadResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetUpload",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_LIST_UPLOADS: ::grpcio::Method<super::service::ListUploadsRequest, super::service::MultiUploadResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListUploads",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_DELETE_UPLOADS: ::grpcio::Method<super::service::DeleteUploadsRequest, super::status::BaseResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/DeleteUploads",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_INPUTS_DATA_SOURCES: ::grpcio::Method<super::service::PostInputsDataSourcesRequest, super::service::MultiInputsAddJobResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostInputsDataSources",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_GET_INPUTS_EXTRACTION_JOB: ::grpcio::Method<super::service::GetInputsExtractionJobRequest, super::service::SingleInputsExtractionJobResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetInputsExtractionJob",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_LIST_INPUTS_EXTRACTION_JOBS: ::grpcio::Method<super::service::ListInputsExtractionJobsRequest, super::service::MultiInputsExtractionJobResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListInputsExtractionJobs",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_CANCEL_INPUTS_EXTRACTION_JOBS: ::grpcio::Method<super::service::CancelInputsExtractionJobsRequest, super::service::MultiInputsExtractionJobResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/CancelInputsExtractionJobs",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_INPUTS_UPLOADS: ::grpcio::Method<super::service::PostInputsUploadsRequest, super::service::MultiInputsAddJobResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostInputsUploads",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_GET_RUNNER: ::grpcio::Method<super::service::GetRunnerRequest, super::service::SingleRunnerResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/GetRunner",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_LIST_RUNNERS: ::grpcio::Method<super::service::ListRunnersRequest, super::service::MultiRunnerResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListRunners",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_RUNNERS: ::grpcio::Method<super::service::PostRunnersRequest, super::service::MultiRunnerResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostRunners",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_DELETE_RUNNERS: ::grpcio::Method<super::service::DeleteRunnersRequest, super::status::BaseResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/DeleteRunners",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_LIST_RUNNER_ITEMS: ::grpcio::Method<super::service::ListRunnerItemsRequest, super::service::MultiRunnerItemResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/ListRunnerItems",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_RUNNER_ITEM_OUTPUTS: ::grpcio::Method<super::service::PostRunnerItemOutputsRequest, super::service::MultiRunnerItemOutputResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostRunnerItemOutputs",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V2_POST_MODEL_VERSIONS_TRAINING_TIME_ESTIMATE: ::grpcio::Method<super::service::PostModelVersionsTrainingTimeEstimateRequest, super::service::MultiTrainingTimeEstimateResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/clarifai.api.V2/PostModelVersionsTrainingTimeEstimate",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct V2Client {
    pub client: ::grpcio::Client,
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

    pub fn list_model_concepts_opt(&self, req: &super::service::ListModelConceptsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiConceptResponse> {
        self.client.unary_call(&METHOD_V2_LIST_MODEL_CONCEPTS, req, opt)
    }

    pub fn list_model_concepts(&self, req: &super::service::ListModelConceptsRequest) -> ::grpcio::Result<super::service::MultiConceptResponse> {
        self.list_model_concepts_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_model_concepts_async_opt(&self, req: &super::service::ListModelConceptsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiConceptResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_MODEL_CONCEPTS, req, opt)
    }

    pub fn list_model_concepts_async(&self, req: &super::service::ListModelConceptsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiConceptResponse>> {
        self.list_model_concepts_async_opt(req, ::grpcio::CallOption::default())
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

    pub fn patch_annotations_searches_opt(&self, req: &super::service::PatchAnnotationsSearchesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiSearchResponse> {
        self.client.unary_call(&METHOD_V2_PATCH_ANNOTATIONS_SEARCHES, req, opt)
    }

    pub fn patch_annotations_searches(&self, req: &super::service::PatchAnnotationsSearchesRequest) -> ::grpcio::Result<super::service::MultiSearchResponse> {
        self.patch_annotations_searches_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_annotations_searches_async_opt(&self, req: &super::service::PatchAnnotationsSearchesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiSearchResponse>> {
        self.client.unary_call_async(&METHOD_V2_PATCH_ANNOTATIONS_SEARCHES, req, opt)
    }

    pub fn patch_annotations_searches_async(&self, req: &super::service::PatchAnnotationsSearchesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiSearchResponse>> {
        self.patch_annotations_searches_async_opt(req, ::grpcio::CallOption::default())
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

    pub fn list_annotation_workers_opt(&self, req: &super::service::ListAnnotationWorkersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiWorkerResponse> {
        self.client.unary_call(&METHOD_V2_LIST_ANNOTATION_WORKERS, req, opt)
    }

    pub fn list_annotation_workers(&self, req: &super::service::ListAnnotationWorkersRequest) -> ::grpcio::Result<super::service::MultiWorkerResponse> {
        self.list_annotation_workers_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_annotation_workers_async_opt(&self, req: &super::service::ListAnnotationWorkersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiWorkerResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_ANNOTATION_WORKERS, req, opt)
    }

    pub fn list_annotation_workers_async(&self, req: &super::service::ListAnnotationWorkersRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiWorkerResponse>> {
        self.list_annotation_workers_async_opt(req, ::grpcio::CallOption::default())
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

    pub fn get_input_video_manifest_opt(&self, req: &super::service::GetVideoManifestRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::GetVideoManifestResponse> {
        self.client.unary_call(&METHOD_V2_GET_INPUT_VIDEO_MANIFEST, req, opt)
    }

    pub fn get_input_video_manifest(&self, req: &super::service::GetVideoManifestRequest) -> ::grpcio::Result<super::service::GetVideoManifestResponse> {
        self.get_input_video_manifest_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_input_video_manifest_async_opt(&self, req: &super::service::GetVideoManifestRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::GetVideoManifestResponse>> {
        self.client.unary_call_async(&METHOD_V2_GET_INPUT_VIDEO_MANIFEST, req, opt)
    }

    pub fn get_input_video_manifest_async(&self, req: &super::service::GetVideoManifestRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::GetVideoManifestResponse>> {
        self.get_input_video_manifest_async_opt(req, ::grpcio::CallOption::default())
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

    pub fn patch_inputs_searches_opt(&self, req: &super::service::PatchInputsSearchesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiSearchResponse> {
        self.client.unary_call(&METHOD_V2_PATCH_INPUTS_SEARCHES, req, opt)
    }

    pub fn patch_inputs_searches(&self, req: &super::service::PatchInputsSearchesRequest) -> ::grpcio::Result<super::service::MultiSearchResponse> {
        self.patch_inputs_searches_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_inputs_searches_async_opt(&self, req: &super::service::PatchInputsSearchesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiSearchResponse>> {
        self.client.unary_call_async(&METHOD_V2_PATCH_INPUTS_SEARCHES, req, opt)
    }

    pub fn patch_inputs_searches_async(&self, req: &super::service::PatchInputsSearchesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiSearchResponse>> {
        self.patch_inputs_searches_async_opt(req, ::grpcio::CallOption::default())
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

    pub fn list_datasets_opt(&self, req: &super::service::ListDatasetsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiDatasetResponse> {
        self.client.unary_call(&METHOD_V2_LIST_DATASETS, req, opt)
    }

    pub fn list_datasets(&self, req: &super::service::ListDatasetsRequest) -> ::grpcio::Result<super::service::MultiDatasetResponse> {
        self.list_datasets_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_datasets_async_opt(&self, req: &super::service::ListDatasetsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiDatasetResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_DATASETS, req, opt)
    }

    pub fn list_datasets_async(&self, req: &super::service::ListDatasetsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiDatasetResponse>> {
        self.list_datasets_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_dataset_opt(&self, req: &super::service::GetDatasetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleDatasetResponse> {
        self.client.unary_call(&METHOD_V2_GET_DATASET, req, opt)
    }

    pub fn get_dataset(&self, req: &super::service::GetDatasetRequest) -> ::grpcio::Result<super::service::SingleDatasetResponse> {
        self.get_dataset_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_dataset_async_opt(&self, req: &super::service::GetDatasetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleDatasetResponse>> {
        self.client.unary_call_async(&METHOD_V2_GET_DATASET, req, opt)
    }

    pub fn get_dataset_async(&self, req: &super::service::GetDatasetRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleDatasetResponse>> {
        self.get_dataset_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_datasets_opt(&self, req: &super::service::PostDatasetsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiDatasetResponse> {
        self.client.unary_call(&METHOD_V2_POST_DATASETS, req, opt)
    }

    pub fn post_datasets(&self, req: &super::service::PostDatasetsRequest) -> ::grpcio::Result<super::service::MultiDatasetResponse> {
        self.post_datasets_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_datasets_async_opt(&self, req: &super::service::PostDatasetsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiDatasetResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_DATASETS, req, opt)
    }

    pub fn post_datasets_async(&self, req: &super::service::PostDatasetsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiDatasetResponse>> {
        self.post_datasets_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_datasets_opt(&self, req: &super::service::PatchDatasetsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiDatasetResponse> {
        self.client.unary_call(&METHOD_V2_PATCH_DATASETS, req, opt)
    }

    pub fn patch_datasets(&self, req: &super::service::PatchDatasetsRequest) -> ::grpcio::Result<super::service::MultiDatasetResponse> {
        self.patch_datasets_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_datasets_async_opt(&self, req: &super::service::PatchDatasetsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiDatasetResponse>> {
        self.client.unary_call_async(&METHOD_V2_PATCH_DATASETS, req, opt)
    }

    pub fn patch_datasets_async(&self, req: &super::service::PatchDatasetsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiDatasetResponse>> {
        self.patch_datasets_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_datasets_opt(&self, req: &super::service::DeleteDatasetsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::status::BaseResponse> {
        self.client.unary_call(&METHOD_V2_DELETE_DATASETS, req, opt)
    }

    pub fn delete_datasets(&self, req: &super::service::DeleteDatasetsRequest) -> ::grpcio::Result<super::status::BaseResponse> {
        self.delete_datasets_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_datasets_async_opt(&self, req: &super::service::DeleteDatasetsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.client.unary_call_async(&METHOD_V2_DELETE_DATASETS, req, opt)
    }

    pub fn delete_datasets_async(&self, req: &super::service::DeleteDatasetsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.delete_datasets_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_dataset_inputs_opt(&self, req: &super::service::ListDatasetInputsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiDatasetInputResponse> {
        self.client.unary_call(&METHOD_V2_LIST_DATASET_INPUTS, req, opt)
    }

    pub fn list_dataset_inputs(&self, req: &super::service::ListDatasetInputsRequest) -> ::grpcio::Result<super::service::MultiDatasetInputResponse> {
        self.list_dataset_inputs_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_dataset_inputs_async_opt(&self, req: &super::service::ListDatasetInputsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiDatasetInputResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_DATASET_INPUTS, req, opt)
    }

    pub fn list_dataset_inputs_async(&self, req: &super::service::ListDatasetInputsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiDatasetInputResponse>> {
        self.list_dataset_inputs_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_dataset_input_opt(&self, req: &super::service::GetDatasetInputRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleDatasetInputResponse> {
        self.client.unary_call(&METHOD_V2_GET_DATASET_INPUT, req, opt)
    }

    pub fn get_dataset_input(&self, req: &super::service::GetDatasetInputRequest) -> ::grpcio::Result<super::service::SingleDatasetInputResponse> {
        self.get_dataset_input_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_dataset_input_async_opt(&self, req: &super::service::GetDatasetInputRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleDatasetInputResponse>> {
        self.client.unary_call_async(&METHOD_V2_GET_DATASET_INPUT, req, opt)
    }

    pub fn get_dataset_input_async(&self, req: &super::service::GetDatasetInputRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleDatasetInputResponse>> {
        self.get_dataset_input_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_dataset_inputs_opt(&self, req: &super::service::PostDatasetInputsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiDatasetInputResponse> {
        self.client.unary_call(&METHOD_V2_POST_DATASET_INPUTS, req, opt)
    }

    pub fn post_dataset_inputs(&self, req: &super::service::PostDatasetInputsRequest) -> ::grpcio::Result<super::service::MultiDatasetInputResponse> {
        self.post_dataset_inputs_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_dataset_inputs_async_opt(&self, req: &super::service::PostDatasetInputsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiDatasetInputResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_DATASET_INPUTS, req, opt)
    }

    pub fn post_dataset_inputs_async(&self, req: &super::service::PostDatasetInputsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiDatasetInputResponse>> {
        self.post_dataset_inputs_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_dataset_inputs_opt(&self, req: &super::service::DeleteDatasetInputsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::status::BaseResponse> {
        self.client.unary_call(&METHOD_V2_DELETE_DATASET_INPUTS, req, opt)
    }

    pub fn delete_dataset_inputs(&self, req: &super::service::DeleteDatasetInputsRequest) -> ::grpcio::Result<super::status::BaseResponse> {
        self.delete_dataset_inputs_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_dataset_inputs_async_opt(&self, req: &super::service::DeleteDatasetInputsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.client.unary_call_async(&METHOD_V2_DELETE_DATASET_INPUTS, req, opt)
    }

    pub fn delete_dataset_inputs_async(&self, req: &super::service::DeleteDatasetInputsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.delete_dataset_inputs_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_dataset_versions_opt(&self, req: &super::service::ListDatasetVersionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiDatasetVersionResponse> {
        self.client.unary_call(&METHOD_V2_LIST_DATASET_VERSIONS, req, opt)
    }

    pub fn list_dataset_versions(&self, req: &super::service::ListDatasetVersionsRequest) -> ::grpcio::Result<super::service::MultiDatasetVersionResponse> {
        self.list_dataset_versions_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_dataset_versions_async_opt(&self, req: &super::service::ListDatasetVersionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiDatasetVersionResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_DATASET_VERSIONS, req, opt)
    }

    pub fn list_dataset_versions_async(&self, req: &super::service::ListDatasetVersionsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiDatasetVersionResponse>> {
        self.list_dataset_versions_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_dataset_version_opt(&self, req: &super::service::GetDatasetVersionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleDatasetVersionResponse> {
        self.client.unary_call(&METHOD_V2_GET_DATASET_VERSION, req, opt)
    }

    pub fn get_dataset_version(&self, req: &super::service::GetDatasetVersionRequest) -> ::grpcio::Result<super::service::SingleDatasetVersionResponse> {
        self.get_dataset_version_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_dataset_version_async_opt(&self, req: &super::service::GetDatasetVersionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleDatasetVersionResponse>> {
        self.client.unary_call_async(&METHOD_V2_GET_DATASET_VERSION, req, opt)
    }

    pub fn get_dataset_version_async(&self, req: &super::service::GetDatasetVersionRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleDatasetVersionResponse>> {
        self.get_dataset_version_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_dataset_version_metrics_groups_opt(&self, req: &super::service::ListDatasetVersionMetricsGroupsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiDatasetVersionMetricsGroupResponse> {
        self.client.unary_call(&METHOD_V2_LIST_DATASET_VERSION_METRICS_GROUPS, req, opt)
    }

    pub fn list_dataset_version_metrics_groups(&self, req: &super::service::ListDatasetVersionMetricsGroupsRequest) -> ::grpcio::Result<super::service::MultiDatasetVersionMetricsGroupResponse> {
        self.list_dataset_version_metrics_groups_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_dataset_version_metrics_groups_async_opt(&self, req: &super::service::ListDatasetVersionMetricsGroupsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiDatasetVersionMetricsGroupResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_DATASET_VERSION_METRICS_GROUPS, req, opt)
    }

    pub fn list_dataset_version_metrics_groups_async(&self, req: &super::service::ListDatasetVersionMetricsGroupsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiDatasetVersionMetricsGroupResponse>> {
        self.list_dataset_version_metrics_groups_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_dataset_versions_opt(&self, req: &super::service::PostDatasetVersionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiDatasetVersionResponse> {
        self.client.unary_call(&METHOD_V2_POST_DATASET_VERSIONS, req, opt)
    }

    pub fn post_dataset_versions(&self, req: &super::service::PostDatasetVersionsRequest) -> ::grpcio::Result<super::service::MultiDatasetVersionResponse> {
        self.post_dataset_versions_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_dataset_versions_async_opt(&self, req: &super::service::PostDatasetVersionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiDatasetVersionResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_DATASET_VERSIONS, req, opt)
    }

    pub fn post_dataset_versions_async(&self, req: &super::service::PostDatasetVersionsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiDatasetVersionResponse>> {
        self.post_dataset_versions_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_dataset_versions_opt(&self, req: &super::service::PatchDatasetVersionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiDatasetVersionResponse> {
        self.client.unary_call(&METHOD_V2_PATCH_DATASET_VERSIONS, req, opt)
    }

    pub fn patch_dataset_versions(&self, req: &super::service::PatchDatasetVersionsRequest) -> ::grpcio::Result<super::service::MultiDatasetVersionResponse> {
        self.patch_dataset_versions_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_dataset_versions_async_opt(&self, req: &super::service::PatchDatasetVersionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiDatasetVersionResponse>> {
        self.client.unary_call_async(&METHOD_V2_PATCH_DATASET_VERSIONS, req, opt)
    }

    pub fn patch_dataset_versions_async(&self, req: &super::service::PatchDatasetVersionsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiDatasetVersionResponse>> {
        self.patch_dataset_versions_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_dataset_versions_opt(&self, req: &super::service::DeleteDatasetVersionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::status::BaseResponse> {
        self.client.unary_call(&METHOD_V2_DELETE_DATASET_VERSIONS, req, opt)
    }

    pub fn delete_dataset_versions(&self, req: &super::service::DeleteDatasetVersionsRequest) -> ::grpcio::Result<super::status::BaseResponse> {
        self.delete_dataset_versions_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_dataset_versions_async_opt(&self, req: &super::service::DeleteDatasetVersionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.client.unary_call_async(&METHOD_V2_DELETE_DATASET_VERSIONS, req, opt)
    }

    pub fn delete_dataset_versions_async(&self, req: &super::service::DeleteDatasetVersionsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.delete_dataset_versions_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn put_dataset_version_exports_opt(&self, req: &super::service::PutDatasetVersionExportsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiDatasetVersionExportResponse> {
        self.client.unary_call(&METHOD_V2_PUT_DATASET_VERSION_EXPORTS, req, opt)
    }

    pub fn put_dataset_version_exports(&self, req: &super::service::PutDatasetVersionExportsRequest) -> ::grpcio::Result<super::service::MultiDatasetVersionExportResponse> {
        self.put_dataset_version_exports_opt(req, ::grpcio::CallOption::default())
    }

    pub fn put_dataset_version_exports_async_opt(&self, req: &super::service::PutDatasetVersionExportsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiDatasetVersionExportResponse>> {
        self.client.unary_call_async(&METHOD_V2_PUT_DATASET_VERSION_EXPORTS, req, opt)
    }

    pub fn put_dataset_version_exports_async(&self, req: &super::service::PutDatasetVersionExportsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiDatasetVersionExportResponse>> {
        self.put_dataset_version_exports_async_opt(req, ::grpcio::CallOption::default())
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

    pub fn list_open_source_licenses_opt(&self, req: &super::service::ListOpenSourceLicensesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::ListOpenSourceLicensesResponse> {
        self.client.unary_call(&METHOD_V2_LIST_OPEN_SOURCE_LICENSES, req, opt)
    }

    pub fn list_open_source_licenses(&self, req: &super::service::ListOpenSourceLicensesRequest) -> ::grpcio::Result<super::service::ListOpenSourceLicensesResponse> {
        self.list_open_source_licenses_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_open_source_licenses_async_opt(&self, req: &super::service::ListOpenSourceLicensesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::ListOpenSourceLicensesResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_OPEN_SOURCE_LICENSES, req, opt)
    }

    pub fn list_open_source_licenses_async(&self, req: &super::service::ListOpenSourceLicensesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::ListOpenSourceLicensesResponse>> {
        self.list_open_source_licenses_async_opt(req, ::grpcio::CallOption::default())
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

    pub fn get_resource_counts_opt(&self, req: &super::service::GetResourceCountsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::GetResourceCountsResponse> {
        self.client.unary_call(&METHOD_V2_GET_RESOURCE_COUNTS, req, opt)
    }

    pub fn get_resource_counts(&self, req: &super::service::GetResourceCountsRequest) -> ::grpcio::Result<super::service::GetResourceCountsResponse> {
        self.get_resource_counts_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_resource_counts_async_opt(&self, req: &super::service::GetResourceCountsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::GetResourceCountsResponse>> {
        self.client.unary_call_async(&METHOD_V2_GET_RESOURCE_COUNTS, req, opt)
    }

    pub fn get_resource_counts_async(&self, req: &super::service::GetResourceCountsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::GetResourceCountsResponse>> {
        self.get_resource_counts_async_opt(req, ::grpcio::CallOption::default())
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

    pub fn patch_model_ids_opt(&self, req: &super::service::PatchModelIdsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiModelResponse> {
        self.client.unary_call(&METHOD_V2_PATCH_MODEL_IDS, req, opt)
    }

    pub fn patch_model_ids(&self, req: &super::service::PatchModelIdsRequest) -> ::grpcio::Result<super::service::MultiModelResponse> {
        self.patch_model_ids_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_model_ids_async_opt(&self, req: &super::service::PatchModelIdsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiModelResponse>> {
        self.client.unary_call_async(&METHOD_V2_PATCH_MODEL_IDS, req, opt)
    }

    pub fn patch_model_ids_async(&self, req: &super::service::PatchModelIdsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiModelResponse>> {
        self.patch_model_ids_async_opt(req, ::grpcio::CallOption::default())
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

    pub fn patch_model_check_consents_opt(&self, req: &super::service::PatchModelCheckConsentsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiModelCheckConsentResponse> {
        self.client.unary_call(&METHOD_V2_PATCH_MODEL_CHECK_CONSENTS, req, opt)
    }

    pub fn patch_model_check_consents(&self, req: &super::service::PatchModelCheckConsentsRequest) -> ::grpcio::Result<super::service::MultiModelCheckConsentResponse> {
        self.patch_model_check_consents_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_model_check_consents_async_opt(&self, req: &super::service::PatchModelCheckConsentsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiModelCheckConsentResponse>> {
        self.client.unary_call_async(&METHOD_V2_PATCH_MODEL_CHECK_CONSENTS, req, opt)
    }

    pub fn patch_model_check_consents_async(&self, req: &super::service::PatchModelCheckConsentsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiModelCheckConsentResponse>> {
        self.patch_model_check_consents_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_model_toolkits_opt(&self, req: &super::service::PatchModelToolkitsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiModelToolkitResponse> {
        self.client.unary_call(&METHOD_V2_PATCH_MODEL_TOOLKITS, req, opt)
    }

    pub fn patch_model_toolkits(&self, req: &super::service::PatchModelToolkitsRequest) -> ::grpcio::Result<super::service::MultiModelToolkitResponse> {
        self.patch_model_toolkits_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_model_toolkits_async_opt(&self, req: &super::service::PatchModelToolkitsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiModelToolkitResponse>> {
        self.client.unary_call_async(&METHOD_V2_PATCH_MODEL_TOOLKITS, req, opt)
    }

    pub fn patch_model_toolkits_async(&self, req: &super::service::PatchModelToolkitsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiModelToolkitResponse>> {
        self.patch_model_toolkits_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_model_use_cases_opt(&self, req: &super::service::PatchModelUseCasesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiModelUseCaseResponse> {
        self.client.unary_call(&METHOD_V2_PATCH_MODEL_USE_CASES, req, opt)
    }

    pub fn patch_model_use_cases(&self, req: &super::service::PatchModelUseCasesRequest) -> ::grpcio::Result<super::service::MultiModelUseCaseResponse> {
        self.patch_model_use_cases_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_model_use_cases_async_opt(&self, req: &super::service::PatchModelUseCasesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiModelUseCaseResponse>> {
        self.client.unary_call_async(&METHOD_V2_PATCH_MODEL_USE_CASES, req, opt)
    }

    pub fn patch_model_use_cases_async(&self, req: &super::service::PatchModelUseCasesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiModelUseCaseResponse>> {
        self.patch_model_use_cases_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_model_languages_opt(&self, req: &super::service::PatchModelLanguagesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiModelLanguageResponse> {
        self.client.unary_call(&METHOD_V2_PATCH_MODEL_LANGUAGES, req, opt)
    }

    pub fn patch_model_languages(&self, req: &super::service::PatchModelLanguagesRequest) -> ::grpcio::Result<super::service::MultiModelLanguageResponse> {
        self.patch_model_languages_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_model_languages_async_opt(&self, req: &super::service::PatchModelLanguagesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiModelLanguageResponse>> {
        self.client.unary_call_async(&METHOD_V2_PATCH_MODEL_LANGUAGES, req, opt)
    }

    pub fn patch_model_languages_async(&self, req: &super::service::PatchModelLanguagesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiModelLanguageResponse>> {
        self.patch_model_languages_async_opt(req, ::grpcio::CallOption::default())
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

    pub fn post_workflow_versions_un_publish_opt(&self, req: &super::service::PostWorkflowVersionsUnPublishRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::status::BaseResponse> {
        self.client.unary_call(&METHOD_V2_POST_WORKFLOW_VERSIONS_UN_PUBLISH, req, opt)
    }

    pub fn post_workflow_versions_un_publish(&self, req: &super::service::PostWorkflowVersionsUnPublishRequest) -> ::grpcio::Result<super::status::BaseResponse> {
        self.post_workflow_versions_un_publish_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_workflow_versions_un_publish_async_opt(&self, req: &super::service::PostWorkflowVersionsUnPublishRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_WORKFLOW_VERSIONS_UN_PUBLISH, req, opt)
    }

    pub fn post_workflow_versions_un_publish_async(&self, req: &super::service::PostWorkflowVersionsUnPublishRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.post_workflow_versions_un_publish_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_workflow_versions_publish_opt(&self, req: &super::service::PostWorkflowVersionsPublishRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::status::BaseResponse> {
        self.client.unary_call(&METHOD_V2_POST_WORKFLOW_VERSIONS_PUBLISH, req, opt)
    }

    pub fn post_workflow_versions_publish(&self, req: &super::service::PostWorkflowVersionsPublishRequest) -> ::grpcio::Result<super::status::BaseResponse> {
        self.post_workflow_versions_publish_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_workflow_versions_publish_async_opt(&self, req: &super::service::PostWorkflowVersionsPublishRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_WORKFLOW_VERSIONS_PUBLISH, req, opt)
    }

    pub fn post_workflow_versions_publish_async(&self, req: &super::service::PostWorkflowVersionsPublishRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.post_workflow_versions_publish_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_model_versions_publish_opt(&self, req: &super::service::PostModelVersionsPublishRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::status::BaseResponse> {
        self.client.unary_call(&METHOD_V2_POST_MODEL_VERSIONS_PUBLISH, req, opt)
    }

    pub fn post_model_versions_publish(&self, req: &super::service::PostModelVersionsPublishRequest) -> ::grpcio::Result<super::status::BaseResponse> {
        self.post_model_versions_publish_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_model_versions_publish_async_opt(&self, req: &super::service::PostModelVersionsPublishRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_MODEL_VERSIONS_PUBLISH, req, opt)
    }

    pub fn post_model_versions_publish_async(&self, req: &super::service::PostModelVersionsPublishRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.post_model_versions_publish_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_model_versions_un_publish_opt(&self, req: &super::service::PostModelVersionsUnPublishRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::status::BaseResponse> {
        self.client.unary_call(&METHOD_V2_POST_MODEL_VERSIONS_UN_PUBLISH, req, opt)
    }

    pub fn post_model_versions_un_publish(&self, req: &super::service::PostModelVersionsUnPublishRequest) -> ::grpcio::Result<super::status::BaseResponse> {
        self.post_model_versions_un_publish_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_model_versions_un_publish_async_opt(&self, req: &super::service::PostModelVersionsUnPublishRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_MODEL_VERSIONS_UN_PUBLISH, req, opt)
    }

    pub fn post_model_versions_un_publish_async(&self, req: &super::service::PostModelVersionsUnPublishRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.post_model_versions_un_publish_async_opt(req, ::grpcio::CallOption::default())
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

    pub fn patch_model_versions_opt(&self, req: &super::service::PatchModelVersionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiModelVersionResponse> {
        self.client.unary_call(&METHOD_V2_PATCH_MODEL_VERSIONS, req, opt)
    }

    pub fn patch_model_versions(&self, req: &super::service::PatchModelVersionsRequest) -> ::grpcio::Result<super::service::MultiModelVersionResponse> {
        self.patch_model_versions_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_model_versions_async_opt(&self, req: &super::service::PatchModelVersionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiModelVersionResponse>> {
        self.client.unary_call_async(&METHOD_V2_PATCH_MODEL_VERSIONS, req, opt)
    }

    pub fn patch_model_versions_async(&self, req: &super::service::PatchModelVersionsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiModelVersionResponse>> {
        self.patch_model_versions_async_opt(req, ::grpcio::CallOption::default())
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

    pub fn post_model_versions_upload_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::service::PostModelVersionsUploadRequest>, ::grpcio::ClientDuplexReceiver<super::service::PostModelVersionsUploadResponse>)> {
        self.client.duplex_streaming(&METHOD_V2_POST_MODEL_VERSIONS_UPLOAD, opt)
    }

    pub fn post_model_versions_upload(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::service::PostModelVersionsUploadRequest>, ::grpcio::ClientDuplexReceiver<super::service::PostModelVersionsUploadResponse>)> {
        self.post_model_versions_upload_opt(::grpcio::CallOption::default())
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

    pub fn post_model_version_evaluations_opt(&self, req: &super::service::PostModelVersionEvaluationsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiEvalMetricsResponse> {
        self.client.unary_call(&METHOD_V2_POST_MODEL_VERSION_EVALUATIONS, req, opt)
    }

    pub fn post_model_version_evaluations(&self, req: &super::service::PostModelVersionEvaluationsRequest) -> ::grpcio::Result<super::service::MultiEvalMetricsResponse> {
        self.post_model_version_evaluations_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_model_version_evaluations_async_opt(&self, req: &super::service::PostModelVersionEvaluationsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiEvalMetricsResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_MODEL_VERSION_EVALUATIONS, req, opt)
    }

    pub fn post_model_version_evaluations_async(&self, req: &super::service::PostModelVersionEvaluationsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiEvalMetricsResponse>> {
        self.post_model_version_evaluations_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_model_version_evaluations_opt(&self, req: &super::service::ListModelVersionEvaluationsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiEvalMetricsResponse> {
        self.client.unary_call(&METHOD_V2_LIST_MODEL_VERSION_EVALUATIONS, req, opt)
    }

    pub fn list_model_version_evaluations(&self, req: &super::service::ListModelVersionEvaluationsRequest) -> ::grpcio::Result<super::service::MultiEvalMetricsResponse> {
        self.list_model_version_evaluations_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_model_version_evaluations_async_opt(&self, req: &super::service::ListModelVersionEvaluationsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiEvalMetricsResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_MODEL_VERSION_EVALUATIONS, req, opt)
    }

    pub fn list_model_version_evaluations_async(&self, req: &super::service::ListModelVersionEvaluationsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiEvalMetricsResponse>> {
        self.list_model_version_evaluations_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_model_version_evaluation_opt(&self, req: &super::service::GetModelVersionEvaluationRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleEvalMetricsResponse> {
        self.client.unary_call(&METHOD_V2_GET_MODEL_VERSION_EVALUATION, req, opt)
    }

    pub fn get_model_version_evaluation(&self, req: &super::service::GetModelVersionEvaluationRequest) -> ::grpcio::Result<super::service::SingleEvalMetricsResponse> {
        self.get_model_version_evaluation_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_model_version_evaluation_async_opt(&self, req: &super::service::GetModelVersionEvaluationRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleEvalMetricsResponse>> {
        self.client.unary_call_async(&METHOD_V2_GET_MODEL_VERSION_EVALUATION, req, opt)
    }

    pub fn get_model_version_evaluation_async(&self, req: &super::service::GetModelVersionEvaluationRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleEvalMetricsResponse>> {
        self.get_model_version_evaluation_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_evaluations_opt(&self, req: &super::service::PostEvaluationsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiEvalMetricsResponse> {
        self.client.unary_call(&METHOD_V2_POST_EVALUATIONS, req, opt)
    }

    pub fn post_evaluations(&self, req: &super::service::PostEvaluationsRequest) -> ::grpcio::Result<super::service::MultiEvalMetricsResponse> {
        self.post_evaluations_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_evaluations_async_opt(&self, req: &super::service::PostEvaluationsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiEvalMetricsResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_EVALUATIONS, req, opt)
    }

    pub fn post_evaluations_async(&self, req: &super::service::PostEvaluationsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiEvalMetricsResponse>> {
        self.post_evaluations_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_evaluations_opt(&self, req: &super::service::ListEvaluationsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiEvalMetricsResponse> {
        self.client.unary_call(&METHOD_V2_LIST_EVALUATIONS, req, opt)
    }

    pub fn list_evaluations(&self, req: &super::service::ListEvaluationsRequest) -> ::grpcio::Result<super::service::MultiEvalMetricsResponse> {
        self.list_evaluations_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_evaluations_async_opt(&self, req: &super::service::ListEvaluationsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiEvalMetricsResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_EVALUATIONS, req, opt)
    }

    pub fn list_evaluations_async(&self, req: &super::service::ListEvaluationsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiEvalMetricsResponse>> {
        self.list_evaluations_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_evaluation_opt(&self, req: &super::service::GetEvaluationRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleEvalMetricsResponse> {
        self.client.unary_call(&METHOD_V2_GET_EVALUATION, req, opt)
    }

    pub fn get_evaluation(&self, req: &super::service::GetEvaluationRequest) -> ::grpcio::Result<super::service::SingleEvalMetricsResponse> {
        self.get_evaluation_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_evaluation_async_opt(&self, req: &super::service::GetEvaluationRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleEvalMetricsResponse>> {
        self.client.unary_call_async(&METHOD_V2_GET_EVALUATION, req, opt)
    }

    pub fn get_evaluation_async(&self, req: &super::service::GetEvaluationRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleEvalMetricsResponse>> {
        self.get_evaluation_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_model_references_opt(&self, req: &super::service::ListModelReferencesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiModelReferenceResponse> {
        self.client.unary_call(&METHOD_V2_LIST_MODEL_REFERENCES, req, opt)
    }

    pub fn list_model_references(&self, req: &super::service::ListModelReferencesRequest) -> ::grpcio::Result<super::service::MultiModelReferenceResponse> {
        self.list_model_references_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_model_references_async_opt(&self, req: &super::service::ListModelReferencesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiModelReferenceResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_MODEL_REFERENCES, req, opt)
    }

    pub fn list_model_references_async(&self, req: &super::service::ListModelReferencesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiModelReferenceResponse>> {
        self.list_model_references_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_model_version_input_example_opt(&self, req: &super::service::GetModelVersionInputExampleRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleModelVersionInputExampleResponse> {
        self.client.unary_call(&METHOD_V2_GET_MODEL_VERSION_INPUT_EXAMPLE, req, opt)
    }

    pub fn get_model_version_input_example(&self, req: &super::service::GetModelVersionInputExampleRequest) -> ::grpcio::Result<super::service::SingleModelVersionInputExampleResponse> {
        self.get_model_version_input_example_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_model_version_input_example_async_opt(&self, req: &super::service::GetModelVersionInputExampleRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleModelVersionInputExampleResponse>> {
        self.client.unary_call_async(&METHOD_V2_GET_MODEL_VERSION_INPUT_EXAMPLE, req, opt)
    }

    pub fn get_model_version_input_example_async(&self, req: &super::service::GetModelVersionInputExampleRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleModelVersionInputExampleResponse>> {
        self.get_model_version_input_example_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_model_version_input_examples_opt(&self, req: &super::service::ListModelVersionInputExamplesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiModelVersionInputExampleResponse> {
        self.client.unary_call(&METHOD_V2_LIST_MODEL_VERSION_INPUT_EXAMPLES, req, opt)
    }

    pub fn list_model_version_input_examples(&self, req: &super::service::ListModelVersionInputExamplesRequest) -> ::grpcio::Result<super::service::MultiModelVersionInputExampleResponse> {
        self.list_model_version_input_examples_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_model_version_input_examples_async_opt(&self, req: &super::service::ListModelVersionInputExamplesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiModelVersionInputExampleResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_MODEL_VERSION_INPUT_EXAMPLES, req, opt)
    }

    pub fn list_model_version_input_examples_async(&self, req: &super::service::ListModelVersionInputExamplesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiModelVersionInputExampleResponse>> {
        self.list_model_version_input_examples_async_opt(req, ::grpcio::CallOption::default())
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

    pub fn patch_workflow_ids_opt(&self, req: &super::service::PatchWorkflowIdsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiWorkflowResponse> {
        self.client.unary_call(&METHOD_V2_PATCH_WORKFLOW_IDS, req, opt)
    }

    pub fn patch_workflow_ids(&self, req: &super::service::PatchWorkflowIdsRequest) -> ::grpcio::Result<super::service::MultiWorkflowResponse> {
        self.patch_workflow_ids_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_workflow_ids_async_opt(&self, req: &super::service::PatchWorkflowIdsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiWorkflowResponse>> {
        self.client.unary_call_async(&METHOD_V2_PATCH_WORKFLOW_IDS, req, opt)
    }

    pub fn patch_workflow_ids_async(&self, req: &super::service::PatchWorkflowIdsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiWorkflowResponse>> {
        self.patch_workflow_ids_async_opt(req, ::grpcio::CallOption::default())
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

    pub fn list_workflow_versions_opt(&self, req: &super::service::ListWorkflowVersionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiWorkflowVersionResponse> {
        self.client.unary_call(&METHOD_V2_LIST_WORKFLOW_VERSIONS, req, opt)
    }

    pub fn list_workflow_versions(&self, req: &super::service::ListWorkflowVersionsRequest) -> ::grpcio::Result<super::service::MultiWorkflowVersionResponse> {
        self.list_workflow_versions_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_workflow_versions_async_opt(&self, req: &super::service::ListWorkflowVersionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiWorkflowVersionResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_WORKFLOW_VERSIONS, req, opt)
    }

    pub fn list_workflow_versions_async(&self, req: &super::service::ListWorkflowVersionsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiWorkflowVersionResponse>> {
        self.list_workflow_versions_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_workflow_version_opt(&self, req: &super::service::GetWorkflowVersionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleWorkflowVersionResponse> {
        self.client.unary_call(&METHOD_V2_GET_WORKFLOW_VERSION, req, opt)
    }

    pub fn get_workflow_version(&self, req: &super::service::GetWorkflowVersionRequest) -> ::grpcio::Result<super::service::SingleWorkflowVersionResponse> {
        self.get_workflow_version_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_workflow_version_async_opt(&self, req: &super::service::GetWorkflowVersionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleWorkflowVersionResponse>> {
        self.client.unary_call_async(&METHOD_V2_GET_WORKFLOW_VERSION, req, opt)
    }

    pub fn get_workflow_version_async(&self, req: &super::service::GetWorkflowVersionRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleWorkflowVersionResponse>> {
        self.get_workflow_version_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_workflow_versions_opt(&self, req: &super::service::DeleteWorkflowVersionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::status::BaseResponse> {
        self.client.unary_call(&METHOD_V2_DELETE_WORKFLOW_VERSIONS, req, opt)
    }

    pub fn delete_workflow_versions(&self, req: &super::service::DeleteWorkflowVersionsRequest) -> ::grpcio::Result<super::status::BaseResponse> {
        self.delete_workflow_versions_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_workflow_versions_async_opt(&self, req: &super::service::DeleteWorkflowVersionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.client.unary_call_async(&METHOD_V2_DELETE_WORKFLOW_VERSIONS, req, opt)
    }

    pub fn delete_workflow_versions_async(&self, req: &super::service::DeleteWorkflowVersionsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.delete_workflow_versions_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_workflow_versions_opt(&self, req: &super::service::PatchWorkflowVersionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiWorkflowVersionResponse> {
        self.client.unary_call(&METHOD_V2_PATCH_WORKFLOW_VERSIONS, req, opt)
    }

    pub fn patch_workflow_versions(&self, req: &super::service::PatchWorkflowVersionsRequest) -> ::grpcio::Result<super::service::MultiWorkflowVersionResponse> {
        self.patch_workflow_versions_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_workflow_versions_async_opt(&self, req: &super::service::PatchWorkflowVersionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiWorkflowVersionResponse>> {
        self.client.unary_call_async(&METHOD_V2_PATCH_WORKFLOW_VERSIONS, req, opt)
    }

    pub fn patch_workflow_versions_async(&self, req: &super::service::PatchWorkflowVersionsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiWorkflowVersionResponse>> {
        self.patch_workflow_versions_async_opt(req, ::grpcio::CallOption::default())
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

    pub fn my_scopes_user_opt(&self, req: &super::service::MyScopesUserRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiScopeUserResponse> {
        self.client.unary_call(&METHOD_V2_MY_SCOPES_USER, req, opt)
    }

    pub fn my_scopes_user(&self, req: &super::service::MyScopesUserRequest) -> ::grpcio::Result<super::service::MultiScopeUserResponse> {
        self.my_scopes_user_opt(req, ::grpcio::CallOption::default())
    }

    pub fn my_scopes_user_async_opt(&self, req: &super::service::MyScopesUserRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiScopeUserResponse>> {
        self.client.unary_call_async(&METHOD_V2_MY_SCOPES_USER, req, opt)
    }

    pub fn my_scopes_user_async(&self, req: &super::service::MyScopesUserRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiScopeUserResponse>> {
        self.my_scopes_user_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn my_scopes_root_opt(&self, req: &super::service::MyScopesRootRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiScopeRootResponse> {
        self.client.unary_call(&METHOD_V2_MY_SCOPES_ROOT, req, opt)
    }

    pub fn my_scopes_root(&self, req: &super::service::MyScopesRootRequest) -> ::grpcio::Result<super::service::MultiScopeRootResponse> {
        self.my_scopes_root_opt(req, ::grpcio::CallOption::default())
    }

    pub fn my_scopes_root_async_opt(&self, req: &super::service::MyScopesRootRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiScopeRootResponse>> {
        self.client.unary_call_async(&METHOD_V2_MY_SCOPES_ROOT, req, opt)
    }

    pub fn my_scopes_root_async(&self, req: &super::service::MyScopesRootRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiScopeRootResponse>> {
        self.my_scopes_root_async_opt(req, ::grpcio::CallOption::default())
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

    pub fn patch_apps_ids_opt(&self, req: &super::service::PatchAppsIdsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiAppResponse> {
        self.client.unary_call(&METHOD_V2_PATCH_APPS_IDS, req, opt)
    }

    pub fn patch_apps_ids(&self, req: &super::service::PatchAppsIdsRequest) -> ::grpcio::Result<super::service::MultiAppResponse> {
        self.patch_apps_ids_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_apps_ids_async_opt(&self, req: &super::service::PatchAppsIdsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiAppResponse>> {
        self.client.unary_call_async(&METHOD_V2_PATCH_APPS_IDS, req, opt)
    }

    pub fn patch_apps_ids_async(&self, req: &super::service::PatchAppsIdsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiAppResponse>> {
        self.patch_apps_ids_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_app_opt(&self, req: &super::service::PatchAppRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleAppResponse> {
        self.client.unary_call(&METHOD_V2_PATCH_APP, req, opt)
    }

    pub fn patch_app(&self, req: &super::service::PatchAppRequest) -> ::grpcio::Result<super::service::SingleAppResponse> {
        self.patch_app_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_app_async_opt(&self, req: &super::service::PatchAppRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleAppResponse>> {
        self.client.unary_call_async(&METHOD_V2_PATCH_APP, req, opt)
    }

    pub fn patch_app_async(&self, req: &super::service::PatchAppRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleAppResponse>> {
        self.patch_app_async_opt(req, ::grpcio::CallOption::default())
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

    pub fn get_user_opt(&self, req: &super::service::GetUserRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleUserResponse> {
        self.client.unary_call(&METHOD_V2_GET_USER, req, opt)
    }

    pub fn get_user(&self, req: &super::service::GetUserRequest) -> ::grpcio::Result<super::service::SingleUserResponse> {
        self.get_user_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_user_async_opt(&self, req: &super::service::GetUserRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleUserResponse>> {
        self.client.unary_call_async(&METHOD_V2_GET_USER, req, opt)
    }

    pub fn get_user_async(&self, req: &super::service::GetUserRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleUserResponse>> {
        self.get_user_async_opt(req, ::grpcio::CallOption::default())
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

    pub fn patch_searches_opt(&self, req: &super::service::PatchSearchesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiSearchResponse> {
        self.client.unary_call(&METHOD_V2_PATCH_SEARCHES, req, opt)
    }

    pub fn patch_searches(&self, req: &super::service::PatchSearchesRequest) -> ::grpcio::Result<super::service::MultiSearchResponse> {
        self.patch_searches_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_searches_async_opt(&self, req: &super::service::PatchSearchesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiSearchResponse>> {
        self.client.unary_call_async(&METHOD_V2_PATCH_SEARCHES, req, opt)
    }

    pub fn patch_searches_async(&self, req: &super::service::PatchSearchesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiSearchResponse>> {
        self.patch_searches_async_opt(req, ::grpcio::CallOption::default())
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

    pub fn list_annotation_filters_opt(&self, req: &super::service::ListAnnotationFiltersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiAnnotationFilterResponse> {
        self.client.unary_call(&METHOD_V2_LIST_ANNOTATION_FILTERS, req, opt)
    }

    pub fn list_annotation_filters(&self, req: &super::service::ListAnnotationFiltersRequest) -> ::grpcio::Result<super::service::MultiAnnotationFilterResponse> {
        self.list_annotation_filters_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_annotation_filters_async_opt(&self, req: &super::service::ListAnnotationFiltersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiAnnotationFilterResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_ANNOTATION_FILTERS, req, opt)
    }

    pub fn list_annotation_filters_async(&self, req: &super::service::ListAnnotationFiltersRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiAnnotationFilterResponse>> {
        self.list_annotation_filters_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_annotation_filter_opt(&self, req: &super::service::GetAnnotationFilterRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleAnnotationFilterResponse> {
        self.client.unary_call(&METHOD_V2_GET_ANNOTATION_FILTER, req, opt)
    }

    pub fn get_annotation_filter(&self, req: &super::service::GetAnnotationFilterRequest) -> ::grpcio::Result<super::service::SingleAnnotationFilterResponse> {
        self.get_annotation_filter_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_annotation_filter_async_opt(&self, req: &super::service::GetAnnotationFilterRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleAnnotationFilterResponse>> {
        self.client.unary_call_async(&METHOD_V2_GET_ANNOTATION_FILTER, req, opt)
    }

    pub fn get_annotation_filter_async(&self, req: &super::service::GetAnnotationFilterRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleAnnotationFilterResponse>> {
        self.get_annotation_filter_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_annotation_filters_opt(&self, req: &super::service::PostAnnotationFiltersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiAnnotationFilterResponse> {
        self.client.unary_call(&METHOD_V2_POST_ANNOTATION_FILTERS, req, opt)
    }

    pub fn post_annotation_filters(&self, req: &super::service::PostAnnotationFiltersRequest) -> ::grpcio::Result<super::service::MultiAnnotationFilterResponse> {
        self.post_annotation_filters_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_annotation_filters_async_opt(&self, req: &super::service::PostAnnotationFiltersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiAnnotationFilterResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_ANNOTATION_FILTERS, req, opt)
    }

    pub fn post_annotation_filters_async(&self, req: &super::service::PostAnnotationFiltersRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiAnnotationFilterResponse>> {
        self.post_annotation_filters_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_annotation_filters_opt(&self, req: &super::service::PatchAnnotationFiltersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiAnnotationFilterResponse> {
        self.client.unary_call(&METHOD_V2_PATCH_ANNOTATION_FILTERS, req, opt)
    }

    pub fn patch_annotation_filters(&self, req: &super::service::PatchAnnotationFiltersRequest) -> ::grpcio::Result<super::service::MultiAnnotationFilterResponse> {
        self.patch_annotation_filters_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_annotation_filters_async_opt(&self, req: &super::service::PatchAnnotationFiltersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiAnnotationFilterResponse>> {
        self.client.unary_call_async(&METHOD_V2_PATCH_ANNOTATION_FILTERS, req, opt)
    }

    pub fn patch_annotation_filters_async(&self, req: &super::service::PatchAnnotationFiltersRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiAnnotationFilterResponse>> {
        self.patch_annotation_filters_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_annotation_filters_opt(&self, req: &super::service::DeleteAnnotationFiltersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::status::BaseResponse> {
        self.client.unary_call(&METHOD_V2_DELETE_ANNOTATION_FILTERS, req, opt)
    }

    pub fn delete_annotation_filters(&self, req: &super::service::DeleteAnnotationFiltersRequest) -> ::grpcio::Result<super::status::BaseResponse> {
        self.delete_annotation_filters_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_annotation_filters_async_opt(&self, req: &super::service::DeleteAnnotationFiltersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.client.unary_call_async(&METHOD_V2_DELETE_ANNOTATION_FILTERS, req, opt)
    }

    pub fn delete_annotation_filters_async(&self, req: &super::service::DeleteAnnotationFiltersRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.delete_annotation_filters_async_opt(req, ::grpcio::CallOption::default())
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

    pub fn get_resource_price_opt(&self, req: &super::service::GetResourcePriceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::GetResourcePriceResponse> {
        self.client.unary_call(&METHOD_V2_GET_RESOURCE_PRICE, req, opt)
    }

    pub fn get_resource_price(&self, req: &super::service::GetResourcePriceRequest) -> ::grpcio::Result<super::service::GetResourcePriceResponse> {
        self.get_resource_price_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_resource_price_async_opt(&self, req: &super::service::GetResourcePriceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::GetResourcePriceResponse>> {
        self.client.unary_call_async(&METHOD_V2_GET_RESOURCE_PRICE, req, opt)
    }

    pub fn get_resource_price_async(&self, req: &super::service::GetResourcePriceRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::GetResourcePriceResponse>> {
        self.get_resource_price_async_opt(req, ::grpcio::CallOption::default())
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

    pub fn post_label_orders_opt(&self, req: &super::service::PostLabelOrdersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiLabelOrderResponse> {
        self.client.unary_call(&METHOD_V2_POST_LABEL_ORDERS, req, opt)
    }

    pub fn post_label_orders(&self, req: &super::service::PostLabelOrdersRequest) -> ::grpcio::Result<super::service::MultiLabelOrderResponse> {
        self.post_label_orders_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_label_orders_async_opt(&self, req: &super::service::PostLabelOrdersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiLabelOrderResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_LABEL_ORDERS, req, opt)
    }

    pub fn post_label_orders_async(&self, req: &super::service::PostLabelOrdersRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiLabelOrderResponse>> {
        self.post_label_orders_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_label_order_opt(&self, req: &super::service::GetLabelOrderRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleLabelOrderResponse> {
        self.client.unary_call(&METHOD_V2_GET_LABEL_ORDER, req, opt)
    }

    pub fn get_label_order(&self, req: &super::service::GetLabelOrderRequest) -> ::grpcio::Result<super::service::SingleLabelOrderResponse> {
        self.get_label_order_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_label_order_async_opt(&self, req: &super::service::GetLabelOrderRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleLabelOrderResponse>> {
        self.client.unary_call_async(&METHOD_V2_GET_LABEL_ORDER, req, opt)
    }

    pub fn get_label_order_async(&self, req: &super::service::GetLabelOrderRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleLabelOrderResponse>> {
        self.get_label_order_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_label_orders_opt(&self, req: &super::service::ListLabelOrdersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiLabelOrderResponse> {
        self.client.unary_call(&METHOD_V2_LIST_LABEL_ORDERS, req, opt)
    }

    pub fn list_label_orders(&self, req: &super::service::ListLabelOrdersRequest) -> ::grpcio::Result<super::service::MultiLabelOrderResponse> {
        self.list_label_orders_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_label_orders_async_opt(&self, req: &super::service::ListLabelOrdersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiLabelOrderResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_LABEL_ORDERS, req, opt)
    }

    pub fn list_label_orders_async(&self, req: &super::service::ListLabelOrdersRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiLabelOrderResponse>> {
        self.list_label_orders_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_label_orders_opt(&self, req: &super::service::PatchLabelOrdersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiLabelOrderResponse> {
        self.client.unary_call(&METHOD_V2_PATCH_LABEL_ORDERS, req, opt)
    }

    pub fn patch_label_orders(&self, req: &super::service::PatchLabelOrdersRequest) -> ::grpcio::Result<super::service::MultiLabelOrderResponse> {
        self.patch_label_orders_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_label_orders_async_opt(&self, req: &super::service::PatchLabelOrdersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiLabelOrderResponse>> {
        self.client.unary_call_async(&METHOD_V2_PATCH_LABEL_ORDERS, req, opt)
    }

    pub fn patch_label_orders_async(&self, req: &super::service::PatchLabelOrdersRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiLabelOrderResponse>> {
        self.patch_label_orders_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_label_orders_opt(&self, req: &super::service::DeleteLabelOrdersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::status::BaseResponse> {
        self.client.unary_call(&METHOD_V2_DELETE_LABEL_ORDERS, req, opt)
    }

    pub fn delete_label_orders(&self, req: &super::service::DeleteLabelOrdersRequest) -> ::grpcio::Result<super::status::BaseResponse> {
        self.delete_label_orders_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_label_orders_async_opt(&self, req: &super::service::DeleteLabelOrdersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.client.unary_call_async(&METHOD_V2_DELETE_LABEL_ORDERS, req, opt)
    }

    pub fn delete_label_orders_async(&self, req: &super::service::DeleteLabelOrdersRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.delete_label_orders_async_opt(req, ::grpcio::CallOption::default())
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

    pub fn post_trending_metrics_view_opt(&self, req: &super::service::PostTrendingMetricsViewRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::status::BaseResponse> {
        self.client.unary_call(&METHOD_V2_POST_TRENDING_METRICS_VIEW, req, opt)
    }

    pub fn post_trending_metrics_view(&self, req: &super::service::PostTrendingMetricsViewRequest) -> ::grpcio::Result<super::status::BaseResponse> {
        self.post_trending_metrics_view_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_trending_metrics_view_async_opt(&self, req: &super::service::PostTrendingMetricsViewRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_TRENDING_METRICS_VIEW, req, opt)
    }

    pub fn post_trending_metrics_view_async(&self, req: &super::service::PostTrendingMetricsViewRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.post_trending_metrics_view_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_trending_metrics_views_opt(&self, req: &super::service::ListTrendingMetricsViewsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiTrendingMetricsViewResponse> {
        self.client.unary_call(&METHOD_V2_LIST_TRENDING_METRICS_VIEWS, req, opt)
    }

    pub fn list_trending_metrics_views(&self, req: &super::service::ListTrendingMetricsViewsRequest) -> ::grpcio::Result<super::service::MultiTrendingMetricsViewResponse> {
        self.list_trending_metrics_views_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_trending_metrics_views_async_opt(&self, req: &super::service::ListTrendingMetricsViewsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiTrendingMetricsViewResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_TRENDING_METRICS_VIEWS, req, opt)
    }

    pub fn list_trending_metrics_views_async(&self, req: &super::service::ListTrendingMetricsViewsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiTrendingMetricsViewResponse>> {
        self.list_trending_metrics_views_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_module_opt(&self, req: &super::service::GetModuleRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleModuleResponse> {
        self.client.unary_call(&METHOD_V2_GET_MODULE, req, opt)
    }

    pub fn get_module(&self, req: &super::service::GetModuleRequest) -> ::grpcio::Result<super::service::SingleModuleResponse> {
        self.get_module_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_module_async_opt(&self, req: &super::service::GetModuleRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleModuleResponse>> {
        self.client.unary_call_async(&METHOD_V2_GET_MODULE, req, opt)
    }

    pub fn get_module_async(&self, req: &super::service::GetModuleRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleModuleResponse>> {
        self.get_module_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_modules_opt(&self, req: &super::service::ListModulesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiModuleResponse> {
        self.client.unary_call(&METHOD_V2_LIST_MODULES, req, opt)
    }

    pub fn list_modules(&self, req: &super::service::ListModulesRequest) -> ::grpcio::Result<super::service::MultiModuleResponse> {
        self.list_modules_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_modules_async_opt(&self, req: &super::service::ListModulesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiModuleResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_MODULES, req, opt)
    }

    pub fn list_modules_async(&self, req: &super::service::ListModulesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiModuleResponse>> {
        self.list_modules_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_modules_opt(&self, req: &super::service::PostModulesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiModuleResponse> {
        self.client.unary_call(&METHOD_V2_POST_MODULES, req, opt)
    }

    pub fn post_modules(&self, req: &super::service::PostModulesRequest) -> ::grpcio::Result<super::service::MultiModuleResponse> {
        self.post_modules_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_modules_async_opt(&self, req: &super::service::PostModulesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiModuleResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_MODULES, req, opt)
    }

    pub fn post_modules_async(&self, req: &super::service::PostModulesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiModuleResponse>> {
        self.post_modules_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_modules_opt(&self, req: &super::service::PatchModulesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiModuleResponse> {
        self.client.unary_call(&METHOD_V2_PATCH_MODULES, req, opt)
    }

    pub fn patch_modules(&self, req: &super::service::PatchModulesRequest) -> ::grpcio::Result<super::service::MultiModuleResponse> {
        self.patch_modules_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_modules_async_opt(&self, req: &super::service::PatchModulesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiModuleResponse>> {
        self.client.unary_call_async(&METHOD_V2_PATCH_MODULES, req, opt)
    }

    pub fn patch_modules_async(&self, req: &super::service::PatchModulesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiModuleResponse>> {
        self.patch_modules_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_modules_opt(&self, req: &super::service::DeleteModulesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::status::BaseResponse> {
        self.client.unary_call(&METHOD_V2_DELETE_MODULES, req, opt)
    }

    pub fn delete_modules(&self, req: &super::service::DeleteModulesRequest) -> ::grpcio::Result<super::status::BaseResponse> {
        self.delete_modules_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_modules_async_opt(&self, req: &super::service::DeleteModulesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.client.unary_call_async(&METHOD_V2_DELETE_MODULES, req, opt)
    }

    pub fn delete_modules_async(&self, req: &super::service::DeleteModulesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.delete_modules_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_module_version_opt(&self, req: &super::service::GetModuleVersionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleModuleVersionResponse> {
        self.client.unary_call(&METHOD_V2_GET_MODULE_VERSION, req, opt)
    }

    pub fn get_module_version(&self, req: &super::service::GetModuleVersionRequest) -> ::grpcio::Result<super::service::SingleModuleVersionResponse> {
        self.get_module_version_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_module_version_async_opt(&self, req: &super::service::GetModuleVersionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleModuleVersionResponse>> {
        self.client.unary_call_async(&METHOD_V2_GET_MODULE_VERSION, req, opt)
    }

    pub fn get_module_version_async(&self, req: &super::service::GetModuleVersionRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleModuleVersionResponse>> {
        self.get_module_version_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_module_versions_opt(&self, req: &super::service::ListModuleVersionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiModuleVersionResponse> {
        self.client.unary_call(&METHOD_V2_LIST_MODULE_VERSIONS, req, opt)
    }

    pub fn list_module_versions(&self, req: &super::service::ListModuleVersionsRequest) -> ::grpcio::Result<super::service::MultiModuleVersionResponse> {
        self.list_module_versions_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_module_versions_async_opt(&self, req: &super::service::ListModuleVersionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiModuleVersionResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_MODULE_VERSIONS, req, opt)
    }

    pub fn list_module_versions_async(&self, req: &super::service::ListModuleVersionsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiModuleVersionResponse>> {
        self.list_module_versions_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_module_versions_opt(&self, req: &super::service::PostModuleVersionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiModuleVersionResponse> {
        self.client.unary_call(&METHOD_V2_POST_MODULE_VERSIONS, req, opt)
    }

    pub fn post_module_versions(&self, req: &super::service::PostModuleVersionsRequest) -> ::grpcio::Result<super::service::MultiModuleVersionResponse> {
        self.post_module_versions_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_module_versions_async_opt(&self, req: &super::service::PostModuleVersionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiModuleVersionResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_MODULE_VERSIONS, req, opt)
    }

    pub fn post_module_versions_async(&self, req: &super::service::PostModuleVersionsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiModuleVersionResponse>> {
        self.post_module_versions_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_module_versions_opt(&self, req: &super::service::PatchModuleVersionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiModuleVersionResponse> {
        self.client.unary_call(&METHOD_V2_PATCH_MODULE_VERSIONS, req, opt)
    }

    pub fn patch_module_versions(&self, req: &super::service::PatchModuleVersionsRequest) -> ::grpcio::Result<super::service::MultiModuleVersionResponse> {
        self.patch_module_versions_opt(req, ::grpcio::CallOption::default())
    }

    pub fn patch_module_versions_async_opt(&self, req: &super::service::PatchModuleVersionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiModuleVersionResponse>> {
        self.client.unary_call_async(&METHOD_V2_PATCH_MODULE_VERSIONS, req, opt)
    }

    pub fn patch_module_versions_async(&self, req: &super::service::PatchModuleVersionsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiModuleVersionResponse>> {
        self.patch_module_versions_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_module_versions_opt(&self, req: &super::service::DeleteModuleVersionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::status::BaseResponse> {
        self.client.unary_call(&METHOD_V2_DELETE_MODULE_VERSIONS, req, opt)
    }

    pub fn delete_module_versions(&self, req: &super::service::DeleteModuleVersionsRequest) -> ::grpcio::Result<super::status::BaseResponse> {
        self.delete_module_versions_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_module_versions_async_opt(&self, req: &super::service::DeleteModuleVersionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.client.unary_call_async(&METHOD_V2_DELETE_MODULE_VERSIONS, req, opt)
    }

    pub fn delete_module_versions_async(&self, req: &super::service::DeleteModuleVersionsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.delete_module_versions_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_module_version_usage_count_opt(&self, req: &super::service::GetModuleVersionUsageCountRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleModuleVersionUsageCountResponse> {
        self.client.unary_call(&METHOD_V2_GET_MODULE_VERSION_USAGE_COUNT, req, opt)
    }

    pub fn get_module_version_usage_count(&self, req: &super::service::GetModuleVersionUsageCountRequest) -> ::grpcio::Result<super::service::SingleModuleVersionUsageCountResponse> {
        self.get_module_version_usage_count_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_module_version_usage_count_async_opt(&self, req: &super::service::GetModuleVersionUsageCountRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleModuleVersionUsageCountResponse>> {
        self.client.unary_call_async(&METHOD_V2_GET_MODULE_VERSION_USAGE_COUNT, req, opt)
    }

    pub fn get_module_version_usage_count_async(&self, req: &super::service::GetModuleVersionUsageCountRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleModuleVersionUsageCountResponse>> {
        self.get_module_version_usage_count_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_installed_module_version_opt(&self, req: &super::service::GetInstalledModuleVersionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleInstalledModuleVersionResponse> {
        self.client.unary_call(&METHOD_V2_GET_INSTALLED_MODULE_VERSION, req, opt)
    }

    pub fn get_installed_module_version(&self, req: &super::service::GetInstalledModuleVersionRequest) -> ::grpcio::Result<super::service::SingleInstalledModuleVersionResponse> {
        self.get_installed_module_version_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_installed_module_version_async_opt(&self, req: &super::service::GetInstalledModuleVersionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleInstalledModuleVersionResponse>> {
        self.client.unary_call_async(&METHOD_V2_GET_INSTALLED_MODULE_VERSION, req, opt)
    }

    pub fn get_installed_module_version_async(&self, req: &super::service::GetInstalledModuleVersionRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleInstalledModuleVersionResponse>> {
        self.get_installed_module_version_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_installed_module_versions_opt(&self, req: &super::service::ListInstalledModuleVersionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiInstalledModuleVersionResponse> {
        self.client.unary_call(&METHOD_V2_LIST_INSTALLED_MODULE_VERSIONS, req, opt)
    }

    pub fn list_installed_module_versions(&self, req: &super::service::ListInstalledModuleVersionsRequest) -> ::grpcio::Result<super::service::MultiInstalledModuleVersionResponse> {
        self.list_installed_module_versions_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_installed_module_versions_async_opt(&self, req: &super::service::ListInstalledModuleVersionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiInstalledModuleVersionResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_INSTALLED_MODULE_VERSIONS, req, opt)
    }

    pub fn list_installed_module_versions_async(&self, req: &super::service::ListInstalledModuleVersionsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiInstalledModuleVersionResponse>> {
        self.list_installed_module_versions_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_installed_module_versions_opt(&self, req: &super::service::PostInstalledModuleVersionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiInstalledModuleVersionResponse> {
        self.client.unary_call(&METHOD_V2_POST_INSTALLED_MODULE_VERSIONS, req, opt)
    }

    pub fn post_installed_module_versions(&self, req: &super::service::PostInstalledModuleVersionsRequest) -> ::grpcio::Result<super::service::MultiInstalledModuleVersionResponse> {
        self.post_installed_module_versions_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_installed_module_versions_async_opt(&self, req: &super::service::PostInstalledModuleVersionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiInstalledModuleVersionResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_INSTALLED_MODULE_VERSIONS, req, opt)
    }

    pub fn post_installed_module_versions_async(&self, req: &super::service::PostInstalledModuleVersionsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiInstalledModuleVersionResponse>> {
        self.post_installed_module_versions_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_installed_module_versions_opt(&self, req: &super::service::DeleteInstalledModuleVersionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::status::BaseResponse> {
        self.client.unary_call(&METHOD_V2_DELETE_INSTALLED_MODULE_VERSIONS, req, opt)
    }

    pub fn delete_installed_module_versions(&self, req: &super::service::DeleteInstalledModuleVersionsRequest) -> ::grpcio::Result<super::status::BaseResponse> {
        self.delete_installed_module_versions_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_installed_module_versions_async_opt(&self, req: &super::service::DeleteInstalledModuleVersionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.client.unary_call_async(&METHOD_V2_DELETE_INSTALLED_MODULE_VERSIONS, req, opt)
    }

    pub fn delete_installed_module_versions_async(&self, req: &super::service::DeleteInstalledModuleVersionsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.delete_installed_module_versions_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_installed_module_versions_key_opt(&self, req: &super::service::PostInstalledModuleVersionsKeyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleKeyResponse> {
        self.client.unary_call(&METHOD_V2_POST_INSTALLED_MODULE_VERSIONS_KEY, req, opt)
    }

    pub fn post_installed_module_versions_key(&self, req: &super::service::PostInstalledModuleVersionsKeyRequest) -> ::grpcio::Result<super::service::SingleKeyResponse> {
        self.post_installed_module_versions_key_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_installed_module_versions_key_async_opt(&self, req: &super::service::PostInstalledModuleVersionsKeyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleKeyResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_INSTALLED_MODULE_VERSIONS_KEY, req, opt)
    }

    pub fn post_installed_module_versions_key_async(&self, req: &super::service::PostInstalledModuleVersionsKeyRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleKeyResponse>> {
        self.post_installed_module_versions_key_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_bulk_operations_opt(&self, req: &super::service::PostBulkOperationsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiBulkOperationsResponse> {
        self.client.unary_call(&METHOD_V2_POST_BULK_OPERATIONS, req, opt)
    }

    pub fn post_bulk_operations(&self, req: &super::service::PostBulkOperationsRequest) -> ::grpcio::Result<super::service::MultiBulkOperationsResponse> {
        self.post_bulk_operations_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_bulk_operations_async_opt(&self, req: &super::service::PostBulkOperationsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiBulkOperationsResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_BULK_OPERATIONS, req, opt)
    }

    pub fn post_bulk_operations_async(&self, req: &super::service::PostBulkOperationsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiBulkOperationsResponse>> {
        self.post_bulk_operations_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_bulk_operations_opt(&self, req: &super::service::ListBulkOperationsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiBulkOperationsResponse> {
        self.client.unary_call(&METHOD_V2_LIST_BULK_OPERATIONS, req, opt)
    }

    pub fn list_bulk_operations(&self, req: &super::service::ListBulkOperationsRequest) -> ::grpcio::Result<super::service::MultiBulkOperationsResponse> {
        self.list_bulk_operations_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_bulk_operations_async_opt(&self, req: &super::service::ListBulkOperationsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiBulkOperationsResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_BULK_OPERATIONS, req, opt)
    }

    pub fn list_bulk_operations_async(&self, req: &super::service::ListBulkOperationsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiBulkOperationsResponse>> {
        self.list_bulk_operations_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_bulk_operation_opt(&self, req: &super::service::GetBulkOperationRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleBulkOperationsResponse> {
        self.client.unary_call(&METHOD_V2_GET_BULK_OPERATION, req, opt)
    }

    pub fn get_bulk_operation(&self, req: &super::service::GetBulkOperationRequest) -> ::grpcio::Result<super::service::SingleBulkOperationsResponse> {
        self.get_bulk_operation_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_bulk_operation_async_opt(&self, req: &super::service::GetBulkOperationRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleBulkOperationsResponse>> {
        self.client.unary_call_async(&METHOD_V2_GET_BULK_OPERATION, req, opt)
    }

    pub fn get_bulk_operation_async(&self, req: &super::service::GetBulkOperationRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleBulkOperationsResponse>> {
        self.get_bulk_operation_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn cancel_bulk_operations_opt(&self, req: &super::service::CancelBulkOperationRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiBulkOperationsResponse> {
        self.client.unary_call(&METHOD_V2_CANCEL_BULK_OPERATIONS, req, opt)
    }

    pub fn cancel_bulk_operations(&self, req: &super::service::CancelBulkOperationRequest) -> ::grpcio::Result<super::service::MultiBulkOperationsResponse> {
        self.cancel_bulk_operations_opt(req, ::grpcio::CallOption::default())
    }

    pub fn cancel_bulk_operations_async_opt(&self, req: &super::service::CancelBulkOperationRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiBulkOperationsResponse>> {
        self.client.unary_call_async(&METHOD_V2_CANCEL_BULK_OPERATIONS, req, opt)
    }

    pub fn cancel_bulk_operations_async(&self, req: &super::service::CancelBulkOperationRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiBulkOperationsResponse>> {
        self.cancel_bulk_operations_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_bulk_operations_opt(&self, req: &super::service::DeleteBulkOperationRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::status::BaseResponse> {
        self.client.unary_call(&METHOD_V2_DELETE_BULK_OPERATIONS, req, opt)
    }

    pub fn delete_bulk_operations(&self, req: &super::service::DeleteBulkOperationRequest) -> ::grpcio::Result<super::status::BaseResponse> {
        self.delete_bulk_operations_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_bulk_operations_async_opt(&self, req: &super::service::DeleteBulkOperationRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.client.unary_call_async(&METHOD_V2_DELETE_BULK_OPERATIONS, req, opt)
    }

    pub fn delete_bulk_operations_async(&self, req: &super::service::DeleteBulkOperationRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.delete_bulk_operations_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_next_task_assignments_opt(&self, req: &super::service::ListNextTaskAssignmentsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiInputResponse> {
        self.client.unary_call(&METHOD_V2_LIST_NEXT_TASK_ASSIGNMENTS, req, opt)
    }

    pub fn list_next_task_assignments(&self, req: &super::service::ListNextTaskAssignmentsRequest) -> ::grpcio::Result<super::service::MultiInputResponse> {
        self.list_next_task_assignments_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_next_task_assignments_async_opt(&self, req: &super::service::ListNextTaskAssignmentsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiInputResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_NEXT_TASK_ASSIGNMENTS, req, opt)
    }

    pub fn list_next_task_assignments_async(&self, req: &super::service::ListNextTaskAssignmentsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiInputResponse>> {
        self.list_next_task_assignments_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn put_task_assignments_opt(&self, req: &super::service::PutTaskAssignmentsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::status::BaseResponse> {
        self.client.unary_call(&METHOD_V2_PUT_TASK_ASSIGNMENTS, req, opt)
    }

    pub fn put_task_assignments(&self, req: &super::service::PutTaskAssignmentsRequest) -> ::grpcio::Result<super::status::BaseResponse> {
        self.put_task_assignments_opt(req, ::grpcio::CallOption::default())
    }

    pub fn put_task_assignments_async_opt(&self, req: &super::service::PutTaskAssignmentsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.client.unary_call_async(&METHOD_V2_PUT_TASK_ASSIGNMENTS, req, opt)
    }

    pub fn put_task_assignments_async(&self, req: &super::service::PutTaskAssignmentsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.put_task_assignments_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_inputs_add_jobs_opt(&self, req: &super::service::ListInputsAddJobsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiInputsAddJobResponse> {
        self.client.unary_call(&METHOD_V2_LIST_INPUTS_ADD_JOBS, req, opt)
    }

    pub fn list_inputs_add_jobs(&self, req: &super::service::ListInputsAddJobsRequest) -> ::grpcio::Result<super::service::MultiInputsAddJobResponse> {
        self.list_inputs_add_jobs_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_inputs_add_jobs_async_opt(&self, req: &super::service::ListInputsAddJobsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiInputsAddJobResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_INPUTS_ADD_JOBS, req, opt)
    }

    pub fn list_inputs_add_jobs_async(&self, req: &super::service::ListInputsAddJobsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiInputsAddJobResponse>> {
        self.list_inputs_add_jobs_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_inputs_add_job_opt(&self, req: &super::service::GetInputsAddJobRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleInputsAddJobResponse> {
        self.client.unary_call(&METHOD_V2_GET_INPUTS_ADD_JOB, req, opt)
    }

    pub fn get_inputs_add_job(&self, req: &super::service::GetInputsAddJobRequest) -> ::grpcio::Result<super::service::SingleInputsAddJobResponse> {
        self.get_inputs_add_job_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_inputs_add_job_async_opt(&self, req: &super::service::GetInputsAddJobRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleInputsAddJobResponse>> {
        self.client.unary_call_async(&METHOD_V2_GET_INPUTS_ADD_JOB, req, opt)
    }

    pub fn get_inputs_add_job_async(&self, req: &super::service::GetInputsAddJobRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleInputsAddJobResponse>> {
        self.get_inputs_add_job_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn cancel_inputs_add_job_opt(&self, req: &super::service::CancelInputsAddJobRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleInputsAddJobResponse> {
        self.client.unary_call(&METHOD_V2_CANCEL_INPUTS_ADD_JOB, req, opt)
    }

    pub fn cancel_inputs_add_job(&self, req: &super::service::CancelInputsAddJobRequest) -> ::grpcio::Result<super::service::SingleInputsAddJobResponse> {
        self.cancel_inputs_add_job_opt(req, ::grpcio::CallOption::default())
    }

    pub fn cancel_inputs_add_job_async_opt(&self, req: &super::service::CancelInputsAddJobRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleInputsAddJobResponse>> {
        self.client.unary_call_async(&METHOD_V2_CANCEL_INPUTS_ADD_JOB, req, opt)
    }

    pub fn cancel_inputs_add_job_async(&self, req: &super::service::CancelInputsAddJobRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleInputsAddJobResponse>> {
        self.cancel_inputs_add_job_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_uploads_opt(&self, req: &super::service::PostUploadsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiUploadResponse> {
        self.client.unary_call(&METHOD_V2_POST_UPLOADS, req, opt)
    }

    pub fn post_uploads(&self, req: &super::service::PostUploadsRequest) -> ::grpcio::Result<super::service::MultiUploadResponse> {
        self.post_uploads_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_uploads_async_opt(&self, req: &super::service::PostUploadsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiUploadResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_UPLOADS, req, opt)
    }

    pub fn post_uploads_async(&self, req: &super::service::PostUploadsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiUploadResponse>> {
        self.post_uploads_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn put_upload_content_parts_opt(&self, req: &super::service::PutUploadContentPartsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleUploadResponse> {
        self.client.unary_call(&METHOD_V2_PUT_UPLOAD_CONTENT_PARTS, req, opt)
    }

    pub fn put_upload_content_parts(&self, req: &super::service::PutUploadContentPartsRequest) -> ::grpcio::Result<super::service::SingleUploadResponse> {
        self.put_upload_content_parts_opt(req, ::grpcio::CallOption::default())
    }

    pub fn put_upload_content_parts_async_opt(&self, req: &super::service::PutUploadContentPartsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleUploadResponse>> {
        self.client.unary_call_async(&METHOD_V2_PUT_UPLOAD_CONTENT_PARTS, req, opt)
    }

    pub fn put_upload_content_parts_async(&self, req: &super::service::PutUploadContentPartsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleUploadResponse>> {
        self.put_upload_content_parts_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_upload_opt(&self, req: &super::service::GetUploadRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleUploadResponse> {
        self.client.unary_call(&METHOD_V2_GET_UPLOAD, req, opt)
    }

    pub fn get_upload(&self, req: &super::service::GetUploadRequest) -> ::grpcio::Result<super::service::SingleUploadResponse> {
        self.get_upload_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_upload_async_opt(&self, req: &super::service::GetUploadRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleUploadResponse>> {
        self.client.unary_call_async(&METHOD_V2_GET_UPLOAD, req, opt)
    }

    pub fn get_upload_async(&self, req: &super::service::GetUploadRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleUploadResponse>> {
        self.get_upload_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_uploads_opt(&self, req: &super::service::ListUploadsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiUploadResponse> {
        self.client.unary_call(&METHOD_V2_LIST_UPLOADS, req, opt)
    }

    pub fn list_uploads(&self, req: &super::service::ListUploadsRequest) -> ::grpcio::Result<super::service::MultiUploadResponse> {
        self.list_uploads_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_uploads_async_opt(&self, req: &super::service::ListUploadsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiUploadResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_UPLOADS, req, opt)
    }

    pub fn list_uploads_async(&self, req: &super::service::ListUploadsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiUploadResponse>> {
        self.list_uploads_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_uploads_opt(&self, req: &super::service::DeleteUploadsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::status::BaseResponse> {
        self.client.unary_call(&METHOD_V2_DELETE_UPLOADS, req, opt)
    }

    pub fn delete_uploads(&self, req: &super::service::DeleteUploadsRequest) -> ::grpcio::Result<super::status::BaseResponse> {
        self.delete_uploads_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_uploads_async_opt(&self, req: &super::service::DeleteUploadsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.client.unary_call_async(&METHOD_V2_DELETE_UPLOADS, req, opt)
    }

    pub fn delete_uploads_async(&self, req: &super::service::DeleteUploadsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.delete_uploads_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_inputs_data_sources_opt(&self, req: &super::service::PostInputsDataSourcesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiInputsAddJobResponse> {
        self.client.unary_call(&METHOD_V2_POST_INPUTS_DATA_SOURCES, req, opt)
    }

    pub fn post_inputs_data_sources(&self, req: &super::service::PostInputsDataSourcesRequest) -> ::grpcio::Result<super::service::MultiInputsAddJobResponse> {
        self.post_inputs_data_sources_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_inputs_data_sources_async_opt(&self, req: &super::service::PostInputsDataSourcesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiInputsAddJobResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_INPUTS_DATA_SOURCES, req, opt)
    }

    pub fn post_inputs_data_sources_async(&self, req: &super::service::PostInputsDataSourcesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiInputsAddJobResponse>> {
        self.post_inputs_data_sources_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_inputs_extraction_job_opt(&self, req: &super::service::GetInputsExtractionJobRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleInputsExtractionJobResponse> {
        self.client.unary_call(&METHOD_V2_GET_INPUTS_EXTRACTION_JOB, req, opt)
    }

    pub fn get_inputs_extraction_job(&self, req: &super::service::GetInputsExtractionJobRequest) -> ::grpcio::Result<super::service::SingleInputsExtractionJobResponse> {
        self.get_inputs_extraction_job_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_inputs_extraction_job_async_opt(&self, req: &super::service::GetInputsExtractionJobRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleInputsExtractionJobResponse>> {
        self.client.unary_call_async(&METHOD_V2_GET_INPUTS_EXTRACTION_JOB, req, opt)
    }

    pub fn get_inputs_extraction_job_async(&self, req: &super::service::GetInputsExtractionJobRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleInputsExtractionJobResponse>> {
        self.get_inputs_extraction_job_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_inputs_extraction_jobs_opt(&self, req: &super::service::ListInputsExtractionJobsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiInputsExtractionJobResponse> {
        self.client.unary_call(&METHOD_V2_LIST_INPUTS_EXTRACTION_JOBS, req, opt)
    }

    pub fn list_inputs_extraction_jobs(&self, req: &super::service::ListInputsExtractionJobsRequest) -> ::grpcio::Result<super::service::MultiInputsExtractionJobResponse> {
        self.list_inputs_extraction_jobs_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_inputs_extraction_jobs_async_opt(&self, req: &super::service::ListInputsExtractionJobsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiInputsExtractionJobResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_INPUTS_EXTRACTION_JOBS, req, opt)
    }

    pub fn list_inputs_extraction_jobs_async(&self, req: &super::service::ListInputsExtractionJobsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiInputsExtractionJobResponse>> {
        self.list_inputs_extraction_jobs_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn cancel_inputs_extraction_jobs_opt(&self, req: &super::service::CancelInputsExtractionJobsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiInputsExtractionJobResponse> {
        self.client.unary_call(&METHOD_V2_CANCEL_INPUTS_EXTRACTION_JOBS, req, opt)
    }

    pub fn cancel_inputs_extraction_jobs(&self, req: &super::service::CancelInputsExtractionJobsRequest) -> ::grpcio::Result<super::service::MultiInputsExtractionJobResponse> {
        self.cancel_inputs_extraction_jobs_opt(req, ::grpcio::CallOption::default())
    }

    pub fn cancel_inputs_extraction_jobs_async_opt(&self, req: &super::service::CancelInputsExtractionJobsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiInputsExtractionJobResponse>> {
        self.client.unary_call_async(&METHOD_V2_CANCEL_INPUTS_EXTRACTION_JOBS, req, opt)
    }

    pub fn cancel_inputs_extraction_jobs_async(&self, req: &super::service::CancelInputsExtractionJobsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiInputsExtractionJobResponse>> {
        self.cancel_inputs_extraction_jobs_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_inputs_uploads_opt(&self, req: &super::service::PostInputsUploadsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiInputsAddJobResponse> {
        self.client.unary_call(&METHOD_V2_POST_INPUTS_UPLOADS, req, opt)
    }

    pub fn post_inputs_uploads(&self, req: &super::service::PostInputsUploadsRequest) -> ::grpcio::Result<super::service::MultiInputsAddJobResponse> {
        self.post_inputs_uploads_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_inputs_uploads_async_opt(&self, req: &super::service::PostInputsUploadsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiInputsAddJobResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_INPUTS_UPLOADS, req, opt)
    }

    pub fn post_inputs_uploads_async(&self, req: &super::service::PostInputsUploadsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiInputsAddJobResponse>> {
        self.post_inputs_uploads_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_runner_opt(&self, req: &super::service::GetRunnerRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::SingleRunnerResponse> {
        self.client.unary_call(&METHOD_V2_GET_RUNNER, req, opt)
    }

    pub fn get_runner(&self, req: &super::service::GetRunnerRequest) -> ::grpcio::Result<super::service::SingleRunnerResponse> {
        self.get_runner_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_runner_async_opt(&self, req: &super::service::GetRunnerRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleRunnerResponse>> {
        self.client.unary_call_async(&METHOD_V2_GET_RUNNER, req, opt)
    }

    pub fn get_runner_async(&self, req: &super::service::GetRunnerRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::SingleRunnerResponse>> {
        self.get_runner_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_runners_opt(&self, req: &super::service::ListRunnersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiRunnerResponse> {
        self.client.unary_call(&METHOD_V2_LIST_RUNNERS, req, opt)
    }

    pub fn list_runners(&self, req: &super::service::ListRunnersRequest) -> ::grpcio::Result<super::service::MultiRunnerResponse> {
        self.list_runners_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_runners_async_opt(&self, req: &super::service::ListRunnersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiRunnerResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_RUNNERS, req, opt)
    }

    pub fn list_runners_async(&self, req: &super::service::ListRunnersRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiRunnerResponse>> {
        self.list_runners_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_runners_opt(&self, req: &super::service::PostRunnersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiRunnerResponse> {
        self.client.unary_call(&METHOD_V2_POST_RUNNERS, req, opt)
    }

    pub fn post_runners(&self, req: &super::service::PostRunnersRequest) -> ::grpcio::Result<super::service::MultiRunnerResponse> {
        self.post_runners_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_runners_async_opt(&self, req: &super::service::PostRunnersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiRunnerResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_RUNNERS, req, opt)
    }

    pub fn post_runners_async(&self, req: &super::service::PostRunnersRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiRunnerResponse>> {
        self.post_runners_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_runners_opt(&self, req: &super::service::DeleteRunnersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::status::BaseResponse> {
        self.client.unary_call(&METHOD_V2_DELETE_RUNNERS, req, opt)
    }

    pub fn delete_runners(&self, req: &super::service::DeleteRunnersRequest) -> ::grpcio::Result<super::status::BaseResponse> {
        self.delete_runners_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_runners_async_opt(&self, req: &super::service::DeleteRunnersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.client.unary_call_async(&METHOD_V2_DELETE_RUNNERS, req, opt)
    }

    pub fn delete_runners_async(&self, req: &super::service::DeleteRunnersRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::status::BaseResponse>> {
        self.delete_runners_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_runner_items_opt(&self, req: &super::service::ListRunnerItemsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiRunnerItemResponse> {
        self.client.unary_call(&METHOD_V2_LIST_RUNNER_ITEMS, req, opt)
    }

    pub fn list_runner_items(&self, req: &super::service::ListRunnerItemsRequest) -> ::grpcio::Result<super::service::MultiRunnerItemResponse> {
        self.list_runner_items_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_runner_items_async_opt(&self, req: &super::service::ListRunnerItemsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiRunnerItemResponse>> {
        self.client.unary_call_async(&METHOD_V2_LIST_RUNNER_ITEMS, req, opt)
    }

    pub fn list_runner_items_async(&self, req: &super::service::ListRunnerItemsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiRunnerItemResponse>> {
        self.list_runner_items_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_runner_item_outputs_opt(&self, req: &super::service::PostRunnerItemOutputsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiRunnerItemOutputResponse> {
        self.client.unary_call(&METHOD_V2_POST_RUNNER_ITEM_OUTPUTS, req, opt)
    }

    pub fn post_runner_item_outputs(&self, req: &super::service::PostRunnerItemOutputsRequest) -> ::grpcio::Result<super::service::MultiRunnerItemOutputResponse> {
        self.post_runner_item_outputs_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_runner_item_outputs_async_opt(&self, req: &super::service::PostRunnerItemOutputsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiRunnerItemOutputResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_RUNNER_ITEM_OUTPUTS, req, opt)
    }

    pub fn post_runner_item_outputs_async(&self, req: &super::service::PostRunnerItemOutputsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiRunnerItemOutputResponse>> {
        self.post_runner_item_outputs_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_model_versions_training_time_estimate_opt(&self, req: &super::service::PostModelVersionsTrainingTimeEstimateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::service::MultiTrainingTimeEstimateResponse> {
        self.client.unary_call(&METHOD_V2_POST_MODEL_VERSIONS_TRAINING_TIME_ESTIMATE, req, opt)
    }

    pub fn post_model_versions_training_time_estimate(&self, req: &super::service::PostModelVersionsTrainingTimeEstimateRequest) -> ::grpcio::Result<super::service::MultiTrainingTimeEstimateResponse> {
        self.post_model_versions_training_time_estimate_opt(req, ::grpcio::CallOption::default())
    }

    pub fn post_model_versions_training_time_estimate_async_opt(&self, req: &super::service::PostModelVersionsTrainingTimeEstimateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiTrainingTimeEstimateResponse>> {
        self.client.unary_call_async(&METHOD_V2_POST_MODEL_VERSIONS_TRAINING_TIME_ESTIMATE, req, opt)
    }

    pub fn post_model_versions_training_time_estimate_async(&self, req: &super::service::PostModelVersionsTrainingTimeEstimateRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::service::MultiTrainingTimeEstimateResponse>> {
        self.post_model_versions_training_time_estimate_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::std::future::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait V2 {
    fn list_concept_relations(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListConceptRelationsRequest, sink: ::grpcio::UnarySink<super::service::MultiConceptRelationResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_concept_relations(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostConceptRelationsRequest, sink: ::grpcio::UnarySink<super::service::MultiConceptRelationResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_concept_relations(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::DeleteConceptRelationsRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_concept_counts(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::GetConceptCountsRequest, sink: ::grpcio::UnarySink<super::service::MultiConceptCountResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_concept(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::GetConceptRequest, sink: ::grpcio::UnarySink<super::service::SingleConceptResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_concepts(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListConceptsRequest, sink: ::grpcio::UnarySink<super::service::MultiConceptResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_model_concepts(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListModelConceptsRequest, sink: ::grpcio::UnarySink<super::service::MultiConceptResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_concepts_searches(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostConceptsSearchesRequest, sink: ::grpcio::UnarySink<super::service::MultiConceptResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_concepts(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostConceptsRequest, sink: ::grpcio::UnarySink<super::service::MultiConceptResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn patch_concepts(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PatchConceptsRequest, sink: ::grpcio::UnarySink<super::service::MultiConceptResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_concept_language(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::GetConceptLanguageRequest, sink: ::grpcio::UnarySink<super::service::SingleConceptLanguageResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_concept_languages(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListConceptLanguagesRequest, sink: ::grpcio::UnarySink<super::service::MultiConceptLanguageResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_concept_languages(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostConceptLanguagesRequest, sink: ::grpcio::UnarySink<super::service::MultiConceptLanguageResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn patch_concept_languages(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PatchConceptLanguagesRequest, sink: ::grpcio::UnarySink<super::service::MultiConceptLanguageResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_knowledge_graphs(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListKnowledgeGraphsRequest, sink: ::grpcio::UnarySink<super::service::MultiKnowledgeGraphResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_knowledge_graphs(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostKnowledgeGraphsRequest, sink: ::grpcio::UnarySink<super::service::MultiKnowledgeGraphResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_concept_mapping_jobs(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostConceptMappingJobsRequest, sink: ::grpcio::UnarySink<super::service::MultiConceptMappingJobResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_annotation(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::GetAnnotationRequest, sink: ::grpcio::UnarySink<super::service::SingleAnnotationResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_annotations(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListAnnotationsRequest, sink: ::grpcio::UnarySink<super::service::MultiAnnotationResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_annotations(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostAnnotationsRequest, sink: ::grpcio::UnarySink<super::service::MultiAnnotationResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn patch_annotations(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PatchAnnotationsRequest, sink: ::grpcio::UnarySink<super::service::MultiAnnotationResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn patch_annotations_status(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PatchAnnotationsStatusRequest, sink: ::grpcio::UnarySink<super::service::PatchAnnotationsStatusResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_annotation(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::DeleteAnnotationRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_annotations(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::DeleteAnnotationsRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn patch_annotations_searches(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PatchAnnotationsSearchesRequest, sink: ::grpcio::UnarySink<super::service::MultiSearchResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_annotations_searches(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostAnnotationsSearchesRequest, sink: ::grpcio::UnarySink<super::service::MultiSearchResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_annotation_workers(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListAnnotationWorkersRequest, sink: ::grpcio::UnarySink<super::service::MultiWorkerResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_input_count(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::GetInputCountRequest, sink: ::grpcio::UnarySink<super::service::SingleInputCountResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn stream_inputs(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::StreamInputsRequest, sink: ::grpcio::UnarySink<super::service::MultiInputResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_input_samples(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::GetInputSamplesRequest, sink: ::grpcio::UnarySink<super::service::MultiInputAnnotationResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_input(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::GetInputRequest, sink: ::grpcio::UnarySink<super::service::SingleInputResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_input_video_manifest(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::GetVideoManifestRequest, sink: ::grpcio::UnarySink<super::service::GetVideoManifestResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_inputs(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListInputsRequest, sink: ::grpcio::UnarySink<super::service::MultiInputResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_inputs(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostInputsRequest, sink: ::grpcio::UnarySink<super::service::MultiInputResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn patch_inputs(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PatchInputsRequest, sink: ::grpcio::UnarySink<super::service::MultiInputResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_input(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::DeleteInputRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_inputs(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::DeleteInputsRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn patch_inputs_searches(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PatchInputsSearchesRequest, sink: ::grpcio::UnarySink<super::service::MultiSearchResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_inputs_searches(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostInputsSearchesRequest, sink: ::grpcio::UnarySink<super::service::MultiSearchResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_model_outputs(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostModelOutputsRequest, sink: ::grpcio::UnarySink<super::service::MultiOutputResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_datasets(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListDatasetsRequest, sink: ::grpcio::UnarySink<super::service::MultiDatasetResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_dataset(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::GetDatasetRequest, sink: ::grpcio::UnarySink<super::service::SingleDatasetResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_datasets(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostDatasetsRequest, sink: ::grpcio::UnarySink<super::service::MultiDatasetResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn patch_datasets(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PatchDatasetsRequest, sink: ::grpcio::UnarySink<super::service::MultiDatasetResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_datasets(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::DeleteDatasetsRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_dataset_inputs(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListDatasetInputsRequest, sink: ::grpcio::UnarySink<super::service::MultiDatasetInputResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_dataset_input(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::GetDatasetInputRequest, sink: ::grpcio::UnarySink<super::service::SingleDatasetInputResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_dataset_inputs(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostDatasetInputsRequest, sink: ::grpcio::UnarySink<super::service::MultiDatasetInputResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_dataset_inputs(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::DeleteDatasetInputsRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_dataset_versions(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListDatasetVersionsRequest, sink: ::grpcio::UnarySink<super::service::MultiDatasetVersionResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_dataset_version(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::GetDatasetVersionRequest, sink: ::grpcio::UnarySink<super::service::SingleDatasetVersionResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_dataset_version_metrics_groups(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListDatasetVersionMetricsGroupsRequest, sink: ::grpcio::UnarySink<super::service::MultiDatasetVersionMetricsGroupResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_dataset_versions(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostDatasetVersionsRequest, sink: ::grpcio::UnarySink<super::service::MultiDatasetVersionResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn patch_dataset_versions(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PatchDatasetVersionsRequest, sink: ::grpcio::UnarySink<super::service::MultiDatasetVersionResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_dataset_versions(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::DeleteDatasetVersionsRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn put_dataset_version_exports(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PutDatasetVersionExportsRequest, sink: ::grpcio::UnarySink<super::service::MultiDatasetVersionExportResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_model_type(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::GetModelTypeRequest, sink: ::grpcio::UnarySink<super::service::SingleModelTypeResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_open_source_licenses(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListOpenSourceLicensesRequest, sink: ::grpcio::UnarySink<super::service::ListOpenSourceLicensesResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_model_types(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListModelTypesRequest, sink: ::grpcio::UnarySink<super::service::MultiModelTypeResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_model(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::GetModelRequest, sink: ::grpcio::UnarySink<super::service::SingleModelResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_model_output_info(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::GetModelRequest, sink: ::grpcio::UnarySink<super::service::SingleModelResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_models(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListModelsRequest, sink: ::grpcio::UnarySink<super::service::MultiModelResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_resource_counts(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::GetResourceCountsRequest, sink: ::grpcio::UnarySink<super::service::GetResourceCountsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_models_searches(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostModelsSearchesRequest, sink: ::grpcio::UnarySink<super::service::MultiModelResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_models(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostModelsRequest, sink: ::grpcio::UnarySink<super::service::SingleModelResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn patch_models(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PatchModelsRequest, sink: ::grpcio::UnarySink<super::service::MultiModelResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn patch_model_ids(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PatchModelIdsRequest, sink: ::grpcio::UnarySink<super::service::MultiModelResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_model(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::DeleteModelRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_models(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::DeleteModelsRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn patch_model_check_consents(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PatchModelCheckConsentsRequest, sink: ::grpcio::UnarySink<super::service::MultiModelCheckConsentResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn patch_model_toolkits(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PatchModelToolkitsRequest, sink: ::grpcio::UnarySink<super::service::MultiModelToolkitResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn patch_model_use_cases(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PatchModelUseCasesRequest, sink: ::grpcio::UnarySink<super::service::MultiModelUseCaseResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn patch_model_languages(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PatchModelLanguagesRequest, sink: ::grpcio::UnarySink<super::service::MultiModelLanguageResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_model_inputs(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListModelInputsRequest, sink: ::grpcio::UnarySink<super::service::MultiInputResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_model_version(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::GetModelVersionRequest, sink: ::grpcio::UnarySink<super::service::SingleModelVersionResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_model_versions(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListModelVersionsRequest, sink: ::grpcio::UnarySink<super::service::MultiModelVersionResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_workflow_versions_un_publish(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostWorkflowVersionsUnPublishRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_workflow_versions_publish(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostWorkflowVersionsPublishRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_model_versions_publish(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostModelVersionsPublishRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_model_versions_un_publish(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostModelVersionsUnPublishRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_model_versions(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostModelVersionsRequest, sink: ::grpcio::UnarySink<super::service::SingleModelResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn patch_model_versions(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PatchModelVersionsRequest, sink: ::grpcio::UnarySink<super::service::MultiModelVersionResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_model_version(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::DeleteModelVersionRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_model_versions_upload(&mut self, ctx: ::grpcio::RpcContext, _stream: ::grpcio::RequestStream<super::service::PostModelVersionsUploadRequest>, sink: ::grpcio::DuplexSink<super::service::PostModelVersionsUploadResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_model_version_metrics(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::GetModelVersionMetricsRequest, sink: ::grpcio::UnarySink<super::service::SingleModelVersionResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_model_version_metrics(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostModelVersionMetricsRequest, sink: ::grpcio::UnarySink<super::service::SingleModelVersionResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_model_version_evaluations(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostModelVersionEvaluationsRequest, sink: ::grpcio::UnarySink<super::service::MultiEvalMetricsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_model_version_evaluations(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListModelVersionEvaluationsRequest, sink: ::grpcio::UnarySink<super::service::MultiEvalMetricsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_model_version_evaluation(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::GetModelVersionEvaluationRequest, sink: ::grpcio::UnarySink<super::service::SingleEvalMetricsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_evaluations(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostEvaluationsRequest, sink: ::grpcio::UnarySink<super::service::MultiEvalMetricsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_evaluations(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListEvaluationsRequest, sink: ::grpcio::UnarySink<super::service::MultiEvalMetricsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_evaluation(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::GetEvaluationRequest, sink: ::grpcio::UnarySink<super::service::SingleEvalMetricsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_model_references(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListModelReferencesRequest, sink: ::grpcio::UnarySink<super::service::MultiModelReferenceResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_model_version_input_example(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::GetModelVersionInputExampleRequest, sink: ::grpcio::UnarySink<super::service::SingleModelVersionInputExampleResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_model_version_input_examples(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListModelVersionInputExamplesRequest, sink: ::grpcio::UnarySink<super::service::MultiModelVersionInputExampleResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_workflow(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::GetWorkflowRequest, sink: ::grpcio::UnarySink<super::service::SingleWorkflowResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_workflows(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListWorkflowsRequest, sink: ::grpcio::UnarySink<super::service::MultiWorkflowResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_workflows(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostWorkflowsRequest, sink: ::grpcio::UnarySink<super::service::MultiWorkflowResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn patch_workflows(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PatchWorkflowsRequest, sink: ::grpcio::UnarySink<super::service::MultiWorkflowResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn patch_workflow_ids(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PatchWorkflowIdsRequest, sink: ::grpcio::UnarySink<super::service::MultiWorkflowResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_workflow(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::DeleteWorkflowRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_workflows(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::DeleteWorkflowsRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_workflow_results(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostWorkflowResultsRequest, sink: ::grpcio::UnarySink<super::service::PostWorkflowResultsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_workflow_results_similarity(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostWorkflowResultsSimilarityRequest, sink: ::grpcio::UnarySink<super::service::PostWorkflowResultsSimilarityResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_workflow_versions(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListWorkflowVersionsRequest, sink: ::grpcio::UnarySink<super::service::MultiWorkflowVersionResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_workflow_version(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::GetWorkflowVersionRequest, sink: ::grpcio::UnarySink<super::service::SingleWorkflowVersionResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_workflow_versions(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::DeleteWorkflowVersionsRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn patch_workflow_versions(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PatchWorkflowVersionsRequest, sink: ::grpcio::UnarySink<super::service::MultiWorkflowVersionResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_key(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::GetKeyRequest, sink: ::grpcio::UnarySink<super::service::SingleKeyResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_keys(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListKeysRequest, sink: ::grpcio::UnarySink<super::service::MultiKeyResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_app_keys(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListAppKeysRequest, sink: ::grpcio::UnarySink<super::service::MultiKeyResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_key(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::DeleteKeyRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_keys(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostKeysRequest, sink: ::grpcio::UnarySink<super::service::MultiKeyResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn patch_keys(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PatchKeysRequest, sink: ::grpcio::UnarySink<super::service::MultiKeyResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn my_scopes(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::MyScopesRequest, sink: ::grpcio::UnarySink<super::service::MultiScopeResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn my_scopes_user(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::MyScopesUserRequest, sink: ::grpcio::UnarySink<super::service::MultiScopeUserResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn my_scopes_root(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::MyScopesRootRequest, sink: ::grpcio::UnarySink<super::service::MultiScopeRootResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_scopes(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListScopesRequest, sink: ::grpcio::UnarySink<super::service::MultiScopeDepsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_app(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::GetAppRequest, sink: ::grpcio::UnarySink<super::service::SingleAppResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_apps(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListAppsRequest, sink: ::grpcio::UnarySink<super::service::MultiAppResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_app(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::DeleteAppRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_apps(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostAppsRequest, sink: ::grpcio::UnarySink<super::service::MultiAppResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn patch_apps(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PatchAppsRequest, sink: ::grpcio::UnarySink<super::service::MultiAppResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn patch_apps_ids(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PatchAppsIdsRequest, sink: ::grpcio::UnarySink<super::service::MultiAppResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn patch_app(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PatchAppRequest, sink: ::grpcio::UnarySink<super::service::SingleAppResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_apps_searches(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostAppsSearchesRequest, sink: ::grpcio::UnarySink<super::service::MultiAppResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_user(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::GetUserRequest, sink: ::grpcio::UnarySink<super::service::SingleUserResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_validate_password(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostValidatePasswordRequest, sink: ::grpcio::UnarySink<super::service::SinglePasswordValidationResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_search(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::GetSearchRequest, sink: ::grpcio::UnarySink<super::service::SingleSearchResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_searches(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListSearchesRequest, sink: ::grpcio::UnarySink<super::service::MultiSearchResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn patch_searches(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PatchSearchesRequest, sink: ::grpcio::UnarySink<super::service::MultiSearchResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_searches(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostSearchesRequest, sink: ::grpcio::UnarySink<super::service::MultiSearchResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_searches_by_id(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostSearchesByIDRequest, sink: ::grpcio::UnarySink<super::service::MultiSearchResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_annotation_search_metrics(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostAnnotationSearchMetricsRequest, sink: ::grpcio::UnarySink<super::service::MultiAnnotationSearchMetricsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_annotation_search_metrics(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::GetAnnotationSearchMetricsRequest, sink: ::grpcio::UnarySink<super::service::MultiAnnotationSearchMetricsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_annotation_search_metrics(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListAnnotationSearchMetricsRequest, sink: ::grpcio::UnarySink<super::service::MultiAnnotationSearchMetricsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_annotation_search_metrics(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::DeleteAnnotationSearchMetricsRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_search(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::DeleteSearchRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_annotation_filters(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListAnnotationFiltersRequest, sink: ::grpcio::UnarySink<super::service::MultiAnnotationFilterResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_annotation_filter(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::GetAnnotationFilterRequest, sink: ::grpcio::UnarySink<super::service::SingleAnnotationFilterResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_annotation_filters(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostAnnotationFiltersRequest, sink: ::grpcio::UnarySink<super::service::MultiAnnotationFilterResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn patch_annotation_filters(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PatchAnnotationFiltersRequest, sink: ::grpcio::UnarySink<super::service::MultiAnnotationFilterResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_annotation_filters(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::DeleteAnnotationFiltersRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_status_codes(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListStatusCodesRequest, sink: ::grpcio::UnarySink<super::service::MultiStatusCodeResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_status_code(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::GetStatusCodeRequest, sink: ::grpcio::UnarySink<super::service::SingleStatusCodeResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_resource_price(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::GetResourcePriceRequest, sink: ::grpcio::UnarySink<super::service::GetResourcePriceResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_collaborators(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListCollaboratorsRequest, sink: ::grpcio::UnarySink<super::service::MultiCollaboratorsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_collaborators(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostCollaboratorsRequest, sink: ::grpcio::UnarySink<super::service::MultiCollaboratorsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn patch_collaborators(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PatchCollaboratorsRequest, sink: ::grpcio::UnarySink<super::service::MultiCollaboratorsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_collaborators(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::DeleteCollaboratorsRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_collaborations(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListCollaborationsRequest, sink: ::grpcio::UnarySink<super::service::MultiCollaborationsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_app_duplications(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostAppDuplicationsRequest, sink: ::grpcio::UnarySink<super::service::MultiAppDuplicationsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_app_duplications(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListAppDuplicationsRequest, sink: ::grpcio::UnarySink<super::service::MultiAppDuplicationsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_app_duplication(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::GetAppDuplicationRequest, sink: ::grpcio::UnarySink<super::service::SingleAppDuplicationResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_tasks(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostTasksRequest, sink: ::grpcio::UnarySink<super::service::MultiTaskResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_task_annotation_count(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::GetTaskCountRequest, sink: ::grpcio::UnarySink<super::service::SingleTaskCountResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_task_input_count(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::GetTaskCountRequest, sink: ::grpcio::UnarySink<super::service::SingleTaskCountResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_task(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::GetTaskRequest, sink: ::grpcio::UnarySink<super::service::SingleTaskResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_tasks(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListTasksRequest, sink: ::grpcio::UnarySink<super::service::MultiTaskResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn patch_tasks(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PatchTasksRequest, sink: ::grpcio::UnarySink<super::service::MultiTaskResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_tasks(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::DeleteTasksRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_label_orders(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostLabelOrdersRequest, sink: ::grpcio::UnarySink<super::service::MultiLabelOrderResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_label_order(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::GetLabelOrderRequest, sink: ::grpcio::UnarySink<super::service::SingleLabelOrderResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_label_orders(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListLabelOrdersRequest, sink: ::grpcio::UnarySink<super::service::MultiLabelOrderResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn patch_label_orders(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PatchLabelOrdersRequest, sink: ::grpcio::UnarySink<super::service::MultiLabelOrderResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_label_orders(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::DeleteLabelOrdersRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_collectors(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostCollectorsRequest, sink: ::grpcio::UnarySink<super::service::MultiCollectorResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_collector(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::GetCollectorRequest, sink: ::grpcio::UnarySink<super::service::SingleCollectorResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_collectors(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListCollectorsRequest, sink: ::grpcio::UnarySink<super::service::MultiCollectorResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn patch_collectors(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PatchCollectorsRequest, sink: ::grpcio::UnarySink<super::service::MultiCollectorResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_collectors(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::DeleteCollectorsRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_stat_values(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostStatValuesRequest, sink: ::grpcio::UnarySink<super::service::MultiStatValueResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_stat_values_aggregate(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostStatValuesAggregateRequest, sink: ::grpcio::UnarySink<super::service::MultiStatValueAggregateResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_trending_metrics_view(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostTrendingMetricsViewRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_trending_metrics_views(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListTrendingMetricsViewsRequest, sink: ::grpcio::UnarySink<super::service::MultiTrendingMetricsViewResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_module(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::GetModuleRequest, sink: ::grpcio::UnarySink<super::service::SingleModuleResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_modules(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListModulesRequest, sink: ::grpcio::UnarySink<super::service::MultiModuleResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_modules(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostModulesRequest, sink: ::grpcio::UnarySink<super::service::MultiModuleResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn patch_modules(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PatchModulesRequest, sink: ::grpcio::UnarySink<super::service::MultiModuleResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_modules(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::DeleteModulesRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_module_version(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::GetModuleVersionRequest, sink: ::grpcio::UnarySink<super::service::SingleModuleVersionResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_module_versions(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListModuleVersionsRequest, sink: ::grpcio::UnarySink<super::service::MultiModuleVersionResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_module_versions(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostModuleVersionsRequest, sink: ::grpcio::UnarySink<super::service::MultiModuleVersionResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn patch_module_versions(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PatchModuleVersionsRequest, sink: ::grpcio::UnarySink<super::service::MultiModuleVersionResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_module_versions(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::DeleteModuleVersionsRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_module_version_usage_count(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::GetModuleVersionUsageCountRequest, sink: ::grpcio::UnarySink<super::service::SingleModuleVersionUsageCountResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_installed_module_version(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::GetInstalledModuleVersionRequest, sink: ::grpcio::UnarySink<super::service::SingleInstalledModuleVersionResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_installed_module_versions(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListInstalledModuleVersionsRequest, sink: ::grpcio::UnarySink<super::service::MultiInstalledModuleVersionResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_installed_module_versions(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostInstalledModuleVersionsRequest, sink: ::grpcio::UnarySink<super::service::MultiInstalledModuleVersionResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_installed_module_versions(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::DeleteInstalledModuleVersionsRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_installed_module_versions_key(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostInstalledModuleVersionsKeyRequest, sink: ::grpcio::UnarySink<super::service::SingleKeyResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_bulk_operations(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostBulkOperationsRequest, sink: ::grpcio::UnarySink<super::service::MultiBulkOperationsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_bulk_operations(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListBulkOperationsRequest, sink: ::grpcio::UnarySink<super::service::MultiBulkOperationsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_bulk_operation(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::GetBulkOperationRequest, sink: ::grpcio::UnarySink<super::service::SingleBulkOperationsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn cancel_bulk_operations(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::CancelBulkOperationRequest, sink: ::grpcio::UnarySink<super::service::MultiBulkOperationsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_bulk_operations(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::DeleteBulkOperationRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_next_task_assignments(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListNextTaskAssignmentsRequest, sink: ::grpcio::UnarySink<super::service::MultiInputResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn put_task_assignments(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PutTaskAssignmentsRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_inputs_add_jobs(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListInputsAddJobsRequest, sink: ::grpcio::UnarySink<super::service::MultiInputsAddJobResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_inputs_add_job(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::GetInputsAddJobRequest, sink: ::grpcio::UnarySink<super::service::SingleInputsAddJobResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn cancel_inputs_add_job(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::CancelInputsAddJobRequest, sink: ::grpcio::UnarySink<super::service::SingleInputsAddJobResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_uploads(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostUploadsRequest, sink: ::grpcio::UnarySink<super::service::MultiUploadResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn put_upload_content_parts(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PutUploadContentPartsRequest, sink: ::grpcio::UnarySink<super::service::SingleUploadResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_upload(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::GetUploadRequest, sink: ::grpcio::UnarySink<super::service::SingleUploadResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_uploads(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListUploadsRequest, sink: ::grpcio::UnarySink<super::service::MultiUploadResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_uploads(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::DeleteUploadsRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_inputs_data_sources(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostInputsDataSourcesRequest, sink: ::grpcio::UnarySink<super::service::MultiInputsAddJobResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_inputs_extraction_job(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::GetInputsExtractionJobRequest, sink: ::grpcio::UnarySink<super::service::SingleInputsExtractionJobResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_inputs_extraction_jobs(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListInputsExtractionJobsRequest, sink: ::grpcio::UnarySink<super::service::MultiInputsExtractionJobResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn cancel_inputs_extraction_jobs(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::CancelInputsExtractionJobsRequest, sink: ::grpcio::UnarySink<super::service::MultiInputsExtractionJobResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_inputs_uploads(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostInputsUploadsRequest, sink: ::grpcio::UnarySink<super::service::MultiInputsAddJobResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_runner(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::GetRunnerRequest, sink: ::grpcio::UnarySink<super::service::SingleRunnerResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_runners(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListRunnersRequest, sink: ::grpcio::UnarySink<super::service::MultiRunnerResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_runners(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostRunnersRequest, sink: ::grpcio::UnarySink<super::service::MultiRunnerResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_runners(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::DeleteRunnersRequest, sink: ::grpcio::UnarySink<super::status::BaseResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_runner_items(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::ListRunnerItemsRequest, sink: ::grpcio::UnarySink<super::service::MultiRunnerItemResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_runner_item_outputs(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostRunnerItemOutputsRequest, sink: ::grpcio::UnarySink<super::service::MultiRunnerItemOutputResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn post_model_versions_training_time_estimate(&mut self, ctx: ::grpcio::RpcContext, _req: super::service::PostModelVersionsTrainingTimeEstimateRequest, sink: ::grpcio::UnarySink<super::service::MultiTrainingTimeEstimateResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
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
    builder = builder.add_unary_handler(&METHOD_V2_LIST_MODEL_CONCEPTS, move |ctx, req, resp| {
        instance.list_model_concepts(ctx, req, resp)
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
    builder = builder.add_unary_handler(&METHOD_V2_PATCH_ANNOTATIONS_SEARCHES, move |ctx, req, resp| {
        instance.patch_annotations_searches(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_ANNOTATIONS_SEARCHES, move |ctx, req, resp| {
        instance.post_annotations_searches(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_LIST_ANNOTATION_WORKERS, move |ctx, req, resp| {
        instance.list_annotation_workers(ctx, req, resp)
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
    builder = builder.add_unary_handler(&METHOD_V2_GET_INPUT_VIDEO_MANIFEST, move |ctx, req, resp| {
        instance.get_input_video_manifest(ctx, req, resp)
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
    builder = builder.add_unary_handler(&METHOD_V2_PATCH_INPUTS_SEARCHES, move |ctx, req, resp| {
        instance.patch_inputs_searches(ctx, req, resp)
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
    builder = builder.add_unary_handler(&METHOD_V2_LIST_DATASETS, move |ctx, req, resp| {
        instance.list_datasets(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_GET_DATASET, move |ctx, req, resp| {
        instance.get_dataset(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_DATASETS, move |ctx, req, resp| {
        instance.post_datasets(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_PATCH_DATASETS, move |ctx, req, resp| {
        instance.patch_datasets(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_DELETE_DATASETS, move |ctx, req, resp| {
        instance.delete_datasets(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_LIST_DATASET_INPUTS, move |ctx, req, resp| {
        instance.list_dataset_inputs(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_GET_DATASET_INPUT, move |ctx, req, resp| {
        instance.get_dataset_input(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_DATASET_INPUTS, move |ctx, req, resp| {
        instance.post_dataset_inputs(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_DELETE_DATASET_INPUTS, move |ctx, req, resp| {
        instance.delete_dataset_inputs(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_LIST_DATASET_VERSIONS, move |ctx, req, resp| {
        instance.list_dataset_versions(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_GET_DATASET_VERSION, move |ctx, req, resp| {
        instance.get_dataset_version(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_LIST_DATASET_VERSION_METRICS_GROUPS, move |ctx, req, resp| {
        instance.list_dataset_version_metrics_groups(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_DATASET_VERSIONS, move |ctx, req, resp| {
        instance.post_dataset_versions(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_PATCH_DATASET_VERSIONS, move |ctx, req, resp| {
        instance.patch_dataset_versions(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_DELETE_DATASET_VERSIONS, move |ctx, req, resp| {
        instance.delete_dataset_versions(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_PUT_DATASET_VERSION_EXPORTS, move |ctx, req, resp| {
        instance.put_dataset_version_exports(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_GET_MODEL_TYPE, move |ctx, req, resp| {
        instance.get_model_type(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_LIST_OPEN_SOURCE_LICENSES, move |ctx, req, resp| {
        instance.list_open_source_licenses(ctx, req, resp)
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
    builder = builder.add_unary_handler(&METHOD_V2_GET_RESOURCE_COUNTS, move |ctx, req, resp| {
        instance.get_resource_counts(ctx, req, resp)
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
    builder = builder.add_unary_handler(&METHOD_V2_PATCH_MODEL_IDS, move |ctx, req, resp| {
        instance.patch_model_ids(ctx, req, resp)
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
    builder = builder.add_unary_handler(&METHOD_V2_PATCH_MODEL_CHECK_CONSENTS, move |ctx, req, resp| {
        instance.patch_model_check_consents(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_PATCH_MODEL_TOOLKITS, move |ctx, req, resp| {
        instance.patch_model_toolkits(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_PATCH_MODEL_USE_CASES, move |ctx, req, resp| {
        instance.patch_model_use_cases(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_PATCH_MODEL_LANGUAGES, move |ctx, req, resp| {
        instance.patch_model_languages(ctx, req, resp)
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
    builder = builder.add_unary_handler(&METHOD_V2_POST_WORKFLOW_VERSIONS_UN_PUBLISH, move |ctx, req, resp| {
        instance.post_workflow_versions_un_publish(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_WORKFLOW_VERSIONS_PUBLISH, move |ctx, req, resp| {
        instance.post_workflow_versions_publish(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_MODEL_VERSIONS_PUBLISH, move |ctx, req, resp| {
        instance.post_model_versions_publish(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_MODEL_VERSIONS_UN_PUBLISH, move |ctx, req, resp| {
        instance.post_model_versions_un_publish(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_MODEL_VERSIONS, move |ctx, req, resp| {
        instance.post_model_versions(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_PATCH_MODEL_VERSIONS, move |ctx, req, resp| {
        instance.patch_model_versions(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_DELETE_MODEL_VERSION, move |ctx, req, resp| {
        instance.delete_model_version(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_V2_POST_MODEL_VERSIONS_UPLOAD, move |ctx, req, resp| {
        instance.post_model_versions_upload(ctx, req, resp)
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
    builder = builder.add_unary_handler(&METHOD_V2_POST_MODEL_VERSION_EVALUATIONS, move |ctx, req, resp| {
        instance.post_model_version_evaluations(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_LIST_MODEL_VERSION_EVALUATIONS, move |ctx, req, resp| {
        instance.list_model_version_evaluations(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_GET_MODEL_VERSION_EVALUATION, move |ctx, req, resp| {
        instance.get_model_version_evaluation(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_EVALUATIONS, move |ctx, req, resp| {
        instance.post_evaluations(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_LIST_EVALUATIONS, move |ctx, req, resp| {
        instance.list_evaluations(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_GET_EVALUATION, move |ctx, req, resp| {
        instance.get_evaluation(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_LIST_MODEL_REFERENCES, move |ctx, req, resp| {
        instance.list_model_references(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_GET_MODEL_VERSION_INPUT_EXAMPLE, move |ctx, req, resp| {
        instance.get_model_version_input_example(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_LIST_MODEL_VERSION_INPUT_EXAMPLES, move |ctx, req, resp| {
        instance.list_model_version_input_examples(ctx, req, resp)
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
    builder = builder.add_unary_handler(&METHOD_V2_PATCH_WORKFLOW_IDS, move |ctx, req, resp| {
        instance.patch_workflow_ids(ctx, req, resp)
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
    builder = builder.add_unary_handler(&METHOD_V2_LIST_WORKFLOW_VERSIONS, move |ctx, req, resp| {
        instance.list_workflow_versions(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_GET_WORKFLOW_VERSION, move |ctx, req, resp| {
        instance.get_workflow_version(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_DELETE_WORKFLOW_VERSIONS, move |ctx, req, resp| {
        instance.delete_workflow_versions(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_PATCH_WORKFLOW_VERSIONS, move |ctx, req, resp| {
        instance.patch_workflow_versions(ctx, req, resp)
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
    builder = builder.add_unary_handler(&METHOD_V2_MY_SCOPES_USER, move |ctx, req, resp| {
        instance.my_scopes_user(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_MY_SCOPES_ROOT, move |ctx, req, resp| {
        instance.my_scopes_root(ctx, req, resp)
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
    builder = builder.add_unary_handler(&METHOD_V2_PATCH_APPS_IDS, move |ctx, req, resp| {
        instance.patch_apps_ids(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_PATCH_APP, move |ctx, req, resp| {
        instance.patch_app(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_APPS_SEARCHES, move |ctx, req, resp| {
        instance.post_apps_searches(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_GET_USER, move |ctx, req, resp| {
        instance.get_user(ctx, req, resp)
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
    builder = builder.add_unary_handler(&METHOD_V2_PATCH_SEARCHES, move |ctx, req, resp| {
        instance.patch_searches(ctx, req, resp)
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
    builder = builder.add_unary_handler(&METHOD_V2_LIST_ANNOTATION_FILTERS, move |ctx, req, resp| {
        instance.list_annotation_filters(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_GET_ANNOTATION_FILTER, move |ctx, req, resp| {
        instance.get_annotation_filter(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_ANNOTATION_FILTERS, move |ctx, req, resp| {
        instance.post_annotation_filters(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_PATCH_ANNOTATION_FILTERS, move |ctx, req, resp| {
        instance.patch_annotation_filters(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_DELETE_ANNOTATION_FILTERS, move |ctx, req, resp| {
        instance.delete_annotation_filters(ctx, req, resp)
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
    builder = builder.add_unary_handler(&METHOD_V2_GET_RESOURCE_PRICE, move |ctx, req, resp| {
        instance.get_resource_price(ctx, req, resp)
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
    builder = builder.add_unary_handler(&METHOD_V2_POST_LABEL_ORDERS, move |ctx, req, resp| {
        instance.post_label_orders(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_GET_LABEL_ORDER, move |ctx, req, resp| {
        instance.get_label_order(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_LIST_LABEL_ORDERS, move |ctx, req, resp| {
        instance.list_label_orders(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_PATCH_LABEL_ORDERS, move |ctx, req, resp| {
        instance.patch_label_orders(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_DELETE_LABEL_ORDERS, move |ctx, req, resp| {
        instance.delete_label_orders(ctx, req, resp)
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
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_STAT_VALUES_AGGREGATE, move |ctx, req, resp| {
        instance.post_stat_values_aggregate(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_TRENDING_METRICS_VIEW, move |ctx, req, resp| {
        instance.post_trending_metrics_view(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_LIST_TRENDING_METRICS_VIEWS, move |ctx, req, resp| {
        instance.list_trending_metrics_views(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_GET_MODULE, move |ctx, req, resp| {
        instance.get_module(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_LIST_MODULES, move |ctx, req, resp| {
        instance.list_modules(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_MODULES, move |ctx, req, resp| {
        instance.post_modules(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_PATCH_MODULES, move |ctx, req, resp| {
        instance.patch_modules(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_DELETE_MODULES, move |ctx, req, resp| {
        instance.delete_modules(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_GET_MODULE_VERSION, move |ctx, req, resp| {
        instance.get_module_version(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_LIST_MODULE_VERSIONS, move |ctx, req, resp| {
        instance.list_module_versions(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_MODULE_VERSIONS, move |ctx, req, resp| {
        instance.post_module_versions(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_PATCH_MODULE_VERSIONS, move |ctx, req, resp| {
        instance.patch_module_versions(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_DELETE_MODULE_VERSIONS, move |ctx, req, resp| {
        instance.delete_module_versions(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_GET_MODULE_VERSION_USAGE_COUNT, move |ctx, req, resp| {
        instance.get_module_version_usage_count(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_GET_INSTALLED_MODULE_VERSION, move |ctx, req, resp| {
        instance.get_installed_module_version(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_LIST_INSTALLED_MODULE_VERSIONS, move |ctx, req, resp| {
        instance.list_installed_module_versions(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_INSTALLED_MODULE_VERSIONS, move |ctx, req, resp| {
        instance.post_installed_module_versions(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_DELETE_INSTALLED_MODULE_VERSIONS, move |ctx, req, resp| {
        instance.delete_installed_module_versions(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_INSTALLED_MODULE_VERSIONS_KEY, move |ctx, req, resp| {
        instance.post_installed_module_versions_key(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_BULK_OPERATIONS, move |ctx, req, resp| {
        instance.post_bulk_operations(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_LIST_BULK_OPERATIONS, move |ctx, req, resp| {
        instance.list_bulk_operations(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_GET_BULK_OPERATION, move |ctx, req, resp| {
        instance.get_bulk_operation(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_CANCEL_BULK_OPERATIONS, move |ctx, req, resp| {
        instance.cancel_bulk_operations(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_DELETE_BULK_OPERATIONS, move |ctx, req, resp| {
        instance.delete_bulk_operations(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_LIST_NEXT_TASK_ASSIGNMENTS, move |ctx, req, resp| {
        instance.list_next_task_assignments(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_PUT_TASK_ASSIGNMENTS, move |ctx, req, resp| {
        instance.put_task_assignments(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_LIST_INPUTS_ADD_JOBS, move |ctx, req, resp| {
        instance.list_inputs_add_jobs(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_GET_INPUTS_ADD_JOB, move |ctx, req, resp| {
        instance.get_inputs_add_job(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_CANCEL_INPUTS_ADD_JOB, move |ctx, req, resp| {
        instance.cancel_inputs_add_job(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_UPLOADS, move |ctx, req, resp| {
        instance.post_uploads(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_PUT_UPLOAD_CONTENT_PARTS, move |ctx, req, resp| {
        instance.put_upload_content_parts(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_GET_UPLOAD, move |ctx, req, resp| {
        instance.get_upload(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_LIST_UPLOADS, move |ctx, req, resp| {
        instance.list_uploads(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_DELETE_UPLOADS, move |ctx, req, resp| {
        instance.delete_uploads(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_INPUTS_DATA_SOURCES, move |ctx, req, resp| {
        instance.post_inputs_data_sources(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_GET_INPUTS_EXTRACTION_JOB, move |ctx, req, resp| {
        instance.get_inputs_extraction_job(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_LIST_INPUTS_EXTRACTION_JOBS, move |ctx, req, resp| {
        instance.list_inputs_extraction_jobs(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_CANCEL_INPUTS_EXTRACTION_JOBS, move |ctx, req, resp| {
        instance.cancel_inputs_extraction_jobs(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_INPUTS_UPLOADS, move |ctx, req, resp| {
        instance.post_inputs_uploads(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_GET_RUNNER, move |ctx, req, resp| {
        instance.get_runner(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_LIST_RUNNERS, move |ctx, req, resp| {
        instance.list_runners(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_RUNNERS, move |ctx, req, resp| {
        instance.post_runners(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_DELETE_RUNNERS, move |ctx, req, resp| {
        instance.delete_runners(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_LIST_RUNNER_ITEMS, move |ctx, req, resp| {
        instance.list_runner_items(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V2_POST_RUNNER_ITEM_OUTPUTS, move |ctx, req, resp| {
        instance.post_runner_item_outputs(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_V2_POST_MODEL_VERSIONS_TRAINING_TIME_ESTIMATE, move |ctx, req, resp| {
        instance.post_model_versions_training_time_estimate(ctx, req, resp)
    });
    builder.build()
}
