use crate::proto::kagimori::encryption::key::v1::{EncryptionKey, EncryptionKeyRef, KeyAlgorithm};
use crate::{Encryptor, Error};
use ciphers::aesgcmsiv::AesGcmSivCipher;
use ciphers::chacha20poly1305::ChaCha20Poly1305Cipher;
use ciphers::oneof::OneOfCipher;
use prost::Message;
use storage::DataStorage;

impl<S> Encryptor<S> {
    fn latest_key(key_id: &str) -> String {
        format!("/encryption/keys/{key_id}/latest")
    }

    fn key(key_id: &str, version: u64) -> String {
        format!("/encryption/keys/{key_id}/versions/{version}")
    }
}

impl<S> Encryptor<S>
where
    S: DataStorage,
{
    async fn get_key(&self, key_id: &str, version: u64) -> Result<EncryptionKey, Error> {
        let key = Self::key(key_id, version);
        let key = self
            .storage
            .get(&key)
            .await
            .map_err(Error::Storage)?
            .ok_or(Error::NotFound)?;
        EncryptionKey::decode(key.as_slice()).map_err(Error::Decode)
    }
}

impl<S> Encryptor<S>
where
    S: DataStorage,
{
    pub(crate) async fn get_cipher(
        &self,
        key_id: &str,
        version: u64,
    ) -> Result<OneOfCipher, Error> {
        let key = self.get_key(key_id, version).await?;
        match KeyAlgorithm::try_from(key.algorithm).map_err(|_| Error::UnsupportedAlgorithm)? {
            KeyAlgorithm::Unknown => Err(Error::UnsupportedAlgorithm),
            KeyAlgorithm::AesGcmSiv => Ok(OneOfCipher::AesGcmSiv(
                AesGcmSivCipher::try_from(key.key).map_err(Error::from)?,
            )),
            KeyAlgorithm::Chacha20Poly1305 => Ok(OneOfCipher::ChaCha20Poly1305(
                ChaCha20Poly1305Cipher::try_from(key.key).map_err(Error::from)?,
            )),
        }
    }

    pub(crate) async fn get_latest_cipher(
        &self,
        key_id: &str,
    ) -> Result<(OneOfCipher, EncryptionKeyRef), Error> {
        let key = Self::latest_key(key_id);
        let key = self
            .storage
            .get(&key)
            .await
            .map_err(Error::Storage)?
            .ok_or(Error::NotFound)?;
        let er = EncryptionKeyRef::decode(key.as_slice()).map_err(Error::Decode)?;
        let c = self.get_cipher(key_id, er.version).await?;

        Ok((c, er))
    }
}
