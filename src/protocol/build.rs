use std::fs;

// extern crate env_logger;
// extern crate log;

// use protobuf_codegen::CodeGen;

fn codegen() -> protobuf_codegen::Codegen {
    let mut codegen = protobuf_codegen::Codegen::new();
    codegen
        // There is no available `protoc` for v4 now, so we use the vendored one.
        .protoc_path(&protoc_bin_vendored::protoc_bin_path().unwrap())
        .out_dir("src/proto")
        .inputs([
            "../../third_party/xresloader-protocol/core/pb_header_v3.proto",
            "../../third_party/xresloader-protocol/core/extensions/v3/xresloader.proto",
            "../../third_party/xresloader-protocol/core/extensions/v3/xresloader_ue.proto",
        ])
        .include("../../third_party/xresloader-protocol/core/extensions/v3")
        .include("../../third_party/xresloader-protocol/core")
        // For v4
        // .dependency(protobuf_well_known_types::get_dependency(
        //     "protobuf_well_known_types",
        // ))
    ;

    codegen
}

fn main() {
    env_logger::init();

    codegen().run().expect("protoc");
    // codegen().generate_and_compile().unwrap();
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
    println!(
        "cargo:rerun-if-changed=../../third_party/xresloader-protocol/core/extensions/v3/xresloader_ue.proto"
    );
}
