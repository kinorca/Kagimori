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

use crate::args::{Args, StorageType};
use audit_log::AuditLogger;
use audit_log::logger::tracing::TracingAuditLogger;
use ciphers::Cipher;
use clap::Parser;
use encryption::Encryptor;
use server::{CertificateDer, KagimoriServer, PemObject, PrivateKeyDer};
use std::path::Path;
use storage::crypto::CryptedStorage;
use storage::lowlevel::LowLevelStorage;
use storage::lowlevel::etcd::{Client, EtcdLowLevelStorage};
use storage::lowlevel::file::FileLowLevelStorage;
use tracing::{debug, info};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::{Layer, layer};
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
    match args.storage {
        StorageType::Etcd => {
            run_server(
                cipher,
                EtcdLowLevelStorage::new(
                    Client::connect(args.storage_etcd_endpoints.as_slice(), None)
                        .await
                        .unwrap(),
                ),
                logger,
                args,
            )
            .await;
        }
        StorageType::File => {
            run_server(
                cipher,
                FileLowLevelStorage::new(args.storage_directory.as_str()).unwrap(),
                logger,
                args,
            )
            .await;
        }
    }
}

async fn run_server<C, S, L>(cipher: C, lls: S, audit_logger: L, args: Args)
where
    C: 'static + Cipher + Clone,
    S: 'static + LowLevelStorage + Clone,
    L: 'static + AuditLogger + Clone,
{
    let storage = CryptedStorage::new(cipher, lls);
    let encryptor = Encryptor::new(storage, audit_logger);

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
