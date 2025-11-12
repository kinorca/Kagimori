use actix_web::web::Json;
use actix_web::{HttpResponse, post};
use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct Request {
    bytes: Option<u32>,
    source: Option<String>,
    format: Option<String>,
}

#[post("/v1/transit/random")]
pub(super) async fn handle_random_post(request: Json<Request>) -> HttpResponse {
    todo!()
}

#[post("/v1/transit/random/{source}")]
pub(super) async fn handle_random_source_post(request: Json<Request>) -> HttpResponse {
    todo!()
}

#[post("/v1/transit/random/{source}/{bytes}")]
pub(super) async fn handle_random_source_bytes_post(request: Json<Request>) -> HttpResponse {
    todo!()
}
