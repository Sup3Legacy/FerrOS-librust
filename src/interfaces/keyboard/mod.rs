use lazy_static::lazy_static;
use spin::Mutex;

use alloc::vec::Vec;
use alloc::string::String;

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
            layout::Effect::Value(layout::KeyEvent::SpecialKey(_v)) => (),
            layout::Effect::Value(layout::KeyEvent::CharaterVec(v)) => {
                for elt in v.iter() {
                    characters[index] = *elt;
                    index += 1;
                }
            }
            _ => (),
        };
    }
    index
}

pub fn translate(scancodes: Vec<u8>, string: &mut String, end: &mut String) {
    for c in scancodes.iter() {
        let character = KEYBOARD_STATUS.lock().process(*c);
        match character {
            layout::Effect::Nothing => (),
            layout::Effect::Value(layout::KeyEvent::Character(a)) => {
                string.push(a);
            },
            layout::Effect::Value(layout::KeyEvent::CharaterVec(v)) => {
                if v.len() == 4 && v[0] == b'\x1b' && v[1] == b'[' && v[2] == 1_u8 {
                    match v[3] {
                        b'A' => {
                            match string.pop() {
                                Some(c) => {
                                    let mut end2 = String::from(c);
                                    end2.push_str(&end);
                                    end.truncate(0);
                                    end.push_str(&end2);
                                },
                                None => (),
                            }
                        },
                        b'B' => {
                            if end.len() != 0 {
                                let mut first = true;
                                let end2 = String::from(end as &String);
                                end.truncate(0);
                                for c in end2.chars() {
                                    if first {
                                        string.push(c);
                                        first = false
                                    } else {
                                        end.push(c)
                                    }
                                }
                            }
                        },
                        _ => (),
                    }
                } else {
                    for elt in v.iter() {
                        string.push(*elt as char);
                    }
                }
            }
            layout::Effect::Value(layout::KeyEvent::SpecialKey(v)) => {
                if v == 0 {
                    string.pop();
                }
            }
        }
    }
}