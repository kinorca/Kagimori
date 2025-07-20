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

use crate::master_key::MasterKeyConfig;
use clap::{Parser, ValueEnum};
use std::fs::File;

#[derive(Debug, Copy, Clone, ValueEnum)]
pub(crate) enum StorageType {
    Etcd,
    File,
}

#[derive(Debug, Parser)]
pub(crate) struct Args {
    // server
    #[arg(
        long,
        help = "Listen address (tcp://HOST:PORT or unix://PATH)",
        default_value = "tcp://0.0.0.0:8602"
    )]
    pub listen: String,

    // KMS v2
    #[arg(long, help = "Enable Kubernetes KMS v2")]
    pub kms_v2: bool,

    // TLS
    #[arg(long, help = "Path to TLS certificate PEM file")]
    pub tls_certificate: Option<String>,
    #[arg(long, help = "Path to TLS private key PEM file")]
    pub tls_private_key: Option<String>,

    // Master key
    #[arg(long, help = "Path to master key configuration file")]
    pub master_key: String,

    // storage
    #[arg(
        long,
        help = "Storage etcd endpoints (if --storage=etcd)",
        value_delimiter = ','
    )]
    pub storage_etcd_endpoints: Vec<String>,
    #[arg(
        long,
        help = "Storage directory (if --storage=file)",
        default_value = "/var/lib/kagimori/storage"
    )]
    pub storage_directory: String,

    #[arg(long, help = "Storage type", default_value = "file")]
    pub storage: StorageType,
}

impl Args {
    pub(crate) fn create_master_key(&self) -> MasterKeyConfig {
        let file = File::open(&self.master_key).unwrap();
        serde_yml::from_reader(file).unwrap()
    }
}
