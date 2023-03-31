use clipboard_win::{get_clipboard_string, set_clipboard_string};
use windows_hotkeys::{
    keys::{ModKey, VKey},
    HotkeyManager, HotkeyManagerImpl,
};

fn main() {
    // Prepare hotkey CTRL + ALT + K
    let mod_keys = [ModKey::Ctrl, ModKey::Alt];
    let main_key = VKey::K;

    // Create the manager
    let mut hkm = HotkeyManager::new();

    hkm.register(main_key, &mod_keys, || {
        // Get the current clipboard text
        let orig = match get_clipboard_string() {
            Ok(it) => it,
            Err(_) => {
                eprintln!("Failed to get clipboard contents");
                return;
            }
        };

        let mut sponge = String::new();

        // Rebuild the clipboard text, but use alternating upper and lower case letters
        orig.char_indices().for_each(|(i, c)| {
            if i % 2 == 0 {
                sponge.push(c.to_ascii_lowercase());
            } else {
                sponge.push(c.to_ascii_uppercase());
            }
        });

        // Set the clipboard text to the spongebob case string
        if let Err(e) = set_clipboard_string(&sponge) {
            eprintln!("Failed to set clipboard contents '{e}'");
        }
    })
    .unwrap();

    hkm.event_loop();
}
