use iron::prelude::*;
use iron::status;
use persistent;
use serde_json;
use super::super::mysql_pool::MysqlPool;
use super::super::super::notebook::Notebook;
use super::super::super::string_error::StringError;
use super::super::uid::Uid;

pub fn post(req: &mut Request) -> IronResult<Response> {
    let arc = req.get::<persistent::Read<MysqlPool>>().unwrap();
    let mysql_pool = arc.as_ref();

    let uid = Uid::uid(&req);

    let result = mysql_pool.prep_exec(
        r"insert into notebook (
                    created_by
                ) values (
                    :created_by
                )",
        (params!{
            "created_by" => &uid,
        })
    );

    match result {
        Ok(query_result) => {
            let notebook = Notebook::new(
                query_result.last_insert_id(),
                Some(uid),
            );

            let notebook = match serde_json::to_string(&notebook) {
                Ok(notebook) => notebook,
                Err(err) => return Err(IronError::new(
                    StringError(format!("{}", err)),
                    status::InternalServerError
                )),
            };

            Ok(Response::with((
                status::Ok,
                super::content_type(),
                notebook
            )))
        },
        Err(err) => return Err(IronError::new(
            StringError(format!("{}", err)),
            status::InternalServerError
        )),
    }
}
