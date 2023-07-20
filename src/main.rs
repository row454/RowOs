#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(type_alias_impl_trait)]

extern crate alloc;

use core::panic::PanicInfo;
use alloc::{boxed::Box, vec::Vec, rc::Rc, vec};
use bootloader::{entry_point, BootInfo};
use row_os::{
    hlt_loop,
    memory::{BootInfoFrameAllocator, EmptyFrameAllocator},
    println,
	allocator
};
use x86_64::{
    structures::paging::{Page, Translate},
    VirtAddr,
};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    hlt_loop()
}
entry_point!(kernel_main);
#[allow(clippy::empty_loop)]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    row_os::init();
    println!("Hello World! {}", 1_f32 / 9_f32);

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { row_os::memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    let page = Page::containing_address(VirtAddr::new(0x7e2700));
    row_os::memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

	allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

		let heap_value = Box::new(41);
		println!("heap_value at {:p}", heap_value);
	
		// create a dynamically sized vector
		let mut vec = Vec::new();
		for i in 0..500 {
			vec.push(i);
		}
		println!("vec at {:p}", vec.as_slice());
	
		// create a reference counted vector -> will be freed when count reaches 0
		let reference_counted = Rc::new(vec![1, 2, 3]);
		let cloned_reference = reference_counted.clone();
		println!("current reference count is {}", Rc::strong_count(&cloned_reference));
		core::mem::drop(reference_counted);
		println!("reference count is {} now", Rc::strong_count(&cloned_reference));
	
    println!("i didnt crash!");
    hlt_loop()
}
