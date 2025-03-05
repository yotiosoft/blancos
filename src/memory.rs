use x86_64::{ VirtAddr, PhysAddr };
use x86_64::structures::paging::{ PageTable, OffsetPageTable };

/// 新しい OffsetPageTable を初期化する
pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}

/// 与えられた仮想アドレスを対応する物理アドレスに変換
pub unsafe fn translate_addr(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
    translate_addr_inner(addr, physical_memory_offset)
}

/// 有効な level4 テーブルへの可変参照を渡す
/// この関数は unsafe であり、一度しか呼び出してはならない
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_str: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_str
}

/// 有効な level4 テーブルへの可変参照を渡す
fn translate_addr_inner(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
    use x86_64::structures::paging::page_table::FrameError;
    use x86_64::registers::control::Cr3;

    // 有効な level4 フレームを読み込み
    let (level_4_table_frame, _) = Cr3::read();

    let table_indexes = [
        addr.p4_index(), addr.p3_index(), addr.p2_index(), addr.p1_index()
    ];
    let mut frame = level_4_table_frame;

    // 角層のページテーブルをたどる
    for &index in &table_indexes {
        // フレームをページテーブルの参照に変換
        let virt = physical_memory_offset + frame.start_address().as_u64();
        let table_ptr: *const PageTable = virt.as_ptr();
        let table = unsafe { &*table_ptr };

        // ページテーブルを読み込み、frame を更新
        let entry = &table[index];
        frame = match entry.frame() {
            Ok(frame) => frame,
            Err(FrameError::FrameNotPresent) => return None,
            Err(FrameError::HugeFrame) => panic!("huge pages not supported"),
        };
    }

    // 目的の物理アドレスを計算
    Some(frame.start_address() + u64::from(addr.page_offset()))
}
