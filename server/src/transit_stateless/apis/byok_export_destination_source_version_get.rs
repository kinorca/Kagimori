use actix_web::{HttpResponse, get};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
struct Response {
    data: ResponseData,
}

#[derive(Serialize)]
struct ResponseData {
    name: String,
    keys: HashMap<String, String>,
}

#[get("/v1/transit/byok/export/{destination}/{source}/{version}")]
pub(super) async fn handle_byok_export_destination_source_version_get() -> HttpResponse {
    handle_impl().await
}

#[get("/v1/transit/byok/export/{destination}/{source}")]
pub(super) async fn handle_byok_export_destination_source_get() -> HttpResponse {
    handle_impl().await
}

async fn handle_impl() -> HttpResponse {
    HttpResponse::Ok().json(Response {
        data: ResponseData {
            name: "key".to_string(),
            keys: HashMap::new(),
        },
    })
}
