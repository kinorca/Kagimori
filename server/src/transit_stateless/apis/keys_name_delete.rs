use actix_web::{HttpResponse, delete};

#[delete("/v1/transit/keys/{name}")]
pub(super) async fn handle_keys_name_delete() -> HttpResponse {
    HttpResponse::NoContent().finish()
}
