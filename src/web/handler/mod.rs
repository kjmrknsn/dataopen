use iron::headers::ContentType;
use iron::mime;
use iron::mime::{Attr, Mime, TopLevel, SubLevel};
use iron::modifiers::Header;
use iron::prelude::*;
use iron::status;
use mysql::Transaction;

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
