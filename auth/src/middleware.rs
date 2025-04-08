use actix_web::{dev::ServiceRequest, Error, HttpResponse};
use actix_web::dev::{Service, Transform, ServiceResponse};
use futures::future::{ok, Ready};
use futures::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use crate::auth::verify_jwt;
use actix_web::body::BoxBody;

pub struct Middleware;

impl<S, B> Transform<S, ServiceRequest> for Middleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static, // Ensuring B has a 'static lifetime
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = MiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(MiddlewareService { service })
    }
}

pub struct MiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for MiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .map(|header| header.trim_start_matches("Bearer "))
            .unwrap_or("");

        let secret = 50 as u8; // Correct secret key definition
        let validation = verify_jwt(token, &[secret]);
        
        if validation.is_ok() {
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        } else {
            // Return 401 Unauthorized with a static message if the token is invalid
            let response = HttpResponse::Unauthorized()
                .content_type("text/plain")
                .body("Access denied: Unauthorized request");
            Box::pin(async move { Ok(req.into_response(response.map_into_boxed_body())) }) // Ensuring the response type matches
        }
    }
}