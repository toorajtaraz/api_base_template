use actix_web::{get, HttpResponse, Responder};

#[get("/test")]
pub async fn get_test() -> impl Responder {
    println!("test");
    HttpResponse::Ok().json("YEAY")
}

#[get("/test")]
pub async fn get_api_test() -> impl Responder {
    println!("test");
    HttpResponse::Ok().json("YEAY")
}

#[get("/test_root")]
pub async fn get_api_root_test() -> impl Responder {
    println!("test");
    HttpResponse::Ok().json("YEAY")
}
