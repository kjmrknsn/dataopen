use iron::prelude::*;
use iron::status;
use persistent;
use serde_json;
use super::super::mysql_pool::MysqlPool;
use super::super::super::notebook::Notebook;
use super::super::super::string_error::StringError;

pub fn post(req: &mut Request) -> IronResult<Response> {
    let arc = req.get::<persistent::Read<MysqlPool>>().unwrap();
    let mysql_pool = arc.as_ref();

    let result = mysql_pool.prep_exec(
        "insert into notebook () values ()",
        ()
    );

    match result {
        Ok(query_result) => {
            let notebook = Notebook::new(
                query_result.last_insert_id()
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
