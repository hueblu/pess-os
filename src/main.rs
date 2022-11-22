#![no_std]
#![no_main]

extern crate lazy_static;
extern crate spin;
extern crate volatile;

mod vga_buffer;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello F{}t{}er", "a", "h");
    print!("Hello ");
    println!("Again");
    panic!("Panicking panicked");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
