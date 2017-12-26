use mysql::{self, Transaction};
use super::str_opt;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Notebook {
    pub id: u64,
}

impl Notebook {
    pub fn new(id: u64) -> Self {
        Notebook {
            id,
        }
    }

    pub fn insert(transaction: &mut Transaction, created_by: &str)
        -> Result<Self, mysql::error::Error> {
        let query_result = transaction.prep_exec(r"
            insert into notebook (
                created_by
            ) values (
                coalesce(:created_by, '')
            )
        ", params! {
            "created_by" => str_opt(created_by),
        })?;

        Ok(Self::new(query_result.last_insert_id()))
    }
}
