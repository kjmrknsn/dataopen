use iron::prelude::*;
use iron::status;
use persistent;
use super::prelude::*;
use super::super::mysql_pool::MysqlPool;
use super::super::super::paragraph::Paragraph;

pub fn list(req: &mut Request) -> IronResult<Response> {
    let notebook_id = param(&req, "notebook_id")?;
    let notebook_history_id = param(&req, "notebook_history_id")?;

    let mysql_pool= req.get::<persistent::Read<MysqlPool>>().unwrap();

    let paragraphs = result(Paragraph::list(
        &mut conn(mysql_pool.as_ref())?,
        notebook_id,
        notebook_history_id
    ))?;

    let paragraphs = json(&paragraphs)?;

    Ok(Response::with((
        status::Ok,
        content_type(),
        paragraphs
    )))
}
