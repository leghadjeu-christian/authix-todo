use crate::diesel;
use diesel::prelude::*;

use actix_web::{web, HttpResponse, HttpRequest};

use super::utils::return_state;

use crate::database::establish_connection;
use crate::json_serialization::to_do_item::ToDoItem;
// use crate::auth::jwt::JwtToken; // Replaced by jsonwebtoken
use crate::models::item::item::Item;
use crate::schema::to_do;


/// This function deletes a to do item's status.
///
/// # Arguments
/// * to_di_item (web::Json<ToDoItem>): This serializes the JSON body via the ToDoItem struct
///
/// # Returns
/// (HttpResponse): response body to be passed to the viewer.
pub async fn delete(to_do_item: web::Json<ToDoItem>, req: HttpRequest) -> HttpResponse {
    let title_ref: String = to_do_item.title.clone();
    
    // The user_id should be extracted from the validated token in the middleware
    // and made available in the request extensions or app data.
    // For now, this logic is commented out.
    // let token: JwtToken = JwtToken::decode_from_request(req).unwrap();
    let user_id = 1; // Placeholder: Replace with actual user_id from token in future
    
    let mut connection = establish_connection();
    let items = to_do::table
    .filter(to_do::columns::title.eq(title_ref.as_str()))
    .filter(to_do::columns::user_id.eq(&user_id))
    .order(to_do::columns::id.asc())
    .load::<Item>(&mut connection)
    .unwrap();
    let _ = diesel::delete(&items[0]).execute(&mut connection);
    return HttpResponse::Ok().json(return_state(&user_id))
}