use crate::diesel;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;

use crate::auth::jwt::JwtToken;
use crate::database::establish_connection;
use crate::json_serialization::login::Login;
use crate::models::user::user::User;
use crate::schema::users;

pub async fn login(credentials: web::Json<Login>) -> HttpResponse {
    let username: String = credentials.username.clone();
    let password: String = credentials.password.clone();
    
    let mut connection = establish_connection();
    let users = users::table
    .filter(users::columns::username.eq(username.as_str()))
    .load::<User>(&mut connection)
    .unwrap();
    
    if users.len() == 0 {
        return HttpResponse::NotFound().await.unwrap()
    } else if users.len() > 1 {
        log::error!("multiple users have the username: {}",
        credentials.username.clone());
        return HttpResponse::Conflict().await.unwrap()
    }
    
    match users[0].clone().verify(password) {
        true => {
            let token: String = JwtToken::encode(users[0].clone().id);
            HttpResponse::Ok().header("token", token).await.unwrap()
        }
        false => HttpResponse::Unauthorized().await.unwrap(),
    }
}
