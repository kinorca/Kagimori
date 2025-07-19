use crate::{AuditLog, AuditLogger};
use async_trait::async_trait;

#[derive(Debug, Clone, Copy, Default)]
pub struct TracingAuditLogger;

#[async_trait]
impl AuditLogger for TracingAuditLogger {
    async fn log(&self, log: AuditLog) {
        tracing::info!("AuditLog: {log:?}");
    }
}
