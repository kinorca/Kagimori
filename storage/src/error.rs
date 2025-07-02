use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    Io(std::io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
