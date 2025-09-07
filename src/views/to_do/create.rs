use crate::database::establish_connection;
use crate::diesel;
use crate::models::item::item::Item;
use crate::models::item::new_item::NewItem;
use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::HttpMessage; // Import HttpMessage for extensions()
use diesel::prelude::*;
use log::{warn, info}; // Import log for warnings and info

use super::utils::return_state;
use crate::schema::to_do;
use crate::auth::processes::Claims; // Import Claims struct

pub async fn create(req: HttpRequest) -> impl Responder {
    info!("Attempting to create a new to-do item for an authenticated user.");

    let claims = req.extensions().get::<Claims>().cloned();

    let user_id = match claims {
        Some(c) => {
            info!("Claims found in request extensions. User ID: {}", c.sub);
            c.sub
        },
        None => {
            warn!("Claims not found in request extensions. Unauthorized access attempt.");
            return HttpResponse::Unauthorized().body("Unauthorized: Missing user claims");
        }
    };

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
        let new_post = NewItem::new(title, user_id.clone()); // Pass user_id as String
        let _ = diesel::insert_into(to_do::table)
            .values(&new_post)
            .execute(&mut connection);
    }
    HttpResponse::Ok().json(return_state(&user_id))
}