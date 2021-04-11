//! Provides some functions usefull to test some aspects of the kernel or the user-space

/// This function raises (at least if the corresponding page isn't allcoated. It should not be)
/// a pagefault. The user-program should crash but the kernel must recover.
pub fn pagefault() {
    unsafe{*(0x223 as *mut u8) = 5;}
}