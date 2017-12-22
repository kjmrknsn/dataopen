use chrono::prelude::*;

const ACCESS: &str = "access";
const INFO: &str = "info";
const WARN: &str = "warn";
const ERROR: &str = "error";

pub fn escape_tab(s: &str) -> String {
    s.replace("\t", "\\t")
}

pub fn access(msg: &str) {
    print_ltsv(ACCESS, msg);
}

pub fn info(msg: &str) {
    print(INFO, msg);
}

pub fn warn(msg: &str) {
    print(WARN, msg);
}

pub fn error(msg: &str) {
    print(ERROR, msg);
}

fn print(kind: &str, msg: &str) {
    print_ltsv(kind, &format!("msg:{}", escape_tab(msg)));
}

fn print_ltsv(kind: &str,ltsv: &str) {
    eprintln!("time:{}\tkind:{}\t{}", Local::now(), kind, ltsv);
}
