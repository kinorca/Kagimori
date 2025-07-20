use tracing::debug;

pub(crate) trait DebugLog {
    fn debug_log(self) -> Self;
}

impl<T, E> DebugLog for Result<T, E>
where
    E: std::fmt::Debug,
{
    fn debug_log(self) -> Self {
        match self {
            Ok(v) => Ok(v),
            Err(e) => {
                debug!("Error: {:?}", e);
                Err(e)
            }
        }
    }
}
