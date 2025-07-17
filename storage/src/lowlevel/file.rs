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
use std::path::{Path, PathBuf};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct FileLowLevelStorage {
    directory: PathBuf,
}

impl FileLowLevelStorage {
    pub fn new(directory: impl AsRef<Path>) -> Result<Self, Error> {
        let directory = directory.as_ref().to_path_buf();
        if !directory.exists() {
            std::fs::create_dir_all(&directory).map_err(Error::Io)?;
        }
        Ok(Self { directory })
    }

    fn path(&self, key: &str) -> PathBuf {
        self.directory.join(key)
    }
}

impl LowLevelStorage for FileLowLevelStorage {}

#[async_trait]
impl DataStorage for FileLowLevelStorage {
    async fn set(&self, key: &str, value: &[u8]) -> Result<(), Error> {
        let file = self.path(key);
        let file = tokio::fs::File::create(&file).await.map_err(Error::Io)?;
        let mut writer = tokio::io::BufWriter::new(file);
        writer.write_all(value).await.map_err(Error::Io)
    }

    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, Error> {
        let file = self.path(key);
        if !file.exists() {
            return Ok(None);
        }
        let file = tokio::fs::File::open(&file).await.map_err(Error::Io)?;
        let mut reader = tokio::io::BufReader::new(file);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).await.map_err(Error::Io)?;
        Ok(Some(buffer))
    }

    async fn delete(&self, key: &str) -> Result<(), Error> {
        let file = self.path(key);
        if file.exists() {
            tokio::fs::remove_file(file).await.map_err(Error::Io)?;
        }
        Ok(())
    }

    async fn exists(&self, key: &str) -> Result<bool, Error> {
        let file = self.path(key);
        Ok(file.exists())
    }
}
