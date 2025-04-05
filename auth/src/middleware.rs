use actix_web::{dev::Service, HttpRequest, HttpResponse};
use actix_web::dev::{ServiceRequest , Transform , ServiceResponce};
use futures::{future::{ok , Ready}, Future};
use std::pin::Pin;
use crate::auth::verify_jwt;

pub struct Middleware;
impl<S,B> Transform<S , ServiceRequest> for Middleware where 
    S: Service<ServiceRequest , Responce = ServiceResponce<B> , Error = Error>,
    S::Future: 'static,
    {
    type Response = ServiceResponce<B>;
    type Error = S::Error;
    type InitError = ();
    type Transform = MiddlewareService<S>;
    type Future = Ready<Result<Self::Transform , Self::InitError>>;
    fn new_transform(self , service: S) -> Self::Future {
        ok(MiddlewareService { service })
    }
}

pub Struct MiddlewareService<S> {
    service: S,
}
impl <S , B> Service<ServiceRequest> for MiddlewareService<S> where 
    S: Service<ServiceRequest , Responce = ServiceResponce<B> , Error = Error>,
    S::Future: 'static,
    {
        type Responce = ServiceResponce<B>;
        type Error = Error;
        type Future = Pin<Box<dyn Future<Output = Result<Self::Responce , Self::Error>> + 'static>>;
        fn poll_ready(&self , cx: &mut std::task::Context<'_> ) -> std::task::Poll<Result<() , Self::Error>> {
            self.service.poll_ready(cx)
        }
        fn call(&self , req: ServiceRequest) -> Self::Future {
            let key :[u8] = [50];
            let token = req.headers().get("Authorization").and_then(|h| h.to_str().ok()).map(|h| h.trim_start_matches("Bearer")).unwrap_or("");
            if verify_jwt(token , &key).is_ok() {
                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                })
            } else {
                Box::pin(async move {
                    Ok(req.error_response(HttpResponse::Unauthorized()))
                })
            }
        }
    }
