use core::arch::asm;
use core::fmt::{Write, Arguments};

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::debug::_print(format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: Arguments) {
    (DebugWriter{}).write_fmt(args).unwrap();
}

pub struct DebugWriter;

impl Write for DebugWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        write_string(s);
        Ok(())
    }
}

fn write_char(ch: u8) {
    unsafe {
        asm! {
            "out 0xE9, al",
            in("al") ch
        }
    }
}

pub fn write_string(s: &str) {
    for ch in s.bytes() {
        write_char(ch);
    }
}

