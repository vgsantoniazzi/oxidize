#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod screen;
mod macros;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to Oxidize!!");

    loop {}
}

// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
