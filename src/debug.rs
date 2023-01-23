use core::fmt;
use core::arch::asm;

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

pub struct Writer;

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write_string(s); 
        Ok(())
    }
}

#[allow(unused)]
pub fn print(args: fmt::Arguments) {
    use fmt::Write;
    Writer{}.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::debug::print(format_args!($($arg)*));
    }}
}

#[macro_export]
macro_rules! println {
    () => {{
        print!("\n");
    }};

    ($($arg:tt)*) => {{
        print!("{}\n", format_args!($($arg)*));
    }}
}

