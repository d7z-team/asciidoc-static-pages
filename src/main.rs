extern crate yaml_rust;
extern crate core;

use clap::Parser;


use std::path::Path;

///  AsciiDoc 文档渲染工具包
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Set build Config Path
    #[clap(short, long, value_parser = check_path, default_value_t = String::from(".pages.yaml"))]
    pub config: String,
    /// Add Document variable
    #[clap(short, long, value_parser)]
    pub variable: Vec<String>,
    /// Build Path
    #[clap(short, long, value_parser,default_value_t = String::from("build"))]
    pub build: String,
}

fn main() {
    let args: Args = Args::parse();
    println!("{}", args.config);
    // let current_path = fs::canonicalize("./").unwrap().to_str().unwrap().to_string();
    // let mut conf_path = "".to_string();
    // conf_path.push_str(&current_path);
    // let args1: Vec<String> = env::args().collect();
    // if args1.len() < 2 {
    //     conf_path.push_str("/.pages.yaml");
    // } else {
    //     conf_path.push_str(args1.get(1).unwrap());
    //     conf_path.push_str("/.pages.yaml");
    // }
    // println!("{}", conf_path);
    // // let mut vec: Vec<OsString> = Vec::new();
    // // let dir = fs::read_dir("./").unwrap();
    // // for data in dir {
    // //     let data = data.unwrap();
    // //     let _display = data.path().display().to_string();
    // //     let result = fs::canonicalize(data.path()).unwrap();
    // //     println!("{:?}", result)
    // // }
}

fn check_path(path: &str) -> Result<String, String> {
    let path = Path::new(&path);
    if path.is_file() {
        Ok(path.canonicalize().unwrap().to_str().unwrap().to_string())
    } else {
        Err("config not exists.".to_string())
    }
}
