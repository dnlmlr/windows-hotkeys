use windows_hotkeys::{
    keys::{ModKey, VKey},
    HotkeyManager,
};

fn main() {
    // Create a HotkeyManager
    let mut hkm = HotkeyManager::<()>::new();

    // Create VKey from the enum variant (recommended if possible)
    let vk_b1 = VKey::B;
    let vk_up1 = VKey::Up;

    // Create VKey from the case insensitive key char (only for 'a'-'z' and '0'-'9')
    let vk_b2 = VKey::from_char('b').unwrap();
    let vk_b3 = VKey::from_char('B').unwrap();

    // Create VKey from key name string. This works with "A"-"Z", "0"-"9", the constant names
    // "VK_*" and hex codes prefixed with "0x*"
    // Useful when reading hotkeys in text form through config files or user input
    let vk_b4 = VKey::from_str("b").unwrap();
    let vk_b5 = VKey::from_str("B").unwrap();
    let vk_b6 = VKey::from_str("0x42").unwrap();
    let vk_up2 = VKey::from_str("VK_UP").unwrap();

    // Create VKey directly from the virtual keycode
    // See: https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
    let vk_b7 = VKey::CustomKeyCode(0x42);

    // All of the methods are comparable with equal and usable for hashing
    assert_eq!(vk_b1, vk_b2);
    assert_eq!(vk_b1, vk_b3);
    assert_eq!(vk_b1, vk_b4);
    assert_eq!(vk_b1, vk_b5);
    assert_eq!(vk_b1, vk_b6);
    assert_eq!(vk_b1, vk_b7);

    assert_eq!(vk_up1, vk_up2);

    // **Alert**
    // Matching the variants does not always work, since a key can be represented through the enum
    // variant, or with a `CustomKeyCode`
    match vk_b7 {
        VKey::B => println!(
            "CustomKeyCode(0x42) matches against B (this will not show up, because it is not true)"
        ),
        VKey::A => println!("Just checking if the key matches A"),
        _ => println!("CustomKeyCode(0x42) does not match against B"),
    }

    // Instead use if chains if this is really needed for now, at least until a better solution is
    // implemented
    if vk_b7 == VKey::B {
        println!("CustomKeyCode(0x42) is equal to B");
    } else if vk_b7 == VKey::A {
        println!("Just checking if the key is equal to A");
    } else {
        println!(
            "CustomKeyCode(0x42) is not equal to B \
            (this will not show up, because it works fine when using if)"
        );
    }

    // Create ModKey from the enum variant
    let mod_alt1 = ModKey::Alt;
    // Create ModKey from key name string
    let mod_alt2 = ModKey::from_str("ALT").unwrap();

    // With modkeys, there is no `CustomKeyCode`, so they can be safely matched and compared
    assert_eq!(mod_alt1, mod_alt2);

    hkm.register_extrakeys(vk_b1, &[mod_alt1], &[vk_up1], || {
        println!("Hotkey ALT + B + UP pressed");
    })
    .unwrap();

    hkm.event_loop();
}
