use crate::println;

use self::idt::init_idt;

pub mod idt;
pub mod pics;

pub fn init() {
    init_idt();
    println!("idt inited");
    unsafe { pics::PICS.lock().initialize() };
    println!("pics inited");
    x86_64::instructions::interrupts::enable();
}
