use x86_64::structures::idt::InterruptDescriptorTable;

use crate::{gdt, interrupts::pics::InterruptIndex};
use lazy_static::lazy_static;
mod handlers;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(handlers::breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(handlers::double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX); // new
        }
        idt.page_fault.set_handler_fn(handlers::page_fault_handler);

        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(handlers::timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(handlers::keyboard_interrupt_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}
