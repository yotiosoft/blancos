use super::context::Context;
use core::{arch::naked_asm, mem::offset_of};

/// コンテキストスイッチ
#[unsafe(naked)]
pub unsafe extern "C" fn switch_context(old: *mut Context, new: *const Context) {
    naked_asm!(
        "mov [rdi + {r15_offset}], r15",
        "mov [rdi + {r14_offset}], r14",
        "mov [rdi + {r13_offset}], r13",
        "mov [rdi + {r12_offset}], r12",
        "mov [rdi + {rbx_offset}], rbx",
        "mov [rdi + {rbp_offset}], rbp",

        "lea rax, [rsp + 8]",
        "mov [rdi + {rsp_offset}], rax",

        "mov rax, [rsp]",
        "mov [rdi + {rip_offset}], rax",

        "mov r15, [rsi + {r15_offset}]",
        "mov r14, [rsi + {r14_offset}]",
        "mov r13, [rsi + {r13_offset}]",
        "mov r12, [rsi + {r12_offset}]",
        "mov rbx, [rsi + {rbx_offset}]",
        "mov rbp, [rsi + {rbp_offset}]",
        "mov rsp, [rsi + {rsp_offset}]",

        "jmp [rsi + {rip_offset}]",

        r15_offset = const offset_of!(Context, r15),
        r14_offset = const offset_of!(Context, r14),
        r13_offset = const offset_of!(Context, r13),
        r12_offset = const offset_of!(Context, r12),
        rbx_offset = const offset_of!(Context, rbx),
        rbp_offset = const offset_of!(Context, rbp),
        rsp_offset = const offset_of!(Context, rsp),
        rip_offset = const offset_of!(Context, rip),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn test_context_size() {
        assert_eq!(core::mem::size_of::<Context>(), 64);
    }
}
