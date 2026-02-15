use super::{ ProcessState, PROCESS_TABLE, NPROC };
use super::context::Context;
use super::switch::switch_context;
use crate::cpu;
use crate::process::switch::save_context;
use lazy_static::lazy_static;

static mut CURRENT_PID: usize = 0;
pub static mut SCHEDULER_STARTED: bool = false;

lazy_static! {
    static ref CPU: spin::Mutex<cpu::Cpu> = spin::Mutex::new(cpu::Cpu::new(0));
}

pub fn start_scheduler() -> ! {
    unsafe {
        if SCHEDULER_STARTED {
            panic!("Scheduler already started");
        }
        SCHEDULER_STARTED = true;
    }

    let mut cpu = CPU.lock();
    unsafe {
        save_context(&mut cpu.scheduler as *mut Context);
    }
    crate::println!("context: {}", cpu.scheduler.rbp);
    drop(cpu);

    scheduler()
}

/// スケジューラ
pub fn scheduler() -> ! {
    loop {
        let (old_context, new_context) = {
            let mut table = PROCESS_TABLE.lock();

            let current_pid = unsafe {
                CURRENT_PID
            };

            // ラウンドロビンで次のプロセスを探す
            let mut next_pid = (current_pid + 1) % NPROC;
            loop {
                use crate::println;
                if table[next_pid].state == ProcessState::Runnable {
                    println!("NEXT: {}", next_pid);
                    break;
                }

                // すべて探して Runnable が見つからなければ idle 状態へ
                if next_pid == current_pid {
                    println!("hlt");
                    use x86_64::instructions::interrupts::enable_and_hlt;
                    enable_and_hlt();
                }

                next_pid = (next_pid + 1) % NPROC;
            }

            // プロセス状態を更新
            table[next_pid].state = ProcessState::Running;
            if table[current_pid].state == ProcessState::Running {
                table[current_pid].state = ProcessState::Runnable;
            }

            unsafe {
                CURRENT_PID = next_pid;
            }

            // コンテキストスイッチ
            let old_context = {
                let mut cpu = CPU.lock();
                &mut cpu.scheduler as *mut Context
            };
            let new_context = &table[next_pid].context as *const Context;

            (old_context, new_context)
        };
        unsafe {
            x86_64::instructions::interrupts::enable();
            use crate::println; println!("switch");
            switch_context(old_context, new_context);
        }
    }
}

pub fn yield_from_context() {
    x86_64::instructions::interrupts::disable();

    let current_pid = unsafe {
        CURRENT_PID
    };

    let mut table = PROCESS_TABLE.lock();

    if table[current_pid].state != ProcessState::Running {
        x86_64::instructions::interrupts::enable();
        return;
    }

    use crate::println;
    println!("yield");

    // Runnable に変更
    table[current_pid].state = ProcessState::Runnable;

    // スケジューラへコンテキストスイッチ
    let old_context = &mut table[current_pid].context as *mut Context;
    let new_context = {
        let cpu = CPU.lock();
        &cpu.scheduler as *const Context
    };

    drop(table);

    unsafe {
        x86_64::instructions::interrupts::enable();
        switch_context(old_context, new_context);
    }
}
