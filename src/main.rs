#![no_std]      // std ライブラリを使わない
#![no_main]     // main 関数を使わない

#![feature(custom_test_frameworks)] 
#![test_runner(blancos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{ BootInfo, entry_point };
use core::panic::PanicInfo;
use blancos::println;
use blancos::memory;

entry_point!(kernel_main);

/// エントリポイント
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use blancos::memory::translate_addr;
    use x86_64::{ structures::paging::Translate, VirtAddr };

    println!("Hello World{}", "!");

    blancos::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mapper = unsafe { memory::init(phys_mem_offset) };
    let addresses = [
        // VGA buffer page
        0xb8000,
        // code page
        0x201008,
        // stack page
        0x0100_0020_1a10,
        // 物理アドレス 0 にマップされている仮想アドレス
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }

    #[cfg(test)]
    test_main();
    
    println!("It did not crash!");
    blancos::hlt_loop();
}

/// パニックハンドラ
/// パニック時に呼ばれる
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    blancos::hlt_loop();
}

/// テスト時に使うパニックハンドラ
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blancos::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
