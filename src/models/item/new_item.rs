use crate::schema::to_do;

#[derive(Insertable)]
#[diesel(table_name = to_do)]
pub struct NewItem {
    pub title: String,
    pub user_id: String,
    pub status: String,
}

impl NewItem {
    pub fn new(title: String, user_id: String) -> NewItem {
        NewItem {
            title,
            user_id,
            status: String::from("pending"),
        }
    }
}
