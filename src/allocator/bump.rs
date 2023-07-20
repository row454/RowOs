use core::{alloc::GlobalAlloc, ptr::null_mut};


use super::{Locked, align_up};

pub struct BumpAllocator {
	heap_start: usize,
	heap_end: usize,
	next: usize,
	allocations: usize,
}
impl BumpAllocator {
	pub const fn new() -> Self {
		Self {
			heap_start: 0,
			heap_end: 0,
			next: 0,
			allocations: 0,
		}
	}
	pub fn init(&mut self, heap_start: usize, heap_size: usize) {
		self.heap_start = heap_start;
		self.heap_end = heap_start + heap_size;
		self.next = heap_start;
	}
}
unsafe impl GlobalAlloc for Locked<BumpAllocator> {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
		let mut allocator = self.lock();
		let alloc_start = align_up(allocator.next, layout.align());
		let alloc_end = alloc_start + layout.size();

        if alloc_end <= allocator.heap_end {
			allocator.next = alloc_end;
			allocator.allocations += 1;
			alloc_start as *mut u8

		} else {
			null_mut()
		}
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        let mut allocator = self.lock();
		allocator.allocations -= 1;
		if allocator.allocations == 0 {
			allocator.next = allocator.heap_start;
		}
    }
}

