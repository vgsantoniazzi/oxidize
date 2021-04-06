#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod macros;
mod screen;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to Oxidize!!");
    panic!("Error!");
}

// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
