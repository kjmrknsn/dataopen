use iron::prelude::*;
use iron::status;
use super::prelude::*;
use super::super::uid::Uid;

pub fn get(req: &mut Request) -> IronResult<Response> {
    let uid = Uid::new(uid(&req));

    let uid = json(&uid)?;

    Ok(Response::with((
        status::Ok,
        content_type(),
        uid
    )))
}
