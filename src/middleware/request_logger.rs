use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Payload},
    Error, web::Bytes, HttpResponse,
};
use futures_util::{
    future::{self, LocalBoxFuture, Ready},
    StreamExt,
};
use std::{rc::Rc, cell::RefCell};
use log::{info, warn, error};
use bytes::{BytesMut, BufMut};
use crate::auth; // Import the auth module
use crate::auth::processes::Claims; // Import Claims struct directly from processes
use actix_web::body::{MessageBody, BoxBody}; // To ensure B can be BoxBody
use actix_web::HttpMessage; // For extensions_mut()

// There are two types of middleware in actix-web.
// 1. Middleware for the Service: actix_web::dev::Transform
// 2. Middleware for the Request: actix_web::dev::Service

// This is the "factory" for our middleware. It's responsible for creating
// a new instance of RequestLoggerService for each incoming connection.
pub struct RequestLogger;

impl<S, B> actix_web::dev::Transform<S, ServiceRequest> for RequestLogger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static, // B must implement MessageBody
{
    type Response = ServiceResponse<BoxBody>; // Changed to BoxBody
    type Error = Error;
    type InitError = ();
    type Transform = RequestLoggerService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ok(RequestLoggerService {
            service: Rc::new(RefCell::new(service)),
        })
    }
}

pub struct RequestLoggerService<S> {
    service: Rc<RefCell<S>>,
}

impl<S, B> Service<ServiceRequest> for RequestLoggerService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static, // B must implement MessageBody
{
    type Response = ServiceResponse<BoxBody>; // Changed to BoxBody
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>; // Type Future also needs to reflect BoxBody

    fn poll_ready(&self, ctx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();

        Box::pin(async move {
            let (mut http_req, mut payload) = req.into_parts();
            
            let request_url = http_req.uri().path().to_string();
            let request_method = http_req.method().to_string();
            let mut header_info = String::new();
            for (name, value) in http_req.headers().iter() {
                header_info.push_str(&format!("{}: {:?}, ", name, value));
            }

            // Read the entire request body
            let mut body_bytes = BytesMut::new();
            while let Some(chunk) = payload.next().await {
                body_bytes.put(chunk?);
            }
            let body = body_bytes.freeze();

            let body_str = String::from_utf8_lossy(&body);
            info!(
                "Incoming request: Method={}, URI={}, Headers=[{}], Body='{}'",
                request_method, request_url, header_info, body_str
            );

            // Store the body in http_req's extensions before reconstructing the ServiceRequest
            http_req.extensions_mut().insert(body.clone());

            // Reconstruct the ServiceRequest with the modified http_req and an empty payload
            let mut new_req = ServiceRequest::from_parts(http_req, Payload::None);

            let passed: bool;
            if request_url.contains("/api/v1/item/") {
                info!("API item path detected: {}", request_url);
                // Retrieve jwks_uri from app data
                let jwks_uri_data = match new_req.app_data::<actix_web::web::Data<String>>() {
                    Some(data) => data.clone(),
                    None => {
                        error!("JWKS URI not found in application data.");
                        // Handle the error: perhaps return Unauthorized immediately
                        return Ok(ServiceResponse::new(
                            new_req.into_parts().0,
                            HttpResponse::InternalServerError().finish().map_into_boxed_body(),
                        ));
                    }
                };

                match auth::process_token(&new_req, jwks_uri_data).await { // Pass jwks_uri_data and await
                    Ok(claims) => {
                        info!("Token processed successfully for: {}. User ID: {}", request_url, claims.sub);
                        new_req.extensions_mut().insert(claims); // Store claims in request extensions
                        passed = true;
                    },
                    Err(message) => {
                        warn!("Token processing failed for {}: {}", request_url, message);
                        passed = false;
                    }
                }
            } else {
                passed = true;
            }
            
            info!("RequestLogger: 'passed' status before decision: {}", passed); // New log
            let res = if passed {
                service.call(new_req).await?.map_into_boxed_body()
            } else {
                error!("Unauthorized access attempt to: {}", request_url);
                // When unauthorized, use the http_req from new_req to build the response.
                let (original_http_req, _) = new_req.into_parts();
                ServiceResponse::new(original_http_req, HttpResponse::Unauthorized().finish().map_into_boxed_body())
            };
            
            log::info!(
                "{} {} -> {} (Body: '{}')",
                request_method, request_url, &res.status(), body_str
            );
            Ok(res)
        })
    }
}