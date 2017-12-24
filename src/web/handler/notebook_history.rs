use router::Router;
use iron::prelude::*;
use iron::status;
use mysql::IsolationLevel::RepeatableRead;
use persistent;
use serde_json;
use super::super::mysql_pool::MysqlPool;
use super::super::super::notebook_history::NotebookHistory;
use super::super::uid::Uid;

pub fn notebook_id(req: &Request) -> Result<u64, IronError> {
    let notebook_id = req.extensions
        .get::<Router>().unwrap()
        .find("notebook_id").unwrap()
        .to_string();

    match notebook_id.parse() {
        Ok(notebook_id) => Ok(notebook_id),
        Err(err) => return Err(IronError::new(
            err,
            status::BadRequest,
        ))
    }
}
pub fn post(req: &mut Request) -> IronResult<Response> {
    let uid = Uid::uid(&req);
    let notebook_id = notebook_id(&req)?;

    let arc = req.get::<persistent::Read<MysqlPool>>().unwrap();
    let mysql_pool = arc.as_ref();

    let mut transaction = match mysql_pool.start_transaction(
        true,
        Some(RepeatableRead),
        None
    ) {
        Ok(transaction) => transaction,
        Err(err) => return Err(IronError::new(
            err,
            status::InternalServerError
        )),
    };

    let notebook_history = match NotebookHistory::get_draft(
        &mut transaction,
        notebook_id,
        &uid,
    ) {
        Ok(Some(notebook_history)) => notebook_history,
        Ok(None) => {
            match NotebookHistory::insert(&mut transaction, notebook_id, &uid) {
                Ok(notebook_history) => notebook_history,
                Err(err) => return Err(IronError::new(
                    err,
                    status::InternalServerError
                )),
            }
        },
        Err(err) => return Err(IronError::new(
            err,
            status::InternalServerError
        )),
    };


    let notebook_history = match serde_json::to_string(&notebook_history) {
        Ok(notebook_history) => notebook_history,
        Err(err) => return Err(IronError::new(
            err,
            status::InternalServerError
        ))
    };

    super::commit(transaction)?;

    Ok(Response::with((
        status::Ok,
        super::content_type(),
        notebook_history,
    )))
}
