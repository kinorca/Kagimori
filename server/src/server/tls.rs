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
use audit_log::AuditLogger;
use encryption::DataStorage;
use hyper::http;
use hyper::server::conn::http2::Builder;
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::service::TowerToHyperService;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;
use tokio_rustls::rustls::ServerConfig;
use tonic::body::Body;
use tower::ServiceExt;
use tracing::{error, info};

pub struct KagimoriTlsServer<S, L> {
    inner: KagimoriServer<S, L>,
    config: ServerConfig,
    listen: SocketAddr,
}

impl<S, L> KagimoriTlsServer<S, L> {
    pub(super) fn new(
        inner: KagimoriServer<S, L>,
        config: ServerConfig,
        listen: SocketAddr,
    ) -> Self {
        Self {
            inner,
            config,
            listen,
        }
    }
}

impl<S, L> KagimoriTlsServer<S, L>
where
    S: 'static + DataStorage + Clone,
    L: 'static + AuditLogger + Clone,
{
    pub async fn run(self) -> std::io::Result<()> {
        let listener = TcpListener::bind(self.listen).await?;
        let tls_acceptor = TlsAcceptor::from(Arc::new(self.config));

        let svc = self.inner.create_service();
        let svc = tower::ServiceBuilder::new().service(svc);

        let http = Builder::new(TokioExecutor::new());

        loop {
            let (conn, addr) = match listener.accept().await {
                Ok(incoming) => incoming,
                Err(e) => {
                    error!("Failed to accept connection: {e}");
                    continue;
                }
            };

            info!("Accepted connection from: {addr}");

            let http = http.clone();
            let tls_acceptor = tls_acceptor.clone();
            let svc = svc.clone();

            tokio::spawn(async move {
                let mut certificates = Vec::new();

                match tls_acceptor
                    .accept_with(conn, |info| {
                        if let Some(certs) = info.peer_certificates() {
                            for cert in certs {
                                certificates.push(cert.clone());
                            }
                        }
                    })
                    .await
                {
                    Ok(conn) => {
                        if let Err(e) = http
                            .serve_connection(
                                TokioIo::new(conn),
                                TowerToHyperService::new(
                                    svc.map_request(|req: http::Request<_>| req.map(Body::new)),
                                ),
                            )
                            .await
                        {
                            error!("Failed to serve connection: {e}");
                        }
                    }
                    Err(e) => {
                        error!("Failed to accept TLS connection: {e}");
                        return;
                    }
                }
            });
        }
    }
}
