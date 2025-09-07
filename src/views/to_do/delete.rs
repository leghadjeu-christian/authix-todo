use actix_web::{web, HttpResponse};
use log::{warn, info, error};

use diesel::prelude::*;
use diesel::RunQueryDsl;

use super::utils::return_state;
use crate::database::establish_connection;
use crate::models::item::item::Item;
use crate::schema::to_do;
use crate::auth::processes::Claims;
use crate::models::item::delete_item::DeleteItem; // Import DeleteItem

/// This function deletes a to-do item for the authenticated user.
///
/// # Arguments
/// * claims (Claims): Authenticated user claims extracted from the request.
/// * delete_data (web::Json<DeleteItem>): The title of the to-do item to be deleted from the request body.
///
/// # Returns
/// * (HttpResponse): Response body to be passed to the viewer.
pub async fn delete(claims: Claims, delete_data: web::Json<DeleteItem>) -> HttpResponse {
    info!("Attempting to delete to-do item '{}' for authenticated user: {}", delete_data.title, claims.sub);

    let title: String = delete_data.title.clone(); // Clone the title for use in filter and logging
    let mut connection = establish_connection();

    let items = to_do::table
        .filter(to_do::columns::title.eq(&title))
        .filter(to_do::columns::user_id.eq(&claims.sub))
        .order(to_do::columns::id.asc())
        .load::<Item>(&mut connection)
        .unwrap_or_else(|e| {
            error!("Error loading items during delete for user {}: {}", claims.sub, e);
            vec![]
        });

    if !items.is_empty() {
        diesel::delete(to_do::table.filter(to_do::columns::title.eq(&title).and(to_do::columns::user_id.eq(&claims.sub))))
            .execute(&mut connection)
            .unwrap_or_else(|e| {
                error!("Error deleting to-do item '{}' for user {}: {}", title, claims.sub, e);
                0
            });
    } else {
        warn!("Attempted to delete non-existent item or item not owned by user '{}' for user {}", title, claims.sub);
    }

    HttpResponse::Ok().json(return_state(&claims.sub))
}