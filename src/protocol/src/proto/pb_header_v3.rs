// This file is generated by rust-protobuf 3.2.0. Do not edit
// .proto file is parsed by protoc 3.19.4
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

//! Generated file from `pb_header_v3.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:org.xresloader.pb.xresloader_data_source)
pub struct Xresloader_data_source {
    // message fields
    // @@protoc_insertion_point(field:org.xresloader.pb.xresloader_data_source.file)
    pub file: ::std::string::String,
    // @@protoc_insertion_point(field:org.xresloader.pb.xresloader_data_source.sheet)
    pub sheet: ::std::string::String,
    // @@protoc_insertion_point(field:org.xresloader.pb.xresloader_data_source.count)
    pub count: i32,
    // special fields
    // @@protoc_insertion_point(special_field:org.xresloader.pb.xresloader_data_source.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Xresloader_data_source {
    fn default() -> &'a Xresloader_data_source {
        <Xresloader_data_source as ::protobuf::Message>::default_instance()
    }
}

impl Xresloader_data_source {
    pub fn new() -> Xresloader_data_source {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "file",
            |m: &Xresloader_data_source| { &m.file },
            |m: &mut Xresloader_data_source| { &mut m.file },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "sheet",
            |m: &Xresloader_data_source| { &m.sheet },
            |m: &mut Xresloader_data_source| { &mut m.sheet },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "count",
            |m: &Xresloader_data_source| { &m.count },
            |m: &mut Xresloader_data_source| { &mut m.count },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Xresloader_data_source>(
            "xresloader_data_source",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Xresloader_data_source {
    const NAME: &'static str = "xresloader_data_source";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.file = is.read_string()?;
                },
                18 => {
                    self.sheet = is.read_string()?;
                },
                24 => {
                    self.count = is.read_int32()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.file.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.file);
        }
        if !self.sheet.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.sheet);
        }
        if self.count != 0 {
            my_size += ::protobuf::rt::int32_size(3, self.count);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.file.is_empty() {
            os.write_string(1, &self.file)?;
        }
        if !self.sheet.is_empty() {
            os.write_string(2, &self.sheet)?;
        }
        if self.count != 0 {
            os.write_int32(3, self.count)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> Xresloader_data_source {
        Xresloader_data_source::new()
    }

    fn clear(&mut self) {
        self.file.clear();
        self.sheet.clear();
        self.count = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Xresloader_data_source {
        static instance: Xresloader_data_source = Xresloader_data_source {
            file: ::std::string::String::new(),
            sheet: ::std::string::String::new(),
            count: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Xresloader_data_source {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("xresloader_data_source").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Xresloader_data_source {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Xresloader_data_source {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:org.xresloader.pb.xresloader_header)
pub struct Xresloader_header {
    // message fields
    // @@protoc_insertion_point(field:org.xresloader.pb.xresloader_header.xres_ver)
    pub xres_ver: ::std::string::String,
    // @@protoc_insertion_point(field:org.xresloader.pb.xresloader_header.data_ver)
    pub data_ver: ::std::string::String,
    // @@protoc_insertion_point(field:org.xresloader.pb.xresloader_header.count)
    pub count: u32,
    // @@protoc_insertion_point(field:org.xresloader.pb.xresloader_header.hash_code)
    pub hash_code: ::std::string::String,
    // @@protoc_insertion_point(field:org.xresloader.pb.xresloader_header.description)
    pub description: ::std::string::String,
    // @@protoc_insertion_point(field:org.xresloader.pb.xresloader_header.data_source)
    pub data_source: ::std::vec::Vec<Xresloader_data_source>,
    // special fields
    // @@protoc_insertion_point(special_field:org.xresloader.pb.xresloader_header.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Xresloader_header {
    fn default() -> &'a Xresloader_header {
        <Xresloader_header as ::protobuf::Message>::default_instance()
    }
}

impl Xresloader_header {
    pub fn new() -> Xresloader_header {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "xres_ver",
            |m: &Xresloader_header| { &m.xres_ver },
            |m: &mut Xresloader_header| { &mut m.xres_ver },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "data_ver",
            |m: &Xresloader_header| { &m.data_ver },
            |m: &mut Xresloader_header| { &mut m.data_ver },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "count",
            |m: &Xresloader_header| { &m.count },
            |m: &mut Xresloader_header| { &mut m.count },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "hash_code",
            |m: &Xresloader_header| { &m.hash_code },
            |m: &mut Xresloader_header| { &mut m.hash_code },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "description",
            |m: &Xresloader_header| { &m.description },
            |m: &mut Xresloader_header| { &mut m.description },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "data_source",
            |m: &Xresloader_header| { &m.data_source },
            |m: &mut Xresloader_header| { &mut m.data_source },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Xresloader_header>(
            "xresloader_header",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Xresloader_header {
    const NAME: &'static str = "xresloader_header";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.xres_ver = is.read_string()?;
                },
                18 => {
                    self.data_ver = is.read_string()?;
                },
                24 => {
                    self.count = is.read_uint32()?;
                },
                34 => {
                    self.hash_code = is.read_string()?;
                },
                42 => {
                    self.description = is.read_string()?;
                },
                90 => {
                    self.data_source.push(is.read_message()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.xres_ver.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.xres_ver);
        }
        if !self.data_ver.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.data_ver);
        }
        if self.count != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.count);
        }
        if !self.hash_code.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.hash_code);
        }
        if !self.description.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.description);
        }
        for value in &self.data_source {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.xres_ver.is_empty() {
            os.write_string(1, &self.xres_ver)?;
        }
        if !self.data_ver.is_empty() {
            os.write_string(2, &self.data_ver)?;
        }
        if self.count != 0 {
            os.write_uint32(3, self.count)?;
        }
        if !self.hash_code.is_empty() {
            os.write_string(4, &self.hash_code)?;
        }
        if !self.description.is_empty() {
            os.write_string(5, &self.description)?;
        }
        for v in &self.data_source {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> Xresloader_header {
        Xresloader_header::new()
    }

    fn clear(&mut self) {
        self.xres_ver.clear();
        self.data_ver.clear();
        self.count = 0;
        self.hash_code.clear();
        self.description.clear();
        self.data_source.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Xresloader_header {
        static instance: Xresloader_header = Xresloader_header {
            xres_ver: ::std::string::String::new(),
            data_ver: ::std::string::String::new(),
            count: 0,
            hash_code: ::std::string::String::new(),
            description: ::std::string::String::new(),
            data_source: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Xresloader_header {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("xresloader_header").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Xresloader_header {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Xresloader_header {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:org.xresloader.pb.xresloader_datablocks)
pub struct Xresloader_datablocks {
    // message fields
    // @@protoc_insertion_point(field:org.xresloader.pb.xresloader_datablocks.header)
    pub header: ::protobuf::MessageField<Xresloader_header>,
    // @@protoc_insertion_point(field:org.xresloader.pb.xresloader_datablocks.data_block)
    pub data_block: ::std::vec::Vec<::std::vec::Vec<u8>>,
    // @@protoc_insertion_point(field:org.xresloader.pb.xresloader_datablocks.data_message_type)
    pub data_message_type: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:org.xresloader.pb.xresloader_datablocks.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Xresloader_datablocks {
    fn default() -> &'a Xresloader_datablocks {
        <Xresloader_datablocks as ::protobuf::Message>::default_instance()
    }
}

impl Xresloader_datablocks {
    pub fn new() -> Xresloader_datablocks {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, Xresloader_header>(
            "header",
            |m: &Xresloader_datablocks| { &m.header },
            |m: &mut Xresloader_datablocks| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "data_block",
            |m: &Xresloader_datablocks| { &m.data_block },
            |m: &mut Xresloader_datablocks| { &mut m.data_block },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "data_message_type",
            |m: &Xresloader_datablocks| { &m.data_message_type },
            |m: &mut Xresloader_datablocks| { &mut m.data_message_type },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Xresloader_datablocks>(
            "xresloader_datablocks",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Xresloader_datablocks {
    const NAME: &'static str = "xresloader_datablocks";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.header)?;
                },
                18 => {
                    self.data_block.push(is.read_bytes()?);
                },
                26 => {
                    self.data_message_type = is.read_string()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.data_block {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        if !self.data_message_type.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.data_message_type);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.header.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        for v in &self.data_block {
            os.write_bytes(2, &v)?;
        };
        if !self.data_message_type.is_empty() {
            os.write_string(3, &self.data_message_type)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> Xresloader_datablocks {
        Xresloader_datablocks::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.data_block.clear();
        self.data_message_type.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Xresloader_datablocks {
        static instance: Xresloader_datablocks = Xresloader_datablocks {
            header: ::protobuf::MessageField::none(),
            data_block: ::std::vec::Vec::new(),
            data_message_type: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Xresloader_datablocks {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("xresloader_datablocks").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Xresloader_datablocks {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Xresloader_datablocks {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12pb_header_v3.proto\x12\x11org.xresloader.pb\"X\n\x16xresloader_dat\
    a_source\x12\x12\n\x04file\x18\x01\x20\x01(\tR\x04file\x12\x14\n\x05shee\
    t\x18\x02\x20\x01(\tR\x05sheet\x12\x14\n\x05count\x18\x03\x20\x01(\x05R\
    \x05count\"\xea\x01\n\x11xresloader_header\x12\x19\n\x08xres_ver\x18\x01\
    \x20\x01(\tR\x07xresVer\x12\x19\n\x08data_ver\x18\x02\x20\x01(\tR\x07dat\
    aVer\x12\x14\n\x05count\x18\x03\x20\x01(\rR\x05count\x12\x1b\n\thash_cod\
    e\x18\x04\x20\x01(\tR\x08hashCode\x12\x20\n\x0bdescription\x18\x05\x20\
    \x01(\tR\x0bdescription\x12J\n\x0bdata_source\x18\x0b\x20\x03(\x0b2).org\
    .xresloader.pb.xresloader_data_sourceR\ndataSource\"\xa0\x01\n\x15xreslo\
    ader_datablocks\x12<\n\x06header\x18\x01\x20\x01(\x0b2$.org.xresloader.p\
    b.xresloader_headerR\x06header\x12\x1d\n\ndata_block\x18\x02\x20\x03(\
    \x0cR\tdataBlock\x12*\n\x11data_message_type\x18\x03\x20\x01(\tR\x0fdata\
    MessageTypeb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(3);
            messages.push(Xresloader_data_source::generated_message_descriptor_data());
            messages.push(Xresloader_header::generated_message_descriptor_data());
            messages.push(Xresloader_datablocks::generated_message_descriptor_data());
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
