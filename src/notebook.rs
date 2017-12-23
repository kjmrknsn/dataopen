#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Notebook {
    id: u64,
}

impl Notebook {
    pub fn new(id: u64) -> Self {
        Notebook {
            id
        }
    }
}
