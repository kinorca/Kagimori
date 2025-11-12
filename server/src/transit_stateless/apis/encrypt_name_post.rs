use actix_web::web::Json;
use actix_web::{HttpResponse, post};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub(super) struct Request {
    plaintext: Option<String>,
    batch_input: Option<Vec<BatchInputItem>>,
}

#[derive(Deserialize)]
struct BatchInputItem {
    plaintext: String,
    context: String,
}

#[derive(Serialize)]
struct Response {
    data: ResponseData,
}

#[derive(Serialize)]
struct ResponseData {
    ciphertext: Option<String>,
    batch_results: Option<Vec<BatchResultItem>>,
}

#[derive(Serialize)]
struct BatchResultItem {
    ciphertext: String,
    reference: String,
}

#[post("/v1/transit/encrypt/{name}")]
pub(super) async fn handle_encrypt_name_post(request: Json<Request>) -> HttpResponse {
    todo!()
}
