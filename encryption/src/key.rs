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

use crate::{Encryptor, Error, KeyAlgorithm};
use ciphers::Cipher;
use ciphers::aesgcmsiv::AesGcmSivCipher;
use ciphers::chacha20poly1305::ChaCha20Poly1305Cipher;
use ciphers::oneof::OneOfCipher;

impl KeyAlgorithm {
    fn id(self) -> [u8; 2] {
        match self {
            KeyAlgorithm::ChaCha20Poly1305 => [0x00, 0x01],
            KeyAlgorithm::AesGcmSiv => [0x00, 0x02],
        }
    }
}

impl TryFrom<&[u8]> for KeyAlgorithm {
    type Error = Error;
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        match bytes[..2] {
            [0x00, 0x01] => Ok(KeyAlgorithm::ChaCha20Poly1305),
            [0x00, 0x02] => Ok(KeyAlgorithm::AesGcmSiv),
            _ => Err(Error::UnsupportedAlgorithm),
        }
    }
}

impl<L> Encryptor<L> {
    pub(crate) async fn create_cipher(&self) -> Result<(OneOfCipher, Vec<u8>), Error> {
        let cipher = match self.algorithm {
            KeyAlgorithm::AesGcmSiv => OneOfCipher::AesGcmSiv(AesGcmSivCipher::default()),
            KeyAlgorithm::ChaCha20Poly1305 => {
                OneOfCipher::ChaCha20Poly1305(ChaCha20Poly1305Cipher::default())
            }
        };
        let mut dek = self.algorithm.id().to_vec();
        dek.extend(
            self.kek
                .encrypt(cipher.key())
                .await
                .map_err(Error::Encryption)?,
        );

        Ok((cipher, dek))
    }

    pub(crate) fn extract_cipher(&self, dek: &[u8]) -> Result<OneOfCipher, Error> {
        let algorithm: KeyAlgorithm = dek[..2].try_into()?;
        match algorithm {
            KeyAlgorithm::ChaCha20Poly1305 => Ok(OneOfCipher::ChaCha20Poly1305(
                ChaCha20Poly1305Cipher::try_from(&dek[2..])
                    .map_err(|_| Error::UnsupportedAlgorithm)?,
            )),
            KeyAlgorithm::AesGcmSiv => Ok(OneOfCipher::AesGcmSiv(
                AesGcmSivCipher::try_from(&dek[2..]).map_err(|_| Error::UnsupportedAlgorithm)?,
            )),
        }
    }
}
