use crate::diesel;
use diesel::prelude::*;
use actix_web::{web, HttpResponse, HttpRequest};
use actix_web::HttpMessage; // Import HttpMessage for extensions()
use log::{warn, info}; // Import log for warnings and info

use super::utils::return_state;
use crate::database::establish_connection;
use crate::json_serialization::to_do_item::ToDoItem;
use crate::models::item::item::Item;
use crate::schema::to_do;
use crate::auth::processes::Claims; // Import Claims struct

/// This function deletes a to do item for the authenticated user.
///
/// # Arguments
/// * to_do_item (web::Json<ToDoItem>): This serializes the JSON body via the ToDoItem struct
/// * req (HttpRequest): The incoming HTTP request, containing user claims in its extensions.
///
/// # Returns
/// (HttpResponse): response body to be passed to the viewer.
pub async fn delete(to_do_item: web::Json<ToDoItem>, req: HttpRequest) -> HttpResponse {
    info!("Attempting to delete a to-do item for an authenticated user.");

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
    
    let title_ref: String = to_do_item.title.clone();
    
    let mut connection = establish_connection();
    let items = to_do::table
        .filter(to_do::columns::title.eq(title_ref.as_str()))
        .filter(to_do::columns::user_id.eq(&user_id)) // Pass &user_id as &str
        .order(to_do::columns::id.asc())
        .load::<Item>(&mut connection)
        .unwrap();

    if items.len() > 0 { // Check if items exist before attempting to delete
        let _ = diesel::delete(&items[0]).execute(&mut connection);
    } else {
        warn!("Attempted to delete non-existent item: {} for user: {}", to_do_item.title, user_id);
    }
    
    HttpResponse::Ok().json(return_state(&user_id))
}