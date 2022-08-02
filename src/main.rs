mod libs;

extern crate core;

use std::fs;
use std::ops::Add;
use clap::Parser;


use std::path::Path;
use crate::libs::config::{ConfigRoot};

///  AsciiDoc Document Builder
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// build Config Path.
    #[clap(short, long = "--config", value_parser = check_path, default_value_t = String::from(".pages.yaml"))]
    pub config_file_path: String,
    /// Add Document variable.
    #[clap(short, long, value_parser)]
    pub variable: Vec<String>,
    /// Web Output Directory.
    #[clap(short = 'O', long = "--output", value_parser, default_value_t = String::from("build"))]
    pub build_dir: String,
}

fn main() {
    let args: Args = Args::parse();
    let path1 = std::path::Path::new(&args.config_file_path).to_path_buf();
    let x2 = path1.parent().unwrap();
    println!("{}", x2.to_str().unwrap());
    let conf_data = fs::read_to_string(args.config_file_path).unwrap();
    let mut values: ConfigRoot = serde_yaml::from_str(&conf_data).expect("malformed yaml!");
    // pull command attrs.
    for attrs in args.variable {
        values.pages.attr.push(attrs)
    }
    let x1: String = String::from("./").add(&values.pages.location.root);
    for x in fs::read_dir(&x1).unwrap() {
        println!("{:?}", x.unwrap().path().to_str())
    }
    // println!("{:?}", values)
}

fn check_path(path: &str) -> Result<String, String> {
    let path = Path::new(&path);
    if path.is_file() {
        Ok(path.canonicalize().unwrap().to_str().unwrap().to_string())
    } else {
        Err("config not exists.".to_string())
    }
}
