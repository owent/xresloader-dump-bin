use std::any::Any;
use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use std::rc::Rc;

use super::dump_options::DumpOptions;
use super::dump_plugin;
use super::utility;

use protobuf::MessageDyn;

struct StringTableContent {
    pub head: Rc<dump_plugin::DumpPluginBlockDataSource>,
    pub body: HashMap<String, HashSet<dump_plugin::DumpPluginItemDataSource>>,
}

impl dump_plugin::DumpPluginBlockInterface for StringTableContent {
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
struct StringTableFilter {
    pub value_include_regex_rules: Vec<regex::Regex>,
    pub value_exclude_regex_rules: Vec<regex::Regex>,
    pub include_message_paths: HashSet<String>,
    pub exclude_message_paths: HashSet<String>,
    pub include_field_paths: HashSet<String>,
    pub exclude_field_paths: HashSet<String>,
}

impl StringTableFilter {
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

    pub fn filter_field(&self, field_desc: &protobuf::reflect::FieldDescriptor) -> bool {
        if self.include_field_paths.is_empty() && self.exclude_field_paths.is_empty() {
            return true;
        }

        let full_name = field_desc.full_name();

        if !self.include_field_paths.is_empty() && !self.include_field_paths.contains(&full_name) {
            return false;
        }

        if self.exclude_field_paths.contains(&full_name) {
            return false;
        }

        true
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

impl StringTableContent {
    pub fn load_message(
        &mut self,
        message: &dyn MessageDyn,
        filter: &StringTableFilter,
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
                        match v {
                            protobuf::reflect::ReflectValueRef::Message(m) => {
                                self.load_message(m.deref(), filter, data_source);
                            }
                            protobuf::reflect::ReflectValueRef::String(s) => {
                                if !filter.filter_field(&field) {
                                    return;
                                }

                                if !filter.filter_value(s) {
                                    return;
                                }

                                let value = v.to_string();
                                if let Some(item) = self.body.get_mut(&value) {
                                    let item_ds = &data_source.item;
                                    if !item.contains(item_ds) {
                                        item.insert(item_ds.as_ref().clone());
                                    }
                                } else {
                                    let mut hs = HashSet::new();
                                    hs.insert(data_source.item.as_ref().clone());
                                    self.body.insert(value, hs);
                                }
                            }
                            _ => {}
                        }
                    }
                }
                protobuf::reflect::RuntimeFieldType::Repeated(_) => {
                    if !filter.filter_field(&field) {
                        return;
                    }

                    field
                        .get_repeated(message)
                        .into_iter()
                        .for_each(|v| match v {
                            protobuf::reflect::ReflectValueRef::Message(m) => {
                                self.load_message(m.deref(), filter, data_source);
                            }
                            protobuf::reflect::ReflectValueRef::String(s) => {
                                if !filter.filter_field(&field) {
                                    return;
                                }

                                if !filter.filter_value(s) {
                                    return;
                                }

                                let value = v.to_string();
                                if let Some(item) = self.body.get_mut(&value) {
                                    let item_ds = &data_source.item;
                                    if !item.contains(item_ds) {
                                        item.insert(item_ds.as_ref().clone());
                                    }
                                } else {
                                    let mut hs = HashSet::new();
                                    hs.insert(data_source.item.as_ref().clone());
                                    self.body.insert(value, hs);
                                }
                            }
                            _ => {}
                        })
                }
                protobuf::reflect::RuntimeFieldType::Map(_, _) => {
                    field.get_map(message).into_iter().for_each(|(k, v)| {
                        match k {
                            protobuf::reflect::ReflectValueRef::Message(m) => {
                                self.load_message(m.deref(), filter, data_source);
                            }
                            protobuf::reflect::ReflectValueRef::String(s) => {
                                if !filter.filter_field(&field) {
                                    return;
                                }

                                if !filter.filter_value(s) {
                                    return;
                                }

                                let value = v.to_string();
                                if let Some(item) = self.body.get_mut(&value) {
                                    let item_ds = &data_source.item;
                                    if !item.contains(item_ds) {
                                        item.insert(item_ds.as_ref().clone());
                                    }
                                } else {
                                    let mut hs = HashSet::new();
                                    hs.insert(data_source.item.as_ref().clone());
                                    self.body.insert(value, hs);
                                }
                            }
                            _ => {}
                        }

                        match v {
                            protobuf::reflect::ReflectValueRef::Message(m) => {
                                self.load_message(m.deref(), filter, data_source);
                            }
                            protobuf::reflect::ReflectValueRef::String(s) => {
                                if !filter.filter_field(&field) {
                                    return;
                                }

                                if !filter.filter_value(s) {
                                    return;
                                }

                                let value = v.to_string();
                                if let Some(item) = self.body.get_mut(&value) {
                                    let item_ds = &data_source.item;
                                    if !item.contains(item_ds) {
                                        item.insert(item_ds.as_ref().clone());
                                    }
                                } else {
                                    let mut hs = HashSet::new();
                                    hs.insert(data_source.item.as_ref().clone());
                                    self.body.insert(value, hs);
                                }
                            }
                            _ => {}
                        }
                    });
                }
            });
    }

    pub fn to_json(
        &self,
        json_item_head: json::JsonValue,
        output_ordered: bool,
    ) -> json::JsonValue {
        let mut json_item = json::JsonValue::new_object();
        let _ = json_item.insert("head", json_item_head);

        if output_ordered {
            let mut json_item_body = json::JsonValue::new_array();

            utility::for_each_ordered_hash_map(&self.body, |key, value| {
                let mut body_item = json::JsonValue::new_object();
                let mut body_item_source = json::JsonValue::new_array();
                utility::for_each_ordered_hash_set_by(
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

fn build_string_table_filter(args: &DumpOptions) -> (StringTableFilter, bool) {
    let mut ret: StringTableFilter = StringTableFilter::default();
    let mut has_error = false;

    for regex_rule in &args.string_table_include_value_regex_rule {
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

    for file_path in &args.string_table_include_value_regex_file {
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

    for regex_rule in &args.string_table_exclude_value_regex_rule {
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

    for file_path in &args.string_table_exclude_value_regex_file {
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

    for field_path_file in &args.string_table_include_field_path_file {
        utility::load_file_by_lines(field_path_file, "field path", &mut has_error, |line| {
            ret.include_field_paths.insert(line.to_string());
            Ok(())
        });
    }

    for field_path_file in &args.string_table_exclude_field_path_file {
        utility::load_file_by_lines(field_path_file, "field path", &mut has_error, |line| {
            ret.exclude_field_paths.insert(line.to_string());
            Ok(())
        });
    }

    for field_path_file in &args.string_table_include_message_path_file {
        utility::load_file_by_lines(field_path_file, "message path", &mut has_error, |line| {
            ret.include_message_paths.insert(line.to_string());
            Ok(())
        });
    }

    for field_path_file in &args.string_table_exclude_message_path_file {
        utility::load_file_by_lines(field_path_file, "message path", &mut has_error, |line| {
            ret.exclude_message_paths.insert(line.to_string());
            Ok(())
        });
    }

    (ret, has_error)
}

pub struct DumpPluginStringTable {
    filter: StringTableFilter,
    content: VecDeque<Box<StringTableContent>>,

    // output
    output_pretty: bool,
    output_ordered: bool,
    write_to_text_file: String,
    write_to_json_file: String,
}

impl DumpPluginStringTable {
    pub fn build(args: &DumpOptions) -> (Option<Box<dyn dump_plugin::DumpPluginInterface>>, bool) {
        if args.output_string_table_json.is_empty() && args.output_string_table_text.is_empty() {
            return (None, false);
        }

        let (string_table_filter, has_string_table_error) = build_string_table_filter(&args);
        if has_string_table_error {
            return (None, has_string_table_error);
        }

        (
            Some(Box::new(DumpPluginStringTable {
                filter: string_table_filter,
                content: VecDeque::new(),
                output_pretty: args.pretty || args.string_table_pretty,
                output_ordered: args.string_table_ordered,
                write_to_text_file: args.output_string_table_text.clone(),
                write_to_json_file: args.output_string_table_json.clone(),
            })),
            false,
        )
    }
}

impl dump_plugin::DumpPluginInterface for DumpPluginStringTable {
    fn create_block(
        &self,
        data_source: Rc<dump_plugin::DumpPluginBlockDataSource>,
    ) -> Option<Box<dyn dump_plugin::DumpPluginBlockInterface>> {
        Some(Box::new(StringTableContent {
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
        if let Some(rb) = block.as_any_mut().downcast_mut::<StringTableContent>() {
            rb.load_message(message, &mut self.filter, &data_source);
        } else {
            error!(
                "In DumpPluginStringTable::load_message, the block is not StringTableContent, ignore this message"
            );
            return;
        }
    }

    fn push_block(&mut self, block: Box<dyn dump_plugin::DumpPluginBlockInterface>) {
        if let Ok(rb) = block.into_any().downcast::<StringTableContent>() {
            self.content.push_back(rb);
        } else {
            error!(
                "In DumpPluginStringTable::push_block, the block is not StringTableContent, ignore this message"
            );
            return;
        }
    }

    fn to_json(&self) -> Vec<json::JsonValue> {
        let mut ret = Vec::with_capacity(self.content.len());
        for tagged_field in &self.content {
            ret.push(tagged_field.to_json(
                self.header_to_json(tagged_field.head.as_ref()),
                self.output_ordered,
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
        if self.output_ordered {
            ret.sort();
        }
        ret
    }

    fn flush(&self) -> dump_plugin::DumpPluginFlushResult {
        let mut ret = Ok(());
        if !self.write_to_text_file.is_empty() {
            if let Err(e) = self.dump_to_text_file(&self.write_to_text_file) {
                ret = Err(e);
            }
        }

        if !self.write_to_json_file.is_empty() {
            if let Err(e) = self.dump_to_json_file(&self.write_to_json_file, self.output_pretty) {
                ret = Err(e);
            }
        }

        ret
    }
}
