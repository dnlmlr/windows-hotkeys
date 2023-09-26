#[cfg(not(target_os = "windows"))]
compile_error!("Only supported on windows");

use std::collections::HashMap;
use std::marker::PhantomData;

use winapi::shared::windef::HWND;
use winapi::um::libloaderapi::GetModuleHandleA;
use winapi::um::winuser::{
    CreateWindowExA, DestroyWindow, GetMessageW, RegisterHotKey, UnregisterHotKey, HWND_MESSAGE,
    MSG, WM_HOTKEY, WM_NULL, WS_DISABLED, WS_EX_NOACTIVATE,
};

use crate::{
    error::HkError, get_global_keystate, keys::*, HotkeyCallback, HotkeyId, HotkeyManagerImpl,
    InterruptHandle,
};

/// The HotkeyManager is used to register, unregister and await hotkeys with their callback
/// functions.
///
/// # Note
/// Due to limitations with the windows event system the HotkeyManager can't be moved to other
/// threads.
///
pub struct HotkeyManager<T> {
    /// Handle to the hidden window that is used to receive the hotkey events
    hwnd: HwndDropper,
    id_offset: i32,
    handlers: HashMap<HotkeyId, HotkeyCallback<T>>,
    /// Automatically set the `ModKey::NoRepeat` when registering hotkeys. Defaults to `true`
    no_repeat: bool,

    /// Make sure that `HotkeyManager` is not Send / Sync. This prevents it from being moved
    /// between threads, which would prevent hotkey-events from being received.
    ///
    /// Being stuck on the same thread is an inherent limitation of the windows event system.
    _unimpl_send_sync: PhantomData<*const u8>,
}

impl<T> Default for HotkeyManager<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> HotkeyManager<T> {
    /// Enable or disable the automatically applied `ModKey::NoRepeat` modifier. By default, this
    /// option is set to `true` which causes all hotkey registration calls to add the `NoRepeat`
    /// modifier, thereby disabling automatic retriggers of hotkeys when holding down the keys.
    ///
    /// When this option is disabled, the `ModKey::NoRepeat` can still be manually added while
    /// registering hotkeys.
    ///
    /// Note: Setting this flag doesn't change previously registered hotkeys. It only applies to
    /// registrations performed after calling this function.
    pub fn set_no_repeat(&mut self, no_repeat: bool) {
        self.no_repeat = no_repeat;
    }
}

impl<T> HotkeyManagerImpl<T> for HotkeyManager<T> {
    /// Create a new HotkeyManager instance. This instance can't be moved to other threads due to
    /// limitations in the windows events system.
    ///
    fn new() -> HotkeyManager<T> {
        // Try to create a hidden window to receive the hotkey events for the HotkeyManager.
        // If the window creation fails, HWND 0 (null) is used which registers hotkeys to the thread
        // message queue and gets messages from all thread associated windows
        let hwnd = create_hidden_window().unwrap_or(HwndDropper(std::ptr::null_mut()));
        HotkeyManager {
            hwnd,
            id_offset: 0,
            handlers: HashMap::new(),
            no_repeat: true,
            _unimpl_send_sync: PhantomData,
        }
    }

    fn register_extrakeys(
        &mut self,
        key: VKey,
        key_modifiers: &[ModKey],
        extra_keys: &[VKey],
        callback: impl Fn() -> T + Send + 'static,
    ) -> Result<HotkeyId, HkError> {
        let register_id = HotkeyId(self.id_offset);
        self.id_offset += 1;

        let mut modifiers = ModKey::combine(key_modifiers);
        if self.no_repeat {
            modifiers |= ModKey::NoRepeat.to_mod_code();
        }

        // Try to register the hotkey combination with windows
        let reg_ok = unsafe {
            RegisterHotKey(
                self.hwnd.0,
                register_id.0,
                modifiers,
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

    fn register(
        &mut self,
        key: VKey,
        key_modifiers: &[ModKey],
        callback: impl Fn() -> T + Send + 'static,
    ) -> Result<HotkeyId, HkError> {
        self.register_extrakeys(key, key_modifiers, &[], callback)
    }

    fn unregister(&mut self, id: HotkeyId) -> Result<(), HkError> {
        let ok = unsafe { UnregisterHotKey(self.hwnd.0, id.0) };

        match ok {
            0 => Err(HkError::UnregistrationFailed),
            _ => {
                self.handlers.remove(&id);
                Ok(())
            }
        }
    }

    fn unregister_all(&mut self) -> Result<(), HkError> {
        let ids: Vec<_> = self.handlers.keys().copied().collect();
        for id in ids {
            self.unregister(id)?;
        }

        Ok(())
    }

    fn handle_hotkey(&self) -> Option<T> {
        loop {
            let mut msg = std::mem::MaybeUninit::<MSG>::uninit();

            // Block and read a message from the message queue. Filtered to receive messages from
            // WM_NULL to WM_HOTKEY
            let ok = unsafe { GetMessageW(msg.as_mut_ptr(), self.hwnd.0, WM_NULL, WM_HOTKEY) };

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
                            return Some((handler.callback)());
                        }
                    }
                } else if WM_NULL == msg.message {
                    return None;
                }
            }
        }
    }

    fn event_loop(&self) {
        while self.handle_hotkey().is_some() {}
    }

    fn interrupt_handle(&self) -> InterruptHandle {
        InterruptHandle(self.hwnd.0)
    }
}

impl<T> Drop for HotkeyManager<T> {
    fn drop(&mut self) {
        let _ = self.unregister_all();
    }
}

/// Wrapper around a HWND windows pointer that destroys the window on drop
///
struct HwndDropper(HWND);

impl Drop for HwndDropper {
    fn drop(&mut self) {
        if !self.0.is_null() {
            let _ = unsafe { DestroyWindow(self.0) };
        }
    }
}

/// Try to create a hidden "message-only" window
///
fn create_hidden_window() -> Result<HwndDropper, ()> {
    let hwnd = unsafe {
        // Get the current module handle
        let hinstance = GetModuleHandleA(std::ptr::null_mut());
        CreateWindowExA(
            WS_EX_NOACTIVATE,
            // The "Static" class is not intended for windows, but this shouldn't matter since the
            // window is hidden anyways
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
