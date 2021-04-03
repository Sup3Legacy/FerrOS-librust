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
pub mod memory;

/// To be able to use all `alloc` structures
extern crate alloc;

/// To directly use `core` and `alloc` in user-space programs
pub use core;
use core::panic::PanicInfo;


#[panic_handler]
pub fn panic(_: &PanicInfo) -> ! {
    unsafe {
        syscall::syscall(20, 420, 0, 0, 0, 0);
        asm!("push 0", "ret");
    }
    loop {}
}
