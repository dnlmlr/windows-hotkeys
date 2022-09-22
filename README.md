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

### Controlling the event loop
Currently the biggest limitation is the reconfiguration (registration / unregistration) of hotkeys 
while at the same time having the event loop running. The `HotkeyManager::event_loop` function will
run and block indefinitely and calling `HotkeyManager::poll_event` will also block until a hotkey 
is triggered. So using a custom loop allows to break the loop (and reconfigure hotkeys), but only 
when a hotkey is actually triggered.

Technically registering and unregistering hotkeys with the winapi can be done fully independent of 
the actual event loop, so this could be done while still having the event loop running. This would
require synchronization around the `HotkeyManager`, since the modifications and event loop would be
running on different threads.

Simply stopping the loop is another problem. Using the same synchronization approach as discussed 
previously the actual loop can be stopped. The issue here is that the `HotkeyManager::poll_event` 
blocks at least until a `WM_HOTKEY` window event is received. A possible solution could be to 
additionally wait for `WM_USER` messages and send one of those in order to stop the loop.

### Threading
Due to limitations in the windows API, hotkey events can only be received and unregistered on the 
same thread as they were initially registered. This means that a `HotkeyManager` instance can't be 
moved between threads. 

Using `windows-hotkeys` with multithreading is still possible, but the `HotkeyManager` must be 
created and used on the same thread.
