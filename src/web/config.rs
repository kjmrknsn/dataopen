use std::fs::File;
use std::io::prelude::*;
use toml;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub mysql: Mysql,
    pub http: HTTP,
}

impl Config {
    pub fn from(path: &str) -> Config {
        let mut f = File::open(path).unwrap();
        let mut contents = String::new();
        f.read_to_string(&mut contents).unwrap();
        toml::from_str(contents.as_str()).unwrap()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct HTTP {
    pub addr: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Mysql {
    pub url: String,
}
