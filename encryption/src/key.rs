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

use crate::proto::kagimori::encryption::key::v1::{
    EncryptionKey, EncryptionKeyForService, EncryptionKeyRef, KeyAlgorithm,
};
use crate::{Encryptor, Error};
use ciphers::Cipher;
use ciphers::aesgcmsiv::AesGcmSivCipher;
use ciphers::chacha20poly1305::ChaCha20Poly1305Cipher;
use ciphers::oneof::OneOfCipher;
use prost::Message;
use storage::DataStorage;
use tracing::debug;
use uuid::Uuid;

impl<S, L> Encryptor<S, L> {
    fn latest_key(key_id: &str) -> String {
        format!("/encryption/keys/{key_id}/latest")
    }

    fn key(key_id: &str, version: u64) -> String {
        format!("/encryption/keys/{key_id}/versions/{version}")
    }

    fn key_for_service(service: &str) -> String {
        format!("/encryption/services/{service}/keys/latest")
    }
}

impl<S, L> Encryptor<S, L>
where
    S: DataStorage,
{
    #[tracing::instrument(skip(self))]
    async fn get_key(&self, key_id: &str, version: u64) -> Result<EncryptionKey, Error> {
        let key = Self::key(key_id, version);
        let key = self
            .storage
            .get(&key)
            .await
            .map_err(Error::Storage)?
            .ok_or(Error::NotFound)?;
        EncryptionKey::decode(key.as_slice()).map_err(Error::Decode)
    }

    #[tracing::instrument(skip(self))]
    async fn set_key(&self, service: &str, ek: EncryptionKey) -> Result<EncryptionKey, Error> {
        let key = Self::key(ek.id.as_str(), ek.version);
        self.storage
            .set(&key, ek.encode_to_vec().as_slice())
            .await
            .map_err(Error::Storage)?;

        let service_key = Self::key_for_service(service);
        let is_put = self
            .storage
            .set_if_absent(
                &service_key,
                EncryptionKeyForService {
                    id: ek.id.to_string(),
                }
                .encode_to_vec()
                .as_slice(),
            )
            .await
            .map_err(Error::Storage)?;

        if is_put {
            Ok(ek)
        } else {
            self.storage.delete(&key).await.map_err(Error::Storage)?;
            self.get_key(&ek.id, ek.version).await
        }
    }

    #[tracing::instrument(skip(self))]
    async fn get_key_for_service(
        &self,
        service: &str,
    ) -> Result<Option<EncryptionKeyForService>, Error> {
        let key = Self::key_for_service(service);
        let key = self.storage.get(&key).await.map_err(Error::Storage)?;

        match key {
            None => Ok(None),
            Some(key) => EncryptionKeyForService::decode(key.as_slice())
                .map_err(Error::Decode)
                .map(Some),
        }
    }
}

impl<S, L> Encryptor<S, L>
where
    S: DataStorage,
{
    #[tracing::instrument(skip(self))]
    pub(crate) async fn get_cipher(
        &self,
        key_id: &str,
        version: u64,
    ) -> Result<OneOfCipher, Error> {
        let key = self.get_key(key_id, version).await?;
        match KeyAlgorithm::try_from(key.algorithm).map_err(|_| Error::UnsupportedAlgorithm)? {
            KeyAlgorithm::Unknown => Err(Error::UnsupportedAlgorithm),
            KeyAlgorithm::AesGcmSiv => Ok(OneOfCipher::AesGcmSiv(
                AesGcmSivCipher::try_from(key.key).map_err(Error::from)?,
            )),
            KeyAlgorithm::Chacha20Poly1305 => Ok(OneOfCipher::ChaCha20Poly1305(
                ChaCha20Poly1305Cipher::try_from(key.key).map_err(Error::from)?,
            )),
        }
    }

    #[tracing::instrument(skip(self))]
    async fn get_latest_cipher(
        &self,
        key_id: &str,
    ) -> Result<(OneOfCipher, EncryptionKeyRef), Error> {
        let key = Self::latest_key(key_id);
        let key = self
            .storage
            .get(&key)
            .await
            .map_err(Error::Storage)?
            .ok_or(Error::NotFound)?;
        let er = EncryptionKeyRef::decode(key.as_slice()).map_err(Error::Decode)?;
        let c = self.get_cipher(key_id, er.version).await?;

        Ok((c, er))
    }

    #[tracing::instrument(skip(self))]
    pub(crate) async fn get_latest_cipher_for_service(
        &self,
        service: &str,
    ) -> Result<(OneOfCipher, EncryptionKeyRef), Error> {
        let key = self.get_key_for_service(service).await?;
        debug!("Key for service '{service}': {key:?}");
        match key {
            None => {
                let cipher = ChaCha20Poly1305Cipher::default();
                let key = EncryptionKey {
                    id: Uuid::new_v4().to_string(),
                    version: 1,
                    algorithm: KeyAlgorithm::Chacha20Poly1305 as i32,
                    key: cipher.key(),
                };
                let ekr = EncryptionKeyRef {
                    id: key.id.clone(),
                    version: key.version,
                };
                self.set_key(service, key).await?;
                Ok((OneOfCipher::ChaCha20Poly1305(cipher), ekr))
            }
            Some(key) => self.get_latest_cipher(&key.id).await,
        }
    }
}
