pub mod context;
mod switch;
pub mod scheduler;

extern crate alloc;

use context::Context;
use x86_64::structures::paging::OffsetPageTable;
use crate::println;

static STACK_SIZE: usize = 4096 * 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Unused,
    Embryo,
    Sleeping,
    Runnable,
    Running,
    Zombie,
}

/// Process Control Block
#[derive(Debug, Clone, Copy)]
pub struct Process {
    pub pid: usize,             // Process ID
    pub state: ProcessState,    // プロセスの状態
    pub context: Context,       // プロセスのコンテキスト
    pub kstack: u64,            // このプロセス用のカーネルスタック
}

impl Process {
    pub fn new() -> Self {
        Process {
            pid: 0,
            state: ProcessState::Unused,
            context: Context::new(),
            kstack: 0,
        }
    }
}

pub const NPROC: usize = 64;
use spin::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref PROCESS_TABLE: Mutex<[Process; NPROC]> = {
        Mutex::new([Process::new(); NPROC])
    };
}

/// カーネルスレッド作成
pub fn create_kernel_thread(entry: fn() -> !, pid: usize) {
    // スタックを作成
    let stack = unsafe {
        let layout = alloc::alloc::Layout::from_size_align(STACK_SIZE, 16).unwrap();
        alloc::alloc::alloc(layout)
    };
    let stack_top = stack as u64 + STACK_SIZE as u64;

    let mut table = PROCESS_TABLE.lock();
    table[pid].pid = pid;
    table[pid].state = ProcessState::Runnable;
    table[pid].kstack = stack_top;

    // コンテキストを初期化する
    table[pid].context.rsp = stack_top;
    table[pid].context.rip = entry as u64;
    table[pid].context.rflags = 0x200;  // IF (Interrupt Flag) を有効化

    println!("Created kernel thread {} at {:p}", pid, entry as *const ());
}
