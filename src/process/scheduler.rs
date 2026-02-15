use super::{ Process, ProcessState, PROCESS_TABLE, NPROC };
use super::context::Context;
use super::switch::switch_context;

static mut CURRENT_PID: usize = 0;

/// スケジューラ
pub fn scheduler() {
    let mut table = PROCESS_TABLE.lock();

    let current_pid = unsafe {
        CURRENT_PID
    };

    // ラウンドロビンで次のプロセスを探す
    let mut next_pid = (current_pid + 1) % NPROC;
    loop {
        if table[next_pid].state == ProcessState::Runnable {
            break;
        }
        next_pid = (next_pid + 1) % NPROC;

        // すべて探して Runnable が見つからなければ idle 状態へ
        if next_pid == current_pid {
            return;
        }
    }

    // プロセス状態を更新
    table[current_pid].state = ProcessState::Runnable;
    table[current_pid].state = ProcessState::Running;

    // コンテキストスイッチ
    let old_context = &mut table[current_pid].context as *mut Context;
    let new_context = &table[next_pid].context as *const Context;
    unsafe {
        CURRENT_PID = next_pid;

        switch_context(old_context, new_context);
    }
}
