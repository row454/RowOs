#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(type_alias_impl_trait)]

extern crate alloc;

pub mod allocator;
pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod vga;
pub fn init() {
    gdt::init();
    interrupts::init();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
