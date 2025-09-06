use actix_web::Responder;
use actix_web::HttpRequest;

use super::utils::return_state;
use crate::auth::processes::Claims; // Import Claims
use actix_web::HttpMessage; // To access request extensions
use log::{info, warn}; // Import warn macro
use crate::json_serialization::to_do_items::ToDoItems; // Import ToDoItems

/// This view gets all of the saved to do items that are stored in the state.json file.
///
/// # Arguments
/// None
///
/// # Returns
/// * (web::Json): all of the stored to do items
use actix_web::{web, HttpResponse}; // Import HttpResponse

pub async fn get(req: HttpRequest) -> HttpResponse { // Changed return type to HttpResponse
    // The user_id should be extracted from the validated token in the middleware
    // and made available in the request extensions or app data.
    // For now, this logic is commented out.
    let extensions = req.extensions();
    let claims = match extensions.get::<Claims>() {
        Some(c) => {
            info!("Claims successfully extracted in get handler. User ID: {}", c.sub);
            c
        },
        None => {
            warn!("Claims not found in request extensions for /api/v1/item/get. Returning 401 Unauthorized.");
            return HttpResponse::Unauthorized().finish(); // Return HttpResponse directly
        }
    };
    let user_id = &claims.sub;
    log::info!("get handler: Successfully extracted user_id: {}", user_id); // New log statement
    return HttpResponse::Ok().json(return_state(user_id)); // Return HttpResponse::Ok().json()
}