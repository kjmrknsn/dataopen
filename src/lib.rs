extern crate argparse;
extern crate chrono;
#[macro_use]
extern crate hyper;
extern crate iron;
#[macro_use]
extern crate mysql;
extern crate persistent;
extern crate router;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate toml;

pub mod log;
pub mod notebook;
pub mod notebook_history;
pub mod web;

pub fn str_opt(s: &str) -> Option<&str> {
    if s.len() == 0 {
        None
    } else {
        Some(s)
    }
}
