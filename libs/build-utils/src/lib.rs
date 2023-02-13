use std::{env, fs, io};
use std::fs::{DirEntry, metadata};
use std::path::{Path, PathBuf};

#[macro_export]
macro_rules! echo {
    ($($msg: tt)*) => {
        println!("cargo:warning={}", format!($($msg)*))
    }
}

#[macro_export]
macro_rules! watch {
    ($($msg: tt)*) => {
        println!("cargo:rerun-if-changed={}", format!($($msg)*))
    }
}

//#[macro_export]
//macro_rules! read_dirs {
//    ($($msg: tt)*) => {
//        $crate::_read_dirs(&format!($($msg)*))
//    }
//}

pub fn root_path() -> PathBuf {
    let output = std::process::Command::new(env!("CARGO"))
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()
        .unwrap()
        .stdout;

    let path = Path::new(std::str::from_utf8(&output).unwrap().trim());

    path.parent().unwrap().to_path_buf()
    //    .into_os_string().into_string().unwrap()
}

// CARGO_MANIFEST_DIR: D:\rust\projs-rust-sample\libs\msg-prost
pub fn project_path() -> PathBuf {
    //env::var("CARGO_MANIFEST_DIR").unwrap()
    Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).to_path_buf()
}

// OUT_DIR: D:\rust\projs-rust-sample\target\debug\build\msg-prost-3fda4518daa9dfa7\out
pub fn output_path() -> PathBuf {
    //env::var("OUT_DIR").unwrap()
    Path::new(&env::var("OUT_DIR").unwrap()).to_path_buf()
}

pub fn read_dirs(path: &Path) -> Vec<String> {
    let mut vec = Vec::new();
    _read_dirs_impl(path, &mut vec);
    vec
}

fn _read_dirs_impl (path: &Path, output: &mut Vec<String>) {
    if metadata(&path).unwrap().is_dir() {
        for path_result in fs::read_dir(&path).unwrap() {
            let path_buf = path_result.unwrap().path();
            if metadata(&path_buf).unwrap().is_dir() {
                _read_dirs_impl(&path_buf, output);
            } else {
                output.push(
                path_buf.into_os_string().into_string().unwrap()
                );
            }
        }
    }
}