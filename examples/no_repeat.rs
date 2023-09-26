use windows_hotkeys::{
    keys::{ModKey, VKey},
    HotkeyManagerImpl,
};

fn main() {
    // Create a HotkeyManager.
    // By default, the hotkey registration will add the NoRepeat modifier. This causes the callback
    // to only be triggered once, when the combination is held down.
    let mut hkm = windows_hotkeys::threadsafe::HotkeyManager::new();

    // Disable automatically applying the NoRepeat modifier. After this call, all registrations
    // will trigger repeatedly when the hotkey is held down. This behavior can be manually changed
    // for each registration by adding the `ModKey::NoRepeat` modifier.
    hkm.set_no_repeat(false);

    // Register a system-wide hotkey with the main key `A` and the modifier key `ALT`
    //
    // NOTE: This will trigger multiple times when the combination is held down
    hkm.register(VKey::A, &[ModKey::Alt], || {
        println!("Hotkey ALT + A was pressed");
    })
    .unwrap();

    // Register a system-wide hotkey with the main key `B` and multiple modifier keys
    // (`CTRL` + `ALT`)
    //
    // NOTE: This will only be triggered once and not repeatedly when being held down, since the
    //       NoRepeat modifier is added manually.
    hkm.register(
        VKey::B,
        &[ModKey::Ctrl, ModKey::Alt, ModKey::NoRepeat],
        || {
            println!("Hotkey CTRL + ALT + B was pressed");
        },
    )
    .unwrap();

    // Register a system-wide hotkey for `ALT` + `B` with extra keys `Left` + `Right`. This will
    // trigger only if the `Left` + `Right` keys are also pressed during `ALT` + `B`. So just
    // pressing `ALT` + `B` alone won't execute the closure
    //
    // NOTE: This will trigger multiple times when the combination is held down
    hkm.register_extrakeys(VKey::B, &[ModKey::Alt], &[VKey::Left, VKey::Right], || {
        println!("Hotkey ALT + B + Left + Right was pressed");
    })
    .unwrap();

    // Run the event handler in a blocking loop. This will block forever and execute the set
    // callbacks when registered hotkeys are detected
    hkm.event_loop();
}
