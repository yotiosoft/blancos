#![no_std]      // std ライブラリを使わない
#![no_main]     // main 関数を使わない

#![feature(custom_test_frameworks)] 
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod vga_buffer;

/// パニックハンドラ
/// パニック時に呼ばれる
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop{}
}

/// エントリポイント
/// no_mangle: 関数名を変更しない. エントリポイントをリンカに伝えるために必須
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();
    
    loop{}
}

// QEMU Exit Code
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

/// test_runner
#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... "); // "些末なアサーション……"
    assert_eq!(1, 1);
    println!("[ok]");
}
