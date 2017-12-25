use mysql::{self, Row, Transaction};
use mysql::prelude::*;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookHistory {
    pub id: u64,
    pub notebook_id: u64,
    pub title: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct NotebookHistoryTitle {
    pub title: String,
}

impl NotebookHistory {
    pub fn new(id: u64, notebook_id: u64, title: String) -> Self {
        NotebookHistory {
            id,
            notebook_id,
            title,
        }
    }

    pub fn from(row: Row) -> Self {
        let (id, notebook_id, title) = mysql::from_row(row);

        Self::new(id, notebook_id, title)
    }

    pub fn get<T>(conn: &mut T, id: u64, notebook_id: u64)
        -> Result<Option<Self>, mysql::error::Error>
        where T: GenericConnection {
        let row: Option<Row> = conn.first_exec(r"
            select
                id
            ,   notebook_id
            ,   title
            from
                notebook_history
            where
                id = :id
            and notebook_id = :notebook_id
        ", params! {
            "id" => id,
            "notebook_id" => notebook_id
        })?;

        match row {
            Some(row) => Ok(Some(Self::from(row))),
            None => Ok(None)
        }
    }

    pub fn get_draft<T>(conn: &mut T, notebook_id: u64, created_by: &str)
        -> Result<Option<Self>, mysql::error::Error>
        where T: GenericConnection {
        let row: Option<Row> = conn.first_exec(r"
            select
                id
            ,   notebook_id
            ,   title
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

        Ok(Self::new(query_result.last_insert_id(), notebook_id, String::new()))
    }

    pub fn update_title(transaction: &mut Transaction, id: u64, notebook_id: u64, title: &str)
        -> Result<(), mysql::error::Error> {
        transaction.prep_exec(r"
            update notebook_history
            set
                title = :title
            where
                id = :id
            and notebook_id = :notebook_id
        ", params! {
            "id" => id,
            "notebook_id" => notebook_id,
            "title" => title,
        })?;

        Ok(())
    }
}
