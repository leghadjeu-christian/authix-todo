use actix_web::{web, HttpResponse};
use log::info;

use diesel::prelude::*;
use diesel::RunQueryDsl;

use super::utils::return_state;
use crate::database::establish_connection;
use crate::models::item::update_item::UpdateItem; // Import the new UpdateItem struct
use crate::schema::to_do;
use crate::auth::processes::Claims;

/// This function edits a to-do item's status for the authenticated user.
///
/// # Arguments
/// * claims (Claims): Authenticated user claims extracted from the request.
/// * update_data (web::Json<UpdateItem>): This serializes the JSON body via the UpdateItem struct.
///
/// # Returns
/// * (HttpResponse): Response body to be passed to the viewer.
pub async fn edit(claims: Claims, update_data: web::Json<UpdateItem>) -> HttpResponse {
    info!("Attempting to edit a to-do item for authenticated user: {}", claims.sub);
    info!("Received update_data: {:?}", update_data); // Debug log

    let mut connection = establish_connection();

    let cloned_title = update_data.title.clone(); // Clone the title for the filter

    let results = to_do::table
        .filter(to_do::columns::title.eq(&cloned_title))
        .filter(to_do::columns::user_id.eq(&claims.sub));

    let _ = diesel::update(results)
        .set(update_data.into_inner()) // Use into_inner() to apply AsChangeset
        .execute(&mut connection);

    HttpResponse::Ok().json(return_state(&claims.sub))
}