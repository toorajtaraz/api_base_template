use crate::actors::database::users::CreateUser;
use crate::api_handlers::auth::schemas::get_sign_up_schema;
use crate::api_handlers::utils::auth::hash_password;
use crate::api_handlers::utils::validator::validate;
use crate::models::AppState;
use crate::models_structs::users::UserDataSignUp;
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use serde_json::json;

#[post("/signup")]
pub async fn sign_up(user: Json<UserDataSignUp>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let salt = &state.as_ref().salt;
    let salt = salt.to_string();
    let user = user.into_inner();
    let user_json = match serde_json::to_value(&user) {
        Ok(val) => val,
        _ => {
            return HttpResponse::InternalServerError().json("Something went wrong");
        }
    };
    match validate(get_sign_up_schema(), user_json) {
        Ok(_) => {}
        Err(e) => {
            return HttpResponse::BadRequest().json(json!({ "issues": e }));
        }
    }
    let hashed_password = match hash_password(salt, user.password.unwrap()) {
        Ok(val) => val,
        _ => {
            return HttpResponse::Unauthorized().json(json!({
                "issues" : ["wrong password or email"],
            }));
        }
    };
    match db
        .send(CreateUser {
            first_name: user.first_name.unwrap(),
            last_name: user.last_name.unwrap(),
            email: user.email.unwrap(),
            password: hashed_password,
            created_at: chrono::Utc::now().naive_utc(),
            access_level: 1,
        })
        .await
    {
        Ok(Ok(_)) => HttpResponse::Ok().json(json!({
            "message": "Your account created successfully, use login api to get auth token"
        })),
        Ok(Err(_)) => HttpResponse::Unauthorized().json(json!({
            "issues" : ["wrong password or email"]
        })),
        _ => HttpResponse::InternalServerError().json(json!({
            "issues" : ["something went wrong"]
        })),
    }
}
