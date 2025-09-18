use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;

#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().json(json!({"status": "ok"}))
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(health);
}
