use actix_web::web::Json;
use actix_web::{HttpResponse, post};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub(super) struct Request {
    input: String,
    algorithm: Option<String>,
    format: Option<String>,
}

#[derive(Serialize)]
struct Response {
    data: ResponseData,
}

#[derive(Serialize)]
struct ResponseData {
    sum: String,
}

#[post("/v1/transit/hash")]
pub(super) async fn handle_hash_post(request: Json<Request>) -> HttpResponse {
    todo!()
}

#[post("/v1/transit/hash/{algorithm}")]
pub(super) async fn handle_hash_algorithm_post(request: Json<Request>) -> HttpResponse {
    todo!()
}
