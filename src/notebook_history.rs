use mysql::{self, Transaction};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookHistory {
    pub id: u64,
}

impl NotebookHistory {
    pub fn new(id: u64) -> Self {
        NotebookHistory {
            id,
        }
    }

    pub fn insert(transaction: &mut Transaction, notebook_id: u64, created_by: String)
        -> Result<Self, mysql::error::Error> {
        let query_result = transaction.prep_exec(r"
            insert into notebook_history (
                notebook_id
            ,   created_by
            ) values (
                :notebook_id
            ,   :created_by
            )
        ", params! {
            "notebook_id" => notebook_id,
            "created_by" => &created_by,
        })?;

        Ok(Self::new(query_result.last_insert_id()))
    }
}
