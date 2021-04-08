use alloc::string::String;

// callee-saved : rax, rdi, rsi, rdx, rcx, r8, r9, r10, r11
// syscall : rax, rdi, rsi, rdx, r10, r8, r9
// param : rdi, rsi, rdx, rcx, r8, r9
pub unsafe extern "C" fn syscall_old(
    number: u64,
    arg0: u64,
    arg1: u64,
    arg2: u64,
    arg3: u64,
    arg4: u64,
) -> u64 {
    let x;
    asm!(
        "mov rax, rdi",
        "mov rdi, rsi",
        "mov rsi, rdx",
        "mov rdx, r10",
        "mov r10, r8",
        "mov r8, r9",
        "int 0x80",
        // Result MUST be put into rax before returning from the context_switch
        "mov {0}, rax",
        out(reg) x
    );
    x
}

#[inline(never)]
pub extern "C" fn syscall(nb: u64, arg0: u64, arg1: u64, arg2: u64, arg3 : u64, arg4 : u64) -> usize {
    let res;
    unsafe {
        asm!(
            "mov rax, {}", 
            "mov rdi, {}",
            "mov rsi, {}",
            "mov rdx, {}",
            "mov r10, {}",
            "mov r8, {}",
            "int 80h",
            "mov {}, rax", 
            in(reg) nb, in(reg) arg0, in(reg) arg1, in(reg) arg2, in(reg) arg3, in(reg) arg4, out(reg) res)
    };
    res
}

pub unsafe fn read(file_descriptor: usize, buffer: *mut usize, count: usize) -> usize {
    todo!()
}

pub unsafe fn write(file_descriptor: usize, buffer: *mut usize, count: usize) -> usize {
    todo!()
}

/// Maybe we can pass a whole String for the path
pub unsafe fn open(path: *mut usize, length: usize, flags: u64) -> usize {
    syscall(2, path as u64, length as u64, flags, 0, 0) as usize
}

pub unsafe fn close(file_descriptor: usize) -> usize {
    syscall(3, file_descriptor as u64, 0, 0, 0, 0) as usize
}

pub unsafe fn memrequest(number : usize) -> usize {
    syscall(21, number as u64, 0, 0, 0, 0) as usize
}

#[repr(C)]
pub enum OpenFlags {
    OCREAT,
    ODIRECTORY,
    OEXCEL,
    OPATH,
}
