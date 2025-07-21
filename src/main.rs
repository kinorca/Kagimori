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

mod args;
mod master_key;

use crate::args::{Args, CipherAlgorithm};
use audit_log::AuditLogger;
use audit_log::logger::tracing::TracingAuditLogger;
use ciphers::rotatable::RotatableCipher;
use clap::Parser;
use encryption::{Encryptor, KeyAlgorithm};
use server::{CertificateDer, KagimoriServer, PemObject, PrivateKeyDer};
use std::path::Path;
use tracing::{debug, info};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(Layer::new())
        .with(EnvFilter::from_env("LOG_LEVEL"))
        .init();

    info!("Kagimori {VERSION} (Licensed under the GNU General Public License v3)");

    let args = Args::parse();
    debug!("Command line arguments: {args:?}");

    let logger = TracingAuditLogger;

    let cipher = args.create_master_key().into_cipher();

    run_server(cipher, logger, args).await;
}

async fn run_server<L>(cipher: RotatableCipher, audit_logger: L, args: Args)
where
    L: 'static + AuditLogger + Clone,
{
    let encryptor = Encryptor::new(
        audit_logger,
        match args.dek_algorithm {
            CipherAlgorithm::Chacha20Poly1305 => KeyAlgorithm::ChaCha20Poly1305,
            CipherAlgorithm::AesGcmSiv => KeyAlgorithm::AesGcmSiv,
        },
        cipher.default_key_id(),
        cipher,
    );

    let mut server = KagimoriServer::new(encryptor);
    if args.kms_v2 {
        server = server.enable_kms_v2();
    }
    if let Some(sock_addr) = args.listen.strip_prefix("tcp://") {
        if let Some(cert) = args.tls_certificate
            && let Some(private_key) = args.tls_private_key
        {
            let cert = CertificateDer::pem_file_iter(cert)
                .unwrap()
                .map(Result::unwrap);
            let private_key = PrivateKeyDer::from_pem_file(private_key).unwrap();

            server
                .bind_tls(sock_addr.parse().unwrap(), cert.collect(), private_key)
                .unwrap()
                .run()
                .await
                .unwrap();
        } else {
            server.bind(sock_addr.parse().unwrap()).run().await.unwrap();
        }
    } else if let Some(path) = args.listen.strip_prefix("unix://") {
        server.bind_uds(Path::new(path)).run().await.unwrap();
    }
}
