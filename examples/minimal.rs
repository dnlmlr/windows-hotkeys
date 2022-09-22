use windows_hotkeys::{
    keys::{ModKey, VKey},
    HotkeyManager,
};

fn main() {
    // Create a HotkeyManager
    let mut hkm = HotkeyManager::new();

    // Register a system-wide hotkey with the main key `A` and the modifier key `ALT`
    hkm.register(VKey::A, &[ModKey::Alt], || {
        println!("Hotkey ALT + A was pressed");
    })
    .unwrap();

    // Register a system-wide hotkey with the main key `B` and multiple modifier keys
    // (`CTRL` + `ALT`)
    hkm.register(VKey::B, &[ModKey::Ctrl, ModKey::Alt], || {
        println!("Hotkey CTRL + ALT + B was pressed");
    })
    .unwrap();

    // Register a system-wide hotkey for `ALT` + `B` with extra keys `Left` + `Right`. This will
    // trigger only if the `Left` + `Right` keys are also pressed during `ALT` + `B`. So just
    // pressing `ALT` + `B` alone won't execute the closure
    hkm.register_extrakeys(VKey::B, &[ModKey::Alt], &[VKey::Left, VKey::Right], || {
        println!("Hotkey ALT + B + Left + Right was pressed");
    })
    .unwrap();

    // Run the event handler in a blocking loop. This will block forever and execute the set
    // callbacks when registered hotkeys are detected. In order to not block, this can also simply
    // be executed in another thread
    hkm.event_loop();
}
