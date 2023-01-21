SECTIONS
{
    .multiboot : ALIGN(4)
    {
        KEEP(*(.multiboot.STATIC));
    }
}

