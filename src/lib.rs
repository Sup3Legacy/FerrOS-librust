#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(core_intrinsics)]
#![feature(gen_future)]
#![feature(const_mut_refs)]

pub mod syscall;
pub mod screen;
pub mod io;
pub mod allocator;
