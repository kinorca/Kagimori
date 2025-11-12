use actix_web::{HttpResponse, post};

#[post("/v1/transit/keys/{name}/import")]
pub(super) async fn handle_keys_name_import_post() -> HttpResponse {
    HttpResponse::NoContent().finish()
}
