use lazy_static::lazy_static;
use spin::Mutex;

pub mod layout;

lazy_static! {
    pub static ref KEYBOARD_STATUS: Mutex<layout::KeyBoardStatus> =
        Mutex::new(layout::KeyBoardStatus::new(0));
}

pub fn decode_buffer(scancodes: &[u8], characters: &mut [u8], length: usize) -> usize {
    assert_eq!(scancodes.len(), characters.len());
    assert!(length < scancodes.len());
    let mut index = 0;
    for i in 0..length {
        let character = KEYBOARD_STATUS.lock().process(scancodes[i]);
        match character {
            layout::Effect::Nothing => (),
            layout::Effect::Value(layout::KeyEvent::Character(a)) => {
                characters[index] = a as u8;
                index += 1;
            }
            layout::Effect::Value(layout::KeyEvent::SpecialKey(s)) => (),
        };
    }
    index
}
