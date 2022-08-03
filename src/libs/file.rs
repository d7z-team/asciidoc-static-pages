use std::fs;
use std::ops::Not;

use std::path::Path;
use std::time::{SystemTime};

pub fn new_path(parent: &str, name: &str) -> String {
    Path::new(parent).join(name).to_str().unwrap().to_string()
}

/// 递归获取某个目录下的所有文件
pub fn list_pub_files(path_str: &str, container: &mut Vec<String>, skip: &Vec<String>) {
    list_files(path_str, container, skip, true)
}

/// 递归获取某个目录下的所有文件
pub fn list_files(path_str: &str, container: &mut Vec<String>, skip: &Vec<String>, skip_hide: bool) {
    let dir_path = Path::new(path_str);
    if dir_path.is_file() {
        container.push(path_str.to_string());
        return;
    }
    let dir_name = dir_path.file_name().unwrap().to_str().unwrap();
    for skip_path in skip {
        if path_str.starts_with(skip_path) || (dir_name.starts_with(".") && skip_hide) {
            return;
        }
    }
    for item in fs::read_dir(dir_path).unwrap() {
        let item = item.unwrap().path();
        let file_path = item.to_str().unwrap();
        let file_name = item.file_name().unwrap().to_str().unwrap();
        if item.is_dir() {
            list_files(file_path, container, skip, skip_hide)
        } else if (file_name.starts_with(".") && skip_hide).not() {
            container.push(file_path.to_string())
        }
    }
}

#[derive(Debug)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub ext: String,
    pub create_time: SystemTime,
    pub update_time: SystemTime,
}

impl FileInfo {
    pub fn get_info(path_str: &str) -> FileInfo {
        let path = Path::new(path_str);
        let metadata = path.metadata().unwrap();
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        let option = file_name.rfind(".").map(|e| file_name.split_at(e + 1).1.to_string()).unwrap_or(String::new());
        FileInfo {
            path: path_str.to_string(),
            ext: option,
            name: file_name,
            create_time: metadata.created().unwrap_or(SystemTime::now()),
            update_time: metadata.accessed().unwrap_or(SystemTime::now()),
        }
    }
}
