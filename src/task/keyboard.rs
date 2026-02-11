use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use core::{ pin::Pin, task::{ Poll, Context } };
use futures_util::stream::Stream;
use crate::println;

static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();

/// キーボードのコードをスキャンする
/// キーボード割り込みハンドラから呼び出される
pub(crate) fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        if let Err(_) = queue.push(scancode) {
            // スキャンコードキューが満杯の場合
            println!("WARNING: scancode queue full; dropping keyboard input");
        }
    }
    else {
        // スキャンコードキューが初期化されていない場合
        println!("WARNING: scancode queue uninitalized");
    }
}
