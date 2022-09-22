# Windows Hotkeys
> An opinionated, lightweight crate to handle system-wide hotkeys on windows

The `windows-hotkeys` crate abstracts and handles all interactions with the winapi, including 
registering hotkeys and handling the events. A hotkey manager instance is used to register key
combinations together with easy callbacks.

## Features
- Full highlevel abstraction over the winapi functions and events
- Easy to use
- Register hotkeys with Key + Modifier
- Register hotkeys with Key + Modifier and require additional keys to be pressed at the same time
- Set rust callback functions or closures that are executed on hotkey trigger
- High level rust abstractions over the Virtual Keys (`VK_*` constants) and Modifier Keys (`MOD_*` constants)
- Create `VKey`s (Virtual Keys) and `ModKey`s (Modifier Keys) from name strings

## How to use

1. Create a `HotkeyManager` instance
2. Register a hokey by specifying a `VKey` and one or more `ModKey`s, together with a callback
3. Run the event loop to react to the incomming hotkey triggers

```rust
use windows_hotkeys::keys::{ModKey, VKey};
use windows_hotkeys::HotkeyManager;

fn main() {
    let mut hkm = HotkeyManager::new();

    hkm.register(VKey::A, &[ModKey::Alt], || {
        println!("Hotkey ALT + A was pressed");
    })
    .unwrap();

    hkm.event_loop();
}
```


