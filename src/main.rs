// main.rs

// don't link the Rust standard library
#![no_std] 
// disable all Rust-level entry points
#![no_main]

use core::panic::PanicInfo;

// this function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// don't mangle the name of this function
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for the function
    // named `_start` by default
    loop {}
}
