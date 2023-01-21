#![no_std]
#![no_main]

mod multiboot;
mod debug;

use debug::write_string;


#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    /*
     * We need to use the symbol or it will be stripped from the executable
     */
    let _use_multiboot = &multiboot::MULTIBOOT;
    write_string("It works!\n");
    loop {
    }
}

