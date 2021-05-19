#![no_std]
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
#![feature(const_fn_trait_bound)]

pub mod allocator;
pub mod env;
pub mod interfaces;
pub mod io;
pub mod screen;
pub mod syscall;
pub mod terminal;
pub mod testing;

/// To be able to use all `alloc` structures
extern crate alloc;

/// To directly use `core` and `alloc` in user-space programs
pub use core;
use core::panic::PanicInfo;

#[macro_use]
extern crate num_derive;

#[panic_handler]
pub fn panic(_: &PanicInfo) -> ! {
    unsafe {
        syscall::syscall(20, 420, 0, 0, 0, 0);
        syscall::exit(42);
    }
    loop {}
}
