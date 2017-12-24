use mysql::{self, Transaction};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Notebook {
    id: u64,
    created_by: Option<String>,
}

impl Notebook {
    pub fn new(id: u64, created_by: Option<String>) -> Self {
        Notebook {
            id,
            created_by,
        }
    }

    pub fn insert(transaction: &mut Transaction, created_by: Option<String>) -> Result<Self, mysql::error::Error> {
        let query_result = transaction.prep_exec(r"
            insert into notebook (
                created_by
            ) values (
                :created_by
            )
        ", params! {
            "created_by" => created_by.as_ref(),
        })?;

        Ok(Notebook {
            id: query_result.last_insert_id(),
            created_by,
        })
    }
}
