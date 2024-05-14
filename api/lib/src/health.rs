use actix_web::{HttpResponse, web::{get, ServiceConfig}};

async fn health() -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("version", "0.0.1"))
        .finish()
}

pub fn service(cfg: &mut ServiceConfig) {
    cfg.route("/health", get().to(health));
}
