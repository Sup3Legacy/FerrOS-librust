#![no_std]
#![no_main]
#![cfg_attr(test, no_main)]
#![feature(alloc_error_handler)]
#![feature(custom_test_frameworks)]
#![feature(core_intrinsics)]
#![feature(gen_future)]
#![feature(const_mut_refs)]
#![feature(naked_functions)]
#![feature(abi_x86_interrupt)]
#![feature(asm)]
#![feature(intra_doc_pointers)]

pub mod allocator;
pub mod env;
pub mod io;
pub mod screen;
pub mod syscall;

/// To be able to use all `alloc` structures
extern crate alloc;

/// To directly use `core` and `alloc` in user-space programs
pub use core;
use core::panic::PanicInfo;


#[panic_handler]
pub fn panic(_: &PanicInfo) -> ! {
    unsafe {
        syscall::syscall(20, 420, 0, 0, 0, 0);
        asm!("push 1", "ret");
    }
    loop {}
}

#[inline(never)]
pub extern "C" fn syscall(nb: u64, arg0: u64, arg1: u64, arg2: u64) -> usize {
    let res;
    unsafe {
        asm!(
            "mov rax, {}", 
            "mov rdi, {}",
            "mov rsi, {}",
            "mov rdx, {}",
            "int 80h",
            "mov {}, rax", 
            in(reg) nb, in(reg) arg0, in(reg) arg1, in(reg) arg2, out(reg) res)
    };
    res
}