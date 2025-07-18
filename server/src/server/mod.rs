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

pub mod h2c;
pub mod tls;

use crate::kms::KmsService;
use crate::proto::kubernetes::kms::v2::key_management_service_server::KeyManagementServiceServer;
use crate::server::h2c::KagimoriH2cServer;
use crate::server::tls::KagimoriTlsServer;
use encryption::{DataStorage, Encryptor};
use std::net::SocketAddr;
use tokio_rustls::rustls::ServerConfig;
use tonic::service::Routes;

pub use tokio_rustls::rustls::pki_types::pem::PemObject;
pub use tokio_rustls::rustls::pki_types::{CertificateDer, PrivateKeyDer};

pub struct KagimoriServer<S> {
    encryptor: Encryptor<S>,
    kms_v2_key_id: Option<String>,
}

impl<S> KagimoriServer<S> {
    pub fn new(encryptor: Encryptor<S>) -> Self {
        Self {
            encryptor,
            kms_v2_key_id: None,
        }
    }
}

impl<S> KagimoriServer<S> {
    pub fn enable_kms_v2(mut self, key_id: String) -> Self {
        self.kms_v2_key_id = Some(key_id);
        self
    }
}

impl<S> KagimoriServer<S> {
    pub fn bind_tls(
        self,
        listen: SocketAddr,
        certificate: Vec<CertificateDer>,
        private_key: PrivateKeyDer,
    ) -> Result<KagimoriTlsServer<S>, tokio_rustls::rustls::Error> {
        let config = ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(
                certificate.into_iter().map(|c| c.into_owned()).collect(),
                private_key.clone_key(),
            )?;
        Ok(KagimoriTlsServer::new(self, config, listen))
    }

    pub fn bind(self, listen: SocketAddr) -> KagimoriH2cServer<S> {
        KagimoriH2cServer::new(self, listen)
    }
}

impl<S> KagimoriServer<S>
where
    S: 'static,
    S: DataStorage,
    S: Clone,
{
    fn create_service(self) -> Routes {
        let mut routes = Routes::default();
        if let Some(kid) = self.kms_v2_key_id {
            routes = routes.add_service(KeyManagementServiceServer::new(KmsService::new(
                self.encryptor.clone(),
                kid,
            )));
        }
        routes
    }
}
