use std::{fs, iter};
use std::io::Result;
use std::env;
use std::path::{Path, PathBuf};
use prost_build::Config;

//const PROTO_ROOT: &str = "protos";
//const OUT_DIR: &str = "src";

pub trait PathBufUtils<A> {
    fn combine<T: IntoIterator<Item = A>>(&self, iter: T) -> Self;
    //fn into_string(&self) -> String;
}

impl<P: AsRef<Path>> PathBufUtils<P> for PathBuf {
    fn combine<I: IntoIterator<Item = P>>(&self, iter: I) -> Self {
        let mut clone = self.clone();
        clone.extend(iter);
        clone
    }
}


fn main() -> Result<()> {
    const OUT_DIR: &str = "src";

    build::echo!(">>> Enter msg-protos");
    build::watch!("protos/sample.proto");

    /*
        CARGO_MANIFEST_DIR: D:\rust\projs-rust-sample\libs\msg-prost
        OUT_DIR: D:\rust\projs-rust-sample\target\debug\build\msg-prost-3fda4518daa9dfa7\out
    */
    build::echo!("workspace: {:?}", build::root_path());
    build::echo!("==========================");

    //let mut root = build::root_path();
    //root.extend(["libs", "protos"]);

    let root = build::root_path().combine(["libs", "protos"]);

    //protos: &[impl AsRef<Path>],
    //includes: &[impl AsRef<Path>],

    //let mut protos = Vec::<PathBuf>::new();
    //for file in build::read_dirs(&root) {
    //    //build::echo!("{}", file);
    //    protos.push(PathBuf::from(&file));
    //}

    let protos: Vec<PathBuf> = build::read_dirs(&root).iter()
        .map(|file| PathBuf::from(&file))
        .collect();

    let includes = &[
        root.into_os_string().into_string().unwrap()
    ];

    Config::new()
        .include_file("lib.rs")
        .out_dir(OUT_DIR)
        .compile_protos(&protos, includes)
        .unwrap();

    //for (n,v) in env::vars() {
    //    build_utils::echo!("{}: {}", n,v);
    //}

    //for (n,v) in env::vars() {
    //    build_utils::echo!("{}: {}", n,v);
    //}

    //build_utils::echo!("==========================");

    //for (n,v) in env::vars_os() {
    //    build_utils::echo!("{}: {}", n.into_string().unwrap(), v.into_string().unwrap());
    //}


    let _ = fs::create_dir("src");

    build::echo!("<<< Leave msg-protos");
    Ok(())
}