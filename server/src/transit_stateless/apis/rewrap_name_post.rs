use actix_web::{HttpResponse, post};
use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct Request {
    ciphertext: Option<String>,
    batch_input: Option<Vec<BatchInputItem>>,
}

#[derive(Deserialize)]
struct BatchInputItem {}

#[post("/v1/transit/rewrap/{name}")]
pub(super) async fn handle_rewrap_name_post() -> HttpResponse {
    todo!()
}
