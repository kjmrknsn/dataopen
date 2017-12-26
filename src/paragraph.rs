use mysql::{self, Transaction};
use mysql::prelude::*;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Paragraph {
    pub id: u64,
    pub notebook_id: u64,
    pub notebook_history_id: u64,
    pub code: String,
    pub result: String,
}

impl Paragraph {
    pub fn new(
        id: u64,
        notebook_id: u64,
        notebook_history_id: u64,
        code: String,
        result: String
    ) -> Self {
        Paragraph {
            id,
            notebook_id,
            notebook_history_id,
            code,
            result,
        }
    }

    pub fn insert(transaction: &mut Transaction, notebook_id: u64, notebook_history_id: u64)
        -> Result<Self, mysql::error::Error> {
        let query_result = transaction.prep_exec(r"
            insert into paragraph (
                notebook_id
            ,   notebook_history_id
            ,   code
            ,   result
            ) values (
                :notebook_id
            ,   :notebook_history_id
            ,   ''
            ,   ''
            )
        ", params! {
            "notebook_id" => notebook_id,
            "notebook_history_id" => notebook_history_id
        })?;

        Ok(Self::new(
            query_result.last_insert_id(),
            notebook_id,
            notebook_history_id,
            String::new(),
            String::new(),
        ))
    }

    pub fn list<T>(conn: &mut T, notebook_id: u64, notebook_history_id: u64)
        -> Result<Vec<Self>, mysql::error::Error>
        where T: GenericConnection {
        conn.prep_exec(r"
            select
                id
            ,   notebook_id
            ,   notebook_history_id
            ,   code
            ,   result
            from
                paragraph
            where
                notebook_id = :notebook_id
            and notebook_history_id = :notebook_history_id
            order by
                id
        ", params! {
            "notebook_id" => notebook_id,
            "notebook_history_id" => notebook_history_id
        }).map(|result| {
            result.map(|x| x.unwrap()).map(|row| {
                let (
                    id,
                    notebook_id,
                    notebook_history_id,
                    code,
                    result
                ) = mysql::from_row(row);

                Self::new(
                    id,
                    notebook_id,
                    notebook_history_id,
                    code,
                    result,
                )
            }).collect()
        })
    }
}
