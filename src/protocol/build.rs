use std::fs;

extern crate env_logger;
extern crate log;

extern crate protobuf_codegen;
extern crate protoc_bin_vendored;

fn codegen() -> protobuf_codegen::Codegen {
    let mut codegen = protobuf_codegen::Codegen::new();
    codegen
        .protoc_path(&protoc_bin_vendored::protoc_bin_path().unwrap())
        .out_dir("src/proto")
        .inputs(&[
            "../../third_party/xresloader-protocol/core/pb_header_v3.proto",
            "../../third_party/xresloader-protocol/core/extensions/v3/xresloader.proto",
            "../../third_party/xresloader-protocol/core/extensions/v3/xresloader_ue.proto",
        ])
        .include("../../third_party/xresloader-protocol/core/extensions/v3")
        .include("../../third_party/xresloader-protocol/core");

    codegen
}

fn main() {
    env_logger::init();

    codegen().run().expect("protoc");
    fs::write(
        "src/proto/mod.rs",
        "pub mod pb_header_v3;
pub mod xresloader;
pub mod xresloader_ue;
",
    )
    .unwrap();

    println!(
        "cargo:rerun-if-changed=../../third_party/xresloader-protocol/core/pb_header_v3.proto"
    );
    println!(
        "cargo:rerun-if-changed=../../third_party/xresloader-protocol/core/extensions/v3/xresloader.proto"
    );
    println!("cargo:rerun-if-changed=../../third_party/xresloader-protocol/core/extensions/v3/xresloader_ue.proto");
}
