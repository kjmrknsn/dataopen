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
}
