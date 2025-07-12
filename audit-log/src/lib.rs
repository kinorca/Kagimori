use crate::data::AuditLog;

mod data;

pub trait AuditLogger {
    fn log(&self, log: AuditLog);
}
