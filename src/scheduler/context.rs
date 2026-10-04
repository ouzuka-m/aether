#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Context {
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub rbx: u64,
    pub rbp: u64,
    pub rsp: u64,
}

impl core::fmt::Debug for Context {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Context")
            .field("r15", &format_args!("{:#018x}", self.r15))
            .field("r14", &format_args!("{:#018x}", self.r14))
            .field("r13", &format_args!("{:#018x}", self.r13))
            .field("r12", &format_args!("{:#018x}", self.r12))
            .field("rbx", &format_args!("{:#018x}", self.rbx))
            .field("rbp", &format_args!("{:#018x}", self.rbp))
            .field("rsp", &format_args!("{:#018x}", self.rsp))
            .finish()
    }
}
#[unsafe(naked)]
pub unsafe extern "C" fn launch(ctx: *const Context) {
    core::arch::naked_asm!(
        "
        mov r15, [rdi + 0]
        mov r14, [rdi + 8]
        mov r13, [rdi + 16]
        mov r12, [rdi + 24]
        mov rbx, [rdi + 32]
        mov rbp, [rdi + 40]
        mov rsp, [rdi + 48]

        ret
        ",
    );
}

#[unsafe(naked)]
pub unsafe extern "C" fn switch(old: *mut Context, new: *const Context) {
    core::arch::naked_asm!(
        "
        mov [rdi + 0], r15
        mov [rdi + 8], r14
        mov [rdi + 16], r13
        mov [rdi + 24], r12
        mov [rdi + 32], rbx
        mov [rdi + 40], rbp
        mov [rdi + 48], rsp

        mov r15, [rsi + 0]
        mov r14, [rsi + 8]
        mov r13, [rsi + 16]
        mov r12, [rsi + 24]
        mov rbx, [rsi + 32]
        mov rbp, [rsi + 40]
        mov rsp, [rsi + 48]

        ret
        ",
    );
}
