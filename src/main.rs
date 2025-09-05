#[macro_use] extern crate diesel;
extern crate dotenv;

use actix_web::{App, HttpServer, HttpResponse};
use actix_service::Service;
use futures::future::{ok, Either};

use log;
use env_logger;
use actix_web_middleware_keycloak_auth::{DecodingKey, KeycloakAuth};

mod schema;
mod database;
mod processes;
mod models;
mod state;
mod to_do;
mod json_serialization;
mod views;
mod auth;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    
    env_logger::init();
    HttpServer::new(|| {
        let keycloak_auth = KeycloakAuth::default_with_pk(
            DecodingKey::from_rsa_pem(include_bytes!("../keycloak_pub.pem")).unwrap()
        );
        
        let app = App::new()
        .wrap_fn(|req, srv| {
            // srv => routing
            // req => service request
            
            let request_url: String = String::from(*&req.uri().path().clone());
            let passed: bool;
            
            if *&req.path().contains("/api/v1/item/") {
                match auth::process_token(&req) {
                    Ok(_token) => {
                        passed = true;
                    },
                    Err(_message) => {
                        passed = false;
                    }
                }
            }
            else {
                passed = true;
            }
            
            let end_result = match passed {
                true => {
                    Either::Left(srv.call(req))
                },
                false => {
                    Either::Right(
                        ok(req.into_response(
                            HttpResponse::Unauthorized()
                            .finish()
                        ))
                    )
                }
            };
            
            async move {
                let result = end_result.await?;
                log::info!("{} -> {}", request_url, &result.status());
                Ok(result)
            }
        })
        .wrap(keycloak_auth.clone()) // ← Insert here
        .configure(views::views_factory);
        return app
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}