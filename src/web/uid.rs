use iron::prelude::*;

header! { (XUid, "X-Uid") => [String] }

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Uid {
    pub uid: String,
}

impl Uid {
    pub fn from(req: &Request) -> Self {
        let uid = match req.headers.get::<XUid>() {
            Some(uid) => uid.to_string(),
            None => String::new(),
        };

        Uid { uid }
    }
}
