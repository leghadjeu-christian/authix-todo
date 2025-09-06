use crate::diesel;
use diesel::prelude::*;

use actix_web::{web, HttpRequest, HttpResponse};
use actix_web::HttpMessage; // Added for extensions()
use log::{error, info};
use diesel::prelude::*;

use super::utils::return_state;

use crate::database::establish_connection;
use crate::json_serialization::to_do_item::ToDoItem;
use crate::auth::processes::Claims; // Import Claims struct
use crate::models::item::item::Item;
use crate::schema::to_do;


/// This function deletes a to do item.
///
/// # Arguments
/// * to_do_item (web::Json<ToDoItem>): This serializes the JSON body via the ToDoItem struct
/// * req (HttpRequest): The request being made
///
/// # Returns
/// (HttpResponse): response body to be passed to the viewer.
pub async fn delete(to_do_item: web::Json<ToDoItem>, req: HttpRequest) -> HttpResponse {
    let claims = req.extensions().get::<Claims>().cloned();

    match claims {
        Some(c) => {
            info!("Claims found for user in delete: {}", c.sub);
            let user_id = c.sub;
            let title_ref: String = to_do_item.title.clone();
            
            let mut connection = establish_connection();
            let items = to_do::table
                .filter(to_do::columns::title.eq(title_ref.as_str()))
                .filter(to_do::columns::user_id.eq(&user_id))
                .order(to_do::columns::id.asc())
                .load::<Item>(&mut connection)
                .unwrap();
            
            if items.is_empty() {
                error!("No item found to delete for user {} with title {}", user_id, title_ref);
                return HttpResponse::NotFound().finish();
            }

            let _ = diesel::delete(&items[0])
                .execute(&mut connection);
            
            HttpResponse::Ok().json(return_state(&user_id))
        }
        None => {
            error!("Claims not found in request extensions for /api/v1/item/delete. Unauthorized.");
            HttpResponse::Unauthorized().finish()
        }
    }
}