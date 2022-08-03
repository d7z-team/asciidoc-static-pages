use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::Add;
use std::path::Path;
use clap::Parser;
use crate::libs::file;
use crate::PropRoot;


type Result<T, E = String> = std::result::Result<T, E>;

pub struct Config {
    pub project_path: String,
    pub document_path: String,
    pub output_path: String,
    pub attrs: HashMap<String, String>,
    pub doc_ext: HashSet<String>,
    pub attr_ext: HashSet<String>,
}

impl Config {
    pub fn load() -> Result<Config> {
        let args: Args = Args::parse();
        let mut attrs = HashMap::new();
        let project_path = Path::new(&args.config_file_path)
            .to_path_buf().parent()
            .unwrap().to_str()
            .unwrap().to_string();
        let conf_str = fs::read_to_string(args.config_file_path).unwrap();
        let config_root: PropRoot = serde_yaml::from_str(&conf_str).map_err(|e| e.to_string())?;
        // pull command attrs
        for args_attr in args.variable {
            let n: Vec<&str> = args_attr.splitn(2, "=").collect();
            if n.len() == 2 {
                attrs.insert(n[0].to_string(), n[1].to_string());
            } else {
                panic!("isn't valid attr '{}', eg. key=value", args_attr)
            }
        }
        // load extra attrs
        let attr_file_path: String = file::new_path(&project_path, &config_root.pages.attr_ref);
        if Path::new(&attr_file_path).is_file() {
            fs::read_to_string(attr_file_path).unwrap().lines()
                .map(|e| -> Vec<&str> { e.splitn(2, "=").collect() })
                .filter(|e| e.len() == 2)
                .for_each(|e| {
                    attrs.insert(e[0].to_string(), e[1].to_string());
                });
        }

        let document_root = file::new_path(&project_path, &config_root.pages.location.root);
        let output_path = file::new_path(&project_path, &args.build_dir);
        let conf = &config_root.pages.conf;
        Ok(Config {
            project_path,
            document_path: document_root,
            output_path,
            attrs,
            doc_ext: conf.doc_ext.iter().map(|e| e.to_string()).collect(),
            attr_ext: conf.attr_ext.iter().map(|e| e.to_string()).collect(),
        })
    }
}

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
    /// try build
    #[clap(short = 't', long = "--try", value_parser, default_value_t = false)]
    pub try_build: bool,
}

fn check_path(path: &str) -> Result<String> {
    let path = Path::new(&path);
    if path.is_file() {
        Ok(path.canonicalize().unwrap().to_str().unwrap().to_string())
    } else {
        Err("config not exists.".to_string())
    }
}