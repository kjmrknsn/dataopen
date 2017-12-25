use iron::headers::ContentType;
use iron::mime;
use iron::mime::{Attr, Mime, TopLevel, SubLevel};
use iron::modifiers::Header;
use iron::prelude::*;
use iron::status;
use mysql::IsolationLevel::RepeatableRead;
use mysql::{Pool, PooledConn, Transaction};
use router::Router;
use serde::de::Deserialize;
use serde::ser::Serialize;
use serde_json;
use std;
use std::fmt;
use std::io::Read;
use std::str::FromStr;

const UID_HEADER_NAME: &str = "X-Uid";

header! { (UidHeader, UID_HEADER_NAME) => [String] }

pub fn content_type() -> Header<ContentType> {
    Header(ContentType(Mime(
        TopLevel::Application,
        SubLevel::Json,
        vec![(Attr::Charset, mime::Value::Utf8)])
    ))
}

pub fn commit(transaction: Transaction) -> Result<(), IronError> {
    result(transaction.commit())
}

pub fn json<T>(v: &T) -> Result<String, IronError>
    where T: Serialize {
    result(serde_json::to_string(v))
}

pub fn json_de<'a, T>(v: &'a str) -> Result<T, IronError>
    where T: Deserialize<'a> {
    result_bad_request(serde_json::from_str(v))
}

pub fn conn(mysql_pool: &Pool) -> Result<PooledConn, IronError> {
    result(mysql_pool.get_conn())
}

pub fn transaction(mysql_pool: &Pool) -> Result<Transaction, IronError> {
    result(mysql_pool.start_transaction(
        true,
        Some(RepeatableRead),
        None
    ))
}

pub fn result<T, E: 'static>(result: Result<T, E>) -> Result<T, IronError>
    where E: std::error::Error + Send {
    match result {
        Ok(result) => Ok(result),
        Err(err) => Err(IronError::new(
            err,
            status::InternalServerError,
        ))
    }
}

pub fn result_bad_request<T, E: 'static>(result: Result<T, E>) -> Result<T, IronError>
    where E: std::error::Error + Send {
    match result {
        Ok(result) => Ok(result),
        Err(err) => Err(IronError::new(
            err,
            status::BadRequest,
        ))
    }
}

pub fn param<T>(req: &Request, key: &str) -> Result<T, IronError>
    where T: FromStr,
          <T as FromStr>::Err: 'static + Send + std::error::Error {
    result_bad_request(
        req.extensions.get::<Router>().unwrap()
            .find(key).unwrap()
            .to_string().parse()
    )
}

pub fn uid(req: &Request) -> String {
    match req.headers.get::<UidHeader>() {
        Some(uid) => uid.to_string(),
        None => String::new(),
    }
}

pub fn body_string(req: &mut Request) -> Result<String, IronError> {
    let mut v = String::new();
    result(req.body.read_to_string(&mut v))?;
    Ok(v)
}

#[derive(Debug)]
pub struct StringError(pub String);

impl fmt::Display for StringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for StringError {
    fn description(&self) -> &str { &*self.0 }
}
