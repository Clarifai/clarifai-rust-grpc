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
//! Generated file from `proto/clarifai/api/utils/test_proto.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_20_0;

#[derive(PartialEq,Clone,Default)]
pub struct TestProto {
    // message fields
    pub id: ::std::string::String,
    pub message: ::std::string::String,
    pub value: f64,
    pub image_bytes: ::std::vec::Vec<u8>,
    // message oneof groups
    pub one_of_field: ::std::option::Option<TestProto_oneof_one_of_field>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a TestProto {
    fn default() -> &'a TestProto {
        <TestProto as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum TestProto_oneof_one_of_field {
    string_oneof(::std::string::String),
    bool_oneof(bool),
    message_oneof(TestProto2),
}

impl TestProto {
    pub fn new() -> TestProto {
        ::std::default::Default::default()
    }

    // string id = 1;


    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.id, ::std::string::String::new())
    }

    // string message = 2;


    pub fn get_message(&self) -> &str {
        &self.message
    }
    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut ::std::string::String {
        &mut self.message
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.message, ::std::string::String::new())
    }

    // double value = 3;


    pub fn get_value(&self) -> f64 {
        self.value
    }
    pub fn clear_value(&mut self) {
        self.value = 0.;
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: f64) {
        self.value = v;
    }

    // bytes image_bytes = 4;


    pub fn get_image_bytes(&self) -> &[u8] {
        &self.image_bytes
    }
    pub fn clear_image_bytes(&mut self) {
        self.image_bytes.clear();
    }

    // Param is passed by value, moved
    pub fn set_image_bytes(&mut self, v: ::std::vec::Vec<u8>) {
        self.image_bytes = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_image_bytes(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.image_bytes
    }

    // Take field
    pub fn take_image_bytes(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.image_bytes, ::std::vec::Vec::new())
    }

    // string string_oneof = 5;


    pub fn get_string_oneof(&self) -> &str {
        match self.one_of_field {
            ::std::option::Option::Some(TestProto_oneof_one_of_field::string_oneof(ref v)) => v,
            _ => "",
        }
    }
    pub fn clear_string_oneof(&mut self) {
        self.one_of_field = ::std::option::Option::None;
    }

    pub fn has_string_oneof(&self) -> bool {
        match self.one_of_field {
            ::std::option::Option::Some(TestProto_oneof_one_of_field::string_oneof(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_string_oneof(&mut self, v: ::std::string::String) {
        self.one_of_field = ::std::option::Option::Some(TestProto_oneof_one_of_field::string_oneof(v))
    }

    // Mutable pointer to the field.
    pub fn mut_string_oneof(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(TestProto_oneof_one_of_field::string_oneof(_)) = self.one_of_field {
        } else {
            self.one_of_field = ::std::option::Option::Some(TestProto_oneof_one_of_field::string_oneof(::std::string::String::new()));
        }
        match self.one_of_field {
            ::std::option::Option::Some(TestProto_oneof_one_of_field::string_oneof(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_string_oneof(&mut self) -> ::std::string::String {
        if self.has_string_oneof() {
            match self.one_of_field.take() {
                ::std::option::Option::Some(TestProto_oneof_one_of_field::string_oneof(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    // bool bool_oneof = 6;


    pub fn get_bool_oneof(&self) -> bool {
        match self.one_of_field {
            ::std::option::Option::Some(TestProto_oneof_one_of_field::bool_oneof(v)) => v,
            _ => false,
        }
    }
    pub fn clear_bool_oneof(&mut self) {
        self.one_of_field = ::std::option::Option::None;
    }

    pub fn has_bool_oneof(&self) -> bool {
        match self.one_of_field {
            ::std::option::Option::Some(TestProto_oneof_one_of_field::bool_oneof(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_bool_oneof(&mut self, v: bool) {
        self.one_of_field = ::std::option::Option::Some(TestProto_oneof_one_of_field::bool_oneof(v))
    }

    // .clarifai.api.utils.TestProto2 message_oneof = 7;


    pub fn get_message_oneof(&self) -> &TestProto2 {
        match self.one_of_field {
            ::std::option::Option::Some(TestProto_oneof_one_of_field::message_oneof(ref v)) => v,
            _ => <TestProto2 as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn clear_message_oneof(&mut self) {
        self.one_of_field = ::std::option::Option::None;
    }

    pub fn has_message_oneof(&self) -> bool {
        match self.one_of_field {
            ::std::option::Option::Some(TestProto_oneof_one_of_field::message_oneof(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_message_oneof(&mut self, v: TestProto2) {
        self.one_of_field = ::std::option::Option::Some(TestProto_oneof_one_of_field::message_oneof(v))
    }

    // Mutable pointer to the field.
    pub fn mut_message_oneof(&mut self) -> &mut TestProto2 {
        if let ::std::option::Option::Some(TestProto_oneof_one_of_field::message_oneof(_)) = self.one_of_field {
        } else {
            self.one_of_field = ::std::option::Option::Some(TestProto_oneof_one_of_field::message_oneof(TestProto2::new()));
        }
        match self.one_of_field {
            ::std::option::Option::Some(TestProto_oneof_one_of_field::message_oneof(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_message_oneof(&mut self) -> TestProto2 {
        if self.has_message_oneof() {
            match self.one_of_field.take() {
                ::std::option::Option::Some(TestProto_oneof_one_of_field::message_oneof(v)) => v,
                _ => panic!(),
            }
        } else {
            TestProto2::new()
        }
    }
}

impl ::protobuf::Message for TestProto {
    fn is_initialized(&self) -> bool {
        if let Some(TestProto_oneof_one_of_field::message_oneof(ref v)) = self.one_of_field {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.message)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.value = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.image_bytes)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.one_of_field = ::std::option::Option::Some(TestProto_oneof_one_of_field::string_oneof(is.read_string()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.one_of_field = ::std::option::Option::Some(TestProto_oneof_one_of_field::bool_oneof(is.read_bool()?));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.one_of_field = ::std::option::Option::Some(TestProto_oneof_one_of_field::message_oneof(is.read_message()?));
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.id);
        }
        if !self.message.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.message);
        }
        if self.value != 0. {
            my_size += 9;
        }
        if !self.image_bytes.is_empty() {
            my_size += ::protobuf::rt::bytes_size(4, &self.image_bytes);
        }
        if let ::std::option::Option::Some(ref v) = self.one_of_field {
            match v {
                &TestProto_oneof_one_of_field::string_oneof(ref v) => {
                    my_size += ::protobuf::rt::string_size(5, &v);
                },
                &TestProto_oneof_one_of_field::bool_oneof(v) => {
                    my_size += 2;
                },
                &TestProto_oneof_one_of_field::message_oneof(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.id.is_empty() {
            os.write_string(1, &self.id)?;
        }
        if !self.message.is_empty() {
            os.write_string(2, &self.message)?;
        }
        if self.value != 0. {
            os.write_double(3, self.value)?;
        }
        if !self.image_bytes.is_empty() {
            os.write_bytes(4, &self.image_bytes)?;
        }
        if let ::std::option::Option::Some(ref v) = self.one_of_field {
            match v {
                &TestProto_oneof_one_of_field::string_oneof(ref v) => {
                    os.write_string(5, v)?;
                },
                &TestProto_oneof_one_of_field::bool_oneof(v) => {
                    os.write_bool(6, v)?;
                },
                &TestProto_oneof_one_of_field::message_oneof(ref v) => {
                    os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> TestProto {
        TestProto::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "id",
                |m: &TestProto| { &m.id },
                |m: &mut TestProto| { &mut m.id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "message",
                |m: &TestProto| { &m.message },
                |m: &mut TestProto| { &mut m.message },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                "value",
                |m: &TestProto| { &m.value },
                |m: &mut TestProto| { &mut m.value },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "image_bytes",
                |m: &TestProto| { &m.image_bytes },
                |m: &mut TestProto| { &mut m.image_bytes },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                "string_oneof",
                TestProto::has_string_oneof,
                TestProto::get_string_oneof,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor::<_>(
                "bool_oneof",
                TestProto::has_bool_oneof,
                TestProto::get_bool_oneof,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, TestProto2>(
                "message_oneof",
                TestProto::has_message_oneof,
                TestProto::get_message_oneof,
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<TestProto>(
                "TestProto",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static TestProto {
        static instance: ::protobuf::rt::LazyV2<TestProto> = ::protobuf::rt::LazyV2::INIT;
        instance.get(TestProto::new)
    }
}

impl ::protobuf::Clear for TestProto {
    fn clear(&mut self) {
        self.id.clear();
        self.message.clear();
        self.value = 0.;
        self.image_bytes.clear();
        self.one_of_field = ::std::option::Option::None;
        self.one_of_field = ::std::option::Option::None;
        self.one_of_field = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TestProto2 {
    // message fields
    pub id: ::std::string::String,
    pub flip: bool,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a TestProto2 {
    fn default() -> &'a TestProto2 {
        <TestProto2 as ::protobuf::Message>::default_instance()
    }
}

impl TestProto2 {
    pub fn new() -> TestProto2 {
        ::std::default::Default::default()
    }

    // string id = 1;


    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.id, ::std::string::String::new())
    }

    // bool flip = 2;


    pub fn get_flip(&self) -> bool {
        self.flip
    }
    pub fn clear_flip(&mut self) {
        self.flip = false;
    }

    // Param is passed by value, moved
    pub fn set_flip(&mut self, v: bool) {
        self.flip = v;
    }
}

impl ::protobuf::Message for TestProto2 {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.flip = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.id);
        }
        if self.flip != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.id.is_empty() {
            os.write_string(1, &self.id)?;
        }
        if self.flip != false {
            os.write_bool(2, self.flip)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> TestProto2 {
        TestProto2::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "id",
                |m: &TestProto2| { &m.id },
                |m: &mut TestProto2| { &mut m.id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "flip",
                |m: &TestProto2| { &m.flip },
                |m: &mut TestProto2| { &mut m.flip },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<TestProto2>(
                "TestProto2",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static TestProto2 {
        static instance: ::protobuf::rt::LazyV2<TestProto2> = ::protobuf::rt::LazyV2::INIT;
        instance.get(TestProto2::new)
    }
}

impl ::protobuf::Clear for TestProto2 {
    fn clear(&mut self) {
        self.id.clear();
        self.flip = false;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TestProto2 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TestProto2 {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n)proto/clarifai/api/utils/test_proto.proto\x12\x12clarifai.api.utils\
    \x1a)proto/clarifai/api/utils/extensions.proto\"\xab\x02\n\tTestProto\
    \x12\x0e\n\x02id\x18\x01\x20\x01(\tR\x02id\x121\n\x07message\x18\x02\x20\
    \x01(\tR\x07messageB\x17\x80\xb5\x18\x01\x8a\xb5\x18\x0fprotos\x20are\
    \x20cool\x12\x1d\n\x05value\x18\x03\x20\x01(\x01R\x05valueB\x07\xd5\xb5\
    \x18\0\0\x80?\x12\x1f\n\x0bimage_bytes\x18\x04\x20\x01(\x0cR\nimageBytes\
    \x12#\n\x0cstring_oneof\x18\x05\x20\x01(\tH\0R\x0bstringOneof\x12\x1f\n\
    \nbool_oneof\x18\x06\x20\x01(\x08H\0R\tboolOneof\x12E\n\rmessage_oneof\
    \x18\x07\x20\x01(\x0b2\x1e.clarifai.api.utils.TestProto2H\0R\x0cmessageO\
    neofB\x0e\n\x0cone_of_field\"0\n\nTestProto2\x12\x0e\n\x02id\x18\x01\x20\
    \x01(\tR\x02id\x12\x12\n\x04flip\x18\x02\x20\x01(\x08R\x04flipB\x9d\x01\
    \n\x1bcom.clarifai.grpc.api.utilsP\x01Zugithub.com/Clarifai/clarifai-go-\
    grpc/proto/clarifai/api/github.com/Clarifai/clarifai-go-grpc/proto/clari\
    fai/api/utils\xa2\x02\x04CAIPJ\x87\x06\n\x06\x12\x04\0\0\x1c\x01\n\x08\n\
    \x01\x0c\x12\x03\0\0\x12\n\t\n\x02\x03\0\x12\x03\x02\03\n\x08\n\x01\x02\
    \x12\x03\x04\0\x1b\n\t\n\x01\x08\x12\x04\x06\0\x8c\x01\n\n\n\x02\x08\x0b\
    \x12\x04\x06\0\x8c\x01\n\x08\n\x01\x08\x12\x03\x07\0\"\n\t\n\x02\x08\n\
    \x12\x03\x07\0\"\n\x08\n\x01\x08\x12\x03\x08\04\n\t\n\x02\x08\x01\x12\
    \x03\x08\04\n\x08\n\x01\x08\x12\x03\t\0\"\n\t\n\x02\x08$\x12\x03\t\0\"\n\
    \n\n\x02\x04\0\x12\x04\x0b\0\x17\x01\n\n\n\x03\x04\0\x01\x12\x03\x0b\x08\
    \x11\n\x0b\n\x04\x04\0\x02\0\x12\x03\x0c\x02\x10\n\x0c\n\x05\x04\0\x02\0\
    \x05\x12\x03\x0c\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x0c\t\x0b\n\
    \x0c\n\x05\x04\0\x02\0\x03\x12\x03\x0c\x0e\x0f\n\x0b\n\x04\x04\0\x02\x01\
    \x12\x03\r\x02`\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\r\x02\x08\n\x0c\n\
    \x05\x04\0\x02\x01\x01\x12\x03\r\t\x10\n\x0c\n\x05\x04\0\x02\x01\x03\x12\
    \x03\r\x13\x14\n\x0c\n\x05\x04\0\x02\x01\x08\x12\x03\r\x15_\n\x0f\n\x08\
    \x04\0\x02\x01\x08\xd0\x86\x03\x12\x03\r\x165\n\x0f\n\x08\x04\0\x02\x01\
    \x08\xd1\x86\x03\x12\x03\r7^\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x0e\x024\
    \n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x0e\x02\x08\n\x0c\n\x05\x04\0\x02\
    \x02\x01\x12\x03\x0e\t\x0e\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x0e\x11\
    \x12\n\x0c\n\x05\x04\0\x02\x02\x08\x12\x03\x0e\x133\n\x0f\n\x08\x04\0\
    \x02\x02\x08\xda\x86\x03\x12\x03\x0e\x142\n\x0b\n\x04\x04\0\x02\x03\x12\
    \x03\x0f\x02\x18\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03\x0f\x02\x07\n\x0c\
    \n\x05\x04\0\x02\x03\x01\x12\x03\x0f\x08\x13\n\x0c\n\x05\x04\0\x02\x03\
    \x03\x12\x03\x0f\x16\x17\n\x0c\n\x04\x04\0\x08\0\x12\x04\x12\x02\x16\x03\
    \n\x0c\n\x05\x04\0\x08\0\x01\x12\x03\x12\x08\x14\n\x0b\n\x04\x04\0\x02\
    \x04\x12\x03\x13\x04\x1c\n\x0c\n\x05\x04\0\x02\x04\x05\x12\x03\x13\x04\n\
    \n\x0c\n\x05\x04\0\x02\x04\x01\x12\x03\x13\x0b\x17\n\x0c\n\x05\x04\0\x02\
    \x04\x03\x12\x03\x13\x1a\x1b\n\x0b\n\x04\x04\0\x02\x05\x12\x03\x14\x04\
    \x18\n\x0c\n\x05\x04\0\x02\x05\x05\x12\x03\x14\x04\x08\n\x0c\n\x05\x04\0\
    \x02\x05\x01\x12\x03\x14\t\x13\n\x0c\n\x05\x04\0\x02\x05\x03\x12\x03\x14\
    \x16\x17\n\x0b\n\x04\x04\0\x02\x06\x12\x03\x15\x04!\n\x0c\n\x05\x04\0\
    \x02\x06\x06\x12\x03\x15\x04\x0e\n\x0c\n\x05\x04\0\x02\x06\x01\x12\x03\
    \x15\x0f\x1c\n\x0c\n\x05\x04\0\x02\x06\x03\x12\x03\x15\x1f\x20\n\n\n\x02\
    \x04\x01\x12\x04\x19\0\x1c\x01\n\n\n\x03\x04\x01\x01\x12\x03\x19\x08\x12\
    \n\x0b\n\x04\x04\x01\x02\0\x12\x03\x1a\x02\x10\n\x0c\n\x05\x04\x01\x02\0\
    \x05\x12\x03\x1a\x02\x08\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x1a\t\x0b\
    \n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x1a\x0e\x0f\n\x0b\n\x04\x04\x01\
    \x02\x01\x12\x03\x1b\x02\x10\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\x1b\
    \x02\x06\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x1b\x07\x0b\n\x0c\n\x05\
    \x04\x01\x02\x01\x03\x12\x03\x1b\x0e\x0fb\x06proto3\
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
