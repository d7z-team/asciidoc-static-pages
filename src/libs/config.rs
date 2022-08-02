use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ConfigRoot {
    pub pages: PageConfig,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct PageConfig {
    pub info: PageInfo,
    pub location: PageLocation,
    pub conf: PageConf,
    pub attr: Vec<String>,
    pub attr_ref: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct PageInfo {
    pub title: String,
    pub home: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct PageConf {
    pub source_url: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct PageLocation {
    pub root: String,
    pub menu: String,
    pub main: String,
    pub icon: String,
    pub style: String,
    pub script: String,
}
