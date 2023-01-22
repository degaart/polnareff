#![no_std]
#![no_main]

mod multiboot;
mod debug;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    /*
     * We need to use the symbol or it will be stripped from the executable
     */
    let _use_multiboot = &multiboot::MULTIBOOT;
    println!("It works!");
    loop {
    }
}

