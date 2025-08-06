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
use crate::kms::DEK_KEY;
use crate::proto::kinorca::kagimori::v1::kagimori_key_management_service_server::KagimoriKeyManagementService;
use crate::proto::kinorca::kagimori::v1::{
    DecryptRequest, DecryptResponse, EncryptRequest, EncryptResponse, GetInformationRequest,
    GetInformationResponse,
};
use audit_log::AuditLogger;
use encryption::{Ciphertext, Encryptor, RequestInfo};
use std::collections::HashMap;
use tonic::{Request, Response, Status, async_trait};
use tracing::info;
use uuid::Uuid;

pub(crate) struct KagimoriService<L> {
    encryptor: Encryptor<L>,
}

impl<L> KagimoriService<L> {
    pub(crate) fn new(encryptor: Encryptor<L>) -> Self {
        Self { encryptor }
    }
}

#[async_trait]
impl<L> KagimoriKeyManagementService for KagimoriService<L>
where
    L: 'static + AuditLogger,
{
    async fn get_information(
        &self,
        _request: Request<GetInformationRequest>,
    ) -> Result<Response<GetInformationResponse>, Status> {
        Ok(GetInformationResponse {
            version: "kagimori.kinorca.com/v1".to_string(),
            kek_id: self.encryptor.get_key_id(),
        }
        .into())
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
                    service: req.service,
                    user: req.uid,
                    data_key: None,
                },
                &req.plaintext,
            )
            .await
            .debug_log()
            .map_err(|e| Status::internal(format!("Internal: {e:?}")))?;

        Ok(EncryptResponse {
            ciphertext: ciphertext.ciphertext,
            kek_id: ciphertext.key_id,
            annotations: HashMap::from([(DEK_KEY.to_string(), ciphertext.dek)]),
        }
        .into())
    }

    async fn decrypt(
        &self,
        request: Request<DecryptRequest>,
    ) -> Result<Response<DecryptResponse>, Status> {
        info!("v2.KeyManagementService.Decrypt called");
        let req = request.into_inner();

        let key_id = Uuid::parse_str(&req.kek_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid key_id: {e:?}")))?;
        if !self.encryptor.contains_key(&key_id) {
            return Err(Status::not_found("key not found"));
        }

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
                    service: req.service,
                    user: req.uid,
                    data_key: None,
                },
                Ciphertext {
                    key_id: req.kek_id,
                    ciphertext: req.ciphertext,
                    dek,
                },
            )
            .await
            .debug_log()
            .map_err(|e| Status::internal(format!("Internal: {e:?}")))?;
        Ok(DecryptResponse { plaintext }.into())
    }
}
