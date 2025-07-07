use std::collections::{HashMap, HashSet, LinkedList};
use std::fs::File;
use std::io::Write;
use std::ops::Deref;

use super::dump_options::DumpOptions;
use super::utility;

use protobuf::reflect::ReflectValueRef;

use protobuf::{Message, MessageDyn};
use xresloader_protocol::proto::pb_header_v3::{Xresloader_data_source, Xresloader_datablocks};
// use xresloader_protocol::proto::xresloader::exts::field_tag;
// use xresloader_protocol::proto::xresloader::exts::oneof_tag;

#[derive(Clone)]
pub struct TaggedFieldDataSource {
    pub file: ::std::string::String,
    pub sheet: ::std::string::String,
    pub count: i32,
}

pub struct TaggedFieldBinarySource {
    pub xres_ver: ::std::string::String,
    pub data_ver: ::std::string::String,
    pub bin_file: ::std::string::String,
    pub count: u32,
    pub hash_code: ::std::string::String,
    pub description: ::std::string::String,
    pub data_source: ::std::vec::Vec<TaggedFieldDataSource>,
}

pub struct TaggedFieldItemSource {
    pub file: ::std::string::String,
    pub sheet: ::std::string::String,
}

pub struct TaggedFieldContent {
    pub head: TaggedFieldBinarySource,
    pub body: HashMap<String, LinkedList<TaggedFieldItemSource>>,
}

#[derive(Default)]
pub struct TaggedFieldFilter {
    blacklist_full_names: HashSet<String>,
    whitelist_full_names: HashSet<String>,

    pub select_field_tags: HashSet<String>,
    pub select_oneof_tags: HashSet<String>,

    pub value_include_regex_rules: Vec<regex::Regex>,
    pub value_exclude_regex_rules: Vec<regex::Regex>,
    pub include_message_paths: HashSet<String>,
    pub exclude_message_paths: HashSet<String>,
    pub include_field_paths: HashSet<String>,
    pub exclude_field_paths: HashSet<String>,
}

impl TaggedFieldDataSource {
    pub fn default() -> Self {
        TaggedFieldDataSource {
            file: String::from("[UNKNOWN]"),
            sheet: String::from("[UNKNOWN]"),
            count: 0,
        }
    }

    pub fn new(data_source: &Xresloader_data_source) -> Self {
        TaggedFieldDataSource {
            file: data_source.file.clone(),
            sheet: data_source.sheet.clone(),
            count: data_source.count,
        }
    }
}

impl TaggedFieldBinarySource {
    pub fn new(data_blocks: &Xresloader_datablocks, bin_file: String) -> Self {
        TaggedFieldBinarySource {
            xres_ver: data_blocks.header.xres_ver.clone(),
            data_ver: data_blocks.header.data_ver.clone(),
            bin_file,
            count: data_blocks.header.count,
            hash_code: data_blocks.header.hash_code.clone(),
            description: data_blocks.header.description.clone(),
            data_source: {
                let mut data_source = Vec::new();
                for source in &data_blocks.header.data_source {
                    data_source.push(TaggedFieldDataSource {
                        file: source.file.clone(),
                        sheet: source.sheet.clone(),
                        count: source.count,
                    });
                }
                data_source
            },
        }
    }
}

impl TaggedFieldFilter {
    pub fn filter_value(&self, input: &str) -> bool {
        if input.trim().is_empty() {
            return false;
        }

        if !self.value_include_regex_rules.is_empty() {
            let mut matched = false;
            for rule in &self.value_include_regex_rules {
                if rule.is_match(input) {
                    matched = true;
                    break;
                }
            }
            if !matched {
                return false;
            }
        }
        for rule in &self.value_exclude_regex_rules {
            if rule.is_match(input) {
                return false;
            }
        }

        true
    }

    fn internal_filter_field(&mut self, field_desc: &protobuf::reflect::FieldDescriptor) -> bool {
        if self.select_field_tags.is_empty() && self.select_oneof_tags.is_empty() {
            return false;
        }

        let full_name = field_desc.full_name();
        let mut has_field_tag = false;
        if let Some(ext) = field_desc.proto().options.as_ref() {
            let nfs = ext.unknown_fields();
            // field_tag.field_number is 1022 and it's private
            // FIXME: use a public API to get field number after upgrade to protobuf v4+
            // let field_tag_type = ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_STRING;
            let field_tag_number = 1022;

            for (k, v) in nfs.iter() {
                if k != field_tag_number {
                    continue;
                }

                if let protobuf::UnknownValueRef::LengthDelimited(tag_values) = v {
                    match String::from_utf8(tag_values.to_vec()) {
                        Ok(one_field_tag) => {
                            if self.select_field_tags.contains(&one_field_tag) {
                                has_field_tag = true;
                            }
                        }
                        Err(e) => {
                            error!("Failed to parse field tag {}(which should be org.xresloader.field_tag) as string, maybe corrupted data, {}", field_tag_number, e);
                        }
                    }
                }

                if has_field_tag {
                    break;
                }
            }
        }

        if !has_field_tag && !self.select_oneof_tags.is_empty() {
            if let Some(oneof_desc) = field_desc.containing_oneof() {
                let oneof_full_name = oneof_desc.full_name();
                if self.whitelist_full_names.contains(&oneof_full_name) {
                    has_field_tag = true;
                } else if self.blacklist_full_names.contains(&oneof_full_name) {
                    has_field_tag = false;
                } else if let Some(ext) = oneof_desc.proto().options.as_ref() {
                    let nfs = ext.unknown_fields();
                    // field_tag.field_number is 1005 and it's private
                    // FIXME: use a public API to get field number after upgrade to protobuf v4+
                    // let field_tag_type = ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_STRING;
                    let field_oneof_number = 1005;

                    for (k, v) in nfs.iter() {
                        if k != field_oneof_number {
                            continue;
                        }

                        if let protobuf::UnknownValueRef::LengthDelimited(tag_values) = v {
                            match String::from_utf8(tag_values.to_vec()) {
                                Ok(one_oneof_tag) => {
                                    if self.select_oneof_tags.contains(&one_oneof_tag) {
                                        has_field_tag = true;
                                    }
                                }
                                Err(e) => {
                                    error!("Failed to parse field tag {}(which should be org.xresloader.oneof_tag) as string, maybe corrupted data, {}", field_oneof_number, e);
                                }
                            }
                        }

                        if has_field_tag {
                            break;
                        }
                    }

                    if has_field_tag {
                        self.whitelist_full_names.insert(oneof_full_name);
                    } else {
                        self.blacklist_full_names.insert(oneof_full_name);
                    }
                }
            }
        }

        if !has_field_tag {
            return false;
        }

        if self.include_field_paths.is_empty() && self.exclude_field_paths.is_empty() {
            return true;
        }

        if !self.include_field_paths.is_empty() && !self.include_field_paths.contains(&full_name) {
            return false;
        }

        if self.exclude_field_paths.contains(&full_name) {
            return false;
        }

        true
    }

    pub fn filter_field(&mut self, field_desc: &protobuf::reflect::FieldDescriptor) -> bool {
        let full_name = field_desc.full_name();
        if self.blacklist_full_names.contains(&full_name) {
            return false;
        }
        if self.whitelist_full_names.contains(&full_name) {
            return true;
        }

        let ret = self.internal_filter_field(field_desc);
        if ret {
            self.whitelist_full_names.insert(full_name);
        } else {
            self.blacklist_full_names.insert(full_name);
        }

        ret
    }

    pub fn filter_message(&self, message_desc: &protobuf::reflect::MessageDescriptor) -> bool {
        if !self.include_message_paths.is_empty()
            && !self
                .include_message_paths
                .contains(message_desc.full_name())
        {
            return false;
        }

        if self
            .exclude_message_paths
            .contains(message_desc.full_name())
        {
            return false;
        }

        true
    }
}

impl TaggedFieldContent {
    fn pb_value_to_string(&self, v: &ReflectValueRef) -> String {
        match v {
            ReflectValueRef::U32(u32) => u32.to_string(),
            ReflectValueRef::U64(u64) => u64.to_string(),
            ReflectValueRef::I32(i32) => i32.to_string(),
            ReflectValueRef::I64(i64) => i64.to_string(),
            ReflectValueRef::F32(f32) => f32.to_string(),
            ReflectValueRef::F64(f64) => f64.to_string(),
            ReflectValueRef::Bool(bool) => bool.to_string(),
            ReflectValueRef::String(s) => s.to_string(),
            ReflectValueRef::Message(m) => m.to_string(),
            ReflectValueRef::Enum(e, i) => e.value_by_number(*i).unwrap().full_name(),
            ReflectValueRef::Bytes(b) => format!("{:?}", b),
        }
    }

    pub fn load_message(
        &mut self,
        message: &dyn MessageDyn,
        filter: &mut TaggedFieldFilter,
        data_source: &TaggedFieldDataSource,
    ) {
        if !filter.filter_message(&message.descriptor_dyn()) {
            return;
        }

        message
            .descriptor_dyn()
            .fields()
            .for_each(|field| match field.runtime_field_type() {
                protobuf::reflect::RuntimeFieldType::Singular(_) => {
                    if let Some(v) = field.get_singular(message) {
                        if let protobuf::reflect::ReflectValueRef::Message(m) = v {
                            self.load_message(m.deref(), filter, data_source);
                        } else {
                            if !filter.filter_field(&field) {
                                return;
                            }

                            let s = self.pb_value_to_string(&v);
                            if !filter.filter_value(&s) {
                                return;
                            }

                            let value = v.to_string();
                            if let Some(item) = self.body.get_mut(&value) {
                                item.push_back(TaggedFieldItemSource {
                                    file: data_source.file.clone(),
                                    sheet: data_source.sheet.clone(),
                                });
                            } else {
                                let mut ls = LinkedList::new();
                                ls.push_back(TaggedFieldItemSource {
                                    file: data_source.file.clone(),
                                    sheet: data_source.sheet.clone(),
                                });
                                self.body.insert(value, ls);
                            }
                        }
                    }
                }
                protobuf::reflect::RuntimeFieldType::Repeated(_) => {
                    if !filter.filter_field(&field) {
                        return;
                    }

                    field.get_repeated(message).into_iter().for_each(|v| {
                        if let protobuf::reflect::ReflectValueRef::Message(m) = v {
                            self.load_message(m.deref(), filter, data_source);
                        } else {
                            if !filter.filter_field(&field) {
                                return;
                            }

                            let s = self.pb_value_to_string(&v);
                            if !filter.filter_value(&s) {
                                return;
                            }

                            let value = v.to_string();
                            if let Some(item) = self.body.get_mut(&value) {
                                item.push_back(TaggedFieldItemSource {
                                    file: data_source.file.clone(),
                                    sheet: data_source.sheet.clone(),
                                });
                            } else {
                                let mut ls = LinkedList::new();
                                ls.push_back(TaggedFieldItemSource {
                                    file: data_source.file.clone(),
                                    sheet: data_source.sheet.clone(),
                                });
                                self.body.insert(value, ls);
                            }
                        }
                    });
                }
                protobuf::reflect::RuntimeFieldType::Map(_, _) => {
                    field.get_map(message).into_iter().for_each(|(k, v)| {
                        if let protobuf::reflect::ReflectValueRef::Message(m) = k {
                            self.load_message(m.deref(), filter, data_source);
                        } else {
                            if !filter.filter_field(&field) {
                                return;
                            }

                            let s = self.pb_value_to_string(&k);

                            if !filter.filter_value(&s) {
                                return;
                            }

                            let value = v.to_string();
                            if let Some(item) = self.body.get_mut(&value) {
                                item.push_back(TaggedFieldItemSource {
                                    file: data_source.file.clone(),
                                    sheet: data_source.sheet.clone(),
                                });
                            } else {
                                let mut ls = LinkedList::new();
                                ls.push_back(TaggedFieldItemSource {
                                    file: data_source.file.clone(),
                                    sheet: data_source.sheet.clone(),
                                });
                                self.body.insert(value, ls);
                            }
                        };

                        if let protobuf::reflect::ReflectValueRef::Message(m) = v {
                            self.load_message(m.deref(), filter, data_source);
                        } else {
                            if !filter.filter_field(&field) {
                                return;
                            }

                            let s = self.pb_value_to_string(&v);

                            if !filter.filter_value(&s) {
                                return;
                            }

                            let value = v.to_string();
                            if let Some(item) = self.body.get_mut(&value) {
                                item.push_back(TaggedFieldItemSource {
                                    file: data_source.file.clone(),
                                    sheet: data_source.sheet.clone(),
                                });
                            } else {
                                let mut ls = LinkedList::new();
                                ls.push_back(TaggedFieldItemSource {
                                    file: data_source.file.clone(),
                                    sheet: data_source.sheet.clone(),
                                });
                                self.body.insert(value, ls);
                            }
                        };
                    });
                }
            });
    }

    pub fn to_json(&self) -> json::JsonValue {
        let mut json_item = json::JsonValue::new_object();
        let mut json_item_head = json::JsonValue::new_object();
        let mut json_item_body = json::JsonValue::new_object();

        let _ = json_item_head.insert("xres_ver", self.head.xres_ver.clone());
        let _ = json_item_head.insert("data_ver", self.head.data_ver.clone());
        let _ = json_item_head.insert("bin_file", self.head.bin_file.clone());
        let _ = json_item_head.insert("count", self.head.count);
        let _ = json_item_head.insert("hash_code", self.head.hash_code.clone());
        let _ = json_item_head.insert("description", self.head.description.clone());
        let _ = json_item_head.insert("data_source", {
            let mut ds = json::JsonValue::new_array();
            for source in &self.head.data_source {
                let mut d = json::JsonValue::new_object();
                let _ = d.insert("file", source.file.clone());
                let _ = d.insert("sheet", source.sheet.clone());
                if source.count > 0 {
                    let _ = d.insert("count", source.count);
                }
                let _ = ds.push(d);
            }

            ds
        });

        for row in &self.body {
            let mut body_item = json::JsonValue::new_object();
            let mut body_item_source = json::JsonValue::new_array();
            for source in row.1 {
                let mut d = json::JsonValue::new_object();
                let _ = d.insert("file", source.file.clone());
                let _ = d.insert("sheet", source.sheet.clone());
                let _ = body_item_source.push(d);
            }
            let _ = body_item.insert("source", body_item_source);
            let _ = json_item_body.insert(row.0, body_item);
        }

        let _ = json_item.insert("head", json_item_head);
        let _ = json_item.insert("body", json_item_body);
        json_item
    }

    pub fn to_text(&self) -> HashSet<String> {
        let mut ret = HashSet::new();
        for row in &self.body {
            let _ = ret.insert(row.0.clone());
        }

        ret
    }
}

pub fn build_tagged_field_filter(args: &DumpOptions) -> (TaggedFieldFilter, bool) {
    let mut ret: TaggedFieldFilter = TaggedFieldFilter::default();
    let mut has_error = false;

    for tag in &args.tagged_field_tags {
        if !tag.is_empty() {
            ret.select_field_tags.insert(tag.to_string());
        }
    }

    for tag in &args.tagged_oneof_tags {
        if !tag.is_empty() {
            ret.select_oneof_tags.insert(tag.to_string());
        }
    }

    for regex_rule in &args.tagged_data_include_value_regex_rule {
        match regex::Regex::new(regex_rule) {
            Ok(r) => {
                ret.value_include_regex_rules.push(r);
            }
            Err(e) => {
                error!(
                    "Invalid regex rule: {}, {}, ignore this rule",
                    regex_rule, e
                );
                has_error = true;
            }
        }
    }

    for file_path in &args.tagged_data_include_value_regex_file {
        utility::load_file_by_lines(file_path, "regex rule", &mut has_error, |line| {
            match regex::Regex::new(line) {
                Ok(r) => {
                    ret.value_include_regex_rules.push(r);
                    Ok(())
                }
                Err(e) => Err(format!("{}", e)),
            }
        });
    }

    for regex_rule in &args.tagged_data_exclude_value_regex_rule {
        match regex::Regex::new(regex_rule) {
            Ok(r) => {
                ret.value_exclude_regex_rules.push(r);
            }
            Err(e) => {
                error!(
                    "Invalid regex rule: {}, {}, ignore this rule",
                    regex_rule, e
                );
                has_error = true;
            }
        }
    }

    for file_path in &args.tagged_data_exclude_value_regex_file {
        utility::load_file_by_lines(file_path, "regex rule", &mut has_error, |line| {
            match regex::Regex::new(line) {
                Ok(r) => {
                    ret.value_exclude_regex_rules.push(r);
                    Ok(())
                }
                Err(e) => Err(format!("{}", e)),
            }
        });
    }

    for field_path_file in &args.tagged_data_include_field_path_file {
        utility::load_file_by_lines(field_path_file, "field path", &mut has_error, |line| {
            ret.include_field_paths.insert(line.to_string());
            Ok(())
        });
    }

    for field_path_file in &args.tagged_data_exclude_field_path_file {
        utility::load_file_by_lines(field_path_file, "field path", &mut has_error, |line| {
            ret.exclude_field_paths.insert(line.to_string());
            Ok(())
        });
    }

    for field_path_file in &args.tagged_data_include_message_path_file {
        utility::load_file_by_lines(field_path_file, "message path", &mut has_error, |line| {
            ret.include_message_paths.insert(line.to_string());
            Ok(())
        });
    }

    for field_path_file in &args.tagged_data_exclude_message_path_file {
        utility::load_file_by_lines(field_path_file, "message path", &mut has_error, |line| {
            ret.exclude_message_paths.insert(line.to_string());
            Ok(())
        });
    }

    (ret, has_error)
}

pub fn dump_tagged_field_to_text_file(
    tagged_fields: &Vec<TaggedFieldContent>,
    output_file: &String,
) -> Result<(), ()> {
    let mut has_error = false;
    match File::create(&output_file) {
        Ok(mut f) => {
            let mut text: HashSet<String> = HashSet::new();
            for tagged_field in tagged_fields {
                text.extend(tagged_field.to_text());
            }

            for line in text {
                let _ = f.write(line.as_bytes());
                let _ = f.write(b"\n");
            }
        }
        Err(e) => {
            error!(
                "Try to open {} to write string table failed, {}",
                output_file, e
            );
            has_error = true;
        }
    }

    if has_error {
        Err(())
    } else {
        Ok(())
    }
}

pub fn dump_tagged_field_to_json_file(
    tagged_fields: &Vec<TaggedFieldContent>,
    output_file: &String,
    pretty: bool,
) -> Result<(), ()> {
    let mut has_error = false;

    match File::create(&output_file) {
        Ok(mut f) => {
            let mut json = json::JsonValue::new_array();
            for tagged_field in tagged_fields {
                let _ = json.push(tagged_field.to_json());
            }

            if pretty {
                if let Err(e) = f.write_all(json::stringify_pretty(json, 2).as_bytes()) {
                    error!("Try to write string table to {} failed, {}", output_file, e);
                    has_error = true;
                }
            } else if let Err(e) = f.write_all(json::stringify(json).as_bytes()) {
                error!("Try to write string table to {} failed, {}", output_file, e);
                has_error = true;
            }
        }
        Err(e) => {
            error!(
                "Try to open {} to write string table failed, {}",
                output_file, e
            );
            has_error = true;
        }
    }

    if has_error {
        Err(())
    } else {
        Ok(())
    }
}
