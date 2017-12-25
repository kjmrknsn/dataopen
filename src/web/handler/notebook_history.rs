use iron::prelude::*;
use iron::status;
use persistent;
use super::prelude::*;
use super::super::mysql_pool::MysqlPool;
use super::super::super::notebook_history::NotebookHistory;

pub fn post(req: &mut Request) -> IronResult<Response> {
    let uid = uid(&req);
    let notebook_id = param(&req, "notebook_id")?;

    let arc = req.get::<persistent::Read<MysqlPool>>().unwrap();
    let mysql_pool = arc.as_ref();

    let mut transaction = transaction(mysql_pool)?;

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


    let notebook_history = json(&notebook_history)?;

    commit(transaction)?;

    Ok(Response::with((
        status::Ok,
        content_type(),
        notebook_history,
    )))
}
