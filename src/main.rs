#![no_std]
#![no_main]
use core::arch::asm;

#[repr(C,align(4))]
pub struct Multiboot {
    magic: u32,
    flags: u32,
    checksum: u32,
}

const MULTIBOOT_MAGIC: u32 = 0x1BADB002;

#[link_section = ".multiboot"]
pub static MULTIBOOT: Multiboot = Multiboot {
    magic: MULTIBOOT_MAGIC,
    flags: 0x0,
    checksum: (!MULTIBOOT_MAGIC + 1),
};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    /*
     * We need to use the symbol or it will be stripped from the executable
     */
    let _use_multiboot = &MULTIBOOT;
    unsafe {
        asm! {
            "mov al, 'A'",
            "out 0xE9, al",
            "mov al, 'B'",
            "out 0xE9, al",
            "mov al, 'C'",
            "out 0xE9, al",
            out("al") _
        };
    }
    loop {
    }
}

