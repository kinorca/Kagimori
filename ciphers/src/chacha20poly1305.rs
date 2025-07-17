use crate::Cipher;
use crate::error::Error;
use async_trait::async_trait;
use chacha20poly1305::aead::generic_array::typenum::Unsigned;
use chacha20poly1305::aead::{Aead, Nonce, OsRng};
use chacha20poly1305::{AeadCore, ChaCha20Poly1305, Key, KeyInit, KeySizeUser};

pub struct ChaCha20Poly1305Cipher {
    key: Key,
}

type ChaCha20Poly1305Nonce = Nonce<ChaCha20Poly1305>;

impl ChaCha20Poly1305Cipher {
    pub fn new(key: Key) -> Self {
        Self { key }
    }
}

impl TryFrom<Vec<u8>> for ChaCha20Poly1305Cipher {
    type Error = Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() != <ChaCha20Poly1305 as KeySizeUser>::KeySize::USIZE {
            Err(Error::InvalidKeyLength)
        } else {
            Ok(Self::new(Key::clone_from_slice(value.as_slice())))
        }
    }
}

#[async_trait]
impl Cipher for ChaCha20Poly1305Cipher {
    async fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, Error> {
        let cipher = ChaCha20Poly1305::new(&self.key);
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

        let data = cipher
            .encrypt(&nonce, data)
            .map_err(Error::ChaCha20Poly1305)?;

        let mut result = Vec::with_capacity(nonce.len() + data.len());
        result.extend_from_slice(nonce.as_ref());
        result.extend(data);

        Ok(result)
    }

    async fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, Error> {
        let nonce_size = <ChaCha20Poly1305 as AeadCore>::NonceSize::USIZE;
        let nonce = ChaCha20Poly1305Nonce::from_slice(&data[..nonce_size]);

        let cipher = ChaCha20Poly1305::new(&self.key);
        cipher
            .decrypt(nonce, &data[nonce_size..])
            .map_err(Error::ChaCha20Poly1305)
    }
}

#[cfg(test)]
mod tests {
    use crate::chacha20poly1305::ChaCha20Poly1305Cipher;
    use crate::predefined_tests;
    use chacha20poly1305::Key;

    fn create_sut() -> ChaCha20Poly1305Cipher {
        let key = Key::from_slice(&[0u8; 32]);
        ChaCha20Poly1305Cipher::new(*key)
    }

    predefined_tests!(create_sut);
}
