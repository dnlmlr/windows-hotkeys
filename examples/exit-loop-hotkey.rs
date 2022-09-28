use windows_hotkeys::{
    keys::{ModKey, VKey},
    HotkeyManager,
};

/// A simple control flow enum that can either continue the event handler or stop it
enum ControlFlow {
    /// Continue the handler
    Continue,
    /// Exit (stop) the handler
    Exit,
}

fn main() {
    // The HotkeyManager is generic over the return type of the callback functions. So if the
    // callbacks return data, it is available in the event loop and can be used to determin further
    // actions
    let mut hkm = HotkeyManager::new();

    // A hotkey for CTRL + ALT + A, that will just keep on running and not break the loop
    hkm.register(VKey::A, &[ModKey::Ctrl, ModKey::Alt], || {
        println!("Pressed CTRL + ALT + A");

        // Set the control flow to keep running
        ControlFlow::Continue
    })
    .unwrap();

    // A hotkey for CTRL + ALT + C, that will break and stop the loop
    hkm.register(VKey::C, &[ModKey::Ctrl, ModKey::Alt], || {
        println!("Pressed CTRL + ALT + C");
        println!("Breaking the loop");

        // Set the control flow to exit
        ControlFlow::Exit
    })
    .unwrap();

    loop {
        // Handle one hotkey event. This will block until a hotkey event is triggered and return
        // the return value of the callback
        let control_flow = hkm.poll_event();

        // Since the callbacks return a `ControlFlow` variant, check if the loop should exit
        match control_flow {
            Some(ControlFlow::Exit) | None => break,
            _ => (),
        }
    }

    println!("Loop exited");
}
