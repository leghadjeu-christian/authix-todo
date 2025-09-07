use crate::database::establish_connection;
use crate::diesel;
use crate::models::item::item::Item;
use crate::models::item::new_item::NewItem;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Responder;
use diesel::prelude::*;
// use crate::auth::jwt::JwtToken; // Replaced by jsonwebtoken

use super::utils::return_state;
use crate::schema::to_do;

pub async fn create(req: HttpRequest) -> impl Responder {
    // The user_id should be extracted from the validated token in the middleware
    // and made available in the request extensions or app data.
    // For now, this logic is commented out.
    // let token: JwtToken = JwtToken::decode_from_request(req.clone()).unwrap();
    let user_id = 1; // Placeholder: Replace with actual user_id from token in future
        let title: String = req.match_info().get("title").unwrap().to_string();
        let title_ref: String = title.clone();
        let mut connection = establish_connection();
        let items = to_do::table
        .filter(to_do::columns::title.eq(title_ref.as_str()))
        .filter(to_do::columns::user_id.eq(&user_id))
        .order(to_do::columns::id.asc())
        .load::<Item>(&mut connection)
        .unwrap();
        
        if items.len() == 0 {
            let new_post = NewItem::new(title, 1);
            let _ = diesel::insert_into(to_do::table)
            .values(&new_post)
            .execute(&mut connection);
        }
        HttpResponse::Ok().json(return_state(&user_id))
    }
    