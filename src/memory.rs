use x86_64::{ structures::paging::PageTable, VirtAddr };

/// 有効な level4 テーブルへの可変参照を渡す
/// この関数は unsafe であり、一度しか呼び出してはならない
pub unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_str: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_str
}
