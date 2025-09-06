use crate::diesel;
use diesel::prelude::*;

use actix_web::{web, HttpResponse, HttpRequest};

use super::utils::return_state;

use crate::database::establish_connection;
use crate::json_serialization::to_do_item::ToDoItem;
use crate::auth::processes::Claims; // Import Claims
use actix_web::HttpMessage; // To access request extensions
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
    let extensions = req.extensions();
    let claims = extensions.get::<Claims>().expect("Claims not found in request extensions");
    let user_id = claims.sub.clone();
    
    let title_ref: String = to_do_item.title.clone();
    
    let mut connection = establish_connection();
    let items = to_do::table
    .filter(to_do::columns::title.eq(title_ref.as_str()))
    .filter(to_do::columns::user_id.eq(&user_id))
    .order(to_do::columns::id.asc())
    .select(to_do::all_columns)
    .load::<Item>(&mut connection)
    .unwrap();
    let _ = diesel::delete(&items[0]).execute(&mut connection);
    return HttpResponse::Ok().json(return_state(&user_id))
}