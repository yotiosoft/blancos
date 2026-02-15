use super::context::Context;
use core::{arch::{global_asm, naked_asm}, mem::offset_of};

unsafe extern "C" {
    pub fn switch_context(old: *mut Context, new: *const Context);
    pub fn save_context(context: *mut Context);
}

// コンテキストスイッチ
global_asm!(
    r#"
.globl switch_context
switch_context:
    # 現在のコンテキストを保存
    mov [rdi + 0], r15
    mov [rdi + 8], r14
    mov [rdi + 16], r13
    mov [rdi + 24], r12
    mov [rdi + 32], rbx
    mov [rdi + 40], rbp
    
    # RSP
    mov rax, rsp
    add rax, 8              # return address をスキップ
    mov [rdi + 48], rax
    
    # RIP
    mov rax, [rsp]
    mov [rdi + 56], rax

    # RFLAGS
    pushfq
    pop rax
    mov [rdi + 64], rax
    
    # 新しいコンテキストを復元
    mov r15, [rsi + 0]
    mov r14, [rsi + 8]
    mov r13, [rsi + 16]
    mov r12, [rsi + 24]
    mov rbx, [rsi + 32]
    mov rbp, [rsi + 40]
    mov rsp, [rsi + 48]

    # RFLAGS
    mov rax, [rsi + 64]
    push rax
    popfq

    # 新しいコンテキストへジャンプする
    mov rax, [rsi + 56]
    ret
"#
);

// コンテキストを保存する
global_asm!(
r#"
.global save_context
save_context:
    # 現在のコンテキストを保存
    mov [rdi + 0], r15
    mov [rdi + 8], r14
    mov [rdi + 16], r13
    mov [rdi + 24], r12
    mov [rdi + 32], rbx
    mov [rdi + 40], rbp

    # RSP
    mov rax, rsp
    add rax, 8              # return address をスキップ
    mov [rdi + 48], rax
    
    # RIP
    mov rax, [rsp]
    mov [rdi + 56], rax

    # RFLAGS
    pushfq
    pop rax
    mov [rdi + 64], rax

    ret
"#
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn test_context_size() {
        assert_eq!(core::mem::size_of::<Context>(), 64);
    }
}
