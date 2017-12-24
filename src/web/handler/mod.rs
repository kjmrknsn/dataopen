use iron::headers::ContentType;
use iron::mime;
use iron::mime::{Attr, Mime, TopLevel, SubLevel};
use iron::modifiers::Header;
use iron::prelude::*;
use iron::status;
use mysql::IsolationLevel::RepeatableRead;
use mysql::{Pool, Transaction};
use serde::ser::Serialize;
use serde_json;

pub mod notebook;
pub mod notebook_history;
pub mod uid;

pub fn content_type() -> Header<ContentType> {
    Header(ContentType(Mime(
    TopLevel::Application,
    SubLevel::Json,
    vec![(Attr::Charset, mime::Value::Utf8)])
    ))
}

pub fn commit(transaction: Transaction) -> Result<(), IronError> {
    match transaction.commit() {
        Ok(_) => Ok(()),
        Err(err) => Err(IronError::new(
            err,
            status::InternalServerError
        )),
    }
}

pub fn json<T>(v: &T) -> Result<String, IronError>
    where T: Serialize {
    match serde_json::to_string(v) {
        Ok(v) => Ok(v),
        Err(err) => return Err(IronError::new(
            err,
            status::InternalServerError
        )),
    }
}

pub fn transaction(mysql_pool: &Pool) -> Result<Transaction, IronError> {
    match mysql_pool.start_transaction(
        true,
        Some(RepeatableRead),
        None
    ) {
        Ok(transaction) => Ok(transaction),
        Err(err) => return Err(IronError::new(
            err,
            status::InternalServerError
        )),
    }
}
