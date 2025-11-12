use actix_web::{HttpResponse, post};

#[post("/v1/transit/keys/{name}/rotate")]
pub(super) async fn handle_keys_name_rotate_post() -> HttpResponse {
    HttpResponse::NoContent().finish()
}
