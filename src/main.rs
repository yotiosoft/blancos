#![no_std]      // std ライブラリを使わない
#![no_main]     // main 関数を使わない

use core::panic::PanicInfo;

/// パニックハンドラ
/// パニック時に呼ばれる
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// エントリポイント
/// no_mangle: 関数名を変更しない. エントリポイントをリンカに伝えるために必須
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop{}
}
