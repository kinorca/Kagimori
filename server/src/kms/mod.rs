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

use crate::debug_log::DebugLog;
use crate::proto::kubernetes::kms::v2::key_management_service_server::KeyManagementService;
use crate::proto::kubernetes::kms::v2::{
    DecryptRequest, DecryptResponse, EncryptRequest, EncryptResponse, StatusRequest, StatusResponse,
};
use audit_log::AuditLogger;
use encryption::{Ciphertext, DataStorage, Encryptor, RequestInfo};
use std::collections::HashMap;
use tonic::{Request, Response, Status, async_trait};
use uuid::Uuid;

const KMS_SERVICE_NAME: &str = "kubernetes.io/kms/v2";
const KEY_VERSION_KEY: &str = "kagimori.kinorca.com/key-version";

pub(crate) struct KmsService<S, L> {
    encryptor: Encryptor<S, L>,
}

impl<S, L> KmsService<S, L>
where
    S: DataStorage,
    L: AuditLogger,
{
    pub fn new(encryptor: Encryptor<S, L>) -> Self {
        Self { encryptor }
    }
}

#[async_trait]
impl<S, L> KeyManagementService for KmsService<S, L>
where
    S: 'static,
    S: DataStorage,
    S: Send + Sync,
    L: 'static,
    L: AuditLogger,
    L: Send + Sync,
{
    async fn status(
        &self,
        _request: Request<StatusRequest>,
    ) -> Result<Response<StatusResponse>, Status> {
        let kid = self
            .encryptor
            .get_key_id(KMS_SERVICE_NAME)
            .await
            .debug_log()
            .map_err(|e| Status::internal(format!("Internal: {e:?}")))?;
        Ok(Response::new(StatusResponse {
            version: "v2".to_string(),
            healthz: "ok".to_string(),
            key_id: kid,
        }))
    }

    async fn decrypt(
        &self,
        request: Request<DecryptRequest>,
    ) -> Result<Response<DecryptResponse>, Status> {
        let req = request.into_inner();
        let version = u64::from_le_bytes(
            req.annotations
                .get(KEY_VERSION_KEY)
                .ok_or(Status::invalid_argument("missing key version"))?
                .as_slice()
                .try_into()
                .map_err(|_| Status::invalid_argument("invalid key version"))?,
        );
        let plaintext = self
            .encryptor
            .decrypt(
                RequestInfo {
                    event_id: Uuid::now_v7().to_string(),
                    service: KMS_SERVICE_NAME.to_string(),
                    user: req.uid,
                    data_key: None,
                },
                Ciphertext {
                    key_id: req.key_id,
                    version,
                    ciphertext: req.ciphertext,
                },
            )
            .await
            .debug_log()
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
            .encrypt(
                RequestInfo {
                    event_id: Uuid::now_v7().to_string(),
                    service: KMS_SERVICE_NAME.to_string(),
                    user: req.uid,
                    data_key: None,
                },
                &req.plaintext,
            )
            .await
            .debug_log()
            .map_err(|e| Status::internal(format!("Internal: {e:?}")))?;

        Ok(Response::new(EncryptResponse {
            ciphertext: ciphertext.ciphertext,
            key_id: ciphertext.key_id,
            annotations: HashMap::from([(
                KEY_VERSION_KEY.to_string(),
                ciphertext.version.to_le_bytes().to_vec(),
            )]),
        }))
    }
}
