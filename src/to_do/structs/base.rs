use serde::Serialize;
#[derive(Serialize)]
pub struct Base {
    pub title: String,
    pub status: String,
}

impl Base {
    pub fn new(input_title: &str, input_status: &str) -> Self {
        Self {
            title: input_title.to_string(),
            status: input_status.to_string(),
        }
    }
}
