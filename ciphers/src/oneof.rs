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

use crate::aesgcmsiv::AesGcmSivCipher;
use crate::chacha20poly1305::ChaCha20Poly1305Cipher;
use crate::{Cipher, Error, Unencrypted};
use async_trait::async_trait;

#[derive(Clone)]
pub enum OneOfCipher {
    Unencrypted(Unencrypted),
    AesGcmSiv(AesGcmSivCipher),
    ChaCha20Poly1305(ChaCha20Poly1305Cipher),
}

#[async_trait]
impl Cipher for OneOfCipher {
    fn name(&self) -> &'static str {
        match self {
            OneOfCipher::Unencrypted(c) => c.name(),
            OneOfCipher::AesGcmSiv(c) => c.name(),
            OneOfCipher::ChaCha20Poly1305(c) => c.name(),
        }
    }

    fn key(&self) -> &[u8] {
        match self {
            OneOfCipher::Unencrypted(c) => c.key(),
            OneOfCipher::AesGcmSiv(c) => c.key(),
            OneOfCipher::ChaCha20Poly1305(c) => c.key(),
        }
    }

    async fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, Error> {
        match self {
            OneOfCipher::Unencrypted(c) => c.encrypt(data).await,
            OneOfCipher::AesGcmSiv(c) => c.encrypt(data).await,
            OneOfCipher::ChaCha20Poly1305(c) => c.encrypt(data).await,
        }
    }

    async fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, Error> {
        match self {
            OneOfCipher::Unencrypted(c) => c.decrypt(data).await,
            OneOfCipher::AesGcmSiv(c) => c.decrypt(data).await,
            OneOfCipher::ChaCha20Poly1305(c) => c.decrypt(data).await,
        }
    }
}
