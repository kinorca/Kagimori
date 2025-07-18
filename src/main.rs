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
use ciphers::Cipher;
use clap::Parser;
use encryption::Encryptor;
use server::{CertificateDer, KagimoriServer, PemObject, PrivateKeyDer};
use storage::crypto::CryptedStorage;
use storage::lowlevel::LowLevelStorage;
use storage::lowlevel::etcd::{Client, EtcdLowLevelStorage};
use storage::lowlevel::file::FileLowLevelStorage;
use tracing::{debug, info};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    info!("Kagimori {VERSION} (Licensed under GNU GPL v3)");

    let args = Args::parse();
    debug!("Command line arguments: {args:?}");

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
                args,
            )
            .await;
        }
        StorageType::File => {
            run_server(
                cipher,
                FileLowLevelStorage::new(args.storage_directory.as_str()).unwrap(),
                args,
            )
            .await;
        }
    }
}

async fn run_server<C, S>(cipher: C, lls: S, args: Args)
where
    C: 'static + Cipher + Clone,
    S: 'static + LowLevelStorage + Clone,
{
    let storage = CryptedStorage::new(cipher, lls);
    let encryptor = Encryptor::new(storage);

    let mut server = KagimoriServer::new(encryptor);
    if let Some(key_id) = args.kms_v2_key_id {
        server = server.enable_kms_v2(key_id);
    }
    if let Some(cert) = args.tls_certificate
        && let Some(private_key) = args.tls_private_key
    {
        let cert = CertificateDer::pem_file_iter(cert)
            .unwrap()
            .map(Result::unwrap);
        let private_key = PrivateKeyDer::from_pem_file(private_key).unwrap();

        server
            .bind_tls(args.listen.parse().unwrap(), cert.collect(), private_key)
            .unwrap()
            .run()
            .await
            .unwrap();
    } else {
        server
            .bind(args.listen.parse().unwrap())
            .run()
            .await
            .unwrap();
    }
}
