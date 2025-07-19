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

use crate::oneof::OneOfCipher;
use crate::{Cipher, Error};
use async_trait::async_trait;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone)]
pub struct RotatableCipher {
    default_key_id: Vec<u8>,
    default_cipher: OneOfCipher,
    ciphers: HashMap<Uuid, OneOfCipher>,
}

impl RotatableCipher {
    pub fn new(default_key_id: Uuid, ciphers: HashMap<Uuid, OneOfCipher>) -> Self {
        Self {
            default_key_id: default_key_id.to_bytes_le().to_vec(),
            default_cipher: ciphers.get(&default_key_id).unwrap().clone(),
            ciphers,
        }
    }
}

#[async_trait]
impl Cipher for RotatableCipher {
    fn name(&self) -> &'static str {
        self.default_cipher.name()
    }

    async fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, Error> {
        let ciphertext = self.default_cipher.encrypt(data).await?;

        let mut encoded = self.default_key_id.clone();
        encoded.extend(ciphertext);

        Ok(encoded)
    }

    async fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, Error> {
        let key_id = Uuid::from_slice_le(&data[..16]).map_err(|_| Error::InvalidKeyId)?;
        let ciphertext = &data[16..];
        let cipher = self
            .ciphers
            .get(&key_id)
            .ok_or(Error::KeyNotFound(key_id))?;
        cipher.decrypt(ciphertext).await
    }
}
