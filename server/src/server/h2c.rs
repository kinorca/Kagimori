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

use crate::server::KagimoriServer;
use encryption::DataStorage;
use std::net::SocketAddr;
use tonic::transport::Server;

pub struct KagimoriH2cServer<S> {
    inner: KagimoriServer<S>,
    listen: SocketAddr,
}

impl<S> KagimoriH2cServer<S> {
    pub(super) fn new(inner: KagimoriServer<S>, listen: SocketAddr) -> Self {
        Self { inner, listen }
    }
}

impl<S> KagimoriH2cServer<S>
where
    S: 'static,
    S: DataStorage,
    S: Clone,
{
    pub async fn run(self) -> Result<(), tonic::transport::Error> {
        let svc = self.inner.create_service();
        Server::builder().add_routes(svc).serve(self.listen).await?;
        Ok(())
    }
}
