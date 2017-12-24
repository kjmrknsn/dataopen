use iron::prelude::*;
use iron::status;
use persistent;
use super::super::mysql_pool::MysqlPool;
use super::super::super::notebook::Notebook;
use super::super::uid::Uid;

pub fn post(req: &mut Request) -> IronResult<Response> {
    let arc = req.get::<persistent::Read<MysqlPool>>().unwrap();
    let mysql_pool = arc.as_ref();

    let mut transaction = super::transaction(mysql_pool)?;

    let notebook = match Notebook::insert(
        &mut transaction,
        Uid::uid(&req)
    ) {
        Ok(notebook) => notebook,
        Err(err) => return Err(IronError::new(
            err,
            status::InternalServerError,
        )),
    };

    let notebook = super::json(&notebook)?;

    super::commit(transaction)?;

    Ok(Response::with((
        status::Ok,
        super::content_type(),
        notebook
    )))
}
