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
use crate::server::KagimoriServer;
use audit_log::AuditLogger;
use std::net::SocketAddr;
use tonic::transport::Server;
use tracing::info;

pub struct KagimoriH2cServer<L> {
    inner: KagimoriServer<L>,
    listen: SocketAddr,
}

impl<L> KagimoriH2cServer<L> {
    pub(super) fn new(inner: KagimoriServer<L>, listen: SocketAddr) -> Self {
        Self { inner, listen }
    }
}

impl<L> KagimoriH2cServer<L>
where
    L: 'static + AuditLogger + Clone,
{
    pub async fn run(self) -> Result<(), tonic::transport::Error> {
        let svc = self.inner.create_service();
        info!("Listening on: tcp://{}", self.listen);
        Server::builder()
            .add_routes(svc)
            .serve(self.listen)
            .await
            .debug_log()?;
        Ok(())
    }
}
