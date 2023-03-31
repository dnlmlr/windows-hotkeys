# Windows Hotkeys
[![Crates.io](https://img.shields.io/crates/v/windows-hotkeys?style=flat-square)](https://crates.io/crates/windows-hotkeys)
[![Crates.io](https://img.shields.io/crates/l/windows-hotkeys?style=flat-square)](https://crates.io/crates/windows-hotkeys)
[![Docs.rs](https://img.shields.io/docsrs/windows-hotkeys?style=flat-square)](https://docs.rs/windows-hotkeys/latest/windows_hotkeys)
> An opinionated, lightweight crate to handle system-wide hotkeys on windows

The `windows-hotkeys` crate abstracts and handles all interactions with the winapi, including 
registering hotkeys and handling the events and providing threadsafe access. A hotkey manager 
instance is used to register key combinations together with easy callbacks.

## Features
- Usable over multiple threads, bypassing the WinAPI same-thread requirements for the hotkey API
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
use windows_hotkeys::{HotkeyManager, HotkeyManagerImpl};

fn main() {
    let mut hkm = HotkeyManager::new();

    hkm.register(VKey::A, &[ModKey::Alt], || {
        println!("Hotkey ALT + A was pressed");
    })
    .unwrap();

    hkm.event_loop();
}
```

## Threading
Due to limitations in the windows API, hotkey events can only be received and unregistered on the 
same thread as they were initially registered. This means that a normal 
`singlethreaded::HotkeyManager` instance can't be moved between threads. 

Using the `windows-hotkeys` `singlethreaded` API with multithreading is still possible, but the 
`singlethreaded::HotkeyManager` must be created and used on the same thread.

However by the default enabled `threadsafe` feature add the `threadsafe::HotkeyManager` 
implementation which solved this issue and provides the default `HotkeyManager` implementation.

This is done by launching a background thread when a `threadsafe::HotkeyManager` is instantiated 
that is listening for commands on a channel receiver. There is one command for each of the 
`HotkeyManager` functions and upon receiving a command, the matching function is called from that 
same thread. The `threadsafe::HotkeyManager` is nothing more than a stub that controls the actual 
backend thread via these channel commands. This way all of the hotkey functions are executed on the
same thread, no matter from where the stub functions are called.
