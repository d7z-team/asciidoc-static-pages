use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct PropRoot {
    pub pages: PageConfig,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct PageConfig {
    pub info: PageInfo,
    pub location: PageLocation,
    pub conf: PageConf,
    #[serde(default = "default_attr")]
    pub attr: Vec<String>,
    #[serde(default = "default_ref")]
    pub attr_files: Vec<String>,
}

fn default_ref() -> Vec<String> {
    vec![]
}

fn default_attr() -> Vec<String> {
    vec![]
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct PageInfo {
    pub title: String,
    pub home: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct PageConf {
    pub source_url: String,
    #[serde(default = "default_attr_ext")]
    pub attr_ext: Vec<String>,
    #[serde(default = "default_doc_ext")]
    pub doc_ext: Vec<String>,
}

const ATTR_EXT: &'static [&'static str] = &["adoc", "asciidoc", "txt", "yaml", "sh", "yml"];
const DOC_EXT: &'static [&'static str] = &["adoc", "asciidoc"];

fn default_attr_ext() -> Vec<String> {
    ATTR_EXT.iter().map(|e| e.to_string()).collect()
}

fn default_doc_ext() -> Vec<String> {
    DOC_EXT.iter().map(|e| e.to_string()).collect()
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct PageLocation {
    #[serde(default = "empty_str")]
    pub root: String,
    pub menu: String,
    pub main: String,
    #[serde(default = "empty_str")]
    pub icon: String,
    #[serde(default = "empty_str")]
    pub style: String,
    #[serde(default = "empty_str")]
    pub script: String,
    #[serde(default = "def_output_dir")]
    pub output: String,
}

fn def_output_dir() -> String {
    String::from("/public")
}

fn empty_str() -> String {
    String::from("")
}
