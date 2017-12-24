use mysql::{self, Row, Transaction};

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

    pub fn from(row: Row) -> Self {
        let id = mysql::from_row(row);

        NotebookHistory {
            id,
        }
    }

    pub fn get_draft(transaction: &mut Transaction, notebook_id: u64, created_by: &str)
        -> Result<Option<Self>, mysql::error::Error> {
        let row: Option<Row> = transaction.first_exec(r"
            select
                id
            from
                notebook_history
            where
                notebook_id = :notebook_id
            and is_completed = false
            and created_by = :created_by
            order by
                id desc
            limit 1
        ", params! {
            "notebook_id" => notebook_id,
            "created_by" => &created_by,
        })?;

        match row {
            Some(row) => Ok(Some(Self::from(row))),
            None => Ok(None),
        }
    }

    pub fn insert(transaction: &mut Transaction, notebook_id: u64, created_by: &str)
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
