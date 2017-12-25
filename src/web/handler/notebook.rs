use iron::prelude::*;
use iron::status;
use persistent;
use super::prelude::*;
use super::super::mysql_pool::MysqlPool;
use super::super::super::notebook::Notebook;

pub fn post(req: &mut Request) -> IronResult<Response> {
    let uid = uid(&req);

    let mysql_pool =  req.get::<persistent::Read<MysqlPool>>().unwrap();

    let mut transaction = transaction(mysql_pool.as_ref())?;

    let notebook = result(Notebook::insert(
        &mut transaction,
        &uid
    ))?;

    let notebook = json(&notebook)?;

    commit(transaction)?;

    Ok(Response::with((
        status::Ok,
        content_type(),
        notebook
    )))
}
