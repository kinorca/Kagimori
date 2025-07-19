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

mod key;
mod proto;

use audit_log::{Action, AuditLog, AuditLogger, DecryptionAction, EncryptionAction};
use chrono::Utc;
use ciphers::Cipher;
pub use storage::DataStorage;

#[derive(Debug)]
pub enum Error {
    Storage(storage::Error),
    NotFound,
    Decode(prost::DecodeError),
    UnsupportedAlgorithm,
    InvalidCiphertext,
}

impl From<ciphers::Error> for Error {
    fn from(err: ciphers::Error) -> Self {
        Self::Storage(storage::Error::Cipher(err))
    }
}

pub struct Encryptor<S, L> {
    storage: S,
    audit_logger: L,
}

impl<S, L> Clone for Encryptor<S, L>
where
    S: Clone,
    L: Clone,
{
    fn clone(&self) -> Self {
        Self {
            storage: self.storage.clone(),
            audit_logger: self.audit_logger.clone(),
        }
    }
}

impl<S, L> Encryptor<S, L>
where
    S: DataStorage,
    L: AuditLogger,
{
    pub fn new(storage: S, audit_logger: L) -> Self {
        Self {
            storage,
            audit_logger,
        }
    }
}

pub struct Ciphertext {
    pub key_id: String,
    pub version: u64,
    pub ciphertext: Vec<u8>,
}

pub struct RequestInfo {
    pub event_id: String,
    pub service: String,
    pub user: String,
    pub data_key: Option<String>,
}

impl<S, L> Encryptor<S, L>
where
    S: DataStorage,
    L: AuditLogger,
{
    pub async fn encrypt(
        &self,
        request: RequestInfo,
        key_id: &str,
        data: &[u8],
    ) -> Result<Ciphertext, Error> {
        let (cipher, er) = self.get_latest_cipher(key_id).await?;
        let ciphertext = cipher.encrypt(data).await.map_err(Error::from)?;

        self.audit_logger
            .log(AuditLog {
                timestamp: Utc::now(),
                event_id: request.event_id,
                service: request.service,
                user: request.user,
                action: Action::Encryption(EncryptionAction {
                    data_key: request.data_key,
                    algorithm: cipher.name().to_string(),
                    key_id: key_id.to_string(),
                }),
            })
            .await;

        Ok(Ciphertext {
            key_id: key_id.to_string(),
            version: er.version,
            ciphertext,
        })
    }

    pub async fn decrypt(
        &self,
        request: RequestInfo,
        ciphertext: Ciphertext,
    ) -> Result<Vec<u8>, Error> {
        let cipher = self
            .get_cipher(&ciphertext.key_id, ciphertext.version)
            .await?;

        let plaintext = cipher
            .decrypt(&ciphertext.ciphertext)
            .await
            .map_err(Error::from)?;

        self.audit_logger
            .log(AuditLog {
                timestamp: Utc::now(),
                event_id: request.event_id,
                service: request.service,
                user: request.user,
                action: Action::Decryption(DecryptionAction {
                    data_key: request.data_key,
                    algorithm: cipher.name().to_string(),
                    key_id: ciphertext.key_id,
                }),
            })
            .await;

        Ok(plaintext)
    }
}
