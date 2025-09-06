use crate::database::establish_connection;
use crate::diesel;
use crate::models::item::item::Item;
use crate::models::item::new_item::NewItem;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Responder;
use diesel::prelude::*;
use crate::auth::jwt::JwtToken;

use super::utils::return_state;
use crate::schema::to_do;

pub async fn create(req: HttpRequest) -> impl Responder {
    let token: JwtToken = match JwtToken::decode_from_request(&req) {
        Ok(token) => token,
        Err(_) => return HttpResponse::Unauthorized().body("Unauthorized"),
    };
    let title: String = req.match_info().get("title").unwrap().to_string();
        let title_ref: String = title.clone();
        let mut connection = establish_connection();
        let items = to_do::table
        .filter(to_do::columns::title.eq(title_ref.as_str()))
        .filter(to_do::columns::user_id.eq(&token.user_id))
        .order(to_do::columns::id.asc())
        .load::<Item>(&mut connection)
        .unwrap();
        
        if items.len() == 0 {
            let new_post = NewItem::new(title, 1);
            let _ = diesel::insert_into(to_do::table)
            .values(&new_post)
            .execute(&mut connection);
        }
        HttpResponse::Ok().json(return_state(&token.user_id))
    }
    