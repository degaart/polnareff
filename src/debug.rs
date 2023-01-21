use core::arch::asm;

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

