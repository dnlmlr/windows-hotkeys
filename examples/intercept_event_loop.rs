use std::{
    thread::{sleep, spawn},
    time::Duration,
};

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

    // Grab an interrupt handle that can be used to interrupt / stop the event loop from any thread
    let handle = hkm.interrupt_handle();

    // Create a second thread that will stop the event loop after 5 seconds
    spawn(move || {
        sleep(Duration::from_secs(5));
        handle.interrupt();
    });

    // Run the event handler in a blocking loop. This will block until interrupted and execute the 
    // set callbacks when registered hotkeys are detected
    hkm.event_loop();

    println!("Event Loop interrupted");
}
