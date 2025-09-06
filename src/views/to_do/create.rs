use crate::database::establish_connection;
use crate::diesel;
use crate::models::item::item::Item;
use crate::models::item::new_item::NewItem;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_web::HttpMessage; // Added for extensions()
use log::{error, info};
use diesel::prelude::*;

use crate::auth::processes::Claims; // Import Claims struct
use super::utils::return_state;
use crate::schema::to_do;

pub async fn create(req: HttpRequest) -> HttpResponse {
    let claims = req.extensions().get::<Claims>().cloned();

    match claims {
        Some(c) => {
            info!("Claims found for user in create: {}", c.sub);
            let user_id = c.sub;
            let title: String = req.match_info().get("title").unwrap().to_string();
            let title_ref: String = title.clone();
            let mut connection = establish_connection();

            let items = to_do::table
                .filter(to_do::columns::title.eq(title_ref.as_str()))
                .filter(to_do::columns::user_id.eq(&user_id))
                .order(to_do::columns::id.asc())
                .load::<Item>(&mut connection)
                .unwrap();
            
            if items.is_empty() {
                let new_post = NewItem::new(title, user_id.clone());
                let _ = diesel::insert_into(to_do::table)
                    .values(&new_post)
                    .execute(&mut connection);
            }
            HttpResponse::Ok().json(return_state(&user_id))
        }
        None => {
            error!("Claims not found in request extensions for /api/v1/item/create. Unauthorized.");
            HttpResponse::Unauthorized().finish()
        }
    }
}