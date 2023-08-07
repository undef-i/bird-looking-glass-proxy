use crate::GLOBAL_CONFIG;
use actix_web::body::MessageBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{error, Error};
use futures::future::{ok, Ready};
use futures::Future;
use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};
pub struct Auth;

impl<S, B> Transform<S> for Auth
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware {
            service: Rc::new(RefCell::new(service)),
        })
    }
}

pub struct AuthMiddleware<S> {
    service: Rc<RefCell<S>>,
}

impl<S, B> Service for AuthMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let mut svc = self.service.clone();
        Box::pin(async move {
            let addr = req.peer_addr().unwrap();
            let access_list = &GLOBAL_CONFIG.access_list;
            let shared_secret = &GLOBAL_CONFIG.shared_secret;
            let provided_shared_secret = req
                .headers()
                .get("X-Shared-Secret")
                .map(|header| header.to_str().unwrap_or_default())
                .unwrap_or_default();
            if (access_list.is_empty() && shared_secret.is_empty())
                || access_list.contains(&addr.ip().to_string())
                || shared_secret == provided_shared_secret
                || (access_list.is_empty() && shared_secret == provided_shared_secret)
                || (access_list.contains(&addr.ip().to_string()) && shared_secret.is_empty())
            {
                Ok(svc.call(req).await?)
            } else {
                Err(error::ErrorUnauthorized(
                    "You do not have permission to access.",
                ))
            }
        })
    }
}
