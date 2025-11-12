use actix_web::{HttpResponse, get};
use serde::Serialize;

#[derive(Serialize)]
struct Response {
    data: ResponseData,
    lease_duration: u64,
    lease_id: String,
    renewable: bool,
}

#[derive(Serialize)]
struct ResponseData {
    keys: Vec<String>,
}

#[get("/v1/transit/keys")]
pub(super) async fn handle_keys_get() -> HttpResponse {
    HttpResponse::Ok().json(Response {
        data: ResponseData {
            keys: vec!["key1".to_string()],
        },
        lease_duration: 0,
        lease_id: "".to_string(),
        renewable: false,
    })
}
