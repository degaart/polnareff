#[repr(C,align(4))]
pub struct Multiboot {
    magic: u32,
    flags: u32,
    checksum: u32,
}

const MULTIBOOT_MAGIC: u32 = 0x1BADB002;

#[used]
#[link_section = ".multiboot"]
pub static MULTIBOOT: Multiboot = Multiboot {
    magic: MULTIBOOT_MAGIC,
    flags: 0x0,
    checksum: (!MULTIBOOT_MAGIC + 1),
};

