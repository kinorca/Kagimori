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
    pub data_key: String,
    pub algorithm: String,
    pub key_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecryptionAction {
    pub data_key: String,
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
    pub data_key: String,
    pub algorithm: String,
    pub key_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationAction {
    pub data_key: String,
    pub algorithm: String,
    pub key_id: String,
}
