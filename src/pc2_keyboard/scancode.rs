

#[derive(Debug, Clone, Copy)]
pub enum Key {
    O,
    Q,
    T,

    Enter,
    Backspace,
}


pub unsafe fn decode_keys (key: u8) -> Option<Key> {
    match key { 
        0x10 => Some(Key::Q),
        0x14 => Some(Key::T),
        0x18 => Some(Key::O),
        0x1C => Some(Key::Enter),
        0x0E => Some(Key::Backspace),
        _ => None,
    }
}