mod libs;


use std::process::Command;
use crate::libs::config::Config;
use crate::libs::file;
use crate::libs::prop::{PropRoot};


fn main() {
    let config = Config::load().unwrap();
    let mut files: Vec<String> = vec![];
    let skip = vec![config.output_path];
    file::list_pub_files(&config.document_path, &mut files, &skip);
    files.iter().map(|e| file::FileInfo::get_info(e)).for_each(|e| {
        // println!("{:?}", e)
    });

    println!("{:?}", &config.doc_ext);
    println!("{:?}", &config.attr_ext);
    println!("{:?}", &config.attrs);
    let output = Command::new("pwd").arg("").output().expect("TODO: panic message");
    println!("{}",String::from_utf8_lossy(&output.stdout));
}
