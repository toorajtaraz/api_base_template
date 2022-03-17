use crate::actix::Addr;
use crate::actors::database::urls::GetUrl;
use crate::actors::database::DbActor;
use crate::api_handlers::utils::token_gen::verify_token;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_httpauth::extractors::AuthExtractor;
use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;

pub struct Rbac {
    pub db: Addr<DbActor>,
    pub secret: String,
}

impl<S, B> Transform<S, ServiceRequest> for Rbac
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RbacMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RbacMiddleware {
            service: service,
            db: self.db.clone(),
            secret: self.secret.chars().collect(),
        }))
    }
}

pub struct RbacMiddleware<S> {
    service: S,
    db: Addr<DbActor>,
    secret: String,
}

impl<S, B> Service<ServiceRequest> for RbacMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let path = req.path();
        let db = self.db.clone();
        let bearer_tok = BearerAuth::from_service_request(&req);
        let secret: String = self.secret.chars().collect();
        let get_url = GetUrl {
            url_path: format!("{}", path),
        };
        let fut = self.service.call(req);

        Box::pin(async move {
            let bearer_tok: String = match bearer_tok.await {
                Ok(val) => val.token().chars().filter(|c| !c.is_whitespace()).collect(),
                _ => {
                    return Err(actix_web::error::ErrorUnauthorized(
                        "You don't have access to this resource",
                    ));
                }
            };
            let db = db.clone();
            let ids = match verify_token(bearer_tok, secret, db.clone()).await {
                Ok(ids) => ids,
                _ => {
                    return Err(actix_web::error::ErrorUnauthorized(
                        "You don't have access to this resource",
                    ));
                }
            };
            match db.send(get_url).await {
                Ok(Ok(res)) => {
                    if ids.1 < res.access_level {
                        return Err(actix_web::error::ErrorUnauthorized(
                            "You don't have access to this resource",
                        ));
                    } else {
                        match fut.await {
                            Ok(res) => {
                                return Ok(res);
                            }
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    }
                }
                _ => {
                    return Err(actix_web::error::ErrorUnauthorized(
                        "You don't have access to this resource",
                    ));
                }
            }
        })
    }
}
