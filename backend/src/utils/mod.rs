use actix_web::HttpResponse;

pub fn json_error(msg: &str, code: u32) -> HttpResponse {
    HttpResponse::BadRequest().json(
        serde_json::json!({
            "error": msg,
            "code": code,
        })
    )
}