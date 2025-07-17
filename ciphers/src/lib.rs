pub mod chacha20poly1305;
mod error;

pub mod aesgcmsiv;
pub mod oneof;
#[cfg(test)]
mod test;

pub use crate::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait Cipher: Send + Sync {
    async fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, Error>;
    async fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, Error>;
}

pub struct Unencrypted;

#[async_trait]
impl Cipher for Unencrypted {
    async fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, Error> {
        Ok(data.to_vec())
    }

    async fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, Error> {
        Ok(data.to_vec())
    }
}
