use crate::Encryptor;
use crate::error::Error;
use async_trait::async_trait;
use chacha20poly1305::aead::generic_array::typenum::Unsigned;
use chacha20poly1305::aead::{Aead, Nonce, OsRng};
use chacha20poly1305::{AeadCore, ChaCha20Poly1305, Key, KeyInit};

pub struct ChaCha20Poly1305Encryptor {
    key: Key,
}

type ChaCha20Poly1305Nonce = Nonce<ChaCha20Poly1305>;

impl ChaCha20Poly1305Encryptor {
    pub fn new(key: Key) -> Self {
        Self { key }
    }
}

#[async_trait]
impl Encryptor for ChaCha20Poly1305Encryptor {
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
    use crate::chacha20poly1305::ChaCha20Poly1305Encryptor;
    use crate::predefined_tests;
    use chacha20poly1305::Key;

    fn create_sut() -> ChaCha20Poly1305Encryptor {
        let key = Key::from_slice(&[0u8; 32]);
        ChaCha20Poly1305Encryptor::new(*key)
    }

    predefined_tests!(create_sut);
}
