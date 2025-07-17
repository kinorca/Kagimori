#[derive(Debug)]
pub enum Error {
    ChaCha20Poly1305(chacha20poly1305::Error),
    AesGcmSiv(aes_siv::Error),
    InvalidKeyLength,
}
