use actix_web::{HttpResponse, post};
use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct Request {
    ciphertext: Option<String>,
    batch_input: Option<Vec<BatchInputItem>>,
}

#[derive(Deserialize)]
struct BatchInputItem {
    ciphertext: String,
    context: String,
}

#[post("/v1/transit/decrypt/{name}")]
pub(super) async fn handle_decrypt_name_post() -> HttpResponse {
    todo!()
}
