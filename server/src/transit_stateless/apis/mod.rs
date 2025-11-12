use crate::transit_stateless::apis::export_key_type_name_version_get::{
    handle_export_key_type_name_get, handle_export_key_type_name_version_get,
};
use crate::transit_stateless::apis::keys_get::handle_keys_get;
use crate::transit_stateless::apis::keys_name_config_post::handle_keys_name_config_post;
use crate::transit_stateless::apis::keys_name_delete::handle_keys_name_delete;
use crate::transit_stateless::apis::keys_name_get::handle_keys_name_get;
use crate::transit_stateless::apis::keys_name_import_post::handle_keys_name_import_post;
use crate::transit_stateless::apis::keys_name_import_version_post::handle_keys_name_import_version_post;
use crate::transit_stateless::apis::keys_name_post::handle_keys_name_post;
use crate::transit_stateless::apis::keys_name_rotate_post::handle_keys_name_rotate_post;
use crate::transit_stateless::apis::wrapping_key_get::handle_wrapping_key_get;
use actix_web::web::{ServiceConfig, service};

mod byok_export_destination_source_version_get;
mod decrypt_name_post;
mod encrypt_name_post;
mod export_key_type_name_version_get;
mod hash_algorithm_post;
mod keys_get;
mod keys_name_config_post;
mod keys_name_delete;
mod keys_name_get;
mod keys_name_import_post;
mod keys_name_import_version_post;
mod keys_name_post;
mod keys_name_rotate_post;
mod random_source_bytes_post;
mod rewrap_name_post;
mod wrapping_key_get;

pub(super) async fn register_transit_stateless_apis(config: &mut ServiceConfig) {
    config
        .service(handle_export_key_type_name_get)
        .service(handle_export_key_type_name_version_get)
        .service(handle_keys_get)
        .service(handle_keys_name_config_post)
        .service(handle_keys_name_delete)
        .service(handle_keys_name_get)
        .service(handle_keys_name_import_post)
        .service(handle_keys_name_import_version_post)
        .service(handle_keys_name_post)
        .service(handle_keys_name_rotate_post)
        .service(handle_wrapping_key_get);
}
