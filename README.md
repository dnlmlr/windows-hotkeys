# Windows Hotkeys
[![Crates.io](https://img.shields.io/crates/v/windows-hotkeys?style=flat-square)](https://crates.io/crates/windows-hotkeys)
[![Crates.io](https://img.shields.io/crates/l/windows-hotkeys?style=flat-square)](https://crates.io/crates/windows-hotkeys)
[![Docs.rs](https://img.shields.io/docsrs/windows-hotkeys?style=flat-square)](https://img.shields.io/docsrs/windows-hotkeys)
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
- High level rust abstractions over the Virtual Keys (`VK_*` constants) and Modifier Keys 
  (`MOD_*` constants)
- Create `VKey`s (Virtual Keys) and `ModKey`s (Modifier Keys) from key name strings

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

## Current limitations

### Threading
Due to limitations in the windows API, hotkey events can only be received and unregistered on the 
same thread as they were initially registered. This means that a `HotkeyManager` instance can't be 
moved between threads. 

Using `windows-hotkeys` with multithreading is still possible, but the `HotkeyManager` must be 
created and used on the same thread.

A possible solution to this limitation might be to have each `HotkeyManager` run it's own thread in 
the backgroud that is used to register, unregister and listen for hotkeys. This might be implemented
in the future and would provide a more ergonomic way to use the `HotkeyManager` in general.
