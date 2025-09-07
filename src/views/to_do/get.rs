use actix_web::{web, Responder, HttpRequest, HttpResponse};
use actix_web::HttpMessage; // Import HttpMessage for extensions()
use log::{warn, info};

use super::utils::return_state;
use crate::auth::processes::Claims;

/// This view gets all of the saved to do items for the authenticated user.
///
/// # Arguments
/// * req (HttpRequest): The incoming HTTP request, containing user claims in its extensions.
///
/// # Returns
/// * (web::Json): all of the stored to do items for the authenticated user
/// * (HttpResponse::Unauthorized): if the user is not authenticated
pub async fn get(req: HttpRequest) -> impl Responder {
    info!("Attempting to retrieve to-do items for an authenticated user.");

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

    HttpResponse::Ok().json(return_state(&user_id))
}