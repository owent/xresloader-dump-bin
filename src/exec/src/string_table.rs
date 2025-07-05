use std::collections::{HashMap, HashSet, LinkedList};
use std::fs::File;
use std::io::Write;
use std::ops::Deref;

use super::dump_options::DumpOptions;
use super::utility;

use protobuf::MessageDyn;
use xresloader_protocol::proto::pb_header_v3::{Xresloader_data_source, Xresloader_datablocks};

#[derive(Clone)]
pub struct StringTableDataSource {
    pub file: ::std::string::String,
    pub sheet: ::std::string::String,
    pub count: i32,
}

pub struct StringTableBinarySource {
    pub xres_ver: ::std::string::String,
    pub data_ver: ::std::string::String,
    pub bin_file: ::std::string::String,
    pub count: u32,
    pub hash_code: ::std::string::String,
    pub description: ::std::string::String,
    pub data_source: ::std::vec::Vec<StringTableDataSource>,
}

pub struct StringTableItemSource {
    pub file: ::std::string::String,
    pub sheet: ::std::string::String,
}

pub struct StringTableContent {
    pub head: StringTableBinarySource,
    pub body: HashMap<String, LinkedList<StringTableItemSource>>,
}

#[derive(Default)]
pub struct StringTableFilter {
    pub value_include_regex_rules: Vec<regex::Regex>,
    pub value_exclude_regex_rules: Vec<regex::Regex>,
    pub include_message_paths: HashSet<String>,
    pub exclude_message_paths: HashSet<String>,
    pub include_field_paths: HashSet<String>,
    pub exclude_field_paths: HashSet<String>,
}

impl StringTableDataSource {
    pub fn default() -> Self {
        StringTableDataSource {
            file: String::from("[UNKNOWN]"),
            sheet: String::from("[UNKNOWN]"),
            count: 0,
        }
    }

    pub fn new(data_source: &Xresloader_data_source) -> Self {
        StringTableDataSource {
            file: data_source.file.clone(),
            sheet: data_source.sheet.clone(),
            count: data_source.count,
        }
    }
}

impl StringTableBinarySource {
    pub fn new(data_blocks: &Xresloader_datablocks, bin_file: String) -> Self {
        StringTableBinarySource {
            xres_ver: data_blocks.header.xres_ver.clone(),
            data_ver: data_blocks.header.data_ver.clone(),
            bin_file,
            count: data_blocks.header.count,
            hash_code: data_blocks.header.hash_code.clone(),
            description: data_blocks.header.description.clone(),
            data_source: {
                let mut data_source = Vec::new();
                for source in &data_blocks.header.data_source {
                    data_source.push(StringTableDataSource {
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

impl StringTableFilter {
    pub fn filter_value(&self, input: &str) -> bool {
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

    pub fn filter_field_full_name(&self, input: &str) -> bool {
        if !self.include_field_paths.is_empty() && !self.include_field_paths.contains(input) {
            return false;
        }

        if self.exclude_field_paths.contains(input) {
            return false;
        }

        true
    }

    pub fn filter_message_full_name(&self, input: &str) -> bool {
        if !self.include_message_paths.is_empty() && !self.include_message_paths.contains(input) {
            return false;
        }

        if self.exclude_message_paths.contains(input) {
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
        data_source: &StringTableDataSource,
    ) {
        if !filter.filter_message_full_name(message.descriptor_dyn().full_name()) {
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
                                if !filter.filter_field_full_name(&field.full_name()) {
                                    return;
                                }

                                if !filter.filter_value(s) {
                                    return;
                                }

                                let value = v.to_string();
                                if let Some(item) = self.body.get_mut(&value) {
                                    item.push_back(StringTableItemSource {
                                        file: data_source.file.clone(),
                                        sheet: data_source.sheet.clone(),
                                    });
                                } else {
                                    let mut ls = LinkedList::new();
                                    ls.push_back(StringTableItemSource {
                                        file: data_source.file.clone(),
                                        sheet: data_source.sheet.clone(),
                                    });
                                    self.body.insert(value, ls);
                                }
                            }
                            _ => {}
                        }
                    }
                }
                protobuf::reflect::RuntimeFieldType::Repeated(_) => {
                    if !filter.filter_field_full_name(&field.full_name()) {
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
                                if !filter.filter_value(s) {
                                    return;
                                }

                                let value = v.to_string();
                                if let Some(item) = self.body.get_mut(&value) {
                                    item.push_back(StringTableItemSource {
                                        file: data_source.file.clone(),
                                        sheet: data_source.sheet.clone(),
                                    });
                                } else {
                                    let mut ls = LinkedList::new();
                                    ls.push_back(StringTableItemSource {
                                        file: data_source.file.clone(),
                                        sheet: data_source.sheet.clone(),
                                    });
                                    self.body.insert(value, ls);
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
                                if !filter.filter_field_full_name(&field.full_name()) {
                                    return;
                                }

                                if !filter.filter_value(s) {
                                    return;
                                }

                                let value = v.to_string();
                                if let Some(item) = self.body.get_mut(&value) {
                                    item.push_back(StringTableItemSource {
                                        file: data_source.file.clone(),
                                        sheet: data_source.sheet.clone(),
                                    });
                                } else {
                                    let mut ls = LinkedList::new();
                                    ls.push_back(StringTableItemSource {
                                        file: data_source.file.clone(),
                                        sheet: data_source.sheet.clone(),
                                    });
                                    self.body.insert(value, ls);
                                }
                            }
                            _ => {}
                        }

                        match v {
                            protobuf::reflect::ReflectValueRef::Message(m) => {
                                self.load_message(m.deref(), filter, data_source);
                            }
                            protobuf::reflect::ReflectValueRef::String(s) => {
                                if !filter.filter_field_full_name(&field.full_name()) {
                                    return;
                                }

                                if !filter.filter_value(s) {
                                    return;
                                }

                                let value = v.to_string();
                                if let Some(item) = self.body.get_mut(&value) {
                                    item.push_back(StringTableItemSource {
                                        file: data_source.file.clone(),
                                        sheet: data_source.sheet.clone(),
                                    });
                                } else {
                                    let mut ls = LinkedList::new();
                                    ls.push_back(StringTableItemSource {
                                        file: data_source.file.clone(),
                                        sheet: data_source.sheet.clone(),
                                    });
                                    self.body.insert(value, ls);
                                }
                            }
                            _ => {}
                        }
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

pub fn build_string_table_filter(args: &DumpOptions) -> (bool, StringTableFilter) {
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

    (has_error, ret)
}

pub fn dump_string_table_to_text_file(
    string_tables: &Vec<StringTableContent>,
    output_file: &String,
) -> Result<(), ()> {
    let mut has_error = false;
    match File::create(&output_file) {
        Ok(mut f) => {
            let mut text: HashSet<String> = HashSet::new();
            for string_table in string_tables {
                text.extend(string_table.to_text());
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

pub fn dump_string_table_to_json_file(
    string_tables: &Vec<StringTableContent>,
    output_file: &String,
    pretty: bool,
) -> Result<(), ()> {
    let mut has_error = false;

    match File::create(&output_file) {
        Ok(mut f) => {
            let mut json = json::JsonValue::new_array();
            for string_table in string_tables {
                let _ = json.push(string_table.to_json());
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
