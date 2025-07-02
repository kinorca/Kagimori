pub mod crypto;
mod error;
pub mod lowlevel;

use async_trait::async_trait;
pub use error::Error;

#[async_trait]
pub trait DataStorage: Send + Sync {
    async fn set(&self, key: &str, value: &[u8]) -> Result<(), Error>;
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, Error>;
    async fn delete(&self, key: &str) -> Result<(), Error>;
    async fn exists(&self, key: &str) -> Result<bool, Error>;
}
