// When the `system-alloc` feature is used, use the System Allocator
use std::alloc::System;
#[global_allocator]
static GLOBAL: System = System;

extern crate xresloader_protocol;

extern crate clap;

#[macro_use]
extern crate log;
extern crate bytes;
extern crate env_logger;
extern crate json;
extern crate protobuf_json_mapping;
extern crate regex;

use crate::clap::Parser;

use std::collections::HashMap;
use std::io::Read;

use protobuf::{descriptor::FileDescriptorSet, Message, MessageFull};
// use xresloader_protocol::proto::Xresloader_datablocks;

mod dump_options;
mod file_descriptor_index;
mod logger;
mod ordered_generator;
mod string_table;
mod tagged_field;
mod utility;

type DumpOptions = dump_options::DumpOptions;
use file_descriptor_index::FileDescriptorIndex;

fn main() {
    let args = DumpOptions::parse();

    if args.debug {
        let _ = logger::Logger::new(log::LevelFilter::Debug).init();
    } else {
        let _ = logger::Logger::new(log::LevelFilter::Info).init();
    }

    let mut desc_index = FileDescriptorIndex::new();

    let mut string_tables: Vec<string_table::StringTableContent> = vec![];
    let (string_table_filter, has_string_table_error) =
        string_table::build_string_table_filter(&args);

    let mut tagged_fields: Vec<tagged_field::TaggedFieldContent> = vec![];
    let (mut tagged_field_filter, has_tagged_field_error) =
        tagged_field::build_tagged_field_filter(&args);

    for pb_file in args.pb_file {
        debug!("Load pb file: {}", pb_file);
        match std::fs::OpenOptions::new()
            .read(true)
            .write(false)
            .open(&pb_file)
        {
            Ok(mut f) => {
                let mut bin_data = Vec::new();
                let _ = f.read_to_end(&mut bin_data);
                match FileDescriptorSet::parse_from_bytes(&bin_data) {
                    Ok(pbs) => {
                        debug!("Parse pb file: {} success", pb_file);
                        for pb_file_unit in &pbs.file {
                            debug!(
                                "  Found proto file: {} has {} message(s) and {} enum(s)",
                                pb_file_unit.name(),
                                pb_file_unit.message_type.len(),
                                pb_file_unit.enum_type.len()
                            );
                            desc_index.add_file(pb_file_unit, &pb_file);
                        }
                    }
                    Err(e) => {
                        error!("Parse pb file {} failed, {}, ignore this file", pb_file, e);
                    }
                }
            }
            Err(e) => {
                error!(
                    "Try to open file {} failed, {}, ignore this file",
                    pb_file, e
                );
            }
        }
    }

    let mut has_error = has_string_table_error || has_tagged_field_error;

    for bin_file in args.bin_file {
        debug!("Load xresloader output binary file: {}", bin_file);
        match std::fs::OpenOptions::new()
            .read(true)
            .write(false)
            .open(&bin_file)
        {
            Ok(mut f) => {
                let mut bin_data = Vec::new();
                let _ = f.read_to_end(&mut bin_data);
                match xresloader_protocol::proto::pb_header_v3::Xresloader_datablocks::parse_from_bytes(&bin_data) {
                    Ok(data_blocks) => {
                        if data_blocks.data_message_type.is_empty() {
                            has_error = true;
                            error!("File {} has no data_message_type, please use xresloader 2.6 or upper", &bin_file);
                            continue;
                        }
                        debug!("Parse {} from file: {} success, message type: {}",
                            xresloader_protocol::proto::pb_header_v3::Xresloader_datablocks::descriptor().full_name(), &bin_file,
                            &data_blocks.data_message_type
                        );

                        let message_descriptor = match desc_index.build_message_descriptor(&data_blocks.data_message_type) {
                            Ok(x) => x,
                            Err(_) => {
                                error!("Build message descriptor {} failed", &data_blocks.data_message_type);
                                has_error = true;
                                continue;
                            }
                        };

                        if !args.silence {
                            info!("======================== Header: {} ========================", &bin_file);
                            info!("xresloader version: {}", data_blocks.header.xres_ver);
                            info!("data version: {}", data_blocks.header.data_ver);
                            info!("data count: {}", data_blocks.header.count);
                            info!("hash code: {}", data_blocks.header.hash_code);
                            info!("description: {}", data_blocks.header.description);
                            if !data_blocks.header.data_source.is_empty() {
                                info!("data source:");
                            }
                            for data_source in &data_blocks.header.data_source {
                                if data_source.count > 0 {
                                    info!("  - file: {}, sheet: {}, count: {}", data_source.file, data_source.sheet, data_source.count);
                                } else {
                                    info!("  - file: {}, sheet: {}", data_source.file, data_source.sheet);
                                }
                            }
                        }

                        let mut current_string_table_head : Option<string_table::StringTableBinarySource> = None;

                        if !args.output_string_table_json.is_empty() || !args.output_string_table_text.is_empty() {
                            current_string_table_head = Some(string_table::StringTableBinarySource::new(&data_blocks, bin_file.clone()));
                        }

                        let mut current_tagged_field_head : Option<tagged_field::TaggedFieldBinarySource> = None;
                        if !args.output_tagged_field_json.is_empty() || !args.output_tagged_field_text.is_empty() {
                            current_tagged_field_head = Some(tagged_field::TaggedFieldBinarySource::new(&data_blocks, bin_file.clone()));
                        }

                        if !args.silence {
                            info!("============ Body: {} -> {} ============", &bin_file, &data_blocks.data_message_type);
                        }
                        let mut row_index = 0;
                        if !args.plain && !args.head_only && !args.silence {
                            info!("[");
                        }

                        let mut current_string_table :Option<string_table::StringTableContent> = None;
                        if let Some(head) = current_string_table_head {
                            current_string_table = Some(string_table::StringTableContent {
                                head,
                                body: HashMap::new(),
                            });
                        }
                        let mut fallback_string_table_data_source =
                            string_table::StringTableDataSource::default();

                        let mut current_tagged_field :Option<tagged_field::TaggedFieldContent> = None;
                        if let Some(head) = current_tagged_field_head {
                            current_tagged_field = Some(tagged_field::TaggedFieldContent {
                                head,
                                body: HashMap::new(),
                            });
                        }
                        let mut fallback_tagged_field_data_source =
                            tagged_field::TaggedFieldDataSource::default();

                        let mut current_data_source_idx = 0;
                        let mut current_data_source_left_row = 0;
                        for row_data_block in &data_blocks.data_block {
                            row_index += 1;
                            if current_data_source_left_row <= 0 && current_data_source_idx < data_blocks.header.data_source.len() {
                                current_data_source_left_row = data_blocks.header.data_source[current_data_source_idx].count;
                                fallback_string_table_data_source = string_table::StringTableDataSource::new(&data_blocks.header.data_source[current_data_source_idx]);
                                fallback_tagged_field_data_source = tagged_field::TaggedFieldDataSource::new(&data_blocks.header.data_source[current_data_source_idx]);
                                current_data_source_idx += 1;
                            }
                            if current_data_source_left_row > 0 {
                                current_data_source_left_row -= 1;
                            }

                            match message_descriptor.parse_from_bytes(row_data_block) {
                                Ok(message) => {
                                    if let Some(ref mut string_table) = current_string_table {
                                        string_table.load_message(message.as_ref(), &string_table_filter, &fallback_string_table_data_source);
                                    }

                                    if let Some(ref mut tagged_field) = current_tagged_field {
                                        tagged_field.load_message(message.as_ref(), &mut tagged_field_filter, &fallback_tagged_field_data_source);
                                    }

                                    if args.head_only || args.silence {
                                        continue;
                                    }

                                    if args.pretty {
                                        if args.plain {
                                            info!("  ------------ Row {} ------------\n{}", row_index, protobuf::text_format::print_to_string_pretty(message.as_ref()));
                                            continue;
                                        }
                                        if let Ok(output) = protobuf_json_mapping::print_to_string(message.as_ref()) {
                                            info!("    {},",  ordered_generator::stringify_pretty(json::parse(&output).unwrap(), 2));
                                        } else {
                                            info!("{}", protobuf::text_format::print_to_string_pretty(message.as_ref()));
                                        }
                                    } else {
                                        if args.plain {
                                            info!("{}", protobuf::text_format::print_to_string(message.as_ref()));
                                            continue;
                                        }
                                        if let Ok(output) = protobuf_json_mapping::print_to_string(message.as_ref()) {
                                            info!("    {},",  ordered_generator::stringify(json::parse(&output).unwrap()));
                                        } else {
                                            info!("{}", protobuf::text_format::print_to_string_pretty(message.as_ref()));
                                        }
                                    }
                                }
                                Err(e) => {
                                    error!("Parse row {} to message {} failed, {}", row_index, &data_blocks.data_message_type, e);
                                    has_error = true;
                                    continue;
                                }
                            }
                        }
                        if !args.plain && !args.head_only && !args.silence {
                            info!("]");
                        }

                        if let Some(string_table) = current_string_table {
                            if !string_table.body.is_empty() {
                                string_tables.push(string_table);
                            }
                        }

                        if let Some(tagged_field) = current_tagged_field {
                            if !tagged_field.body.is_empty() {
                                tagged_fields.push(tagged_field);
                            }
                        }
                    }
                    Err(e) => {
                        error!("Parse {} from file {} failed, {}, ignore this file", xresloader_protocol::proto::pb_header_v3::Xresloader_datablocks::descriptor().full_name(), bin_file, e);
                        has_error = true;
                    }
                }
            }
            Err(e) => {
                error!(
                    "Try to open file {} failed, {}, ignore this file",
                    &bin_file, e
                );
                has_error = true;
            }
        }

        // 2.6.0
    }

    // Dump string table json
    if !args.output_string_table_json.is_empty() {
        if let Err(_) = string_table::dump_string_table_to_json_file(
            &string_tables,
            &args.output_string_table_json,
            args.pretty || args.string_table_pretty,
        ) {
            has_error = true;
        }
    }

    // Dump string table text
    if !args.output_string_table_text.is_empty() {
        if let Err(_) = string_table::dump_string_table_to_text_file(
            &string_tables,
            &args.output_string_table_text,
        ) {
            has_error = true;
        }
    }

    // Dump tagged field json
    if !args.output_tagged_field_json.is_empty() {
        if let Err(_) = tagged_field::dump_tagged_field_to_json_file(
            &tagged_fields,
            &args.output_tagged_field_json,
            args.pretty || args.tagged_field_pretty,
        ) {
            has_error = true;
        }
    }

    // Dump tagged field text
    if !args.output_tagged_field_text.is_empty() {
        if let Err(_) = tagged_field::dump_tagged_field_to_text_file(
            &tagged_fields,
            &args.output_tagged_field_text,
        ) {
            has_error = true;
        }
    }

    if has_error {
        std::process::exit(1);
    }
}
