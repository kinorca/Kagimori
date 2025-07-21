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
mod uds;

use crate::kms::KmsService;
use crate::proto::kubernetes::kms::v2::key_management_service_server::KeyManagementServiceServer;
use crate::server::h2c::KagimoriH2cServer;
use crate::server::tls::KagimoriTlsServer;
use encryption::Encryptor;
use std::net::SocketAddr;
use std::path::Path;
use tokio_rustls::rustls::ServerConfig;
use tonic::service::Routes;

use crate::server::uds::KagimoriUnixDomainSocketServer;
use audit_log::AuditLogger;
pub use tokio_rustls::rustls::pki_types::pem::PemObject;
pub use tokio_rustls::rustls::pki_types::{CertificateDer, PrivateKeyDer};

pub struct KagimoriServer<L> {
    encryptor: Encryptor<L>,
    kms_v2_enabled: bool,
}

impl<L> KagimoriServer<L> {
    pub fn new(encryptor: Encryptor<L>) -> Self {
        Self {
            encryptor,
            kms_v2_enabled: false,
        }
    }
}

impl<L> KagimoriServer<L> {
    pub fn enable_kms_v2(mut self) -> Self {
        self.kms_v2_enabled = true;
        self
    }
}

impl<L> KagimoriServer<L> {
    pub fn bind_tls(
        self,
        listen: SocketAddr,
        certificate: Vec<CertificateDer>,
        private_key: PrivateKeyDer,
    ) -> Result<KagimoriTlsServer<L>, tokio_rustls::rustls::Error> {
        let config = ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(
                certificate.into_iter().map(|c| c.into_owned()).collect(),
                private_key.clone_key(),
            )?;
        Ok(KagimoriTlsServer::new(self, config, listen))
    }

    pub fn bind(self, listen: SocketAddr) -> KagimoriH2cServer<L> {
        KagimoriH2cServer::new(self, listen)
    }

    pub fn bind_uds(self, path: impl AsRef<Path>) -> KagimoriUnixDomainSocketServer<L> {
        KagimoriUnixDomainSocketServer::new(self, path.as_ref().to_path_buf())
    }
}

impl<L> KagimoriServer<L>
where
    L: 'static + AuditLogger + Clone,
{
    fn create_service(self) -> Routes {
        let mut routes = Routes::default();
        if self.kms_v2_enabled {
            routes = routes.add_service(KeyManagementServiceServer::new(KmsService::new(
                self.encryptor.clone(),
            )));
        }

        #[cfg(feature = "reflection")]
        {
            routes = routes.add_service(
                tonic_reflection::server::Builder::configure()
                    .register_encoded_file_descriptor_set(tonic::include_file_descriptor_set!(
                        "kagimori_descriptor"
                    ))
                    .build_v1()
                    .unwrap(),
            );
        }

        routes
    }
}
