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
use encryption::{Ciphertext, Encryptor, RequestInfo};
use std::collections::HashMap;
use tonic::{Request, Response, Status, async_trait};
use tracing::{debug, info};
use uuid::Uuid;

const KMS_SERVICE_NAME: &str = "kubernetes.io/kms/v2";
const DEK_KEY: &str = "dek.kagimori.kinorca.com";

pub(crate) struct KmsService<L> {
    encryptor: Encryptor<L>,
}

impl<L> KmsService<L>
where
    L: AuditLogger,
{
    pub fn new(encryptor: Encryptor<L>) -> Self {
        Self { encryptor }
    }
}

#[async_trait]
impl<L> KeyManagementService for KmsService<L>
where
    L: 'static + AuditLogger,
{
    async fn status(
        &self,
        _request: Request<StatusRequest>,
    ) -> Result<Response<StatusResponse>, Status> {
        info!("v2.KeyManagementService.Status called");
        let kid = self.encryptor.get_key_id();
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
        info!("v2.KeyManagementService.Decrypt called");
        let req = request.into_inner();

        let dek = req
            .annotations
            .get(DEK_KEY)
            .ok_or(Status::invalid_argument(
                "annotations must contain dek.kagimori.kinorca.com",
            ))?
            .clone();

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
                    ciphertext: req.ciphertext,
                    dek,
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
        info!("v2.KeyManagementService.Encrypt called");
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
            annotations: HashMap::from([(DEK_KEY.to_string(), ciphertext.dek)]),
        }))
    }
}
