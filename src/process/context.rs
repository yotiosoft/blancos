/// コンテキスト構造体
/// コンテキストスイッチ時の保存/復元に利用
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Context {
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub rbx: u64,
    pub rbp: u64,
    pub rsp: u64,
    pub rip: u64,
    pub rflags: u64,
}

impl Context {
    pub fn new() -> Self {
        Context {
            r15: 0,
            r14: 0,
            r13: 0,
            r12: 0,
            rbx: 0,
            rbp: 0,
            rsp: 0,
            rip: 0,
            rflags: 0x0,
        }
    }
}
