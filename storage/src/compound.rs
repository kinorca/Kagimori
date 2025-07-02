use crate::LowLevelStorage;
use async_trait::async_trait;
use std::sync::Arc;

#[derive(Clone)]
pub struct CompoundLowLevelStorage {
    storages: Vec<Arc<dyn LowLevelStorage>>,
}

pub struct CompoundLowLevelStorageBuilder {
    storages: Vec<Arc<dyn LowLevelStorage>>,
}

impl CompoundLowLevelStorageBuilder {
    pub fn new() -> Self {
        Self {
            storages: Vec::new(),
        }
    }

    pub fn add_storage<S>(mut self, storage: S) -> Self
    where
        S: 'static + LowLevelStorage,
    {
        self.storages.push(Arc::new(storage));
        self
    }

    pub fn build(self) -> CompoundLowLevelStorage {
        CompoundLowLevelStorage::new(self.storages)
    }
}

impl CompoundLowLevelStorage {
    pub fn new(storages: Vec<Arc<dyn LowLevelStorage>>) -> Self {
        Self { storages }
    }

    pub fn builder() -> CompoundLowLevelStorageBuilder {
        CompoundLowLevelStorageBuilder::new()
    }
}

#[async_trait]
impl LowLevelStorage for CompoundLowLevelStorage {
    async fn set(&self, key: &str, value: &[u8]) -> Result<(), crate::Error> {
        for storage in &self.storages {
            storage.set(key, value).await?;
        }

        Ok(())
    }

    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, crate::Error> {
        for storage in &self.storages {
            if let Some(value) = storage.get(key).await? {
                return Ok(Some(value));
            }
        }

        Ok(None)
    }

    async fn delete(&self, key: &str) -> Result<(), crate::Error> {
        for storage in &self.storages {
            storage.delete(key).await?;
        }
        Ok(())
    }

    async fn exists(&self, key: &str) -> Result<bool, crate::Error> {
        for storage in &self.storages {
            if storage.exists(key).await? {
                return Ok(true);
            }
        }

        Ok(false)
    }
}
