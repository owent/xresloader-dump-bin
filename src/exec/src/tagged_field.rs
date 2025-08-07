use std::any::Any;
use std::boxed::Box;
use std::collections::VecDeque;
use std::collections::{HashMap, HashSet, LinkedList};
use std::ops::Deref;
use std::rc::Rc;

use super::dump_options::DumpOptions;
use super::dump_plugin;
use super::utility;

use protobuf::reflect::ReflectValueRef;

use protobuf::{Message, MessageDyn};

struct TaggedFieldContent {
    pub head: Rc<dump_plugin::DumpPluginBlockDataSource>,
    pub body: HashMap<String, LinkedList<Rc<dump_plugin::TaggedFieldItemDataSource>>>,
}

impl dump_plugin::DumpPluginBlockInterface for TaggedFieldContent {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

#[derive(Default)]
struct TaggedFieldFilter {
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
                            error!(
                                "Failed to parse field tag {}(which should be org.xresloader.field_tag) as string, maybe corrupted data, {}",
                                field_tag_number, e
                            );
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
                                    error!(
                                        "Failed to parse field tag {}(which should be org.xresloader.oneof_tag) as string, maybe corrupted data, {}",
                                        field_oneof_number, e
                                    );
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
        data_source: &dump_plugin::DumpPluginSheetDataSource,
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
                                item.push_back(data_source.into());
                            } else {
                                let mut ls = LinkedList::new();
                                ls.push_back(data_source.into());
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
                                item.push_back(data_source.into());
                            } else {
                                let mut ls = LinkedList::new();
                                ls.push_back(data_source.into());
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
                                item.push_back(data_source.into());
                            } else {
                                let mut ls = LinkedList::new();
                                ls.push_back(data_source.into());
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
                                item.push_back(data_source.into());
                            } else {
                                let mut ls = LinkedList::new();
                                ls.push_back(data_source.into());
                                self.body.insert(value, ls);
                            }
                        };
                    });
                }
            });
    }

    pub fn to_json(
        &self,
        json_item_head: json::JsonValue,
        ordered_output: bool,
    ) -> json::JsonValue {
        let mut json_item = json::JsonValue::new_object();
        let _ = json_item.insert("head", json_item_head);

        if ordered_output {
            let mut json_item_body = json::JsonValue::new_array();

            utility::for_each_ordered_hash_map(&self.body, |key, value| {
                let mut body_item = json::JsonValue::new_object();
                let mut body_item_source = json::JsonValue::new_array();
                utility::for_each_ordered_linked_list_by(
                    value,
                    |a, b| {
                        if a.file == b.file {
                            a.sheet.cmp(&b.sheet)
                        } else {
                            a.file.cmp(&b.file)
                        }
                    },
                    |source| {
                        let mut d = json::JsonValue::new_object();
                        let _ = d.insert("file", source.file.clone());
                        let _ = d.insert("sheet", source.sheet.clone());
                        let _ = body_item_source.push(d);
                    },
                );
                let _ = body_item.insert("source", body_item_source);
                let mut json_item_body_data = json::JsonValue::new_object();
                let _ = json_item_body_data.insert(key, body_item);
                let _ = json_item_body.push(json_item_body_data);
            });
            let _ = json_item.insert("body", json_item_body);
        } else {
            let mut json_item_body = json::JsonValue::new_object();
            for (key, value) in &self.body {
                let mut body_item = json::JsonValue::new_object();
                let mut body_item_source = json::JsonValue::new_array();
                for source in value {
                    let mut d = json::JsonValue::new_object();
                    let _ = d.insert("file", source.file.clone());
                    let _ = d.insert("sheet", source.sheet.clone());
                    let _ = body_item_source.push(d);
                }
                let _ = body_item.insert("source", body_item_source);
                let _ = json_item_body.insert(key, body_item);
            }
            let _ = json_item.insert("body", json_item_body);
        }

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

fn build_tagged_field_filter(args: &DumpOptions) -> (TaggedFieldFilter, bool) {
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

pub struct DumpPluginTaggedField {
    filter: TaggedFieldFilter,
    content: VecDeque<Box<TaggedFieldContent>>,

    // output
    ordered_output: bool,
    write_to_text_file: String,
    write_to_json_file: String,
}

impl DumpPluginTaggedField {
    pub fn build(args: &DumpOptions) -> (Option<Box<dyn dump_plugin::DumpPluginInterface>>, bool) {
        if args.output_tagged_data_json.is_empty() && args.output_tagged_data_text.is_empty() {
            return (None, false);
        }

        let (tagged_field_filter, has_tagged_field_error) = build_tagged_field_filter(&args);
        if has_tagged_field_error {
            return (None, has_tagged_field_error);
        }

        (
            Some(Box::new(DumpPluginTaggedField {
                filter: tagged_field_filter,
                content: VecDeque::new(),
                ordered_output: args.tagged_data_ordered,
                write_to_text_file: args.output_tagged_data_text.clone(),
                write_to_json_file: args.output_tagged_data_json.clone(),
            })),
            false,
        )
    }
}

impl dump_plugin::DumpPluginInterface for DumpPluginTaggedField {
    fn create_block(
        &self,
        data_source: Rc<dump_plugin::DumpPluginBlockDataSource>,
    ) -> Option<Box<dyn dump_plugin::DumpPluginBlockInterface>> {
        Some(Box::new(TaggedFieldContent {
            head: data_source,
            body: HashMap::new(),
        }))
    }

    fn load_message(
        &mut self,
        block: &mut Box<dyn dump_plugin::DumpPluginBlockInterface>,
        message: &dyn MessageDyn,
        data_source: &dump_plugin::DumpPluginSheetDataSource,
    ) {
        if let Some(rb) = block.as_any_mut().downcast_mut::<TaggedFieldContent>() {
            rb.load_message(message, &mut self.filter, &data_source);
        } else {
            error!(
                "In DumpPluginTaggedField::load_message, the block is not TaggedFieldContent, ignore this message"
            );
            return;
        }
    }

    fn push_block(&mut self, block: Box<dyn dump_plugin::DumpPluginBlockInterface>) {
        if let Ok(rb) = block.into_any().downcast::<TaggedFieldContent>() {
            self.content.push_back(rb);
        } else {
            error!(
                "In DumpPluginTaggedField::push_block, the block is not TaggedFieldContent, ignore this message"
            );
            return;
        }
    }

    fn to_json(&self) -> Vec<json::JsonValue> {
        let mut ret = Vec::with_capacity(self.content.len());
        for tagged_field in &self.content {
            ret.push(tagged_field.to_json(
                self.header_to_json(tagged_field.head.as_ref()),
                self.ordered_output,
            ));
        }
        ret
    }

    fn to_text(&self) -> Vec<String> {
        let mut text: HashSet<String> = HashSet::new();
        for tagged_field in &self.content {
            text.extend(tagged_field.to_text());
        }

        let mut ret = Vec::with_capacity(text.len());
        ret.extend(text.iter().cloned());
        ret.sort();
        ret
    }

    fn flush(&self, pretty: bool) -> dump_plugin::DumpPluginFlushResult {
        let mut ret = Ok(());
        if !self.write_to_text_file.is_empty() {
            if let Err(e) = self.dump_to_text_file(&self.write_to_text_file) {
                ret = Err(e);
            }
        }

        if !self.write_to_json_file.is_empty() {
            if let Err(e) = self.dump_to_json_file(&self.write_to_json_file, pretty) {
                ret = Err(e);
            }
        }

        ret
    }
}
