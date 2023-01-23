#![no_std]
#![no_main]

mod debug;
use core::arch::asm;

fn exit_qemu(status: u8) -> ! {
    unsafe {
        asm! {
            "out 0xF4, al",
            in("al") status
        }
    }
    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("\n*** PANIC***\n{}", info);
    exit_qemu(2);
}

#[no_mangle]
pub extern "C" fn kmain() {
    println!("*** {} ***", "Kernel started");
}


