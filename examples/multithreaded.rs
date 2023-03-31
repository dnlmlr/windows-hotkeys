use std::{
    sync::{Arc, Mutex},
    thread::spawn,
};

use windows_hotkeys::{
    keys::{ModKey, VKey},
    threadsafe::HotkeyManager, HotkeyManagerImpl,
};

fn main() {
    // Create a HKM1 on main thread
    let mut hkm = HotkeyManager::new();

    println!("Created HKM1 on thread {:?}", std::thread::current().id());

    // Create a HKM2 on main thread
    let mut hkm2 = HotkeyManager::new();

    println!("Created HKM2 on thread {:?}", std::thread::current().id());

    // Get the interrupt handle for HKM2
    let hkm2_interrupt = hkm2.interrupt_handle();

    // Register hotkey for HKM2 on main thread
    hkm2.register(VKey::C, &[ModKey::Alt], move || {
        println!("Hotkey ALT + C was pressed");
    })
    .unwrap();

    println!(
        "Registered Keys for HKM2 on thread {:?}",
        std::thread::current().id()
    );

    let hkm2 = Arc::new(Mutex::new(hkm2));

    // Register hotkey for HKM1 on main thread
    hkm.register(VKey::A, &[ModKey::Alt], move || {
        println!("Hotkey ALT + A was pressed");

        let hkm2 = hkm2.clone();
        // Spawn a new thread and move the HKM2 reference into it. This does not work normally
        spawn(move || {
            // Since the HKM can't be listenin 2x at once, it needs to be locked
            if let Ok(hkm2) = hkm2.try_lock() {
                println!(
                    "Start listening for hotkeys with HKM2 on thread {:?}",
                    std::thread::current().id()
                );

                // Start listening on the new thread with HKM2
                hkm2.event_loop();

                println!("HotkeyManager2 ended");
            } else {
                println!("HotkeyManager2 is already active");
            }
        });
    })
    .unwrap();

    println!(
        "Registered Keys for HKM1 on thread {:?}",
        std::thread::current().id()
    );

    // Register hotkey for HKM1 on main thread
    hkm.register(VKey::B, &[ModKey::Alt], move || {
        println!("Hotkey ALT + B was pressed");

        // Quit the HKM2 EventLoop if it is running
        hkm2_interrupt.interrupt();
    })
    .unwrap();

    println!(
        "Registered Keys for HKM1 on thread {:?}",
        std::thread::current().id()
    );

    spawn(move || {
        println!(
            "Started listening for hotkeys with HKM1 on thread {:?}",
            std::thread::current().id()
        );

        // Start EventLoop for HKM2 on a different thread than the one that was used for registering
        // hotkeys. This doesn't work normally
        hkm.event_loop();
    })
    .join()
    .unwrap();
}
