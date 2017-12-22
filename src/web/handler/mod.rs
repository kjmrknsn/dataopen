use iron::headers::ContentType;
use iron::mime;
use iron::mime::{Attr, Mime, TopLevel, SubLevel};
use iron::modifiers::Header;

pub mod uid;

pub fn content_type() -> Header<ContentType> {
    Header(ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![(Attr::Charset, mime::Value::Utf8)])))
}
