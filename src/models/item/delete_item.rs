use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DeleteItem {
    pub title: String,
}