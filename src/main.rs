#![no_std]      // std ライブラリを使わない
#![no_main]     // main 関数を使わない

use core::panic::PanicInfo;

/// パニックハンドラ
/// パニック時に呼ばれる
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

/// エントリポイント
/// no_mangle: 関数名を変更しない. エントリポイントをリンカに伝えるために必須
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;          // 文字
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;       // 色
        }
    }
    
    loop{}
}
