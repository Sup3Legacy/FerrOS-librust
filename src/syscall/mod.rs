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
pub extern "C" fn syscall(nb: u64, arg0: u64, arg1: u64, arg2: u64, arg3: u64, arg4: u64) -> usize {
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
    let response;
    asm!(
        "int 80h",
        in("rax") 0, in("rsi") buffer, in("rdx") count, lateout("rax") response
    )
    response
}

pub unsafe fn write(file_descriptor: usize, buffer: *mut usize, count: usize) -> usize {
    let response;
    asm!(
        "int 80h",
        in("rax") 1, in("rsi") buffer, in("rdx") count, lateout("rax") response
    )
    response
}

/// Maybe we can pass a whole String for the path
pub unsafe fn open(path: *mut usize, length: usize, flags: u64) -> usize {
    syscall(2, path as u64, length as u64, flags, 0, 0) as usize
}

pub unsafe fn close(file_descriptor: usize) -> usize {
    syscall(3, file_descriptor as u64, 0, 0, 0, 0) as usize
}

pub unsafe fn dup2(fd1: usize, fd2: usize) -> usize {
    syscall(4, fd1, fd2, 0, 0, 0) as usize
}

pub unsafe fn fork() -> usize {
    syscall(5, 0, 0, 0, 0, 0) as usize
}

pub unsafe fn exec(name: String) -> usize {
    syscall(6, name.as_ptr(), 0, 0, 0, 0) as usize
}

pub unsafe fn exit(code: usize) -> usize {
    syscall(7, code as u64, 0, 0, 0, 0) as usize
}

pub unsafe fn sleep() {
    syscall(8, 0, 0, 0, 0, 0);
}

pub unsafe fn shutdown(code: usize) -> usize {
    syscall(9, code as u64, 0, 0, 0, 0) as usize
}

pub unsafe fn get_puid() -> usize {
    syscall(10, 0, 0, 0, 0, 0) as usize
}

pub unsafe fn set_screen_size(height: usize, width: usize) -> usize {
    syscall(11, height as u64, width as u64, 0, 0, 0) as usize
}

pub unsafe fn set_screen_pos(top: usize, left: usize) -> usize {
    syscall(12, top as u64, left as u64, 0, 0, 0) as usize
}

pub unsafe fn getcwd() -> usize {
    syscall(13, 0, 0, 0, 0, 0) as usize
}

pub unsafe fn chdir() -> usize {
    syscall(14, 0, 0, 0, 0, 0) as usize
}

pub unsafe fn mkdir() -> usize {
    syscall(15, 0, 0, 0, 0, 0) as usize
}

pub unsafe fn rmdir() -> usize {
    syscall(16, 0, 0, 0, 0, 0) as usize
}

pub unsafe fn get_layer() -> usize {
    syscall(17, 0, 0, 0, 0, 0) as usize
}

pub unsafe fn set_layer() -> usize {
    syscall(18, 0, 0, 0, 0, 0) as usize
}

pub unsafe fn set_focus() -> usize {
    syscall(19, 0, 0, 0, 0, 0) as usize
}

pub unsafe fn debug(v1: usize, v2: usize) {
    syscall(20, v1 as u64, v2 as u64, 0, 0, 0);
}

pub unsafe fn memrequest(number: usize) -> usize {
    syscall(21, number as u64, 0, 0, 0, 0) as usize
}

pub unsafe fn listen() -> (usize, usize) {
    let res1;
    let res2;
    asm!(
        "int 80h",
        in(rax) 22,
        lateout(rax) res1,
        lateout(rdi) res2,
    );
    (res1, res2)
}

#[repr(C)]
pub enum OpenFlags {
    OCREAT,
    ODIRECTORY,
    OEXCEL,
    OPATH,
}
