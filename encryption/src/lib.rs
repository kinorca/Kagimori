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

use audit_log::{Action, AuditLog, AuditLogger, DecryptionAction, EncryptionAction};
use chrono::Utc;
use ciphers::Cipher;
use ciphers::rotatable::RotatableCipher;
use uuid::Uuid;

#[derive(Debug)]
pub enum Error {
    UnsupportedAlgorithm,
    Encryption(ciphers::Error),
    Decryption(ciphers::Error),
}

#[derive(Debug, Copy, Clone)]
pub enum KeyAlgorithm {
    ChaCha20Poly1305,
    AesGcmSiv,
}

pub struct Encryptor<L> {
    audit_logger: L,
    algorithm: KeyAlgorithm,
    kek_id: String,
    kek: RotatableCipher,
}

impl<L> Clone for Encryptor<L>
where
    L: Clone,
{
    fn clone(&self) -> Self {
        Self {
            audit_logger: self.audit_logger.clone(),
            algorithm: self.algorithm,
            kek_id: self.kek_id.clone(),
            kek: self.kek.clone(),
        }
    }
}

impl<L> Encryptor<L>
where
    L: AuditLogger,
{
    pub fn new(
        audit_logger: L,
        algorithm: KeyAlgorithm,
        kek_id: String,
        kek: RotatableCipher,
    ) -> Self {
        Self {
            audit_logger,
            algorithm,
            kek_id,
            kek,
        }
    }
}

pub struct Ciphertext {
    pub ciphertext: Vec<u8>,
    pub dek: Vec<u8>,
    pub key_id: String,
}

pub struct RequestInfo {
    pub event_id: String,
    pub service: String,
    pub user: String,
    pub data_key: Option<String>,
}

impl<L> Encryptor<L> {
    pub fn contains_key(&self, key_id: &Uuid) -> bool {
        self.kek.contains_key(key_id)
    }
}

impl<L> Encryptor<L>
where
    L: AuditLogger,
{
    pub fn get_key_id(&self) -> String {
        self.kek_id.clone()
    }

    pub async fn encrypt(&self, request: RequestInfo, data: &[u8]) -> Result<Ciphertext, Error> {
        let (cipher, dek) = self.create_cipher().await?;
        let ciphertext = cipher.encrypt(data).await.map_err(Error::Encryption)?;

        self.audit_logger
            .log(AuditLog {
                timestamp: Utc::now(),
                event_id: request.event_id,
                service: request.service,
                user: request.user,
                action: Action::Encryption(EncryptionAction {
                    data_key: request.data_key,
                    algorithm: cipher.name().to_string(),
                }),
            })
            .await;

        Ok(Ciphertext {
            ciphertext,
            dek,
            key_id: self.get_key_id(),
        })
    }

    pub async fn decrypt(
        &self,
        request: RequestInfo,
        ciphertext: Ciphertext,
    ) -> Result<Vec<u8>, Error> {
        let cipher = self.extract_cipher(&ciphertext.dek)?;

        let plaintext = cipher
            .decrypt(&ciphertext.ciphertext)
            .await
            .map_err(Error::Decryption)?;

        self.audit_logger
            .log(AuditLog {
                timestamp: Utc::now(),
                event_id: request.event_id,
                service: request.service,
                user: request.user,
                action: Action::Decryption(DecryptionAction {
                    data_key: request.data_key,
                    algorithm: cipher.name().to_string(),
                }),
            })
            .await;

        Ok(plaintext)
    }
}
