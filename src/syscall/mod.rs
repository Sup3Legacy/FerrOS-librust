use alloc::string::String;
use alloc::vec::Vec;

use x86_64::VirtAddr;

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
pub extern "C" fn syscall(
    nb: usize,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
) -> usize {
    let res;
    unsafe {
        asm!(
            "int 80h",
            in("rax") nb, in("rdi") arg0, in("rsi") arg1, in("rdx") arg2, in("r10") arg3, in("r8") arg4,
            lateout("rax") res
        )
    };
    res
}

pub unsafe fn read(file_descriptor: usize, buffer: *mut u8, count: usize) -> usize {
    let response;
    asm!(
        "int 80h",
        in("rax") 0, in("rdi") file_descriptor, in("rsi") buffer, in("rdx") count, lateout("rax") response
    );
    response
}

pub unsafe fn write(file_descriptor: usize, buffer: *const u8, count: usize) -> usize {
    let response;
    asm!(
        "int 80h",
        in("rax") 1, in("rdi") file_descriptor, in("rsi") buffer, in("rdx") count, lateout("rax") response
    );
    response
}

/// Maybe we can pass a whole String for the path
pub unsafe fn open(path: String, flags: crate::io::OpenFlags) -> usize {
    syscall(
        2,
        VirtAddr::from_ptr(path.as_ptr()).as_u64() as usize,
        path.len(),
        flags.bits(),
        0,
        0,
    )
}

pub unsafe fn close(file_descriptor: usize) -> usize {
    syscall(3, file_descriptor, 0, 0, 0, 0)
}

pub unsafe fn dup2(fd1: usize, fd2: usize) -> usize {
    syscall(4, fd1, fd2, 0, 0, 0)
}

pub unsafe fn fork() -> usize {
    syscall(5, 0, 0, 0, 0, 0)
}

pub unsafe fn exec(name: &String, args: &Vec<String>) -> usize {
    syscall(
        6,
        VirtAddr::from_ptr(name.as_ptr()).as_u64() as usize,
        name.len(),
        (args as *const Vec<String>) as usize,
        0,
        0,
    )
}

pub unsafe fn exit(code: usize) -> usize {
    syscall(7, code, 0, 0, 0, 0)
}

pub unsafe fn sleep() {
    syscall(8, 0, 0, 0, 0, 0);
}

pub unsafe fn shutdown(code: usize) -> usize {
    syscall(9, code, 0, 0, 0, 0)
}

pub unsafe fn get_puid() -> usize {
    syscall(10, 0, 0, 0, 0, 0)
}

pub unsafe fn set_screen_size(height: usize, width: usize) -> usize {
    syscall(11, height, width, 0, 0, 0)
}

pub unsafe fn set_screen_pos(top: usize, left: usize) -> usize {
    syscall(12, top, left, 0, 0, 0)
}

pub unsafe fn getcwd() -> usize {
    syscall(13, 0, 0, 0, 0, 0)
}

pub unsafe fn chdir() -> usize {
    syscall(14, 0, 0, 0, 0, 0)
}

pub unsafe fn mkdir() -> usize {
    syscall(15, 0, 0, 0, 0, 0)
}

pub unsafe fn rmdir() -> usize {
    syscall(16, 0, 0, 0, 0, 0)
}

pub unsafe fn get_layer() -> usize {
    syscall(17, 0, 0, 0, 0, 0)
}

pub unsafe fn set_layer() -> usize {
    syscall(18, 0, 0, 0, 0, 0) as usize
}

pub unsafe fn set_focus() -> usize {
    syscall(19, 0, 0, 0, 0, 0) as usize
}

pub unsafe fn debug(v1: usize, v2: usize) {
    syscall(20, v1, v2, 0, 0, 0);
}

pub unsafe fn memrequest(number: usize) -> usize {
    syscall(21, number, 0, 0, 0, 0)
}

pub unsafe fn listen() -> (usize, usize) {
    let res1;
    let res2;
    asm!(
        "int 80h",
        in("rax") 22,
        in("rdi") 0,
        lateout("rax") res1,
        lateout("rdi") res2,
    );
    (res1, res2)
}

pub unsafe fn listen_proc(id: usize) -> (usize, usize) {
    let res1;
    let res2;
    asm!(
        "int 80h",
        in("rax") 22,
        in("rdi") id,
        lateout("rax") res1,
        lateout("rdi") res2,
    );
    (res1, res2)
}

pub unsafe fn await_end(id: usize) -> usize {
    loop {
        let (r1, r2) = listen_proc(id);
        if id == r1 {
            return r2;
        } else {
            sleep()
        }
    }
}

#[repr(C)]
pub enum OpenFlags {
    OCREAT,
    ODIRECTORY,
    OEXCEL,
    OPATH,
}
