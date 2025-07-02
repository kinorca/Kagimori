use crate::lowlevel::LowLevelStorage;
use crate::{DataStorage, Error};
use async_trait::async_trait;
use encryption::Cipher;

pub struct CryptedStorage<C, S> {
    cipher: C,
    inner: S,
}

impl<C, S> Clone for CryptedStorage<C, S>
where
    C: Clone,
    S: Clone,
{
    fn clone(&self) -> Self {
        Self {
            cipher: self.cipher.clone(),
            inner: self.inner.clone(),
        }
    }
}

impl<C, S> CryptedStorage<C, S>
where
    C: Cipher,
    S: LowLevelStorage,
{
    pub fn new(cipher: C, storage: S) -> Self {
        Self {
            cipher,
            inner: storage,
        }
    }
}

#[async_trait]
impl<C, S> DataStorage for CryptedStorage<C, S>
where
    C: Cipher,
    S: LowLevelStorage,
{
    async fn set(&self, key: &str, value: &[u8]) -> Result<(), Error> {
        let data = self.cipher.encrypt(value).await.map_err(Error::Cipher)?;
        self.inner.set(key, &data).await
    }

    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, Error> {
        let data = self.inner.get(key).await?;
        if let Some(data) = data {
            Ok(Some(
                self.cipher.decrypt(&data).await.map_err(Error::Cipher)?,
            ))
        } else {
            Ok(None)
        }
    }

    async fn delete(&self, key: &str) -> Result<(), Error> {
        self.inner.delete(key).await
    }

    async fn exists(&self, key: &str) -> Result<bool, Error> {
        self.inner.exists(key).await
    }
}
