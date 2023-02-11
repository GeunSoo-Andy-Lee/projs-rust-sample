use std::{fs, io};
//use std::fs::ReadDir;
use std::io::Result;

//use std::env;
//use std::path::Path;
//use std::fs;

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

    console::log!(">>> protobuf-codegen-exam <<<");

    //let _ = fs::create_dir("src/protoc");

    //for file in fs::read_dir("src/protos").unwrap() {
    //    console::log!("{}", file.unwrap().path().display());
    //}

    protobuf_codegen::Codegen::new()
        .protoc()
        //.cargo_out_dir("generated_with_native")
        .includes(&["protos"])
        .inputs(["protos/segment.proto", "protos/sample.proto"])
        .out_dir(OUT_DIR)
        .run_from_script();

    Ok(())
}

