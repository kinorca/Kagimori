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

use crate::lowlevel::LowLevelStorage;
use crate::{DataStorage, Error};
use async_trait::async_trait;
use etcd_client::{Compare, CompareOp, GetOptions, Txn, TxnOp};

pub use etcd_client::Client;

#[derive(Clone)]
pub struct EtcdLowLevelStorage {
    client: Client,
}

impl EtcdLowLevelStorage {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

impl LowLevelStorage for EtcdLowLevelStorage {}

#[async_trait]
impl DataStorage for EtcdLowLevelStorage {
    #[tracing::instrument(skip(self))]
    async fn set(&self, key: &str, value: &[u8]) -> Result<(), Error> {
        let mut client = self.client.clone();
        client.put(key, value, None).await.map_err(Error::Etcd)?;
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, Error> {
        let mut client = self.client.clone();
        let value = client.get(key, None).await.map_err(Error::Etcd)?;
        Ok(value.kvs().first().map(|kv| kv.value().to_vec()))
    }

    #[tracing::instrument(skip(self))]
    async fn delete(&self, key: &str) -> Result<(), Error> {
        let mut client = self.client.clone();
        client.delete(key, None).await.map_err(Error::Etcd)?;
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn exists(&self, key: &str) -> Result<bool, Error> {
        let mut client = self.client.clone();
        let value = client
            .get(key, Some(GetOptions::default().with_count_only()))
            .await
            .map_err(Error::Etcd)?;
        Ok(value.count() > 0)
    }

    #[tracing::instrument(skip(self))]
    async fn set_if_absent(&self, key: &str, value: &[u8]) -> Result<bool, Error> {
        let txn = Txn::default()
            .when(vec![Compare::create_revision(key, CompareOp::Equal, 0)])
            .and_then(vec![TxnOp::put(key, value, None)]);

        let mut client = self.client.clone();
        let res = client.txn(txn).await.map_err(Error::Etcd)?;

        Ok(res.succeeded())
    }
}
