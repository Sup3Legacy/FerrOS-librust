//! Base mod for everything environment-related.

use alloc::string::String;
use alloc::vec::Vec;

/// When passed the address of the page where the arguments are, this function retrieve them
pub fn retrieve_arguments(args_number: u64, address: u64) -> Vec<String> {
    let arg_buffer = unsafe { *(address as *const [u8; 0x1000]) };
    let mut args = Vec::new();
    let mut index = 0;
    let mut number = 0;
    loop {
        let mut new_string = String::new();
        for i in index..0x1000 {
            if arg_buffer[i] == 0 {
                index += i;
                break;
            }
            new_string.push(arg_buffer[i] as char);
        }
        if new_string.len() > 0 {
            args.push(new_string);
        }
        number += 1;
        if index == 0x1000 || number == args_number {
            break;
        }
    }
    args
}
