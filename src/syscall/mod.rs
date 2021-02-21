

// callee-saved : rax, rdi, rsi, rdx, rcx, r8, r9, r10, r11
// syscall : rax, rdi, rsi, rdx, r10, r8, r9
// param : rdi, rsi, rdx, rcx, r8, r9 
#[naked]
pub unsafe fn syscall(number: u64, arg0: u64, arg1: u64, arg2: u64, arg3: u64, arg4: u64) -> u64 {
    asm!(
        "mov rax, rdi",
        "mov rdi, rsi",
        "mov rsi, rdx",
        "mov rdx, r10",
        "mov r10, r8",
        "mov r8, r9",
        "int 0x80",
        "ret"
    )
    loop {}
}