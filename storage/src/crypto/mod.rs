// Copyright 2025 SiLeader.
//
// This file is part of Kagimori.
//
// Kagimori is free software: you can redistribute it and/or modify it under the terms of
// the GNU General Public License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any later version.
//
// Kagimori is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with Kagimori.
// If not, see <https://www.gnu.org/licenses/>.

use crate::lowlevel::LowLevelStorage;
use crate::{DataStorage, Error};
use async_trait::async_trait;
use ciphers::Cipher;

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

    async fn set_if_absent(&self, key: &str, value: &[u8]) -> Result<bool, Error> {
        self.inner.set_if_absent(key, value).await
    }
}
