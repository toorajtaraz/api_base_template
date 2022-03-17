use crate::actors::database::users::GetUser;
use crate::api_handlers::auth::schemas::get_sign_in_schema;
use crate::api_handlers::utils::auth::verify_password;
use crate::api_handlers::utils::token_gen::gen_token;
use crate::api_handlers::utils::validator::validate;
use crate::models::AppState;
use crate::models_structs::users::UserDataSignIn;
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use serde_json::json;

#[post("/login")]
pub async fn sign_in(user: Json<UserDataSignIn>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let secret = &state.as_ref().secret;
    let secret = secret.to_string();
    let user = user.into_inner();
    let user_json = match serde_json::to_value(&user) {
        Ok(val) => val,
        _ => {
            return HttpResponse::InternalServerError().json("Something went wrong");
        }
    };
    match validate(get_sign_in_schema(), user_json) {
        Ok(_) => {}
        Err(e) => {
            return HttpResponse::BadRequest().json(json!({ "issues": e }));
        }
    }
    match db
        .send(GetUser {
            email: user.email.unwrap(),
        })
        .await
    {
        Ok(Ok(res)) => {
            let user_id = res.id;
            let user_password = res.password;
            let user_access_level = res.access_level;
            if verify_password(user_password, user.password.unwrap()) {
                match gen_token(user_id, user_access_level, db.clone(), secret).await {
                    Ok(tok) => HttpResponse::Ok().json(json!({ "token": tok })),
                    _ => HttpResponse::InternalServerError().json(json!({
                        "issues" : ["something went wrong"]
                    })),
                }
            } else {
                HttpResponse::Unauthorized().json(json!({
                    "issues" : ["wrong password or email"],
                }))
            }
        }
        Ok(Err(_)) => HttpResponse::Unauthorized().json(json!({
            "issues" : ["wrong password or email"]
        })),
        _ => HttpResponse::InternalServerError().json(json!({
            "issues" : ["something went wrong"]
        })),
    }
}
