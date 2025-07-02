use crate::lowlevel::LowLevelStorage;
use crate::{DataStorage, Error};
use async_trait::async_trait;
use etcd_client::{Client, GetOptions};

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
    async fn set(&self, key: &str, value: &[u8]) -> Result<(), Error> {
        let mut client = self.client.clone();
        client.put(key, value, None).await.map_err(Error::Etcd)?;
        Ok(())
    }

    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, Error> {
        let mut client = self.client.clone();
        let value = client.get(key, None).await.map_err(Error::Etcd)?;
        Ok(value.kvs().first().map(|kv| kv.value().to_vec()))
    }

    async fn delete(&self, key: &str) -> Result<(), Error> {
        let mut client = self.client.clone();
        client.delete(key, None).await.map_err(Error::Etcd)?;
        Ok(())
    }

    async fn exists(&self, key: &str) -> Result<bool, Error> {
        let mut client = self.client.clone();
        let value = client
            .get(key, Some(GetOptions::default().with_count_only()))
            .await
            .map_err(Error::Etcd)?;
        Ok(value.count() > 0)
    }
}
