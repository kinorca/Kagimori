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
use tracing::debug;
use uuid::Uuid;

#[derive(Clone)]
pub struct RotatableCipher {
    default_key_id: Uuid,
    default_cipher: OneOfCipher,
    ciphers: HashMap<Uuid, OneOfCipher>,
}

impl RotatableCipher {
    pub fn new(default_key_id: Uuid, ciphers: HashMap<Uuid, OneOfCipher>) -> Self {
        debug!("Using encryption key: {default_key_id}");
        Self {
            default_key_id,
            default_cipher: ciphers.get(&default_key_id).unwrap().clone(),
            ciphers,
        }
    }

    pub fn default_key_id(&self) -> String {
        self.default_key_id.to_string()
    }
}

#[async_trait]
impl Cipher for RotatableCipher {
    fn name(&self) -> &'static str {
        self.default_cipher.name()
    }

    fn key(&self) -> &[u8] {
        self.default_cipher.key()
    }

    async fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, Error> {
        debug!("Encrypt using key: {}", self.default_key_id);
        debug!(
            "Encrypt key bytes: {:?}",
            self.default_key_id.to_bytes_le().as_slice()
        );
        let ciphertext = self.default_cipher.encrypt(data).await?;

        let mut encoded = Vec::new();
        encoded.extend_from_slice(self.default_key_id.to_bytes_le().as_slice());
        encoded.extend(ciphertext);

        Ok(encoded)
    }

    async fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, Error> {
        debug!("Decrypt key bytes: {:?}", &data[..16]);
        let key_id = Uuid::from_slice_le(&data[..16]).map_err(|_| Error::InvalidKeyId)?;
        debug!("Decrypt using key: {key_id}");
        let ciphertext = &data[16..];
        let cipher = self
            .ciphers
            .get(&key_id)
            .ok_or(Error::KeyNotFound(key_id))?;
        cipher.decrypt(ciphertext).await
    }
}

#[cfg(test)]
mod test {
    use crate::oneof::OneOfCipher;
    use crate::rotatable::RotatableCipher;
    use crate::{Cipher, Unencrypted, predefined_tests};
    use std::collections::HashMap;
    use uuid::Uuid;

    fn create_sut() -> RotatableCipher {
        let id = Uuid::from_fields(
            0xa1a2a3a4,
            0xb1b2,
            0xc1c2,
            &[0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8],
        );
        RotatableCipher::new(
            id,
            HashMap::from([(id, OneOfCipher::Unencrypted(Unencrypted))]),
        )
    }

    predefined_tests!(create_sut);

    #[tokio::test]
    async fn test_encrypt_and_decrypt() {
        let sut = create_sut();

        let data = b"Hello, world!";
        let ciphertext = sut.encrypt(data).await.unwrap();
        let decrypted = sut.decrypt(&ciphertext).await.unwrap();
        assert_eq!(data, decrypted.as_slice());
    }
}
