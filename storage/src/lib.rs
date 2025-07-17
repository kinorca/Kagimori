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

pub mod crypto;
mod error;
pub mod lowlevel;

use async_trait::async_trait;
pub use error::Error;

#[async_trait]
pub trait DataStorage: Send + Sync {
    async fn set(&self, key: &str, value: &[u8]) -> Result<(), Error>;
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, Error>;
    async fn delete(&self, key: &str) -> Result<(), Error>;
    async fn exists(&self, key: &str) -> Result<bool, Error>;
}
