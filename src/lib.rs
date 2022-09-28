#[cfg(not(target_os = "windows"))]
compile_error!("Only supported on windows");

#[cfg(windows)]
pub mod error;
#[cfg(windows)]
pub mod keys;

#[cfg(windows)]
use std::collections::HashMap;
#[cfg(windows)]
use std::marker::PhantomData;

#[cfg(windows)]
use winapi::shared::windef::HWND;
#[cfg(windows)]
use winapi::um::libloaderapi::GetModuleHandleA;
#[cfg(windows)]
use winapi::um::winuser::{
    self, CreateWindowExA, DestroyWindow, GetAsyncKeyState, GetMessageW, PostMessageW,
    RegisterHotKey, UnregisterHotKey, HWND_MESSAGE, MSG, WM_HOTKEY, WM_NULL, WS_DISABLED,
    WS_EX_NOACTIVATE,
};

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

/// The HotkeyManager is used to register, unregister and await hotkeys with their callback
/// functions.
///
/// # Note
/// Due to limitations with the windows event system the HotkeyManager can't be moved to other
/// threads.
///
#[cfg(windows)]
pub struct HotkeyManager<T> {
    /// Handle to the hidden window that is used to receive the hotkey events
    hwnd: HwndDropper,
    id_offset: i32,
    handlers: HashMap<HotkeyId, HotkeyCallback<T>>,

    /// Make sure that `HotkeyManager` is not Send / Sync. This prevents it from being moved
    /// between threads, which would prevent hotkey-events from being received.
    ///
    /// Being stuck on the same thread is an inherent limitation of the windows event system.
    _unimpl_send_sync: PhantomData<*const u8>,
}

#[cfg(windows)]
impl<T> Default for HotkeyManager<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(windows)]
impl<T> HotkeyManager<T> {
    /// Create a new HotkeyManager instance. This instance can't be moved to other threads due to
    /// limitations in the windows events system.
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
            _unimpl_send_sync: PhantomData,
        }
    }

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
    /// - <https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerhotkey>
    ///
    pub fn register(
        &mut self,
        key: VKey,
        key_modifiers: &[ModKey],
        callback: impl Fn() -> T + 'static,
    ) -> Result<HotkeyId, HkError> {
        self.register_extrakeys(key, key_modifiers, &[], callback)
    }

    /// Unregister a hotkey. This will prevent the hotkey from being triggered in the future.
    ///
    /// # Windows API Functions used
    /// - <https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-unregisterhotkey>
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
    /// HotkeyManager instance.
    ///
    /// # Windows API Functions used
    /// - <https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-unregisterhotkey>
    ///
    pub fn unregister_all(&mut self) -> Result<(), HkError> {
        let ids: Vec<_> = self.handlers.keys().copied().collect();
        for id in ids {
            self.unregister(id)?;
        }

        Ok(())
    }

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
    pub fn handle_hotkey(&self) -> Option<T> {
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

    /// Run the event loop, listening for hotkeys. This will run indefinitely until interrupted and
    /// execute any hotkeys registered before.
    /// 
    pub fn event_loop(&self) {
        while self.handle_hotkey().is_some() {}
    }

    /// Get an `InterruptHandle` for this `HotkeyManager` that can be used to interrupt the event 
    /// loop.
    /// 
    pub fn interrupt_handle(&self) -> InterruptHandle {
        InterruptHandle(self.hwnd.0)
    }
}

#[cfg(windows)]
impl<T> Drop for HotkeyManager<T> {
    fn drop(&mut self) {
        let _ = self.unregister_all();
    }
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

/// Wrapper around a HWND windows pointer that destroys the window on drop
/// 
#[cfg(windows)]
struct HwndDropper(HWND);

#[cfg(windows)]
impl Drop for HwndDropper {
    fn drop(&mut self) {
        if !self.0.is_null() {
            let _ = unsafe { DestroyWindow(self.0) };
        }
    }
}

/// Try to create a hidden "message-only" window
/// 
#[cfg(windows)]
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
