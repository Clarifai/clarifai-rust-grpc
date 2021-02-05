// This file is generated by rust-protobuf 2.20.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![rustfmt::skip]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `proto/clarifai/auth/types/types.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_20_0;

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum AuthType {
    undef = 0,
    NoAuth = 1,
    KeyAuth = 2,
    SessionTokenAuth = 3,
    AdminAuth = 4,
    PATAuth = 5,
}

impl ::protobuf::ProtobufEnum for AuthType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<AuthType> {
        match value {
            0 => ::std::option::Option::Some(AuthType::undef),
            1 => ::std::option::Option::Some(AuthType::NoAuth),
            2 => ::std::option::Option::Some(AuthType::KeyAuth),
            3 => ::std::option::Option::Some(AuthType::SessionTokenAuth),
            4 => ::std::option::Option::Some(AuthType::AdminAuth),
            5 => ::std::option::Option::Some(AuthType::PATAuth),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [AuthType] = &[
            AuthType::undef,
            AuthType::NoAuth,
            AuthType::KeyAuth,
            AuthType::SessionTokenAuth,
            AuthType::AdminAuth,
            AuthType::PATAuth,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<AuthType>("AuthType", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for AuthType {
}

impl ::std::default::Default for AuthType {
    fn default() -> Self {
        AuthType::undef
    }
}

impl ::protobuf::reflect::ProtobufValue for AuthType {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n%proto/clarifai/auth/types/types.proto\x12\x13clarifai.auth.types*`\n\
    \x08AuthType\x12\t\n\x05undef\x10\0\x12\n\n\x06NoAuth\x10\x01\x12\x0b\n\
    \x07KeyAuth\x10\x02\x12\x14\n\x10SessionTokenAuth\x10\x03\x12\r\n\tAdmin\
    Auth\x10\x04\x12\x0b\n\x07PATAuth\x10\x05Bg\n\x1ccom.clarifai.grpc.auth.\
    typesP\x01Z>github.com/Clarifai/clarifai-go-grpc/proto/clarifai/auth/typ\
    es\xa2\x02\x04CAIPJ\xe5\t\n\x06\x12\x04\0\0#\x01\n\x08\n\x01\x0c\x12\x03\
    \0\0\x12\n\x08\n\x01\x02\x12\x03\x02\0\x1c\n\x08\n\x01\x08\x12\x03\x04\0\
    U\n\t\n\x02\x08\x0b\x12\x03\x04\0U\n\x08\n\x01\x08\x12\x03\x05\0\"\n\t\n\
    \x02\x08\n\x12\x03\x05\0\"\n\x08\n\x01\x08\x12\x03\x06\05\n\t\n\x02\x08\
    \x01\x12\x03\x06\05\n\x08\n\x01\x08\x12\x03\x07\0\"\n\t\n\x02\x08$\x12\
    \x03\x07\0\"\n/\n\x02\x05\0\x12\x04\n\0#\x01\x1a#\x20Authorization\x20ty\
    pe\x20for\x20endpoints.\n\n\n\n\x03\x05\0\x01\x12\x03\n\x05\r\n\xcb\x01\
    \n\x04\x05\0\x02\0\x12\x03\x0e\x02\x0c\x1a\xbd\x01\x20introduce\x20undef\
    \x20so\x20that\x20the\x20zero\x20(default/unset)\x20value\x20of\x20the\
    \x20enum\x20is\x20not\x20a\x20real\n\x20permission.\x20\x20undef\x20is\
    \x20only\x20present\x20for\x20this\x20purpose\x20and\x20should\x20not\
    \x20be\x20used\n\x20to\x20indicate\x20any\x20\"real\"\x20value.\n\n\x0c\
    \n\x05\x05\0\x02\0\x01\x12\x03\x0e\x02\x07\n\x0c\n\x05\x05\0\x02\0\x02\
    \x12\x03\x0e\n\x0b\n7\n\x04\x05\0\x02\x01\x12\x03\x11\x02\r\x1a*\x20No\
    \x20authorization\x20need\x20for\x20this\x20endpoint.\n\n\x0c\n\x05\x05\
    \0\x02\x01\x01\x12\x03\x11\x02\x08\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03\
    \x11\x0b\x0c\n\xd4\x01\n\x04\x05\0\x02\x02\x12\x03\x16\x02\x0e\x1a\xc6\
    \x01\x20This\x20authorization\x20requires\x20API\x20keys\x20(both\x20app\
    -spceific\x20keys\x20and\x20personal\x20access\x20tokens).\n\x20The\x20e\
    ndpoints\x20that\x20use\x20this\x20AuthType\x20may\x20also\x20include\
    \x20a\x20list\x20of\n\x20clarifai.auth.utils.cl_depending_scopes.\n\n\
    \x0c\n\x05\x05\0\x02\x02\x01\x12\x03\x16\x02\t\n\x0c\n\x05\x05\0\x02\x02\
    \x02\x12\x03\x16\x0c\r\n\x92\x01\n\x04\x05\0\x02\x03\x12\x03\x1a\x02\x17\
    \x1a\x84\x01\x20This\x20uses\x20a\x20session\x20token\x20from\x20your\
    \x20web\x20browser.\x20This\x20is\x20reserved\x20for\x20users/account\
    \x20level\x20APIs\n\x20that\x20are\x20only\x20needed\x20in\x20a\x20brows\
    er.\n\n\x0c\n\x05\x05\0\x02\x03\x01\x12\x03\x1a\x02\x12\n\x0c\n\x05\x05\
    \0\x02\x03\x02\x12\x03\x1a\x15\x16\nF\n\x04\x05\0\x02\x04\x12\x03\x1d\
    \x02\x10\x1a9\x20This\x20uses\x20a\x20special\x20token\x20for\x20admin\
    \x20access\x20to\x20the\x20APIs.\n\n\x0c\n\x05\x05\0\x02\x04\x01\x12\x03\
    \x1d\x02\x0b\n\x0c\n\x05\x05\0\x02\x04\x02\x12\x03\x1d\x0e\x0f\n\xd2\x01\
    \n\x04\x05\0\x02\x05\x12\x03\"\x02\x0e\x1a\xc4\x01\x20This\x20authorizat\
    ion\x20requires\x20personal\x20access\x20tokens.\x20This\x20is\x20used\
    \x20for\x20endpoints\x20such\x20as\n\x20/users/{user_id}/apps\x20which\
    \x20are\x20not\x20specific.\x20An\x20app-specific\x20API\x20key\x20will\
    \x20not\x20work\n\x20when\x20PATAuth\x20is\x20used.\n\n\x0c\n\x05\x05\0\
    \x02\x05\x01\x12\x03\"\x02\t\n\x0c\n\x05\x05\0\x02\x05\x02\x12\x03\"\x0c\
    \rb\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
