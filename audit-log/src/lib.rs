mod data;

pub trait AuditLogger {
    fn log(&self);
}
