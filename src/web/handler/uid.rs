use iron::prelude::*;
use iron::status;
use serde_json;
use super::super::super::string_error::StringError;
use super::super::uid::Uid;

pub fn get(req: &mut Request) -> IronResult<Response> {
    let uid = Uid::from(&req);

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
