use actix_web::{web, HttpRequest, HttpResponse};
use actix_web::HttpMessage; // Added for extensions()
use log::{error, info};

use crate::auth::processes::Claims; // Import Claims struct
use super::utils::return_state;

/// This view gets all of the saved to do items that are stored in the state.json file.
///
/// # Arguments
/// * req (HttpRequest): The incoming HTTP request, used to extract Claims.
///
/// # Returns
/// * (HttpResponse): A JSON response of all to-do items for the authenticated user, or Unauthorized.
pub async fn get(req: HttpRequest) -> HttpResponse {
    info!("Attempting to retrieve to-do items.");
    let claims = req.extensions().get::<Claims>().cloned();

    match claims {
        Some(c) => {
            info!("Claims found for user: {}", c.sub);
            HttpResponse::Ok().json(return_state(&c.sub)) // Return HttpResponse::Ok().json()
        }
        None => {
            error!("Claims not found in request extensions for /api/v1/item/get. Unauthorized.");
            HttpResponse::Unauthorized().finish()
        }
    }
}