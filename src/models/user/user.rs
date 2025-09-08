extern crate bcrypt;
use crate::schema::users;
use bcrypt::verify;
use diesel::{Identifiable, Queryable};

#[derive(Queryable, Clone, Identifiable)]
#[table_name = "users"]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn verify(self, password: String) -> bool {
        return verify(password.as_str(), &self.password).unwrap();
    }
}
