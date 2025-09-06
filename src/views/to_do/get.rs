use actix_web::Responder;
use actix_web::HttpRequest;

use super::utils::return_state;
// use crate::auth::jwt::JwtToken; // Replaced by jsonwebtoken


/// This view gets all of the saved to do items that are stored in the state.json file.
///
/// # Arguments
/// None
///
/// # Returns
/// * (web::Json): all of the stored to do items
use actix_web::web;

pub async fn get(req: HttpRequest) -> impl Responder {
    // The user_id should be extracted from the validated token in the middleware
    // and made available in the request extensions or app data.
    // For now, this logic is commented out.
    // let token: JwtToken = JwtToken::decode_from_request(req).unwrap();
    let user_id = 1; // Placeholder: Replace with actual user_id from token in future
    return web::Json(return_state(&user_id));
}