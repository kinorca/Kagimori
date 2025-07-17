use crate::aesgcmsiv::AesGcmSivCipher;
use crate::chacha20poly1305::ChaCha20Poly1305Cipher;
use crate::{Cipher, Error, Unencrypted};
use async_trait::async_trait;

pub enum OneOfCipher {
    Unencrypted(Unencrypted),
    AesGcmSiv(AesGcmSivCipher),
    ChaCha20Poly1305(ChaCha20Poly1305Cipher),
}

#[async_trait]
impl Cipher for OneOfCipher {
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
