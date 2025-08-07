use std::any::Any;
use std::fs::File;
use std::io::Write;
use std::rc::Rc;

use protobuf::MessageDyn;
use xresloader_protocol::proto::pb_header_v3::{Xresloader_data_source, Xresloader_datablocks};
// use xresloader_protocol::proto::xresloader::exts::field_tag;
// use xresloader_protocol::proto::xresloader::exts::oneof_tag;

use super::utility;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct DumpPluginItemDataSource {
    pub file: ::std::string::String,
    pub sheet: ::std::string::String,
}

#[derive(Clone)]
pub struct DumpPluginSheetDataSource {
    pub item: Rc<DumpPluginItemDataSource>,
    pub count: i32,
}

pub struct DumpPluginBlockDataSource {
    pub xres_ver: ::std::string::String,
    pub data_ver: ::std::string::String,
    pub file_path: ::std::string::String,
    pub count: u32,
    pub hash_code: ::std::string::String,
    pub description: ::std::string::String,
    pub data_source: ::std::vec::Vec<DumpPluginSheetDataSource>,
}

impl DumpPluginSheetDataSource {
    pub fn default() -> Self {
        DumpPluginSheetDataSource {
            item: Rc::new(DumpPluginItemDataSource {
                file: String::from("[UNKNOWN]"),
                sheet: String::from("[UNKNOWN]"),
            }),
            count: 0,
        }
    }

    pub fn new(data_source: &Xresloader_data_source) -> Self {
        DumpPluginSheetDataSource {
            item: Rc::new(DumpPluginItemDataSource {
                file: data_source.file.clone(),
                sheet: data_source.sheet.clone(),
            }),
            count: data_source.count,
        }
    }
}

pub type DumpPluginFlushResult = Result<(), ()>;

impl Into<Rc<DumpPluginItemDataSource>> for &DumpPluginSheetDataSource {
    fn into(self) -> Rc<DumpPluginItemDataSource> {
        self.item.clone()
    }
}

impl DumpPluginBlockDataSource {
    pub fn new(data_blocks: &Xresloader_datablocks, file_path: String) -> Rc<Self> {
        Rc::new(DumpPluginBlockDataSource {
            xres_ver: data_blocks.header.xres_ver.clone(),
            data_ver: data_blocks.header.data_ver.clone(),
            file_path,
            count: data_blocks.header.count,
            hash_code: data_blocks.header.hash_code.clone(),
            description: data_blocks.header.description.clone(),
            data_source: {
                let mut data_source = Vec::new();
                for source in &data_blocks.header.data_source {
                    data_source.push(DumpPluginSheetDataSource::new(source));
                }
                data_source
            },
        })
    }
}

pub trait DumpPluginBlockInterface: Any {
    // 提供向下转换的辅助方法
    #[allow(dead_code)]
    fn as_any(&self) -> &dyn Any;

    #[allow(dead_code)]
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

pub trait DumpPluginInterface {
    // fn build_into(plugins: &mut Vec<Box<dyn Self>>, args: &DumpOptions) -> bool;

    fn create_block(
        &self,
        data_source: Rc<DumpPluginBlockDataSource>,
    ) -> Option<Box<dyn DumpPluginBlockInterface>>;

    fn load_message(
        &mut self,
        block: &mut Box<dyn DumpPluginBlockInterface>,
        message: &dyn MessageDyn,
        data_source: &DumpPluginSheetDataSource,
    );

    fn push_block(&mut self, block: Box<dyn DumpPluginBlockInterface>);

    fn header_to_json(&self, head: &DumpPluginBlockDataSource) -> json::JsonValue {
        let mut json_item_head = json::JsonValue::new_object();

        let _ = json_item_head.insert("xres_ver", head.xres_ver.clone());
        let _ = json_item_head.insert("data_ver", head.data_ver.clone());
        let _ = json_item_head.insert("file_path", head.file_path.clone());
        let _ = json_item_head.insert("count", head.count);
        let _ = json_item_head.insert("hash_code", head.hash_code.clone());
        let _ = json_item_head.insert("description", head.description.clone());
        let _ = json_item_head.insert("data_source", {
            let mut ds = json::JsonValue::new_array();
            utility::for_each_ordered_vec_by(
                &head.data_source,
                |a, b| {
                    if a.item.file == b.item.file {
                        a.item.sheet.cmp(&b.item.sheet)
                    } else {
                        a.item.file.cmp(&b.item.file)
                    }
                },
                |source| {
                    let mut d = json::JsonValue::new_object();
                    let _ = d.insert("file", source.item.file.clone());
                    let _ = d.insert("sheet", source.item.sheet.clone());
                    if source.count > 0 {
                        let _ = d.insert("count", source.count);
                    }
                    let _ = ds.push(d);
                },
            );

            ds
        });

        json_item_head
    }

    fn to_json(&self) -> Vec<json::JsonValue>;

    fn to_text(&self) -> Vec<String>;

    fn flush(&self) -> DumpPluginFlushResult;

    fn dump_to_json_file(self: &Self, output_file: &String, pretty: bool) -> DumpPluginFlushResult {
        let mut has_error = false;

        match File::create(&output_file) {
            Ok(mut f) => {
                let mut json = json::JsonValue::new_array();
                for field in self.to_json() {
                    let _ = json.push(field);
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

        if has_error { Err(()) } else { Ok(()) }
    }

    fn dump_to_text_file(self: &Self, output_file: &String) -> DumpPluginFlushResult {
        let mut has_error = false;
        match File::create(&output_file) {
            Ok(mut f) => {
                for line in self.to_text() {
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

        if has_error { Err(()) } else { Ok(()) }
    }
}
