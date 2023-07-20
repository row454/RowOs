#![no_std]
#![no_main]

mod vga;
use core::panic::PanicInfo;



#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	println!("{}", info);

    loop {}
}

#[no_mangle]
#[allow(clippy::empty_loop)]
pub extern "C" fn _start() -> ! {
	println!("Hello World! {}", 1_f32/9_f32);
	panic!("i have nothing else to do");
	loop {}
}