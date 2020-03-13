#[macro_use]
extern crate serde_derive;

extern crate walkdir;
extern crate wasm_bindgen;

use walkdir::{DirEntry, WalkDir};
use wasm_bindgen::prelude::*;

use std::io;
use std::path::Path;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Serialize)]
pub struct File {
    // {
    //     name: path.parse(item).base, // "my.fastq"
    //     path: item, // "/Users/rpettett/test_sets/zymo/demo/INPUT_PREFIX/my.fastq"
    //     relative: item.replace(rootFolder, ''), // "INPUT_PREFIX/my.fastq"
    //     size: stat.size,
    //   },
    pub name: String,
    pub path: String,
    pub relative: String,
    pub size: u64,
}

fn is_valid(entry: &DirEntry) -> bool {
    let ignored_dirs = ["downloads", "skip", "fail", "fastq_fail", "tmp"];

    entry
        .file_name()
        .to_str()
        .map(|s| !s.starts_with(".") && !ignored_dirs.contains(&s))
        .unwrap_or(false)
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

pub fn search() -> Vec<File> {
    let exts = vec!["fastq", "fq.gz"];
    let root = "/Library/MinKNOW/data";
    let mut ret_val = vec![];

    let walker = WalkDir::new(root);

    println!("{:?}", walker);
    console_log!("ENTRY FOUND");

    for entry in walker.into_iter().filter_entry(|e| is_valid(e))) {
    // .filter_map(|e| e.ok()
        match entry {
            Ok(entry) => {
                    console_log!("ENTRY FOUND");
                    let f_name = entry.file_name().to_string_lossy();
                    // let sec = entry.metadata()?.modified()?;
                    for ext in &exts {
                        if f_name.ends_with(ext) {
                            // if f_name.ends_with(".json") && sec.elapsed()?.as_secs() < 86400 {
                            let f_path = entry.path().to_string_lossy();
                            let f_relative = f_path.replace(&root, "");
                            let metadata = entry.metadata().unwrap();
                            let f_size = metadata.len();
                            // println!(
                            //     "Name: {}\nPath: {}\nRelative: {}\nSize: {}",
                            //     f_name,
                            //     f_path,
                            //     f_relative,
                            //     f_size.to_string()
                            // );
                            let file_data = File {
                                name: f_name.to_string(),
                                path: f_path.to_string(),
                                relative: f_relative,
                                size: f_size,
                            };
                            ret_val.push(file_data);
                        }
                    }
                    // let x = File {
                    //     name: "blah".to_string(),
                    //     path: "blah".to_string(),
                    //     relative: "blah".to_string(),
                    //     size: 362,
                    // };
                    // ret_val.push(x);
                    return ret_val;
                },
            Err(err) => {
                let path = err.path().unwrap_or(Path::new("")).display();
                console_log!("failed to access entry {}", path);
                if let Some(inner) = err.io_error() {
                    match inner.kind() {
                        io::ErrorKind::InvalidData => {
                            console_log!("entry contains invalid data: {}", inner)
                        }
                        io::ErrorKind::PermissionDenied => {
                            console_log!("Missing permission to read entry: {}", inner)
                        }
                        _ => console_log!("Unexpected error occurred: {}", inner),
                    }
                }
            }
        }
    }
}

#[wasm_bindgen]
pub fn findfiles() -> JsValue {
    let ret_val = search();
    return JsValue::from_serde(&ret_val).unwrap();
}
