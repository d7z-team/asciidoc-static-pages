use std::fs;
use std::fs::File;
use std::io::Write;
use std::ops::Not;

use std::path::Path;
use std::time::{SystemTime};
use chrono::{DateTime, Local};

pub fn new_path(parent: &str, name: &str) -> String {
    let data = if name.starts_with("/") || name.starts_with("\\") {
        name.split_at(1).1
    } else {
        name
    };
    Path::new(parent).join(data).to_str().unwrap().to_string()
}

/// 递归获取某个目录下的所有文件
pub fn list_pub_files(path_str: &str, container: &mut Vec<String>, skip: &Vec<&String>) {
    list_files(path_str, container, skip, true)
}

/// 递归获取某个目录下的所有文件
pub fn list_files(path_str: &str, container: &mut Vec<String>, skip: &Vec<&String>, skip_hide: bool) {
    let dir_path = Path::new(path_str);
    if dir_path.is_file() {
        container.push(path_str.to_string());
        return;
    }
    let dir_name = dir_path.file_name().unwrap().to_str().unwrap();
    for skip_path in skip {
        if path_str.starts_with(*skip_path) || (dir_name.starts_with(".") && skip_hide) {
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

pub fn delete_dir(src: &str) {
    let result = fs::read_dir(src);
    if let Ok(result) = result {
        for x in result {
            let entry = x.unwrap().path();
            let path = entry.to_str().unwrap();
            if entry.is_dir() {
                delete_dir(path);
                fs::remove_dir(path).unwrap();
            } else {
                fs::remove_file(path).unwrap();
            }
        }
    }
}

pub fn replace_file_ext(src: &str, new_ext: &str) -> String {
    let ext_index = src.rfind(".").unwrap();
    let mut new_name = src.to_string();
    new_name.replace_range(ext_index + 1..src.len(), new_ext);
    return new_name;
}

pub fn auto_copy_file(src: &str, dist: &str) {
    let dist_path = Path::new(dist);
    let parent_path = dist_path.parent().unwrap();
    if parent_path.exists().not() {
        fs::create_dir_all(parent_path).expect(&format!("directory {} create fail!", parent_path.to_str().unwrap()));
    }
    fs::copy(src, dist_path).unwrap();
}

pub fn auto_write_file(path: &str, data: &str) {
    let dist_path = Path::new(path);
    if dist_path.is_file() {
        fs::remove_file(path).unwrap();
    }
    let parent_path = dist_path.parent().unwrap();
    if parent_path.exists().not() {
        fs::create_dir_all(parent_path).expect(&format!("directory {} create fail!", parent_path.to_str().unwrap()));
    }
    File::create(dist_path).unwrap().write(data.as_bytes()).expect("write fail!");
}

#[derive(Debug)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub ext: String,
    pub create_time: DateTime<Local>,
    pub update_time: DateTime<Local>,
    pub commit_id: String,
    pub commit_short_id: String,
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
            create_time: metadata.created().unwrap_or(SystemTime::now()).into(),
            update_time: metadata.accessed().unwrap_or(SystemTime::now()).into(),
            commit_id: String::new(),
            commit_short_id: String::new(),
        }
    }
}
