use crate::KagimoriServer;
use audit_log::AuditLogger;
use encryption::DataStorage;
use std::path::PathBuf;
use tokio::net::UnixListener;
use tonic::codegen::tokio_stream::wrappers::UnixListenerStream;
use tonic::transport::Server;

pub struct KagimoriUnixDomainSocketServer<S, L> {
    inner: KagimoriServer<S, L>,
    path: PathBuf,
}

impl<S, L> KagimoriUnixDomainSocketServer<S, L> {
    pub(super) fn new(inner: KagimoriServer<S, L>, path: PathBuf) -> Self {
        Self { inner, path }
    }
}

impl<S, L> KagimoriUnixDomainSocketServer<S, L>
where
    S: 'static + DataStorage + Clone,
    L: 'static + AuditLogger + Clone,
{
    pub async fn run(self) -> std::io::Result<()> {
        if let Some(parent) = self.path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let uds = UnixListener::bind(&self.path)?;
        let uds_stream = UnixListenerStream::new(uds);

        let svc = self.inner.create_service();
        Server::builder()
            .add_routes(svc)
            .serve_with_incoming(uds_stream)
            .await
            .unwrap();
        Ok(())
    }
}
