use actix_web::web::Path;
use actix_web::{HttpResponse, get};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
struct Response {
    data: ResponseData,
}

#[derive(Serialize)]
struct ResponseData {
    #[serde(rename = "type")]
    key_type: String,
    deletion_allowed: bool,
    derived: bool,
    exportable: bool,
    allow_plaintext_backup: bool,
    min_decryption_version: u64,
    min_encryption_version: u64,
    keys: HashMap<String, u64>,
    name: String,
    supports_encryption: bool,
    supports_decryption: bool,
    supports_signing: bool,
    imported: bool,
}

#[get("/v1/transit/keys/{name}")]
pub(super) async fn handle_keys_name_get(name: Path<String>) -> HttpResponse {
    let name = name.into_inner();
    HttpResponse::Ok().json(Response {
        data: ResponseData {
            key_type: "".to_string(),
            deletion_allowed: false,
            derived: false,
            exportable: false,
            allow_plaintext_backup: false,
            min_decryption_version: 1,
            min_encryption_version: 0,
            keys: HashMap::from([("1".to_string(), 1762577550)]),
            name,
            supports_encryption: true,
            supports_decryption: true,
            supports_signing: false,
            imported: false,
        },
    })
}
