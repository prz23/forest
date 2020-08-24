// This file is generated by rust-protobuf 2.17.0. Do not edit
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
//! Generated file from `message.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_17_0;

#[derive(PartialEq,Clone,Default)]
pub struct Message {
    // message fields
    pub completeRequestList: bool,
    pub requests: ::protobuf::RepeatedField<Message_Request>,
    pub responses: ::protobuf::RepeatedField<Message_Response>,
    pub data: ::protobuf::RepeatedField<Message_Block>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Message {
    fn default() -> &'a Message {
        <Message as ::protobuf::Message>::default_instance()
    }
}

impl Message {
    pub fn new() -> Message {
        ::std::default::Default::default()
    }

    // bool completeRequestList = 1;


    pub fn get_completeRequestList(&self) -> bool {
        self.completeRequestList
    }
    pub fn clear_completeRequestList(&mut self) {
        self.completeRequestList = false;
    }

    // Param is passed by value, moved
    pub fn set_completeRequestList(&mut self, v: bool) {
        self.completeRequestList = v;
    }

    // repeated .graphsync.message.pb.Message.Request requests = 2;


    pub fn get_requests(&self) -> &[Message_Request] {
        &self.requests
    }
    pub fn clear_requests(&mut self) {
        self.requests.clear();
    }

    // Param is passed by value, moved
    pub fn set_requests(&mut self, v: ::protobuf::RepeatedField<Message_Request>) {
        self.requests = v;
    }

    // Mutable pointer to the field.
    pub fn mut_requests(&mut self) -> &mut ::protobuf::RepeatedField<Message_Request> {
        &mut self.requests
    }

    // Take field
    pub fn take_requests(&mut self) -> ::protobuf::RepeatedField<Message_Request> {
        ::std::mem::replace(&mut self.requests, ::protobuf::RepeatedField::new())
    }

    // repeated .graphsync.message.pb.Message.Response responses = 3;


    pub fn get_responses(&self) -> &[Message_Response] {
        &self.responses
    }
    pub fn clear_responses(&mut self) {
        self.responses.clear();
    }

    // Param is passed by value, moved
    pub fn set_responses(&mut self, v: ::protobuf::RepeatedField<Message_Response>) {
        self.responses = v;
    }

    // Mutable pointer to the field.
    pub fn mut_responses(&mut self) -> &mut ::protobuf::RepeatedField<Message_Response> {
        &mut self.responses
    }

    // Take field
    pub fn take_responses(&mut self) -> ::protobuf::RepeatedField<Message_Response> {
        ::std::mem::replace(&mut self.responses, ::protobuf::RepeatedField::new())
    }

    // repeated .graphsync.message.pb.Message.Block data = 4;


    pub fn get_data(&self) -> &[Message_Block] {
        &self.data
    }
    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::protobuf::RepeatedField<Message_Block>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    pub fn mut_data(&mut self) -> &mut ::protobuf::RepeatedField<Message_Block> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::protobuf::RepeatedField<Message_Block> {
        ::std::mem::replace(&mut self.data, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for Message {
    fn is_initialized(&self) -> bool {
        for v in &self.requests {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.responses {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.data {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.completeRequestList = tmp;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.requests)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.responses)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.data)?;
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
        if self.completeRequestList != false {
            my_size += 2;
        }
        for value in &self.requests {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.responses {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.data {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.completeRequestList != false {
            os.write_bool(1, self.completeRequestList)?;
        }
        for v in &self.requests {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.responses {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.data {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

    fn new() -> Message {
        Message::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "completeRequestList",
                |m: &Message| { &m.completeRequestList },
                |m: &mut Message| { &mut m.completeRequestList },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Message_Request>>(
                "requests",
                |m: &Message| { &m.requests },
                |m: &mut Message| { &mut m.requests },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Message_Response>>(
                "responses",
                |m: &Message| { &m.responses },
                |m: &mut Message| { &mut m.responses },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Message_Block>>(
                "data",
                |m: &Message| { &m.data },
                |m: &mut Message| { &mut m.data },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Message>(
                "Message",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Message {
        static instance: ::protobuf::rt::LazyV2<Message> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Message::new)
    }
}

impl ::protobuf::Clear for Message {
    fn clear(&mut self) {
        self.completeRequestList = false;
        self.requests.clear();
        self.responses.clear();
        self.data.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Message {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Message {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Message_Request {
    // message fields
    pub id: i32,
    pub root: ::std::vec::Vec<u8>,
    pub selector: ::std::vec::Vec<u8>,
    pub extensions: ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<u8>>,
    pub priority: i32,
    pub cancel: bool,
    pub update: bool,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Message_Request {
    fn default() -> &'a Message_Request {
        <Message_Request as ::protobuf::Message>::default_instance()
    }
}

impl Message_Request {
    pub fn new() -> Message_Request {
        ::std::default::Default::default()
    }

    // int32 id = 1;


    pub fn get_id(&self) -> i32 {
        self.id
    }
    pub fn clear_id(&mut self) {
        self.id = 0;
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: i32) {
        self.id = v;
    }

    // bytes root = 2;


    pub fn get_root(&self) -> &[u8] {
        &self.root
    }
    pub fn clear_root(&mut self) {
        self.root.clear();
    }

    // Param is passed by value, moved
    pub fn set_root(&mut self, v: ::std::vec::Vec<u8>) {
        self.root = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_root(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.root
    }

    // Take field
    pub fn take_root(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.root, ::std::vec::Vec::new())
    }

    // bytes selector = 3;


    pub fn get_selector(&self) -> &[u8] {
        &self.selector
    }
    pub fn clear_selector(&mut self) {
        self.selector.clear();
    }

    // Param is passed by value, moved
    pub fn set_selector(&mut self, v: ::std::vec::Vec<u8>) {
        self.selector = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_selector(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.selector
    }

    // Take field
    pub fn take_selector(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.selector, ::std::vec::Vec::new())
    }

    // repeated .graphsync.message.pb.Message.Request.ExtensionsEntry extensions = 4;


    pub fn get_extensions(&self) -> &::std::collections::HashMap<::std::string::String, ::std::vec::Vec<u8>> {
        &self.extensions
    }
    pub fn clear_extensions(&mut self) {
        self.extensions.clear();
    }

    // Param is passed by value, moved
    pub fn set_extensions(&mut self, v: ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<u8>>) {
        self.extensions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_extensions(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<u8>> {
        &mut self.extensions
    }

    // Take field
    pub fn take_extensions(&mut self) -> ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.extensions, ::std::collections::HashMap::new())
    }

    // int32 priority = 5;


    pub fn get_priority(&self) -> i32 {
        self.priority
    }
    pub fn clear_priority(&mut self) {
        self.priority = 0;
    }

    // Param is passed by value, moved
    pub fn set_priority(&mut self, v: i32) {
        self.priority = v;
    }

    // bool cancel = 6;


    pub fn get_cancel(&self) -> bool {
        self.cancel
    }
    pub fn clear_cancel(&mut self) {
        self.cancel = false;
    }

    // Param is passed by value, moved
    pub fn set_cancel(&mut self, v: bool) {
        self.cancel = v;
    }

    // bool update = 7;


    pub fn get_update(&self) -> bool {
        self.update
    }
    pub fn clear_update(&mut self) {
        self.update = false;
    }

    // Param is passed by value, moved
    pub fn set_update(&mut self, v: bool) {
        self.update = v;
    }
}

impl ::protobuf::Message for Message_Request {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.id = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.root)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.selector)?;
                },
                4 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeBytes>(wire_type, is, &mut self.extensions)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.priority = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.cancel = tmp;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.update = tmp;
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
        if self.id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.id, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.root.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.root);
        }
        if !self.selector.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.selector);
        }
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeBytes>(4, &self.extensions);
        if self.priority != 0 {
            my_size += ::protobuf::rt::value_size(5, self.priority, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.cancel != false {
            my_size += 2;
        }
        if self.update != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.id != 0 {
            os.write_int32(1, self.id)?;
        }
        if !self.root.is_empty() {
            os.write_bytes(2, &self.root)?;
        }
        if !self.selector.is_empty() {
            os.write_bytes(3, &self.selector)?;
        }
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeBytes>(4, &self.extensions, os)?;
        if self.priority != 0 {
            os.write_int32(5, self.priority)?;
        }
        if self.cancel != false {
            os.write_bool(6, self.cancel)?;
        }
        if self.update != false {
            os.write_bool(7, self.update)?;
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

    fn new() -> Message_Request {
        Message_Request::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "id",
                |m: &Message_Request| { &m.id },
                |m: &mut Message_Request| { &mut m.id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "root",
                |m: &Message_Request| { &m.root },
                |m: &mut Message_Request| { &mut m.root },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "selector",
                |m: &Message_Request| { &m.selector },
                |m: &mut Message_Request| { &mut m.selector },
            ));
            fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeBytes>(
                "extensions",
                |m: &Message_Request| { &m.extensions },
                |m: &mut Message_Request| { &mut m.extensions },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "priority",
                |m: &Message_Request| { &m.priority },
                |m: &mut Message_Request| { &mut m.priority },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "cancel",
                |m: &Message_Request| { &m.cancel },
                |m: &mut Message_Request| { &mut m.cancel },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "update",
                |m: &Message_Request| { &m.update },
                |m: &mut Message_Request| { &mut m.update },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Message_Request>(
                "Message.Request",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Message_Request {
        static instance: ::protobuf::rt::LazyV2<Message_Request> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Message_Request::new)
    }
}

impl ::protobuf::Clear for Message_Request {
    fn clear(&mut self) {
        self.id = 0;
        self.root.clear();
        self.selector.clear();
        self.extensions.clear();
        self.priority = 0;
        self.cancel = false;
        self.update = false;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Message_Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Message_Request {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Message_Response {
    // message fields
    pub id: i32,
    pub status: i32,
    pub extensions: ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<u8>>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Message_Response {
    fn default() -> &'a Message_Response {
        <Message_Response as ::protobuf::Message>::default_instance()
    }
}

impl Message_Response {
    pub fn new() -> Message_Response {
        ::std::default::Default::default()
    }

    // int32 id = 1;


    pub fn get_id(&self) -> i32 {
        self.id
    }
    pub fn clear_id(&mut self) {
        self.id = 0;
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: i32) {
        self.id = v;
    }

    // int32 status = 2;


    pub fn get_status(&self) -> i32 {
        self.status
    }
    pub fn clear_status(&mut self) {
        self.status = 0;
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: i32) {
        self.status = v;
    }

    // repeated .graphsync.message.pb.Message.Response.ExtensionsEntry extensions = 3;


    pub fn get_extensions(&self) -> &::std::collections::HashMap<::std::string::String, ::std::vec::Vec<u8>> {
        &self.extensions
    }
    pub fn clear_extensions(&mut self) {
        self.extensions.clear();
    }

    // Param is passed by value, moved
    pub fn set_extensions(&mut self, v: ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<u8>>) {
        self.extensions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_extensions(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<u8>> {
        &mut self.extensions
    }

    // Take field
    pub fn take_extensions(&mut self) -> ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.extensions, ::std::collections::HashMap::new())
    }
}

impl ::protobuf::Message for Message_Response {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.id = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.status = tmp;
                },
                3 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeBytes>(wire_type, is, &mut self.extensions)?;
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
        if self.id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.id, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.status != 0 {
            my_size += ::protobuf::rt::value_size(2, self.status, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeBytes>(3, &self.extensions);
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.id != 0 {
            os.write_int32(1, self.id)?;
        }
        if self.status != 0 {
            os.write_int32(2, self.status)?;
        }
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeBytes>(3, &self.extensions, os)?;
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

    fn new() -> Message_Response {
        Message_Response::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "id",
                |m: &Message_Response| { &m.id },
                |m: &mut Message_Response| { &mut m.id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "status",
                |m: &Message_Response| { &m.status },
                |m: &mut Message_Response| { &mut m.status },
            ));
            fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeBytes>(
                "extensions",
                |m: &Message_Response| { &m.extensions },
                |m: &mut Message_Response| { &mut m.extensions },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Message_Response>(
                "Message.Response",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Message_Response {
        static instance: ::protobuf::rt::LazyV2<Message_Response> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Message_Response::new)
    }
}

impl ::protobuf::Clear for Message_Response {
    fn clear(&mut self) {
        self.id = 0;
        self.status = 0;
        self.extensions.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Message_Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Message_Response {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Message_Block {
    // message fields
    pub prefix: ::std::vec::Vec<u8>,
    pub data: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Message_Block {
    fn default() -> &'a Message_Block {
        <Message_Block as ::protobuf::Message>::default_instance()
    }
}

impl Message_Block {
    pub fn new() -> Message_Block {
        ::std::default::Default::default()
    }

    // bytes prefix = 1;


    pub fn get_prefix(&self) -> &[u8] {
        &self.prefix
    }
    pub fn clear_prefix(&mut self) {
        self.prefix.clear();
    }

    // Param is passed by value, moved
    pub fn set_prefix(&mut self, v: ::std::vec::Vec<u8>) {
        self.prefix = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_prefix(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.prefix
    }

    // Take field
    pub fn take_prefix(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.prefix, ::std::vec::Vec::new())
    }

    // bytes data = 2;


    pub fn get_data(&self) -> &[u8] {
        &self.data
    }
    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for Message_Block {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.prefix)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.data)?;
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
        if !self.prefix.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.prefix);
        }
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.data);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.prefix.is_empty() {
            os.write_bytes(1, &self.prefix)?;
        }
        if !self.data.is_empty() {
            os.write_bytes(2, &self.data)?;
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

    fn new() -> Message_Block {
        Message_Block::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "prefix",
                |m: &Message_Block| { &m.prefix },
                |m: &mut Message_Block| { &mut m.prefix },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "data",
                |m: &Message_Block| { &m.data },
                |m: &mut Message_Block| { &mut m.data },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Message_Block>(
                "Message.Block",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Message_Block {
        static instance: ::protobuf::rt::LazyV2<Message_Block> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Message_Block::new)
    }
}

impl ::protobuf::Clear for Message_Block {
    fn clear(&mut self) {
        self.prefix.clear();
        self.data.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Message_Block {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Message_Block {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rmessage.proto\x12\x14graphsync.message.pb\"\xac\x06\n\x07Message\x12\
    0\n\x13completeRequestList\x18\x01\x20\x01(\x08R\x13completeRequestList\
    \x12A\n\x08requests\x18\x02\x20\x03(\x0b2%.graphsync.message.pb.Message.\
    RequestR\x08requests\x12D\n\tresponses\x18\x03\x20\x03(\x0b2&.graphsync.\
    message.pb.Message.ResponseR\tresponses\x127\n\x04data\x18\x04\x20\x03(\
    \x0b2#.graphsync.message.pb.Message.BlockR\x04data\x1a\xab\x02\n\x07Requ\
    est\x12\x0e\n\x02id\x18\x01\x20\x01(\x05R\x02id\x12\x12\n\x04root\x18\
    \x02\x20\x01(\x0cR\x04root\x12\x1a\n\x08selector\x18\x03\x20\x01(\x0cR\
    \x08selector\x12U\n\nextensions\x18\x04\x20\x03(\x0b25.graphsync.message\
    .pb.Message.Request.ExtensionsEntryR\nextensions\x12\x1a\n\x08priority\
    \x18\x05\x20\x01(\x05R\x08priority\x12\x16\n\x06cancel\x18\x06\x20\x01(\
    \x08R\x06cancel\x12\x16\n\x06update\x18\x07\x20\x01(\x08R\x06update\x1a=\
    \n\x0fExtensionsEntry\x12\x10\n\x03key\x18\x01\x20\x01(\tR\x03key\x12\
    \x14\n\x05value\x18\x02\x20\x01(\x0cR\x05value:\x028\x01\x1a\xc9\x01\n\
    \x08Response\x12\x0e\n\x02id\x18\x01\x20\x01(\x05R\x02id\x12\x16\n\x06st\
    atus\x18\x02\x20\x01(\x05R\x06status\x12V\n\nextensions\x18\x03\x20\x03(\
    \x0b26.graphsync.message.pb.Message.Response.ExtensionsEntryR\nextension\
    s\x1a=\n\x0fExtensionsEntry\x12\x10\n\x03key\x18\x01\x20\x01(\tR\x03key\
    \x12\x14\n\x05value\x18\x02\x20\x01(\x0cR\x05value:\x028\x01\x1a3\n\x05B\
    lock\x12\x16\n\x06prefix\x18\x01\x20\x01(\x0cR\x06prefix\x12\x12\n\x04da\
    ta\x18\x02\x20\x01(\x0cR\x04datab\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
