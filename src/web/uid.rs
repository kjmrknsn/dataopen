use iron::prelude::*;

const UID_HEADER_NAME: &str = "X-Uid";

header! { (UidHeader, UID_HEADER_NAME) => [String] }

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Uid {
    pub uid: String,
}

impl Uid {
    pub fn from(req: &Request) -> Self {
        Uid {
            uid: Self::uid(&req),
        }
    }

    pub fn uid(req: &Request) -> String {
        match req.headers.get::<UidHeader>() {
            Some(uid) => uid.to_string(),
            None => String::new(),
        }
    }
}
