#[cfg(not(target_os = "windows"))]
compile_error!("Only supported on windows");

#[cfg(windows)]
pub mod error;
#[cfg(windows)]
pub mod keys;

#[cfg(windows)]
pub mod singlethreaded;
#[cfg(all(windows, feature = "threadsafe"))]
pub mod threadsafe;

#[cfg(all(windows, feature = "threadsafe"))]
pub use threadsafe::HotkeyManager;

#[cfg(all(windows, not(feature = "threadsafe")))]
pub use singlethreaded::HotkeyManager;

#[cfg(windows)]
use winapi::shared::windef::HWND;
#[cfg(windows)]
use winapi::um::winuser::{GetAsyncKeyState, PostMessageW, WM_NULL};

#[cfg(windows)]
use crate::{error::HkError, keys::*};

/// Identifier of a registered hotkey. This is returned when registering a hotkey and can be used
/// to unregister it later.
///
#[cfg(windows)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct HotkeyId(i32);

/// HotkeyCallback contains the callback function and a list of extra_keys that need to be pressed
/// together with the hotkey when executing the callback.
///
#[cfg(windows)]
struct HotkeyCallback<T> {
    /// Callback function to execute  when the hotkey & extrakeys match
    callback: Box<dyn Fn() -> T + 'static>,
    /// List of additional VKeys that are required to be pressed to execute
    /// the callback
    extra_keys: Vec<VKey>,
}

#[cfg(windows)]
pub trait HotkeyManagerImpl<T> {
    fn new() -> Self;

    /// Register a new hotkey with additional required extra keys.
    ///
    /// This will try to register the specified hotkey with windows, but not actively listen for it.
    /// To listen for hotkeys in order to actually execute the callbacks, the `event_loop` function
    /// must be called.
    ///
    /// # Arguments
    ///
    /// * `key` - The main hotkey. For example `VKey::Return` for the CTRL + ALT + ENTER
    /// combination.
    ///
    /// * `key_modifiers` - The modifier keys that need to be combined with the main key. The
    /// modifier keys are the keys that need to be pressed in addition to the main hotkey in order
    /// for the hotkey event to fire. For example `&[ModKey::Ctrl, ModKey::Alt]` for the
    /// CTRL + ALT + ENTER combination.
    ///
    /// * `extra_keys` - A list of additional VKeys that also need to be pressed for the hotkey
    /// callback to be executed. This is enforced after the windows hotkey event is fired, but
    /// before executing the callback. So these keys need to be pressed before the main hotkey.
    ///
    /// * `callback` - A callback function or closure that will be executed when the hotkey is
    /// triggered. The return type for all callbacks in the same HotkeyManager must be the same.
    ///
    /// # Windows API Functions used
    /// - <https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerhotkey>
    ///
    fn register_extrakeys(
        &mut self,
        key: VKey,
        key_modifiers: &[ModKey],
        extra_keys: &[VKey],
        callback: impl Fn() -> T + Send + 'static,
    ) -> Result<HotkeyId, HkError>;

    /// Same as `register_extrakeys` but without extra keys.
    ///
    /// # Windows API Functions used
    /// - <https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerhotkey>
    ///
    fn register(
        &mut self,
        key: VKey,
        key_modifiers: &[ModKey],
        callback: impl Fn() -> T + Send + 'static,
    ) -> Result<HotkeyId, HkError>;

    /// Unregister a hotkey. This will prevent the hotkey from being triggered in the future.
    ///
    /// # Windows API Functions used
    /// - <https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-unregisterhotkey>
    ///
    fn unregister(&mut self, id: HotkeyId) -> Result<(), HkError>;

    /// Unregister all registered hotkeys. This will be called automatically when dropping the
    /// HotkeyManager instance.
    ///
    /// # Windows API Functions used
    /// - <https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-unregisterhotkey>
    ///
    fn unregister_all(&mut self) -> Result<(), HkError>;

    /// Wait for a single a hotkey event and execute the callback if all keys match. This returns
    /// the callback result if it was not interrupted. The function call will block until a hotkey
    /// is triggered or it is interrupted.
    ///
    /// If the event is interrupted, `None` is returned, otherwise `Some` is returned with the
    /// return value of the executed callback function.
    ///
    /// ## Windows API Functions used
    /// - <https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagew>
    ///
    fn handle_hotkey(&self) -> Option<T>;

    /// Run the event loop, listening for hotkeys. This will run indefinitely until interrupted and
    /// execute any hotkeys registered before.
    ///
    fn event_loop(&self);

    /// Get an `InterruptHandle` for this `HotkeyManager` that can be used to interrupt the event
    /// loop.
    ///
    fn interrupt_handle(&self) -> InterruptHandle;
}

/// The `InterruptHandle` can be used to interrupt the event loop of the originating `HotkeyManager`.
/// This handle can be used from any thread and can be used multiple times.
///
/// # Note
/// This handle will technically stay valid even after the `HotkeyManager` is dropped, but it will
/// simply not do anything.
///
#[cfg(windows)]
pub struct InterruptHandle(HWND);

#[cfg(windows)]
unsafe impl Sync for InterruptHandle {}

#[cfg(windows)]
unsafe impl Send for InterruptHandle {}

#[cfg(windows)]
impl InterruptHandle {
    /// Interrupt the evet loop of the associated `HotkeyManager`.
    ///
    pub fn interrupt(&self) {
        unsafe {
            PostMessageW(self.0, WM_NULL, 0, 0);
        }
    }
}

/// Get the global keystate for a given Virtual Key.
///
/// Return true if the key is pressed, false otherwise.
///
/// ## Windows API Functions used
/// - <https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getasynckeystate>
///
#[cfg(windows)]
pub fn get_global_keystate(vk: VKey) -> bool {
    // Most significant bit represents key state (1 => pressed, 0 => not pressed)
    let key_state = unsafe { GetAsyncKeyState(vk.to_vk_code()) };
    // Get most significant bit only
    let key_state = key_state as u32 >> 31;

    key_state == 1
}
