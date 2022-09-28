use std::fmt::Display;

use crate::{error::HkError, VKey};

/// Modifier Key for hotkeys.
///
/// See: `fsModifiers` from <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerhotkey>
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModKey {
    Alt,
    Ctrl,
    Shift,
    Win,
}

impl ModKey {
    /// Take in a string and interpret it as one of the modifier keys.
    /// Possible values are:
    /// - ALT
    /// - CTRL / CONTROL
    /// - SHIFT
    /// - WIN / WINDOWS / SUPER
    /// - NOREPEAT
    ///
    pub fn from_keyname(val: &str) -> Result<Self, HkError> {
        Ok(match val.to_ascii_uppercase().as_ref() {
            "ALT" => ModKey::Alt,
            "CTRL" | "CONTROL" => ModKey::Ctrl,
            "SHIFT" => ModKey::Shift,
            "WIN" | "WINDOWS" | "SUPER" => ModKey::Win,
            val => return Err(HkError::InvalidKey(val.to_string())),
        })
    }

    /// Obtain the modifier code for the `ModKey`.
    ///
    /// See: `fsModifiers` from <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerhotkey>
    /// 
    pub const fn to_mod_code(&self) -> u32 {
        use winapi::um::winuser::*;

        match self {
            ModKey::Alt => MOD_ALT as u32,
            ModKey::Ctrl => MOD_CONTROL as u32,
            ModKey::Shift => MOD_SHIFT as u32,
            ModKey::Win => MOD_WIN as u32,
        }
    }

    /// Combine multiple `ModKey`s using bitwise OR
    /// 
    pub(crate) fn combine(keys: &[ModKey]) -> u32 {
        keys.iter().fold(0, |a, b| a | b.to_mod_code())
    }
}

impl Display for ModKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let key = match self {
            ModKey::Alt => "ALT",
            ModKey::Ctrl => "CONTROL",
            ModKey::Shift => "SHIFT",
            ModKey::Win => "WIN",
        };
        write!(f, "{}", key)
    }
}

impl From<ModKey> for VKey {
    fn from(mk: ModKey) -> VKey {
        match mk {
            ModKey::Alt => VKey::Menu,
            ModKey::Ctrl => VKey::Control,
            ModKey::Shift => VKey::Shift,
            ModKey::Win => VKey::LWin,
        }
    }
}
