use actix_web::Responder;
use actix_web::HttpRequest;

use super::utils::return_state;
use crate::auth::jwt::JwtToken;


/// This view gets all of the saved to do items that are stored in the state.json file.
///
/// # Arguments
/// None
///
/// # Returns
/// * (web::Json): all of the stored to do items
use actix_web::web;

pub async fn get(req: HttpRequest) -> impl Responder {
    let token: JwtToken = JwtToken::decode_from_request(req).unwrap();
    return web::Json(return_state(&token.user_id));
}