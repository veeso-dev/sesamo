use actix_web::{get, HttpResponse};

#[get("/check")]
pub async fn check_action() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(r#"{"status": "ok"}"#)
}
