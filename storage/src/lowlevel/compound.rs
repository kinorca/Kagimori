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

use crate::DataStorage;
use crate::lowlevel::LowLevelStorage;
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

impl LowLevelStorage for CompoundLowLevelStorage {}

#[async_trait]
impl DataStorage for CompoundLowLevelStorage {
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
