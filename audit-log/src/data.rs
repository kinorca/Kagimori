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

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditLog {
    pub timestamp: DateTime<Utc>,
    pub event_id: String,
    pub service: String,
    pub user: String,
    pub action: Action,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    Encryption(EncryptionAction),
    Decryption(DecryptionAction),
    KeyRotation(KeyRotationAction),
    KeyDeletion(KeyDeletionAction),
    KeyCreation(KeyCreationAction),
    Signing(SigningAction),
    Verification(VerificationAction),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EncryptionAction {
    pub data_key: Option<String>,
    pub algorithm: String,
    pub key_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecryptionAction {
    pub data_key: Option<String>,
    pub algorithm: String,
    pub key_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyRotationAction {
    pub algorithm: String,
    pub key_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyDeletionAction {
    pub algorithm: String,
    pub key_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyCreationAction {
    pub algorithm: String,
    pub key_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SigningAction {
    pub data_key: Option<String>,
    pub algorithm: String,
    pub key_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationAction {
    pub data_key: Option<String>,
    pub algorithm: String,
    pub key_id: String,
}
