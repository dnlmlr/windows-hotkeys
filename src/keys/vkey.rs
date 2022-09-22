use std::{fmt::Display, hash::Hash};

use crate::error::HkError;

use super::ModKey;

/// Virtual Key Code wrapper. The codes and variants follow the virtual key codes.
/// Not supported as enum variants are the mouse buttons, IME keys, `VK_PACKET` and `VK_NONAME`.
/// The letter keys (`A` to `Z`) are added as additionall variants, as well as the number keys
/// (`0` to `9`) which are available as `Vk0` to `Vk9`.
///
/// A `VKey` can be created for any arbitrary keycode by using the `CustomKeyCode` variant.
///
/// See: https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
///
/// ## Note
/// Matching against a `VKey` can be problematic since all of the variants can also be represented
/// using the `CustomKeyCode` variant. If a reliable check for a `VKey` is needed, the keycode
/// from the `VKey::to_vk_code` function should be used to get the unique keycode.
///
#[derive(Debug, Clone, Copy)]
pub enum VKey {
    /// Backspace key
    Back,
    /// Tab key
    Tab,
    /// CLEAR key
    Clear,
    /// ENTER key
    Return,
    /// Shift key
    Shift,
    /// CTRL key
    Control,
    /// ALT key
    Menu,
    /// PAUSE
    Pause,
    /// CAPS LOCK key
    Capital,
    /// ESC key
    Escape,
    /// SPACEBAR
    Space,
    /// PAGE UP key
    Prior,
    /// PAGE DOWN key
    Next,
    /// END key
    End,
    /// HOME key
    Home,
    /// LEFT ARROW key
    Left,
    /// UP ARROW key
    Up,
    /// RIGHT ARROW key
    Right,
    /// DOWN ARROW key
    Down,
    /// SELECT key
    Select,
    /// PRINT key
    Print,
    /// EXECUTE key
    Execute,
    /// PRINT SCREEN key
    Snapshot,
    /// INS key
    Insert,
    /// DEL key
    Delete,
    /// HELP key
    Help,

    /// Left Windows key (Natural keyboard)
    LWin,
    /// Right Windows key (Natural keyboard)
    RWin,
    /// Applications key (Natural keyboard)
    Apps,
    /// Computer Sleep key
    Sleep,
    /// Numeric keypad 0 key
    Numpad0,
    /// Numeric keypad 1 key
    Numpad1,
    /// Numeric keypad 2 key
    Numpad2,
    /// Numeric keypad 3 key
    Numpad3,
    /// Numeric keypad 4 key
    Numpad4,
    /// Numeric keypad 5 key
    Numpad5,
    /// Numeric keypad 6 key
    Numpad6,
    /// Numeric keypad 7 key
    Numpad7,
    /// Numeric keypad 8 key
    Numpad8,
    /// Numeric keypad 9 key
    Numpad9,
    /// Multiply key
    Multiply,
    /// Add key
    Add,
    /// Separator key
    Separator,
    /// Subtract key
    Subtract,
    /// Decimal key
    Decimal,
    /// Divide key
    Divide,
    /// F1 key
    F1,
    /// F2 key
    F2,
    /// F3 key
    F3,
    /// F4 key
    F4,
    /// F5 key
    F5,
    /// F6 key
    F6,
    /// F7 key
    F7,
    /// F8 key
    F8,
    /// F9 key
    F9,
    /// F10 key
    F10,
    /// F11 key
    F11,
    /// F12 key
    F12,
    /// F13 key
    F13,
    /// F14 key
    F14,
    /// F15 key
    F15,
    /// F16 key
    F16,
    /// F17 key
    F17,
    /// F18 key
    F18,
    /// F19 key
    F19,
    /// F20 key
    F20,
    /// F21 key
    F21,
    /// F22 key
    F22,
    /// F23 key
    F23,
    /// F24 key
    F24,
    /// NUM LOCK key
    Numlock,
    /// SCROLL LOCK key
    Scroll,
    /// Left SHIFT key
    LShift,
    /// Right SHIFT key
    RShift,
    /// Left CONTROL key
    LControl,
    /// Right CONTROL key
    RControl,
    /// Left ALT key
    LMenu,
    /// Right ALT key
    RMenu,
    /// Browser Back key
    BrowserBack,
    /// Browser Forward key
    BrowserForward,
    /// Browser Refresh key
    BrowserRefresh,
    /// Browser Stop key
    BrowserStop,
    /// Browser Search key
    BrowserSearch,
    /// Browser Favorites key
    BrowserFavorites,
    /// Browser Start and Home key
    BrowserHome,
    /// Volume Mute key
    VolumeMute,
    /// Volume Down key
    VolumeDown,
    /// Volume Up key
    VolumeUp,
    /// Next Track key
    MediaNextTrack,
    /// Previous Track key
    MediaPrevTrack,
    /// Stop Media key
    MediaStop,
    /// Play/Pause Media key
    MediaPlayPause,
    /// Start Mail key
    LaunchMail,
    /// Select Media key
    LaunchMediaSelect,
    /// Start Application 1 key
    LaunchApp1,
    /// Start Application 2 key
    LaunchApp2,
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard,
    /// the `;:` key
    Oem1,
    /// For any country/region, the `+` key
    OemPlus,
    /// For any country/region, the `,` key
    OemComma,
    /// For any country/region, the `-` key
    OemMinus,
    /// For any country/region, the `.` key
    OemPeriod,
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard,
    /// the `/?` key
    Oem2,
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard,
    /// the `~` key
    Oem3,
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard,
    /// the `[{` key
    Oem4,
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard,
    /// the `\|` key
    Oem5,
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard,
    /// the `]}` key
    Oem6,
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard keyboard,
    /// the `"'` key
    Oem7,
    /// Used for miscellaneous characters; it can vary by keyboard.
    Oem8,
    /// The `<>` keys on the US standard keyboard, or the `\\|` key on the non-US 102-key keyboard
    Oem102,
    /// Attn key
    Attn,
    /// CrSel key
    Crsel,
    /// ExSel key
    Exsel,
    /// Play key
    Play,
    /// Zoom key
    Zoom,
    /// PA1 key
    Pa1,
    /// Clear key
    OemClear,

    /// 0 key
    Vk0,
    /// 1 key
    Vk1,
    /// 2 key
    Vk2,
    /// 3 key
    Vk3,
    /// 4 key
    Vk4,
    /// 5 key
    Vk5,
    /// 6 key
    Vk6,
    /// 7 key
    Vk7,
    /// 8 key
    Vk8,
    /// 9 key
    Vk9,

    /// A key
    A,
    /// B key
    B,
    /// C key
    C,
    /// D key
    D,
    /// E key
    E,
    /// F key
    F,
    /// G key
    G,
    /// H key
    H,
    /// I key
    I,
    /// J key
    J,
    /// K key
    K,
    /// L key
    L,
    /// M key
    M,
    /// N key
    N,
    /// O key
    O,
    /// P key
    P,
    /// Q key
    Q,
    /// R key
    R,
    /// S key
    S,
    /// T key
    T,
    /// U key
    U,
    /// V key
    V,
    /// W key
    W,
    /// X key
    X,
    /// Y key
    Y,
    /// Z key
    Z,

    /// Virtual key specified by the actual keycode. This can be used to create a VKey for keys
    /// that are not covered by the other enum variants.
    ///
    /// See: https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
    CustomKeyCode(i32),
}

impl VKey {
    /// Try to create a VKey from a char. This only works for the simple number and letter keys
    /// ('A' to 'Z' and '0' to '9'). Letters can be upper or lower case
    pub const fn from_char(ch: char) -> Result<Self, HkError> {
        match ch.to_ascii_uppercase() {
            ch @ ('A'..='Z' | '0'..='9') => Ok(Self::CustomKeyCode(ch as i32)),
            ch => Err(HkError::InvalidKeyChar(ch)),
        }
    }

    /// Get the actual windows virtual keycode for the `VKey` for usage with winapi functions
    pub const fn to_vk_code(&self) -> i32 {
        use winapi::um::winuser::*;
        match self {
            VKey::Back => VK_BACK,
            VKey::Tab => VK_TAB,
            VKey::Clear => VK_CLEAR,
            VKey::Return => VK_RETURN,
            VKey::Shift => VK_SHIFT,
            VKey::Control => VK_CONTROL,
            VKey::Menu => VK_MENU,
            VKey::Pause => VK_PAUSE,
            VKey::Capital => VK_CAPITAL,
            VKey::Escape => VK_ESCAPE,
            VKey::Space => VK_SPACE,
            VKey::Prior => VK_PRIOR,
            VKey::Next => VK_NEXT,
            VKey::End => VK_END,
            VKey::Home => VK_HOME,
            VKey::Left => VK_LEFT,
            VKey::Up => VK_UP,
            VKey::Right => VK_RIGHT,
            VKey::Down => VK_DOWN,
            VKey::Select => VK_SELECT,
            VKey::Print => VK_PRINT,
            VKey::Execute => VK_EXECUTE,
            VKey::Snapshot => VK_SNAPSHOT,
            VKey::Insert => VK_INSERT,
            VKey::Delete => VK_DELETE,
            VKey::Help => VK_HELP,
            VKey::LWin => VK_LWIN,
            VKey::RWin => VK_RWIN,
            VKey::Apps => VK_APPS,
            VKey::Sleep => VK_SLEEP,
            VKey::Numpad0 => VK_NUMPAD0,
            VKey::Numpad1 => VK_NUMPAD1,
            VKey::Numpad2 => VK_NUMPAD2,
            VKey::Numpad3 => VK_NUMPAD3,
            VKey::Numpad4 => VK_NUMPAD4,
            VKey::Numpad5 => VK_NUMPAD5,
            VKey::Numpad6 => VK_NUMPAD6,
            VKey::Numpad7 => VK_NUMPAD7,
            VKey::Numpad8 => VK_NUMPAD8,
            VKey::Numpad9 => VK_NUMPAD9,
            VKey::Multiply => VK_MULTIPLY,
            VKey::Add => VK_ADD,
            VKey::Separator => VK_SEPARATOR,
            VKey::Subtract => VK_SUBTRACT,
            VKey::Decimal => VK_DECIMAL,
            VKey::Divide => VK_DIVIDE,
            VKey::F1 => VK_F1,
            VKey::F2 => VK_F2,
            VKey::F3 => VK_F3,
            VKey::F4 => VK_F4,
            VKey::F5 => VK_F5,
            VKey::F6 => VK_F6,
            VKey::F7 => VK_F7,
            VKey::F8 => VK_F8,
            VKey::F9 => VK_F9,
            VKey::F10 => VK_F10,
            VKey::F11 => VK_F11,
            VKey::F12 => VK_F12,
            VKey::F13 => VK_F13,
            VKey::F14 => VK_F14,
            VKey::F15 => VK_F15,
            VKey::F16 => VK_F16,
            VKey::F17 => VK_F17,
            VKey::F18 => VK_F18,
            VKey::F19 => VK_F19,
            VKey::F20 => VK_F20,
            VKey::F21 => VK_F21,
            VKey::F22 => VK_F22,
            VKey::F23 => VK_F23,
            VKey::F24 => VK_F24,
            VKey::Numlock => VK_NUMLOCK,
            VKey::Scroll => VK_SCROLL,
            VKey::LShift => VK_LSHIFT,
            VKey::RShift => VK_RSHIFT,
            VKey::LControl => VK_LCONTROL,
            VKey::RControl => VK_RCONTROL,
            VKey::LMenu => VK_LMENU,
            VKey::RMenu => VK_RMENU,
            VKey::BrowserBack => VK_BROWSER_BACK,
            VKey::BrowserForward => VK_BROWSER_FORWARD,
            VKey::BrowserRefresh => VK_BROWSER_REFRESH,
            VKey::BrowserStop => VK_BROWSER_STOP,
            VKey::BrowserSearch => VK_BROWSER_SEARCH,
            VKey::BrowserFavorites => VK_BROWSER_FAVORITES,
            VKey::BrowserHome => VK_BROWSER_HOME,
            VKey::VolumeMute => VK_VOLUME_MUTE,
            VKey::VolumeDown => VK_VOLUME_DOWN,
            VKey::VolumeUp => VK_VOLUME_UP,
            VKey::MediaNextTrack => VK_MEDIA_NEXT_TRACK,
            VKey::MediaPrevTrack => VK_MEDIA_PREV_TRACK,
            VKey::MediaStop => VK_MEDIA_STOP,
            VKey::MediaPlayPause => VK_MEDIA_PLAY_PAUSE,
            VKey::LaunchMail => VK_LAUNCH_MAIL,
            VKey::LaunchMediaSelect => VK_LAUNCH_MEDIA_SELECT,
            VKey::LaunchApp1 => VK_LAUNCH_APP1,
            VKey::LaunchApp2 => VK_LAUNCH_APP2,
            VKey::Oem1 => VK_OEM_1,
            VKey::OemPlus => VK_OEM_PLUS,
            VKey::OemComma => VK_OEM_COMMA,
            VKey::OemMinus => VK_OEM_MINUS,
            VKey::OemPeriod => VK_OEM_PERIOD,
            VKey::Oem2 => VK_OEM_2,
            VKey::Oem3 => VK_OEM_3,
            VKey::Oem4 => VK_OEM_4,
            VKey::Oem5 => VK_OEM_5,
            VKey::Oem6 => VK_OEM_6,
            VKey::Oem7 => VK_OEM_7,
            VKey::Oem8 => VK_OEM_8,
            VKey::Oem102 => VK_OEM_102,
            VKey::Attn => VK_ATTN,
            VKey::Crsel => VK_CRSEL,
            VKey::Exsel => VK_EXSEL,
            VKey::Play => VK_PLAY,
            VKey::Zoom => VK_ZOOM,
            VKey::Pa1 => VK_PA1,
            VKey::OemClear => VK_OEM_CLEAR,

            VKey::Vk0 => b'0' as i32,
            VKey::Vk1 => b'1' as i32,
            VKey::Vk2 => b'2' as i32,
            VKey::Vk3 => b'3' as i32,
            VKey::Vk4 => b'4' as i32,
            VKey::Vk5 => b'5' as i32,
            VKey::Vk6 => b'6' as i32,
            VKey::Vk7 => b'7' as i32,
            VKey::Vk8 => b'8' as i32,
            VKey::Vk9 => b'9' as i32,
            VKey::A => b'A' as i32,
            VKey::B => b'B' as i32,
            VKey::C => b'C' as i32,
            VKey::D => b'D' as i32,
            VKey::E => b'E' as i32,
            VKey::F => b'F' as i32,
            VKey::G => b'G' as i32,
            VKey::H => b'H' as i32,
            VKey::I => b'I' as i32,
            VKey::J => b'J' as i32,
            VKey::K => b'K' as i32,
            VKey::L => b'L' as i32,
            VKey::M => b'M' as i32,
            VKey::N => b'N' as i32,
            VKey::O => b'O' as i32,
            VKey::P => b'P' as i32,
            VKey::Q => b'Q' as i32,
            VKey::R => b'R' as i32,
            VKey::S => b'S' as i32,
            VKey::T => b'T' as i32,
            VKey::U => b'U' as i32,
            VKey::V => b'V' as i32,
            VKey::W => b'W' as i32,
            VKey::X => b'X' as i32,
            VKey::Y => b'Y' as i32,
            VKey::Z => b'Z' as i32,

            VKey::CustomKeyCode(vk) => *vk,
        }
    }

    /// Take in a string and try to guess what Virtual Key (VK) it is meant to represent.
    /// Returns the VK code as i32 on success (a key representation was recognized).
    ///
    /// - For single character strings the ASCII code is used as VK, this is used to represent
    /// alphanumeric keys
    /// - Many of the most common VKs are represented by their constant name. For example
    /// VK_SPACE => spacebar key
    /// - Any other key can be represented by directly specifying the VK keycode value in 2
    /// digit hex representation. For example 0x08 == VK_TAB (Tab key)
    ///
    /// See https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
    pub fn from_keyname(val: &str) -> Result<Self, HkError> {
        let val = val.to_ascii_uppercase();

        // Single letter => Simply use the ASCII Code
        if val.as_bytes().len() == 1 {
            let val = val.as_bytes()[0];
            if (b'A'..=b'Z').contains(&val) || (b'0'..=b'9').contains(&val) {
                return Ok(Self::CustomKeyCode(val as i32));
            }
        }

        // 1 byte hex code => Use the raw keycode value
        if val.len() >= 3 && val.len() <= 6 && val.starts_with("0x") || val.starts_with("0X") {
            if let Ok(val) = i32::from_str_radix(&val[2..], 16) {
                return Ok(Self::CustomKeyCode(val));
            } else {
                return Err(HkError::InvalidKey(val));
            }
        }

        // Try to match against hardcoded VK_* Key specifiers
        Ok(match val.trim_start_matches("VK_") {
            "BACK" => Self::Back,
            "TAB" => Self::Tab,
            "CLEAR" => Self::Clear,
            "RETURN" => Self::Return,
            "SHIFT" => Self::Shift,
            "CONTROL" => Self::Control,
            "MENU" => Self::Menu,
            "PAUSE" => Self::Pause,
            "CAPITAL" => Self::Capital,
            "ESCAPE" => Self::Escape,
            "SPACE" => Self::Space,
            "PRIOR" => Self::Prior,
            "NEXT" => Self::Next,
            "END" => Self::End,
            "HOME" => Self::Home,
            "LEFT" => Self::Left,
            "UP" => Self::Up,
            "RIGHT" => Self::Right,
            "DOWN" => Self::Down,
            "SELECT" => Self::Select,
            "PRINT" => Self::Print,
            "EXECUTE" => Self::Execute,
            "SNAPSHOT" => Self::Snapshot,
            "INSERT" => Self::Insert,
            "DELETE" => Self::Delete,
            "HELP" => Self::Help,
            "LWIN" => Self::LWin,
            "RWIN" => Self::RWin,
            "APPS" => Self::Apps,
            "SLEEP" => Self::Sleep,
            "NUMPAD0" => Self::Numpad0,
            "NUMPAD1" => Self::Numpad1,
            "NUMPAD2" => Self::Numpad2,
            "NUMPAD3" => Self::Numpad3,
            "NUMPAD4" => Self::Numpad4,
            "NUMPAD5" => Self::Numpad5,
            "NUMPAD6" => Self::Numpad6,
            "NUMPAD7" => Self::Numpad7,
            "NUMPAD8" => Self::Numpad8,
            "NUMPAD9" => Self::Numpad9,
            "MULTIPLY" => Self::Multiply,
            "ADD" => Self::Add,
            "SEPARATOR" => Self::Separator,
            "SUBTRACT" => Self::Subtract,
            "DECIMAL" => Self::Decimal,
            "DIVIDE" => Self::Divide,
            "F1" => Self::F1,
            "F2" => Self::F2,
            "F3" => Self::F3,
            "F4" => Self::F4,
            "F5" => Self::F5,
            "F6" => Self::F6,
            "F7" => Self::F7,
            "F8" => Self::F8,
            "F9" => Self::F9,
            "F10" => Self::F10,
            "F11" => Self::F11,
            "F12" => Self::F12,
            "F13" => Self::F13,
            "F14" => Self::F14,
            "F15" => Self::F15,
            "F16" => Self::F16,
            "F17" => Self::F17,
            "F18" => Self::F18,
            "F19" => Self::F19,
            "F20" => Self::F20,
            "F21" => Self::F21,
            "F22" => Self::F22,
            "F23" => Self::F23,
            "F24" => Self::F24,
            "NUMLOCK" => Self::Numlock,
            "SCROLL" => Self::Scroll,
            "LSHIFT" => Self::LShift,
            "RSHIFT" => Self::RShift,
            "LCONTROL" => Self::LControl,
            "RCONTROL" => Self::RControl,
            "LMENU" => Self::LMenu,
            "RMENU" => Self::RMenu,
            "BROWSER_BACK" => Self::BrowserBack,
            "BROWSER_FORWARD" => Self::BrowserForward,
            "BROWSER_REFRESH" => Self::BrowserRefresh,
            "BROWSER_STOP" => Self::BrowserStop,
            "BROWSER_SEARCH" => Self::BrowserSearch,
            "BROWSER_FAVORITES" => Self::BrowserFavorites,
            "BROWSER_HOME" => Self::BrowserHome,
            "VOLUME_MUTE" => Self::VolumeMute,
            "VOLUME_DOWN" => Self::VolumeDown,
            "VOLUME_UP" => Self::VolumeUp,
            "MEDIA_NEXT_TRACK" => Self::MediaNextTrack,
            "MEDIA_PREV_TRACK" => Self::MediaPrevTrack,
            "MEDIA_STOP" => Self::MediaStop,
            "MEDIA_PLAY_PAUSE" => Self::MediaPlayPause,
            "LAUNCH_MAIL" => Self::LaunchMail,
            "LAUNCH_MEDIA_SELECT" => Self::LaunchMediaSelect,
            "LAUNCH_APP1" => Self::LaunchApp1,
            "LAUNCH_APP2" => Self::LaunchApp2,
            "OEM_1" => Self::Oem1,
            "OEM_PLUS" => Self::OemPlus,
            "OEM_COMMA" => Self::OemComma,
            "OEM_MINUS" => Self::OemMinus,
            "OEM_PERIOD" => Self::OemPeriod,
            "OEM_2" => Self::Oem2,
            "OEM_3" => Self::Oem3,
            "OEM_4" => Self::Oem4,
            "OEM_5" => Self::Oem5,
            "OEM_6" => Self::Oem6,
            "OEM_7" => Self::Oem7,
            "OEM_8" => Self::Oem8,
            "OEM_102" => Self::Oem102,
            "ATTN" => Self::Attn,
            "CRSEL" => Self::Crsel,
            "EXSEL" => Self::Exsel,
            "PLAY" => Self::Play,
            "ZOOM" => Self::Zoom,
            "PA1" => Self::Pa1,
            "OEM_CLEAR" => Self::OemClear,

            _ => return Err(HkError::InvalidKey(val)),
        })
    }
}

impl Display for VKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use winapi::um::winuser::*;

        let code = self.to_vk_code();

        if code >= 'A' as i32 && code <= 'Z' as i32 {
            return write!(f, "{}", code as u8 as char);
        }

        if code >= '0' as i32 && code <= '9' as i32 {
            return write!(f, "{}", code as u8 as char);
        }

        let val = match code {
            VK_BACK => "VK_BACK",
            VK_TAB => "VK_TAB",
            VK_CLEAR => "VK_CLEAR",
            VK_RETURN => "VK_RETURN",
            VK_SHIFT => "VK_SHIFT",
            VK_CONTROL => "VK_CONTROL",
            VK_MENU => "VK_MENU",
            VK_PAUSE => "VK_PAUSE",
            VK_CAPITAL => "VK_CAPITAL",
            VK_ESCAPE => "VK_ESCAPE",
            VK_SPACE => "VK_SPACE",
            VK_PRIOR => "VK_PRIOR",
            VK_NEXT => "VK_NEXT",
            VK_END => "VK_END",
            VK_HOME => "VK_HOME",
            VK_LEFT => "VK_LEFT",
            VK_UP => "VK_UP",
            VK_RIGHT => "VK_RIGHT",
            VK_DOWN => "VK_DOWN",
            VK_SELECT => "VK_SELECT",
            VK_PRINT => "VK_PRINT",
            VK_EXECUTE => "VK_EXECUTE",
            VK_SNAPSHOT => "VK_SNAPSHOT",
            VK_INSERT => "VK_INSERT",
            VK_DELETE => "VK_DELETE",
            VK_HELP => "VK_HELP",
            VK_LWIN => "VK_LWIN",
            VK_RWIN => "VK_RWIN",
            VK_APPS => "VK_APPS",
            VK_SLEEP => "VK_SLEEP",
            VK_NUMPAD0 => "VK_NUMPAD0",
            VK_NUMPAD1 => "VK_NUMPAD1",
            VK_NUMPAD2 => "VK_NUMPAD2",
            VK_NUMPAD3 => "VK_NUMPAD3",
            VK_NUMPAD4 => "VK_NUMPAD4",
            VK_NUMPAD5 => "VK_NUMPAD5",
            VK_NUMPAD6 => "VK_NUMPAD6",
            VK_NUMPAD7 => "VK_NUMPAD7",
            VK_NUMPAD8 => "VK_NUMPAD8",
            VK_NUMPAD9 => "VK_NUMPAD9",
            VK_MULTIPLY => "VK_MULTIPLY",
            VK_ADD => "VK_ADD",
            VK_SEPARATOR => "VK_SEPARATOR",
            VK_SUBTRACT => "VK_SUBTRACT",
            VK_DECIMAL => "VK_DECIMAL",
            VK_DIVIDE => "VK_DIVIDE",
            VK_F1 => "VK_F1",
            VK_F2 => "VK_F2",
            VK_F3 => "VK_F3",
            VK_F4 => "VK_F4",
            VK_F5 => "VK_F5",
            VK_F6 => "VK_F6",
            VK_F7 => "VK_F7",
            VK_F8 => "VK_F8",
            VK_F9 => "VK_F9",
            VK_F10 => "VK_F10",
            VK_F11 => "VK_F11",
            VK_F12 => "VK_F12",
            VK_F13 => "VK_F13",
            VK_F14 => "VK_F14",
            VK_F15 => "VK_F15",
            VK_F16 => "VK_F16",
            VK_F17 => "VK_F17",
            VK_F18 => "VK_F18",
            VK_F19 => "VK_F19",
            VK_F20 => "VK_F20",
            VK_F21 => "VK_F21",
            VK_F22 => "VK_F22",
            VK_F23 => "VK_F23",
            VK_F24 => "VK_F24",
            VK_NUMLOCK => "VK_NUMLOCK",
            VK_SCROLL => "VK_SCROLL",
            VK_LSHIFT => "VK_LSHIFT",
            VK_RSHIFT => "VK_RSHIFT",
            VK_LCONTROL => "VK_LCONTROL",
            VK_RCONTROL => "VK_RCONTROL",
            VK_LMENU => "VK_LMENU",
            VK_RMENU => "VK_RMENU",
            VK_BROWSER_BACK => "VK_BROWSER_BACK",
            VK_BROWSER_FORWARD => "VK_BROWSER_FORWARD",
            VK_BROWSER_REFRESH => "VK_BROWSER_REFRESH",
            VK_BROWSER_STOP => "VK_BROWSER_STOP",
            VK_BROWSER_SEARCH => "VK_BROWSER_SEARCH",
            VK_BROWSER_FAVORITES => "VK_BROWSER_FAVORITES",
            VK_BROWSER_HOME => "VK_BROWSER_HOME",
            VK_VOLUME_MUTE => "VK_VOLUME_MUTE",
            VK_VOLUME_DOWN => "VK_VOLUME_DOWN",
            VK_VOLUME_UP => "VK_VOLUME_UP",
            VK_MEDIA_NEXT_TRACK => "VK_MEDIA_NEXT_TRACK",
            VK_MEDIA_PREV_TRACK => "VK_MEDIA_PREV_TRACK",
            VK_MEDIA_STOP => "VK_MEDIA_STOP",
            VK_MEDIA_PLAY_PAUSE => "VK_MEDIA_PLAY_PAUSE",
            VK_LAUNCH_MAIL => "VK_LAUNCH_MAIL",
            VK_LAUNCH_MEDIA_SELECT => "VK_LAUNCH_MEDIA_SELECT",
            VK_LAUNCH_APP1 => "VK_LAUNCH_APP1",
            VK_LAUNCH_APP2 => "VK_LAUNCH_APP2",
            VK_OEM_1 => "VK_OEM_1",
            VK_OEM_PLUS => "VK_OEM_PLUS",
            VK_OEM_COMMA => "VK_OEM_COMMA",
            VK_OEM_MINUS => "VK_OEM_MINUS",
            VK_OEM_PERIOD => "VK_OEM_PERIOD",
            VK_OEM_2 => "VK_OEM_2",
            VK_OEM_3 => "VK_OEM_3",
            VK_OEM_4 => "VK_OEM_4",
            VK_OEM_5 => "VK_OEM_5",
            VK_OEM_6 => "VK_OEM_6",
            VK_OEM_7 => "VK_OEM_7",
            VK_OEM_8 => "VK_OEM_8",
            VK_OEM_102 => "VK_OEM_102",
            VK_ATTN => "VK_ATTN",
            VK_CRSEL => "VK_CRSEL",
            VK_EXSEL => "VK_EXSEL",
            VK_PLAY => "VK_PLAY",
            VK_ZOOM => "VK_ZOOM",
            VK_PA1 => "VK_PA1",
            VK_OEM_CLEAR => "VK_OEM_CLEAR",
            vk_code => return write!(f, "0x{:x}", vk_code),
        };

        write!(f, "{}", val)
    }
}

impl PartialEq<VKey> for VKey {
    fn eq(&self, other: &VKey) -> bool {
        self.to_vk_code() == other.to_vk_code()
    }
}

impl Eq for VKey {}

impl Hash for VKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.to_vk_code().hash(state);
    }
}

impl TryInto<ModKey> for VKey {
    type Error = ();

    fn try_into(self) -> Result<ModKey, Self::Error> {
        use winapi::um::winuser::*;

        Ok(match self.to_vk_code() {
            VK_MENU | VK_LMENU | VK_RMENU => ModKey::Alt,
            VK_CONTROL | VK_LCONTROL | VK_RCONTROL => ModKey::Ctrl,
            VK_SHIFT | VK_LSHIFT | VK_RSHIFT => ModKey::Shift,
            VK_LWIN | VK_RWIN => ModKey::Win,
            _ => return Err(()),
        })
    }
}
