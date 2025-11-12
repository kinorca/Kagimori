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

#[derive(Debug, Copy, Clone, ValueEnum)]
pub(crate) enum CipherAlgorithm {
    Chacha20Poly1305,
    AesGcmSiv,
}

#[derive(Debug, Parser)]
pub(crate) struct Args {
    // server
    #[arg(
        long,
        help = "Listen address (tcp://HOST:PORT or unix://PATH)",
        default_value = "tcp://0.0.0.0:8602"
    )]
    pub listen_kms_v2: String,
    #[arg(
        long,
        help = "Listen address (tcp://HOST:PORT or unix://PATH)",
        default_value = "tcp://0.0.0.0:8603"
    )]
    pub listen_kagimori_v1: String,
    #[arg(
        long,
        help = "Listen address (http://HOST:PORT)",
        default_value = "http://0.0.0.0:8604"
    )]
    pub listen_transit_stateless_v1: String,

    // Service enabler
    #[arg(long, help = "Enable Kubernetes KMS v2 API")]
    pub kms_v2: bool,
    #[arg(long, help = "Enable Kagimori v1 gRPC API")]
    pub kagimori_v1: bool,
    #[arg(long, help = "Enable Transit v1 API (Stateless mode)")]
    pub transit_stateless_v1: bool,

    // TLS
    #[arg(long, help = "Path to TLS certificate PEM file")]
    pub tls_certificate: Option<String>,
    #[arg(long, help = "Path to TLS private key PEM file")]
    pub tls_private_key: Option<String>,

    // Master key
    #[arg(long, help = "Path to master key configuration file")]
    pub master_key: String,

    // DEK
    #[arg(long, help = "DEK algorithm", default_value = "chacha20-poly1305")]
    pub dek_algorithm: CipherAlgorithm,
}

impl Args {
    pub(crate) fn create_master_key(&self) -> MasterKeyConfig {
        let content = std::fs::read_to_string(&self.master_key).unwrap();
        toml::from_str(&content).unwrap()
    }
}
