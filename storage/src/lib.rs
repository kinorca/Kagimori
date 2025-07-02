mod compound;
mod error;
mod file;

use async_trait::async_trait;

pub use compound::*;
pub use error::Error;
pub use file::*;

#[async_trait]
pub trait LowLevelStorage: Send + Sync {
    async fn set(&self, key: &str, value: &[u8]) -> Result<(), Error>;
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, Error>;
    async fn delete(&self, key: &str) -> Result<(), Error>;
    async fn exists(&self, key: &str) -> Result<bool, Error>;
}
