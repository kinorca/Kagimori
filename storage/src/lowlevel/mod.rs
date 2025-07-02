use crate::DataStorage;

pub mod compound;
#[cfg(feature = "etcd")]
pub mod etcd;
#[cfg(feature = "file")]
pub mod file;

pub trait LowLevelStorage: DataStorage {}
