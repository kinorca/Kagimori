use actix_web::dev::Payload;
use actix_web::http::StatusCode;
use actix_web::{FromRequest, HttpRequest, ResponseError};
use std::fmt::{Debug, Display, Formatter};
use std::future::{Ready, ready};

#[derive(Debug, Clone)]
pub(super) struct VaultToken(String);

#[derive(Debug, Clone)]
pub(super) struct VaultTokenHeaderError;

impl Display for VaultTokenHeaderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vault token header not found")
    }
}

impl ResponseError for VaultTokenHeaderError {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }
}

impl FromRequest for VaultToken {
    type Error = VaultTokenHeaderError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        ready(
            req.headers()
                .get("X-Vault-Token")
                .ok_or(VaultTokenHeaderError)
                .and_then(|h| {
                    h.to_str()
                        .map_err(|_| VaultTokenHeaderError)
                        .map(|v| VaultToken(v.to_string()))
                }),
        )
    }
}
