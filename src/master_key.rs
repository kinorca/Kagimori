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

use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use ciphers::Unencrypted;
use ciphers::aesgcmsiv::AesGcmSivCipher;
use ciphers::chacha20poly1305::ChaCha20Poly1305Cipher;
use ciphers::oneof::OneOfCipher;
use ciphers::rotatable::RotatableCipher;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub(crate) struct MasterKeyConfig {
    default: Uuid,
    keys: Vec<MasterKey>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "algorithm")]
enum MasterKey {
    Unencrypted { id: Uuid },
    ChaCha20Poly1305 { id: Uuid, key: String },
    AesGcmSiv { id: Uuid, key: String },
}

impl MasterKeyConfig {
    pub(crate) fn into_cipher(self) -> RotatableCipher {
        RotatableCipher::new(
            self.default,
            self.keys.into_iter().map(MasterKey::into_cipher).collect(),
        )
    }
}

impl MasterKey {
    fn into_cipher(self) -> (Uuid, OneOfCipher) {
        match self {
            MasterKey::Unencrypted { id } => (id, OneOfCipher::Unencrypted(Unencrypted)),
            MasterKey::ChaCha20Poly1305 { id, key } => (
                id,
                OneOfCipher::ChaCha20Poly1305(
                    ChaCha20Poly1305Cipher::try_from(BASE64_STANDARD.decode(key).unwrap()).unwrap(),
                ),
            ),
            MasterKey::AesGcmSiv { id, key } => (
                id,
                OneOfCipher::AesGcmSiv(
                    AesGcmSivCipher::try_from(BASE64_STANDARD.decode(key).unwrap()).unwrap(),
                ),
            ),
        }
    }
}
