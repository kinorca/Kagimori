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

use crate::KagimoriServer;
use crate::debug_log::DebugLog;
use audit_log::AuditLogger;
use std::path::PathBuf;
use tokio::net::UnixListener;
use tonic::codegen::tokio_stream::wrappers::UnixListenerStream;
use tonic::transport::Server;
use tracing::info;

pub struct KagimoriUnixDomainSocketServer<L> {
    inner: KagimoriServer<L>,
    path: PathBuf,
}

impl<L> KagimoriUnixDomainSocketServer<L> {
    pub(super) fn new(inner: KagimoriServer<L>, path: PathBuf) -> Self {
        Self { inner, path }
    }
}

impl<L> KagimoriUnixDomainSocketServer<L>
where
    L: 'static + AuditLogger + Clone,
{
    pub async fn run(self) -> std::io::Result<()> {
        if let Some(parent) = self.path.parent() {
            tokio::fs::create_dir_all(parent).await.debug_log()?;
        }

        info!(
            "Listening on: unix://{}",
            self.path.as_os_str().to_str().unwrap_or_default()
        );
        let uds = UnixListener::bind(&self.path).debug_log()?;
        let uds_stream = UnixListenerStream::new(uds);

        let svc = self.inner.create_service();

        Server::builder()
            .add_routes(svc)
            .serve_with_incoming(uds_stream)
            .await
            .debug_log()
            .unwrap();

        Ok(())
    }
}
