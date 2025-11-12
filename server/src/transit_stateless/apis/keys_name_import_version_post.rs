use actix_web::{HttpResponse, post};

#[post("/v1/transit/keys/{name}/import_version")]
pub(super) async fn handle_keys_name_import_version_post() -> HttpResponse {
    HttpResponse::NoContent().finish()
}
