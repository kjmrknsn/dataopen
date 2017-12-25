#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Uid {
    pub uid: String,
}

impl Uid {
    pub fn new(uid: String) -> Self {
        Uid {
            uid,
        }
    }
}
