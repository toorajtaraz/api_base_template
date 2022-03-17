use crate::actix::Addr;
use crate::actors::database::ips::{CreateIp, GetIp, NewAccess, ResetAccess};
use crate::actors::database::urls::GetUrl;
use crate::actors::database::DbActor;
use crate::models_structs::urls::{UrlLimit, UrlLimitInSec};
use ipnetwork::IpNetwork;
use std::future::{ready, Ready};
use std::net::IpAddr;
use std::net::SocketAddr;

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;

fn has_reqd_too_many(
    limit_per: i32,
    limit_count: i32,
    access_count: i32,
    last_access: chrono::NaiveDateTime,
) -> bool {
    let now = chrono::Utc::now().naive_utc();
    let elapsed = now.signed_duration_since(last_access).num_seconds();
    match UrlLimit::from_i32(limit_per) {
        UrlLimit::Second => {
            if elapsed <= UrlLimitInSec::Second as i64 && access_count <= limit_count {
                return false;
            }
        }
        UrlLimit::Minute => {
            if elapsed <= UrlLimitInSec::Minute as i64 && access_count <= limit_count {
                return false;
            }
        }
        UrlLimit::Hour => {
            if elapsed <= UrlLimitInSec::Hour as i64 && access_count <= limit_count {
                return false;
            }
        }
        UrlLimit::Day => {
            if elapsed <= UrlLimitInSec::Day as i64 && access_count <= limit_count {
                return false;
            }
        }
        UrlLimit::Month => {
            if elapsed <= UrlLimitInSec::Month as i64 && access_count <= limit_count {
                return false;
            }
        }
        UrlLimit::Year => {
            if elapsed <= UrlLimitInSec::Year as i64 && access_count <= limit_count {
                return false;
            }
        }
    }
    return true;
}

fn has_elapsed_enough_too_many_req(limit_per: i32, first_access: chrono::NaiveDateTime) -> bool {
    let now = chrono::Utc::now().naive_utc();
    let elapsed = now.signed_duration_since(first_access).num_seconds();
    match UrlLimit::from_i32(limit_per) {
        UrlLimit::Second => {
            if elapsed <= UrlLimitInSec::Second as i64 {
                return true;
            }
        }
        UrlLimit::Minute => {
            if elapsed <= UrlLimitInSec::Minute as i64 {
                return true;
            }
        }
        UrlLimit::Hour => {
            if elapsed <= UrlLimitInSec::Hour as i64 {
                return true;
            }
        }
        UrlLimit::Day => {
            if elapsed <= UrlLimitInSec::Day as i64 {
                return true;
            }
        }
        UrlLimit::Month => {
            if elapsed <= UrlLimitInSec::Month as i64 {
                return true;
            }
        }
        UrlLimit::Year => {
            if elapsed <= UrlLimitInSec::Year as i64 {
                return true;
            }
        }
    }
    return false;
}

fn socket_addr_to_ip_network(socket_addr: &SocketAddr) -> IpNetwork {
    let ip = socket_addr.ip();
    IpNetwork::new(ip, single_host_prefix(&ip)).expect("single_host_prefix created invalid prefix")
}

fn single_host_prefix(ip_addr: &IpAddr) -> u8 {
    match ip_addr {
        &IpAddr::V4(_) => 32,
        &IpAddr::V6(_) => 128,
    }
}

pub struct RateLimit {
    pub db: Addr<DbActor>,
}

impl<S, B> Transform<S, ServiceRequest> for RateLimit
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RateLimitMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RateLimitMiddleware {
            service: service,
            db: self.db.clone(),
        }))
    }
}

pub struct RateLimitMiddleware<S> {
    service: S,
    db: Addr<DbActor>,
}

impl<S, B> Service<ServiceRequest> for RateLimitMiddleware<S>
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
        let ip_addr = req.peer_addr();
        let db = self.db.clone();
        let get_url = GetUrl {
            url_path: format!("{}", path),
        };
        let get_ip = GetIp {
            url_path: format!("{}", path),
            ip: socket_addr_to_ip_network(&ip_addr.unwrap()),
        };
        let create_ip = CreateIp {
            url_path: format!("{}", path),
            ip: socket_addr_to_ip_network(&ip_addr.unwrap()),
            access: chrono::Utc::now().naive_utc(),
        };
        let fut = self.service.call(req);

        Box::pin(async move {
            let db = db.clone();
            let url = match db.clone().send(get_url).await {
                Ok(Ok(rtr_url)) => rtr_url,
                Ok(Err(_)) => {
                    return Err(actix_web::error::ErrorNotFound(
                        "This resource does not exist",
                    ));
                }
                _ => {
                    return Err(actix_web::error::ErrorInternalServerError(
                        "Something went wrong",
                    ));
                }
            };
            let _ip_addr = match db.clone().send(get_ip).await {
                Ok(Ok(res)) => {
                    if has_reqd_too_many(
                        url.limit_per,
                        url.limit_count,
                        res.access_count,
                        res.last_access,
                    ) {
                        if has_elapsed_enough_too_many_req(url.limit_per, res.first_access) {
                            return Err(actix_web::error::ErrorTooManyRequests(
                                "Slow down buddy :))",
                            ));
                        } else {
                            match db
                                .clone()
                                .send(ResetAccess {
                                    id: res.id,
                                    access: chrono::Utc::now().naive_utc(),
                                })
                                .await
                            {
                                Ok(Ok(res)) => res,
                                _ => {
                                    return Err(actix_web::error::ErrorInternalServerError(
                                        "Something went wrong",
                                    ));
                                }
                            }
                        }
                    } else {
                        match db
                            .clone()
                            .send(NewAccess {
                                access: chrono::Utc::now().naive_utc(),
                                id: res.id,
                            })
                            .await
                        {
                            Ok(Ok(res)) => res,
                            _ => {
                                return Err(actix_web::error::ErrorInternalServerError(
                                    "Something went wrong",
                                ));
                            }
                        }
                    }
                }
                Ok(Err(_)) => match db.clone().send(create_ip).await {
                    Ok(Ok(res)) => res,
                    _ => {
                        return Err(actix_web::error::ErrorInternalServerError(
                            "Something went wrong",
                        ));
                    }
                },
                _ => {
                    return Err(actix_web::error::ErrorInternalServerError(
                        "Something went wrong",
                    ));
                }
            };
            match fut.await {
                Ok(res) => {
                    return Ok(res);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        })
    }
}
