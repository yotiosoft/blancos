
struct ListNode {
    next: option<&'static mut ListNode>,
}

/// Block sizes
/// これらは2の塁上でなければならない
const BLOCK_SIZES: &[usize] = &[8, 16, 32, 64, 128, 256, 512, 1024, 2048];

pub struct FixedSizeBlockAllocator {
    list_heads: [Option<&'static mut ListNode>; BLOCK_SIZES.len()],
    fallback_allocator: linked_list_allocator::Heap,
}

impl FixedSizeBlockAllocator {
    /// 空の FixedSizeBlockAllcoator を作る
    pub const fn new() -> Self {
        const EMPTY: Option<&'static mut ListNode> = None;
        FixedSizeBlockAllcoator {
            list_heads: [EMPTY; BLOCK_SIZES.len()],
            fallback_allocator: linked_list_allocator::Heap::empty(),
        }
    }

    /// アロケータを与えられたヒープ境界で初期化する
    /// 呼び出し元は渡すヒープ境界が有効でありヒープが未使用であることを保証しなければならない
    /// このメソッドは一度しか呼ばれてはならない
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        unsafe {
            self.fallback_allocator.init(heap_start, heap_size);
        }
    }
}
