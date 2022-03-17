extern crate actix;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate dotenv_codegen;

mod actors;
mod api_handlers;
mod bootstrap_utils;
mod db_utils;
mod middleware;
mod models;
mod models_structs;
mod schema;
//local modules
use crate::api_handlers::auth::sign_in::sign_in;
use crate::api_handlers::auth::sign_up::sign_up;
use crate::api_handlers::dummy::{get_api_root_test, get_api_test, get_test};
use crate::models::AppState;
use actors::database::DbActor;
use bootstrap_utils::add_urls::add_urls;
use db_utils::handler::{get_pool, run_migrations};
use middleware::{rate_limit, rbac};
//external modules
use actix::SyncArbiter;
use actix_web::middleware::Logger;
use actix_web::{web::Data, App, HttpServer};
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url = dotenv!("DATABASE_URL");
    run_migrations(&db_url);
    let db_pool = get_pool(&db_url);
    let db_addr = SyncArbiter::start(5, move || DbActor(db_pool.clone()));
    match add_urls(db_addr.clone()).await {
        Err(_) => {
            panic!("Adding resources failed");
        }
        _ => {}
    };
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(rate_limit::RateLimit {
                db: db_addr.clone(),
            })
            .service(
                actix_web::web::scope("/api")
                    .service(
                        actix_web::web::scope("/service")
                            .wrap(rbac::Rbac {
                                db: db_addr.clone(),
                                secret: dotenv!("SECRET").to_string(),
                            })
                            .service(get_api_test)
                            .service(get_api_root_test),
                    )
                    .service(
                        actix_web::web::scope("/auth")
                            .service(get_test)
                            .service(sign_in)
                            .service(sign_up),
                    ),
            )
            .app_data(Data::new(AppState {
                db: db_addr.clone(),
                salt: dotenv!("SALT_STR").to_string(),
                secret: dotenv!("SECRET").to_string(),
            }))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
