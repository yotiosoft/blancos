#![no_std]      // std ライブラリを使わない
#![no_main]     // main 関数を使わない

use core::panic::PanicInfo;

mod vga_buffer;

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
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();
    
    loop{}
}
