use actix_web::{web, HttpResponse};
use log::{info, error};

use diesel::prelude::*;
use diesel::{RunQueryDsl, Insertable};

use crate::database::establish_connection;
use crate::models::item::new_item::NewItem;
use crate::schema::to_do;
use crate::models::user::user_utils; // Import user_utils

use super::utils::return_state;
use crate::auth::processes::Claims;
use crate::models::item::item::Item; // Import Item to use in filter

/// This view creates a new to do item in the database.
///
/// # Arguments
/// * claims (Claims): Authenticated user claims extracted from the request.
/// * path_title (web::Path<String>): The title of the to-do item from the path.
///
/// # Returns
/// * (HttpResponse): A JSON response containing all of the stored to do items for the authenticated user, or an error.
pub async fn create(claims: Claims, path_title: web::Path<String>) -> HttpResponse {
    info!("Attempting to create a new to-do item for authenticated user: {}", claims.sub);

    // Ensure the user exists in our local database
    let user = match user_utils::find_or_create_user(
        &claims.sub,
        &claims.email,
        &claims.preferred_username,
    ) {
        Ok(u) => u,
        Err(e) => {
            error!("Failed to find or create user for claims.sub {}: {}", claims.sub, e);
            return HttpResponse::InternalServerError().body(format!("Failed to prepare user: {}", e));
        }
    };

    let title = path_title.into_inner();
    let mut connection = establish_connection();

    let items = to_do::table
        .filter(to_do::columns::title.eq(&title))
        .filter(to_do::columns::user_id.eq(&claims.sub))
        .order(to_do::columns::id.asc())
        .load::<Item>(&mut connection)
        .unwrap();

    if items.is_empty() {
        let new_post = NewItem::new(title, claims.sub.clone());
        diesel::insert_into(to_do::table)
            .values(&new_post)
            .execute(&mut connection)
            .expect("Error saving new post");
    }

    HttpResponse::Ok().json(return_state(&claims.sub))
}