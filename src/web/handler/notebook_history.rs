use iron::prelude::*;
use iron::status;
use persistent;
use std::io::Read;
use super::prelude::*;
use super::super::mysql_pool::MysqlPool;
use super::super::super::notebook_history::NotebookHistory;

pub fn get(req: &mut Request) -> IronResult<Response> {
    let id = param(&req, "id")?;
    let notebook_id = param(&req, "notebook_id")?;

    let mysql_pool= req.get::<persistent::Read<MysqlPool>>().unwrap();

    let notebook_history = match result(NotebookHistory::get(
        &mut conn(mysql_pool.as_ref())?,
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

    let notebook_history = result(NotebookHistory::insert(
        &mut transaction,
        notebook_id,
        &uid)
    )?;

    let notebook_history = json(&notebook_history)?;

    commit(transaction)?;

    Ok(Response::with((
        status::Ok,
        content_type(),
        notebook_history,
    )))
}

pub fn patch_title(req: &mut Request) -> IronResult<Response> {
    let id = param(&req, "id")?;
    let notebook_id = param(&req, "notebook_id")?;

    let mut notebook_history = String::new();
    result(req.body.read_to_string(&mut notebook_history))?;
    let notebook_history: NotebookHistory = json_de(&notebook_history)?;

    let mysql_pool= req.get::<persistent::Read<MysqlPool>>().unwrap();

    let mut transaction = transaction(mysql_pool.as_ref())?;

    let affected_rows = result(NotebookHistory::update_title(
        &mut transaction,
        id,
        notebook_id,
        &notebook_history.title,
    ))?;

    if affected_rows == 0 {
        return Err(IronError::new(
            StringError(format!("There was no row to update. (id: {}, notebook_id: {})", id, notebook_id)),
            status::BadRequest
        ));
    }

    Ok(Response::with((
        status::Ok,
        content_type(),
        "{}",
    )))
}
