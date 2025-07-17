use crate::Cipher;
use aes_siv::aead::generic_array::typenum::Unsigned;
use aes_siv::aead::{Aead, OsRng};
use aes_siv::{AeadCore, Aes256SivAead, Key, KeyInit, KeySizeUser};
use async_trait::async_trait;

pub struct AesGcmSivCipher {
    key: Key<Aes256SivAead>,
}

impl AesGcmSivCipher {
    pub fn new(key: Key<Aes256SivAead>) -> Self {
        Self { key }
    }
}

impl TryFrom<Vec<u8>> for AesGcmSivCipher {
    type Error = crate::error::Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() != <Aes256SivAead as KeySizeUser>::KeySize::USIZE {
            Err(Self::Error::InvalidKeyLength)
        } else {
            Ok(Self::new(Key::<Aes256SivAead>::clone_from_slice(
                value.as_slice(),
            )))
        }
    }
}

#[async_trait]
impl Cipher for AesGcmSivCipher {
    async fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, crate::error::Error> {
        let cipher = Aes256SivAead::new(&self.key);
        let nonce = Aes256SivAead::generate_nonce(&mut OsRng);

        let encrypted_data = cipher
            .encrypt(&nonce, data)
            .map_err(crate::error::Error::AesGcmSiv)?;

        let mut result = Vec::with_capacity(nonce.len() + encrypted_data.len());
        result.extend_from_slice(nonce.as_ref());
        result.extend(encrypted_data);

        Ok(result)
    }

    async fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, crate::error::Error> {
        let nonce_size = <Aes256SivAead as AeadCore>::NonceSize::USIZE;
        let nonce = aes_siv::Nonce::from_slice(&data[..nonce_size]);

        let cipher = Aes256SivAead::new(&self.key);
        cipher
            .decrypt(nonce, &data[nonce_size..])
            .map_err(crate::error::Error::AesGcmSiv)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::predefined_tests;

    fn create_sut() -> AesGcmSivCipher {
        let key = Key::<Aes256SivAead>::from_slice(&[0u8; 512 / 8]);
        AesGcmSivCipher::new(*key)
    }

    predefined_tests!(create_sut);
}
