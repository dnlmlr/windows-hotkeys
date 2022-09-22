use thiserror::Error;

use crate::keys::VKey;

#[derive(Debug, Error)]
pub enum HkError {
    #[error("invalid key name `{0}`")]
    InvalidKey(String),
    #[error("invalid key char `{0}`")]
    InvalidKeyChar(char),
    #[error("VKey is not a ModKey `{0}`")]
    NotAModkey(VKey),
    #[error("Hotkey registration failed. Hotkey or Id might be in use already")]
    RegistrationFailed,
    #[error("Hotkey unregistration failed")]
    UnregistrationFailed,
}
