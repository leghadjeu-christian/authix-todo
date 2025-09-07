use actix_web::{web, HttpResponse};
use log::info;

use diesel::prelude::*;
use diesel::RunQueryDsl;

use super::utils::return_state;
use crate::database::establish_connection;
use crate::json_serialization::to_do_item::ToDoItem;
use crate::schema::to_do;
use crate::auth::processes::Claims;

/// This function edits a to-do item's status for the authenticated user.
///
/// # Arguments
/// * claims (Claims): Authenticated user claims extracted from the request.
/// * to_do_item (web::Json<ToDoItem>): This serializes the JSON body via the ToDoItem struct.
///
/// # Returns
/// * (HttpResponse): Response body to be passed to the viewer.
pub async fn edit(claims: Claims, to_do_item: web::Json<ToDoItem>) -> HttpResponse {
    info!("Attempting to edit a to-do item for authenticated user: {}", claims.sub);

    let title_ref: String = to_do_item.title.clone();
    let mut connection = establish_connection();

    let results = to_do::table
        .filter(to_do::columns::title.eq(title_ref))
        .filter(to_do::columns::user_id.eq(&claims.sub));

    let _ = diesel::update(results)
        .set(to_do::columns::status.eq("done"))
        .execute(&mut connection);

    HttpResponse::Ok().json(return_state(&claims.sub))
}