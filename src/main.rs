#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod gdt;
mod macros;
mod screen;
mod keyboard;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    gdt::init();
    keyboard::initialize();
    unsafe { keyboard::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
    println!("Welcome to Oxidize!!");
    loop { x86_64::instructions::hlt(); }
}

// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop { x86_64::instructions::hlt(); }
}
