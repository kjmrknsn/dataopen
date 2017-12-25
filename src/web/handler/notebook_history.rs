use iron::prelude::*;
use iron::status;
use persistent;
use super::prelude::*;
use super::super::mysql_pool::MysqlPool;
use super::super::super::notebook_history::NotebookHistory;

pub fn get(req: &mut Request) -> IronResult<Response> {
    let id = param(&req, "id")?;
    let notebook_id = param(&req, "notebook_id")?;

    let mysql_pool= req.get::<persistent::Read<MysqlPool>>().unwrap();

    let mut transaction = transaction(mysql_pool.as_ref())?;

    let notebook_history = match result(NotebookHistory::get(
        &mut transaction,
        id,
        notebook_id,
    ))? {
        Some(notebook_history) => notebook_history,
        None => return Err(IronError::new(
            StringError(format!("Invalid parameters. (id: {}, notebook_id: {})", id, notebook_id)),
            status::BadRequest
        )),
    };

    let notebook_history = json(&notebook_history)?;

    commit(transaction)?;

    Ok(Response::with((
        status::Ok,
        content_type(),
        notebook_history,
    )))
}

pub fn post(req: &mut Request) -> IronResult<Response> {
    let uid = uid(&req);
    let notebook_id = param(&req, "notebook_id")?;

    let mysql_pool= req.get::<persistent::Read<MysqlPool>>().unwrap();

    let mut transaction = transaction(mysql_pool.as_ref())?;

    let notebook_history = match result(NotebookHistory::get_draft(
        &mut transaction,
        notebook_id,
        &uid,
    ))? {
        Some(notebook_history) => notebook_history,
        None => result(NotebookHistory::insert(
            &mut transaction,
            notebook_id,
            &uid,
        ))?,
    };

    let notebook_history = json(&notebook_history)?;

    commit(transaction)?;

    Ok(Response::with((
        status::Ok,
        content_type(),
        notebook_history,
    )))
}
