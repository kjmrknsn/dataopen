#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotebookHistory {
    pub id: u64,
    pub notebook_id: u64,
    pub created_by: String,
}
