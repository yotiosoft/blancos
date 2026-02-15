pub mod context;
mod switch;
mod scheduler;

use context::Context;
use alloc::string::String;
use x86_64::structures::paging::OffsetPageTable;

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
    pub pid: u32,               // Process ID
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
