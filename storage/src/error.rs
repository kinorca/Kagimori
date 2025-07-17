use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    Cipher(ciphers::Error),
    #[cfg(feature = "file")]
    Io(std::io::Error),
    #[cfg(feature = "etcd")]
    Etcd(etcd_client::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
