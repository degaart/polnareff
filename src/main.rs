#![no_std]
#![no_main]

use core::arch::asm;
use core::fmt::{self, Write};

pub struct Writer;

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write_string(s); 
        Ok(())
    }
}

pub fn write_string(s: &str) {
    for ch in s.bytes() {
        unsafe {
            asm!{
                "out 0xE9, al",
                in("al") ch,
                options(preserves_flags, nomem, nostack)
            };
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    Writer{}.write_fmt(format_args!("\n*** PANIC ***\n{}\n", _info)).unwrap();
    loop {}
}

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    Writer{}.write_fmt(format_args!("{}{}{}\n", "---", "It works!", "---")).unwrap();
    panic!("Kernel end");
}


