use iron::prelude::*;
use iron::status;
use mysql::IsolationLevel::RepeatableRead;
use persistent;
use serde_json;
use super::super::mysql_pool::MysqlPool;
use super::super::super::notebook::Notebook;
use super::super::uid::Uid;

pub fn post(req: &mut Request) -> IronResult<Response> {
    let arc = req.get::<persistent::Read<MysqlPool>>().unwrap();
    let mysql_pool = arc.as_ref();

    let mut tx = match mysql_pool.start_transaction(
        true,
        Some(RepeatableRead),
        None
    ) {
        Ok(tx) => tx,
        Err(err) => return Err(IronError::new(
            err,
            status::InternalServerError
        )),
    };

    let notebook = match Notebook::insert(
        &mut tx,
        Some(Uid::uid(&req))
    ) {
        Ok(notebook) => notebook,
        Err(err) => return Err(IronError::new(
            err,
            status::InternalServerError,
        )),
    };

    if let Err(err) = tx.commit() {
        return Err(IronError::new(
            err,
            status::InternalServerError,
        ));
    };

    let notebook = match serde_json::to_string(&notebook) {
        Ok(notebook) => notebook,
        Err(err) => return Err(IronError::new(
            err,
            status::InternalServerError
        ))
    };

    Ok(Response::with((
        status::Ok,
        super::content_type(),
        notebook
    )))
}
