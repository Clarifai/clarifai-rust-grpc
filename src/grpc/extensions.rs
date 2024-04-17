// This file is generated by rust-protobuf 3.4.0. Do not edit
// .proto file is parsed by protoc --rust-out=...
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `proto/clarifai/api/utils/extensions.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

/// Extension fields
pub mod exts {

    pub const cl_show_if_empty: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(50000, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const cl_moretags: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, ::std::string::String> = ::protobuf::ext::ExtFieldOptional::new(50001, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_STRING);

    pub const cl_default_float: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, f32> = ::protobuf::ext::ExtFieldOptional::new(50010, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_FLOAT);
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n)proto/clarifai/api/utils/extensions.proto\x12\x12clarifai.api.utils\
    \x1a\x20google/protobuf/descriptor.proto:H\n\x10cl_show_if_empty\x18\xd0\
    \x86\x03\x20\x01(\x08\x12\x1d.google.protobuf.FieldOptionsR\rclShowIfEmp\
    ty:@\n\x0bcl_moretags\x18\xd1\x86\x03\x20\x01(\t\x12\x1d.google.protobuf\
    .FieldOptionsR\nclMoretags:I\n\x10cl_default_float\x18\xda\x86\x03\x20\
    \x01(\x02\x12\x1d.google.protobuf.FieldOptionsR\x0eclDefaultFloatBe\n\
    \x1bcom.clarifai.grpc.api.utilsP\x01Z=github.com/Clarifai/clarifai-go-gr\
    pc/proto/clarifai/api/utils\xa2\x02\x04CAIPJ\x90\x07\n\x06\x12\x04\0\0\
    \x18\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\t\n\x02\x03\0\x12\x03\x02\0*\
    \n\x08\n\x01\x02\x12\x03\x04\0\x1b\n\x08\n\x01\x08\x12\x03\x06\0T\n\t\n\
    \x02\x08\x0b\x12\x03\x06\0T\n\x08\n\x01\x08\x12\x03\x07\0\"\n\t\n\x02\
    \x08\n\x12\x03\x07\0\"\n\x08\n\x01\x08\x12\x03\x08\04\n\t\n\x02\x08\x01\
    \x12\x03\x08\04\n\x08\n\x01\x08\x12\x03\t\0\"\n\t\n\x02\x08$\x12\x03\t\0\
    \"\n\t\n\x01\x07\x12\x04\x0b\0\x18\x01\n\x8c\x02\n\x02\x07\0\x12\x03\x0f\
    \x02\x20\x1a\x80\x02\x20If\x20True\x20then\x20we\x20will\x20return\x20th\
    is\x20field\x20with\x20it's\x20zero\x20value\x20even\x20if\x20not\x20set\
    .\n\x20This\x20means\x20in\x20json\x20responses\x20empty\x20lists\x20wil\
    l\x20appear\x20instead\x20of\x20not\x20being\x20returned\x20(which\x20is\
    \n\x20the\x20default\x20convention\x20for\x20proto3).\x20For\x20int's\
    \x20we\x20will\x20show\x200,\x20for\x20floats\x200.0,\x20etc.\n\n\n\n\
    \x03\x07\0\x02\x12\x03\x0b\x07#\n\n\n\x03\x07\0\x05\x12\x03\x0f\x02\x06\
    \n\n\n\x03\x07\0\x01\x12\x03\x0f\x07\x17\n\n\n\x03\x07\0\x03\x12\x03\x0f\
    \x1a\x1f\n\t\n\x02\x07\x01\x12\x03\x11\x02\x1d\n\n\n\x03\x07\x01\x02\x12\
    \x03\x0b\x07#\n\n\n\x03\x07\x01\x05\x12\x03\x11\x02\x08\n\n\n\x03\x07\
    \x01\x01\x12\x03\x11\t\x14\n\n\n\x03\x07\x01\x03\x12\x03\x11\x17\x1c\n\
    \xdd\x02\n\x02\x07\x02\x12\x03\x17\x02!\x1a\xd1\x02\x20For\x20float\x20f\
    ields\x20where\x20this\x20is\x20set,\x20this\x20value\x20will\x20be\x20u\
    sed\x20by\x20the\x20server\x20when\x20parsing\x20the\n\x20request\x20and\
    \x20the\x20field\x20is\x20not\x20present\x20in\x20the\x20request.\x20If\
    \x20the\x20field\x20is\x20present\x20in\x20the\x20request,\n\x20then\x20\
    the\x20value\x20of\x20the\x20field\x20will\x20be\x20used\x20instead.\x20\
    This\x20is\x20ONLY\x20used\x20for\x20json\x20requests\x20as\x20binary\n\
    \x20proto\x20requests\x20are\x20expected\x20to\x20always\x20set\x20the\
    \x20field.\n\n\n\n\x03\x07\x02\x02\x12\x03\x0b\x07#\n\n\n\x03\x07\x02\
    \x05\x12\x03\x17\x02\x07\n\n\n\x03\x07\x02\x01\x12\x03\x17\x08\x18\n\n\n\
    \x03\x07\x02\x03\x12\x03\x17\x1b\x20b\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(::protobuf::descriptor::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(0);
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
