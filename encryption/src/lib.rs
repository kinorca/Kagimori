mod key;
mod proto;

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

pub struct Encryptor<S> {
    storage: S,
}

impl<S> Encryptor<S>
where
    S: DataStorage,
{
    pub fn new(storage: S) -> Self {
        Self { storage }
    }
}

pub struct Ciphertext {
    pub key_id: String,
    pub version: u64,
    pub ciphertext: Vec<u8>,
}

impl<S> Encryptor<S>
where
    S: DataStorage,
{
    pub async fn encrypt(&self, key_id: &str, data: &[u8]) -> Result<Ciphertext, Error> {
        let (cipher, er) = self.get_latest_cipher(key_id).await?;
        let ciphertext = cipher.encrypt(data).await.map_err(Error::from)?;

        // TODO audit log

        Ok(Ciphertext {
            key_id: key_id.to_string(),
            version: er.version,
            ciphertext,
        })
    }

    pub async fn decrypt(&self, ciphertext: Ciphertext) -> Result<Vec<u8>, Error> {
        let cipher = self
            .get_cipher(&ciphertext.key_id, ciphertext.version)
            .await?;

        let plaintext = cipher
            .decrypt(&ciphertext.ciphertext)
            .await
            .map_err(Error::from)?;

        // TODO audit log

        Ok(plaintext)
    }
}
