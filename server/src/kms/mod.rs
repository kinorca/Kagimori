use crate::proto::kubernetes::kms::v2::key_management_service_server::KeyManagementService;
use crate::proto::kubernetes::kms::v2::{
    DecryptRequest, DecryptResponse, EncryptRequest, EncryptResponse, StatusRequest, StatusResponse,
};
use encryption::{Ciphertext, DataStorage, Encryptor};
use std::collections::HashMap;
use tonic::{Request, Response, Status, async_trait};

pub(crate) struct KmsService<S> {
    encryptor: Encryptor<S>,
    key_id: String,
}

#[async_trait]
impl<S> KeyManagementService for KmsService<S>
where
    S: 'static,
    S: DataStorage,
    S: Send + Sync,
{
    async fn status(
        &self,
        _request: Request<StatusRequest>,
    ) -> Result<Response<StatusResponse>, Status> {
        Ok(Response::new(StatusResponse {
            version: "v2".to_string(),
            healthz: "ok".to_string(),
            key_id: self.key_id.clone(),
        }))
    }

    async fn decrypt(
        &self,
        request: Request<DecryptRequest>,
    ) -> Result<Response<DecryptResponse>, Status> {
        let req = request.into_inner();
        let version = u64::from_le_bytes(
            req.annotations
                .get("kagimori.kinorca.com/key-version")
                .ok_or(Status::invalid_argument("missing key version"))?
                .as_slice()
                .try_into()
                .map_err(|_| Status::invalid_argument("invalid key version"))?,
        );
        let plaintext = self
            .encryptor
            .decrypt(Ciphertext {
                key_id: req.key_id,
                version,
                ciphertext: req.ciphertext,
            })
            .await
            .map_err(|e| Status::internal(format!("Internal: {e:?}")))?;
        Ok(Response::new(DecryptResponse { plaintext }))
    }

    async fn encrypt(
        &self,
        request: Request<EncryptRequest>,
    ) -> Result<Response<EncryptResponse>, Status> {
        let req = request.into_inner();

        let ciphertext = self
            .encryptor
            .encrypt(&self.key_id, &req.plaintext)
            .await
            .map_err(|e| Status::internal(format!("Internal: {e:?}")))?;

        Ok(Response::new(EncryptResponse {
            ciphertext: ciphertext.ciphertext,
            key_id: ciphertext.key_id,
            annotations: HashMap::from([(
                "kagimori.kinorca.com/key-version".to_string(),
                ciphertext.version.to_le_bytes().to_vec(),
            )]),
        }))
    }
}
