#[cfg(not(target_os = "windows"))]
compile_error!("Only supported on windows");

pub mod error;
pub mod keys;

use std::collections::HashMap;

use winapi::shared::windef::HWND;
use winapi::um::libloaderapi::GetModuleHandleA;
use winapi::um::winuser::{
    self, CreateWindowExA, DestroyWindow, GetAsyncKeyState, GetMessageW, RegisterHotKey,
    UnregisterHotKey, HWND_MESSAGE, MSG, WM_HOTKEY, WS_DISABLED, WS_EX_NOACTIVATE,
};

use crate::{error::HkError, keys::*};

/// Identifier of a registered hotkey. This is returned when registering a hotkey and can be used
/// to unregister it again.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct HotkeyId(i32);

/// HotkeyCallback contains the callback function and a list of extra_keys.
///
struct HotkeyCallback<T> {
    /// Callback function to execute  when the hotkey matches
    callback: Box<dyn Fn() -> T + 'static>,
    /// List of additional VKs that are required to be pressed to execute
    /// the callback
    extra_keys: Vec<VKey>,
}

/// Register and manage hotkeys with windows, as well as the callbacks.
///
pub struct HotkeyManager<T> {
    hwnd: HwndDropper,
    id_offset: i32,
    handlers: HashMap<HotkeyId, HotkeyCallback<T>>,
}

impl<T> Default for HotkeyManager<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> HotkeyManager<T> {
    /// Create a new HotkeyManager instance.
    ///
    /// The hotkey ids that are registered by this will start at offset 0,
    /// so creating a second instance with `new` will result in failing
    /// hotkey registration due to the ids being in use already. To register
    /// hotkeys with multiple instances see `new_with_id_offset`. Keep in
    /// mind though that only one instance can be listing for hotkeys anyways.
    ///
    pub fn new() -> HotkeyManager<T> {
        // Try to create a hidden window to receive the hotkey events for the HotkeyManager.
        // If the window creation fails, HWND 0 (null) is used which registers hotkeys to the thread
        // message queue and gets messages from all thread associated windows
        let hwnd = create_hidden_window().unwrap_or(HwndDropper(std::ptr::null_mut()));
        HotkeyManager {
            hwnd,
            id_offset: 0,
            handlers: HashMap::new(),
        }
    }

    /// Register a hotkey with callback and require additional extra keys to be pressed.
    ///
    /// This will try to register the hotkey&modifiers with windows and add the callback with
    /// the extra keys to the handlers.
    ///
    /// # Arguments
    ///
    /// * `key` - The main hotkey. For example VK_ENTER for CTRL + ALT + ENTER combination.
    ///
    /// * `key_modifiers` - The modifier keys as combined flags. This can be MOD_ALT, MOD_CONTROL,
    /// MOD_SHIFT or a bitwise combination of those. The modifier keys are the keys that need to
    /// be pressed in addition to the main hotkey in order for the hotkey event to fire.
    ///
    /// * `extra_keys` - A list of additional VKs that also need to be pressed for the hotkey callback
    /// to be executed. This is enforced after the windows hotkey event is fired but before executing
    /// the callback.
    ///
    /// * `callback` - A callback function or closure that will be executed when the hotkey is pressed
    ///
    /// # Windows API Functions used
    /// - https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerhotkey
    ///
    pub fn register_extrakeys(
        &mut self,
        key: VKey,
        key_modifiers: &[ModKey],
        extra_keys: &[VKey],
        callback: impl Fn() -> T + 'static,
    ) -> Result<HotkeyId, HkError> {
        let register_id = HotkeyId(self.id_offset);
        self.id_offset += 1;

        // Try to register the hotkey combination with windows
        let reg_ok = unsafe {
            RegisterHotKey(
                self.hwnd.0,
                register_id.0,
                ModKey::combine(key_modifiers) | winuser::MOD_NOREPEAT as u32,
                key.to_vk_code() as u32,
            )
        };

        if reg_ok == 0 {
            Err(HkError::RegistrationFailed)
        } else {
            // Add the HotkeyCallback to the handlers when the hotkey was registered
            self.handlers.insert(
                register_id,
                HotkeyCallback {
                    callback: Box::new(callback),
                    extra_keys: extra_keys.to_owned(),
                },
            );

            Ok(register_id)
        }
    }

    /// Same as `register_extrakeys` but without extra keys.
    ///
    /// # Windows API Functions used
    /// - https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerhotkey
    ///
    pub fn register(
        &mut self,
        key: VKey,
        key_modifiers: &[ModKey],
        callback: impl Fn() -> T + 'static,
    ) -> Result<HotkeyId, HkError> {
        self.register_extrakeys(key, key_modifiers, &[], callback)
    }

    /// Unregister a hotkey
    ///
    /// # Windows API Functions used
    /// - https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-unregisterhotkey
    ///
    pub fn unregister(&mut self, id: HotkeyId) -> Result<(), HkError> {
        let ok = unsafe { UnregisterHotKey(self.hwnd.0, id.0) };

        match ok {
            0 => Err(HkError::UnregistrationFailed),
            _ => {
                self.handlers.remove(&id);
                Ok(())
            }
        }
    }

    /// Unregister all registered hotkeys. This will be called automatically when dropping the
    /// HotkeyManager instance
    ///
    /// # Windows API Functions used
    /// - https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-unregisterhotkey
    ///
    pub fn unregister_all(&mut self) -> Result<(), HkError> {
        let ids: Vec<_> = self.handlers.keys().copied().collect();
        for id in ids {
            self.unregister(id)?;
        }

        Ok(())
    }

    /// Poll a hotkey event, execute the callback if all keys match and return the callback
    /// result.
    ///
    /// This will block until a hotkey is pressed and therefore not consume any cpu power.
    ///
    /// ## Windows API Functions used
    /// - https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagew
    ///
    pub fn poll_event(&mut self) -> T {
        loop {
            let mut msg = std::mem::MaybeUninit::<MSG>::uninit();

            // Block and read a message from the message queue. Filtered by only WM_HOTKEY messages
            let ok = unsafe { GetMessageW(msg.as_mut_ptr(), self.hwnd.0, WM_HOTKEY, WM_HOTKEY) };

            if ok != 0 {
                let msg = unsafe { msg.assume_init() };

                if WM_HOTKEY == msg.message {
                    let hk_id = HotkeyId(msg.wParam as i32);

                    // Get the callback for the received ID
                    if let Some(handler) = self.handlers.get(&hk_id) {
                        // Check if all extra keys are pressed
                        if !handler
                            .extra_keys
                            .iter()
                            .any(|vk| !get_global_keystate(*vk))
                        {
                            return (handler.callback)();
                        }
                    }
                }
            }
        }
    }

    pub fn event_loop(&mut self) {
        loop {
            self.poll_event();
        }
    }
}

impl<T> Drop for HotkeyManager<T> {
    fn drop(&mut self) {
        let _ = self.unregister_all();
    }
}

/// Get the global keystate for a given Virtual Key.
///
/// Return true if the key is pressed, false otherwise.
///
/// ## Windows API Functions used
/// - https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getasynckeystate
///
pub fn get_global_keystate(vk: VKey) -> bool {
    // Most significant bit represents key state (1 => pressed, 0 => not pressed)
    let key_state = unsafe { GetAsyncKeyState(vk.to_vk_code()) };
    // Get most significant bit only
    let key_state = key_state as u32 >> 31;

    key_state == 1
}

/// Wrapper around a HWND windows pointer that destroys the window on drop
struct HwndDropper(HWND);

impl Drop for HwndDropper {
    fn drop(&mut self) {
        if !self.0.is_null() {
            let _ = unsafe { DestroyWindow(self.0) };
        }
    }
}

/// Try to create a hidden "message-only" window
fn create_hidden_window() -> Result<HwndDropper, ()> {
    let hwnd = unsafe {
        // Get the current module handle
        let hinstance = GetModuleHandleA(std::ptr::null_mut());
        CreateWindowExA(
            WS_EX_NOACTIVATE,
            // A class that is not more for windows, but this shouldn't matter since it is hidden
            b"Static\0".as_ptr() as *const i8,
            b"\0".as_ptr() as *const i8,
            WS_DISABLED,
            0,
            0,
            0,
            0,
            HWND_MESSAGE,
            std::ptr::null_mut(),
            hinstance,
            std::ptr::null_mut(),
        )
    };
    if hwnd.is_null() {
        Err(())
    } else {
        Ok(HwndDropper(hwnd))
    }
}
