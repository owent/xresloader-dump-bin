use protobuf::descriptor::{DescriptorProto, EnumDescriptorProto, FileDescriptorProto};
use protobuf::reflect::{FileDescriptor, MessageDescriptor};

use log::{debug, error, warn};

use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::AsRef;
use std::rc::Rc;

pub struct FileDescriptorCache {
    pub proto: FileDescriptorProto,
    pub pb_file: String,
    pub descriptor: RefCell<Option<Rc<FileDescriptor>>>,
    pub internal_proto: bool,
}

pub type FileDescriptorCacheRef = Rc<FileDescriptorCache>;

pub struct EnumDescriptorCache {
    pub proto: EnumDescriptorProto,
    pub file: FileDescriptorCacheRef,
}
pub type EnumDescriptorCacheRef = Rc<EnumDescriptorCache>;

pub struct MessageDescriptorCache {
    pub proto: DescriptorProto,
    pub file: FileDescriptorCacheRef,
    pub descriptor: RefCell<Option<Rc<MessageDescriptor>>>,
}
pub type MessageDescriptorCacheRef = Rc<MessageDescriptorCache>;

pub struct FileDescriptorIndex {
    pub files: HashMap<String, FileDescriptorCacheRef>,
    pub messages: HashMap<String, MessageDescriptorCacheRef>,
    pub enums: HashMap<String, EnumDescriptorCacheRef>,
}

impl FileDescriptorIndex {
    pub fn new() -> Self {
        let mut ret = FileDescriptorIndex {
            files: HashMap::new(),
            messages: HashMap::new(),
            enums: HashMap::new(),
        };

        // Register protobuf types.
        let internal_protos = [
            protobuf::descriptor::file_descriptor().proto(),
            protobuf::well_known_types::any::file_descriptor().proto(),
            protobuf::well_known_types::api::file_descriptor().proto(),
            protobuf::well_known_types::duration::file_descriptor().proto(),
            protobuf::well_known_types::empty::file_descriptor().proto(),
            protobuf::well_known_types::field_mask::file_descriptor().proto(),
            protobuf::well_known_types::source_context::file_descriptor().proto(),
            protobuf::well_known_types::struct_::file_descriptor().proto(),
            protobuf::well_known_types::timestamp::file_descriptor().proto(),
            protobuf::well_known_types::type_::file_descriptor().proto(),
            protobuf::well_known_types::wrappers::file_descriptor().proto(),
        ];

        for internal_proto in internal_protos {
            trace!("Register internal proto: {}", internal_proto.name());
            ret.add_file_internal(internal_proto, internal_proto.name(), true);
        }

        // Register xresloader built-in types.
        let internal_protos = [
            xresloader_protocol::proto::pb_header_v3::file_descriptor().proto(),
            xresloader_protocol::proto::xresloader::file_descriptor().proto(),
            xresloader_protocol::proto::xresloader_ue::file_descriptor().proto(),
        ];
        for internal_proto in internal_protos {
            trace!("Register internal proto: {}", internal_proto.name());
            ret.add_file_internal(internal_proto, internal_proto.name(), true);
        }

        ret
    }

    pub fn add_file(&mut self, file: &FileDescriptorProto, pb_file: &str) {
        self.add_file_internal(file, pb_file, false)
    }

    pub fn add_file_internal(
        &mut self,
        file: &FileDescriptorProto,
        pb_file: &str,
        internal_proto: bool,
    ) {
        let file_name = file.name().to_string();
        if self.files.contains_key(&file_name) {
            let old = self.files.get(&file_name).unwrap();
            if !internal_proto {
                warn!(
                    "{} is already defined in {}, the definition in {} will be ignored",
                    file_name, old.pb_file, pb_file
                );
            }
            return;
        }

        let file_descriptor = Rc::new(FileDescriptorCache {
            proto: file.clone(),
            pb_file: pb_file.to_string(),
            descriptor: RefCell::new(None),
            internal_proto: internal_proto,
        });

        let nested_ident = String::from("    ");

        self.files.insert(file_name, file_descriptor.clone());
        for enum_decl in &file.enum_type {
            self.add_enum_index(
                file_descriptor.proto.package().to_string(),
                &nested_ident,
                Rc::new(EnumDescriptorCache {
                    proto: enum_decl.clone(),
                    file: file_descriptor.clone(),
                }),
            )
        }

        for message_decl in &file.message_type {
            self.add_message_index(
                file_descriptor.proto.package().to_string(),
                &nested_ident,
                Rc::new(MessageDescriptorCache {
                    proto: message_decl.clone(),
                    file: file_descriptor.clone(),
                    descriptor: RefCell::new(None),
                }),
            )
        }
    }

    fn add_enum_index(&mut self, prefix: String, indent: &String, desc: EnumDescriptorCacheRef) {
        let full_name = if prefix.is_empty() {
            desc.proto.name().to_string()
        } else {
            format!("{}.{}", prefix, desc.proto.name().to_string())
        };

        if self.enums.contains_key(&full_name) {
            let old = self.enums.get(&full_name).unwrap();
            warn!(
                "{} is already defined in {} of {}, the definition in {} of {} will be ignored",
                full_name,
                old.file.proto.name(),
                old.file.pb_file,
                desc.file.proto.name(),
                desc.file.pb_file
            );
            return;
        }
        if desc.file.internal_proto {
            trace!("{}Index enum: {}", indent, full_name);
        } else {
            debug!("{}Index enum: {}", indent, full_name);
        }

        self.enums.insert(full_name, desc.clone());
    }

    fn add_message_index(
        &mut self,
        prefix: String,
        indent: &String,
        desc: MessageDescriptorCacheRef,
    ) {
        let full_name = if prefix.is_empty() {
            desc.proto.name().to_string()
        } else {
            format!("{}.{}", prefix, desc.proto.name().to_string())
        };

        if self.messages.contains_key(&full_name) {
            let old = self.messages.get(&full_name).unwrap();
            warn!(
                "{} is already defined in {} of {}, the definition in {} of {} will be ignored",
                full_name,
                old.file.proto.name(),
                old.file.pb_file,
                desc.file.proto.name(),
                desc.file.pb_file
            );
            return;
        }
        if desc.file.internal_proto {
            trace!("{}Index message: {}", indent, full_name);
        } else {
            debug!("{}Index message: {}", indent, full_name);
        }

        self.messages.insert(full_name.clone(), desc.clone());

        let nested_ident = format!("  {}", indent);
        for nested_enum in &desc.proto.enum_type {
            self.add_enum_index(
                full_name.clone(),
                &nested_ident,
                Rc::new(EnumDescriptorCache {
                    proto: nested_enum.clone(),
                    file: desc.file.clone(),
                }),
            );
        }

        for nested_message in &desc.proto.nested_type {
            self.add_message_index(
                full_name.clone(),
                &nested_ident,
                Rc::new(MessageDescriptorCache {
                    proto: nested_message.clone(),
                    file: desc.file.clone(),
                    descriptor: RefCell::new(None),
                }),
            );
        }
    }

    pub fn build_file_descriptor(&mut self, file_name: &str) -> Result<Rc<FileDescriptor>, ()> {
        //FileDescriptor
        let file_descriptor = if let Some(x) = self.files.get(file_name) {
            x.clone()
        } else {
            error!("Proto file {} not found", file_name);
            return Err(());
        };

        if let Some(x) = file_descriptor.descriptor.borrow().as_ref() {
            return Ok(x.clone());
        }

        // FileDescriptor::new_dynamic(proto, dependencies)
        let mut dependencies: Vec<FileDescriptor> = Vec::new();
        let desc_proto = file_descriptor.proto.clone();
        let mut has_failed = false;
        for depend_name in &desc_proto.dependency {
            let depend_desc = match self.build_file_descriptor(depend_name) {
                Ok(x) => x,
                Err(_) => {
                    has_failed = true;
                    error!("Build depenedency {} of {} failed", depend_name, file_name);
                    continue;
                }
            };

            dependencies.push(depend_desc.as_ref().clone());
        }
        if has_failed {
            return Err(());
        }

        let descriptor = match FileDescriptor::new_dynamic(desc_proto, &dependencies) {
            Ok(x) => x,
            Err(e) => {
                error!(
                    "Build file descriptor {} failed: {}",
                    file_descriptor.pb_file, e
                );
                return Err(());
            }
        };

        let ret = Rc::new(descriptor);
        {
            let mut mut_desc = file_descriptor.descriptor.borrow_mut();
            *mut_desc = Some(ret.clone());
        }

        Ok(ret)
    }

    pub fn build_message_descriptor(
        &mut self,
        message_full_name: &str,
    ) -> Result<Rc<MessageDescriptor>, ()> {
        let message_descriptor = if let Some(x) = self.messages.get(message_full_name) {
            x.clone()
        } else {
            error!("Proto message {} not found", message_full_name);
            return Err(());
        };

        if let Some(x) = message_descriptor.descriptor.borrow().as_ref() {
            return Ok(x.clone());
        }

        let file_descriptor =
            match self.build_file_descriptor(&message_descriptor.file.proto.name()) {
                Ok(x) => x,
                Err(e) => {
                    error!(
                        "Build file descriptor {} for message {} failed.",
                        message_descriptor.file.pb_file, message_full_name
                    );
                    return Err(e);
                }
            };

        let pb_desc = if message_full_name.starts_with(".") {
            if let Some(x) = file_descriptor.message_by_full_name(&message_full_name) {
                x
            } else {
                return Err(());
            }
        } else {
            let format_message_name = format!(".{}", message_full_name);
            if let Some(x) = file_descriptor.message_by_full_name(&format_message_name) {
                x
            } else {
                return Err(());
            }
        };

        let ret = Rc::new(pb_desc);
        {
            let mut mut_desc = message_descriptor.descriptor.borrow_mut();
            *mut_desc = Some(ret.clone());
        }

        Ok(ret)
    }
}
