use iron::prelude::*;
use iron::status;
use serde_json;
use super::super::string_error::StringError;
use super::super::uid::Uid;

header! { (XUid, "X-Uid") => [String] }

pub fn get(req: &mut Request) -> IronResult<Response> {
    let uid = match req.headers.get::<XUid>() {
        Some(uid) => uid.to_string(),
        None => String::new(),
    };

    let uid = Uid { uid };

    let uid = match serde_json::to_string(&uid) {
        Ok(uid) => uid,
        Err(err) => return Err(IronError::new(
            StringError(format!("{}", err)),
            status::InternalServerError
        ))
    };

    Ok(Response::with((
        status::Ok,
        super::content_type(),
        uid
    )))
}
