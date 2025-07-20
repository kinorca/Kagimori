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

pub mod chacha20poly1305;
mod error;

pub mod aesgcmsiv;
pub mod oneof;
pub mod rotatable;
#[cfg(test)]
mod test;

pub use crate::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait Cipher: Send + Sync {
    fn name(&self) -> &'static str;
    fn key(&self) -> Vec<u8>;
    async fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, Error>;
    async fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, Error>;
}

#[derive(Clone, Copy)]
pub struct Unencrypted;

#[async_trait]
impl Cipher for Unencrypted {
    fn name(&self) -> &'static str {
        "Unencrypted"
    }

    fn key(&self) -> Vec<u8> {
        Vec::new()
    }

    async fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, Error> {
        Ok(data.to_vec())
    }

    async fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, Error> {
        Ok(data.to_vec())
    }
}
