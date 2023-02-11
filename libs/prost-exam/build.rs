use std::fs;
//use std::fs::ReadDir;
use std::io::Result;

//use std::env;
//use std::path::Path;
//use std::fs;

use prost_build::Config;

const PROTO_ROOT: &str = "protos";
const OUT_DIR: &str = "src";

//#[macro_export]
mod console {
    macro_rules! log {
        ($($msg: tt)*) => {
            println!("cargo:warning={}", format!($($msg)*))
        }
    }

    pub(crate) use log;
}

fn main() -> Result<()> {

    console::log!(">>> prost-exam <<<");

    let _ = fs::create_dir("src");

    //for file in fs::read_dir("src/protos").unwrap() {
    //    console::log!("{}", file.unwrap().path().display());
    //}

    let protos = &[
        "protos/segment.proto",
        "protos/sample.proto"
    ];

    let includes = &[PROTO_ROOT];

    Config::new()
        .include_file("lib.rs")
        .out_dir(OUT_DIR)
        .compile_protos(protos, includes)
        .unwrap();

    //builder.file_descriptor_set_path("src/file_descriptor.bin");

    //builder.out_dir(OUT_DIR);
    ////builder.btree_map(&["."]);
    ////builder.enum_attribute(".", "#[derive(Sized)]");
    //builder.compile_protos(&[
    //    "protos/segment.proto",
    //    "protos/sample.proto"
    //    ],
    //    &["protos"]
    //).unwrap();

    Ok(())
}

