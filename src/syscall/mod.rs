use alloc::string::String;

// callee-saved : rax, rdi, rsi, rdx, rcx, r8, r9, r10, r11
// syscall : rax, rdi, rsi, rdx, r10, r8, r9
// param : rdi, rsi, rdx, rcx, r8, r9
#[naked]
pub unsafe extern "C" fn syscall(
    number: u64,
    arg0: u64,
    arg1: u64,
    arg2: u64,
    arg3: u64,
    arg4: u64,
) -> u64 {
    asm!(
        "mov rax, rdi",
        "mov rdi, rsi",
        "mov rsi, rdx",
        "mov rdx, r10",
        "mov r10, r8",
        "mov r8, r9",
        "int 0x80",
        "ret"
    );
    3
}

pub unsafe fn read(file_descriptor: usize, buffer: *mut usize, count: usize) -> usize {
    todo!()
}

pub unsafe fn write(file_descriptor: usize, buffer: *mut usize, count: usize) -> usize {
    todo!()
}

#[repr(C)]
pub enum OpenFlags {
    OCREAT,
    ODIRECTORY,
    OEXCEL,
    OPATH,
}

pub unsafe fn open(path: String, flag: OpenFlags, count: usize) -> usize {
    todo!()
}

pub unsafe fn close(file_descriptor: usize) -> usize {
    todo!()
}
