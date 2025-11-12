use actix_web::{HttpResponse, post};

#[post("/v1/transit/keys/{name}/config")]
pub(super) async fn handle_keys_name_config_post() -> HttpResponse {
    HttpResponse::NoContent().finish()
}
