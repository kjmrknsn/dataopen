use mysql::{self, Transaction};

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
}
