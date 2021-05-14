use super::syscall;

use alloc::string::String;
use alloc::vec::Vec;
use core::mem;

pub const STD_IN: usize = 0;
pub const STD_OUT: usize = 1;
pub const STD_ERR: usize = 2;

pub fn push_sound(fd: u64, tone: u64, length: u64, begin: u64) {
    let sound_buffer: [u8; 24] = unsafe { mem::transmute([tone, length, begin]) };
    unsafe {
        syscall::write(fd as usize, sound_buffer.as_ptr(), 24);
    }
}

pub fn read_input(fd: usize, length: usize) -> Vec<u8> {
    let mut buffer = [0_u8; 512];
    let got = unsafe { syscall::read(fd, &mut buffer as *mut u8, length) };
    let mut res = Vec::new();
    for i in 0..got {
        if buffer[i] == 0 {
            break;
        }
        res.push(buffer[i]);
    }
    res
}

pub fn read_to_string(fd: usize, length: usize) -> String {
    let mut buffer = [0_u8; 512];
    unsafe {
        syscall::debug(fd, 43);
    }
    let got = unsafe { syscall::read(fd, &mut buffer as *mut u8, length) };
    let mut res = String::new();
    for i in 0..got {
        if buffer[i] == 0 {
            break;
        }
        res.push(buffer[i] as char);
    }
    res
}

pub fn print_buffer(buffer: &[u8], size: usize) {
    let mut t: [u8; 256] = [0; 256];

    for c in 0..size {
        //syscall(20, index as u64, c as u64, 0);
        t[c] = buffer[c];
    }
    unsafe {
        syscall::write(1, &t as *const u8, size);
    }
}

pub fn print(a: &String) {
    let mut t: [u8; 128] = [0; 128];
    let mut index = 0_usize;

    for c in a.bytes() {
        t[index] = c;
        index += 1;
        if index == 128 {
            t[index - 1] = 0; // We put a guard
            break;
        }
    }
    unsafe {
        syscall::write(1, &t as *const u8, index);
    }
}
