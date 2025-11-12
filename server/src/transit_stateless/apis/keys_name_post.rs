use crate::transit_stateless::headers::VaultToken;
use actix_web::web::Json;
use actix_web::{HttpResponse, post};

#[post("/v1/transit/keys/{name}")]
pub(super) async fn handle_keys_name_post(
    _req: Json<serde_json::Value>,
    _vault_token: VaultToken,
) -> HttpResponse {
    HttpResponse::NoContent().finish()
}
