ENTRY(_start)
SECTIONS
{
    __KERNEL_START__ = .;
    .multiboot : ALIGN(4096)
    {
        KEEP(*(.multiboot*));
    }

    .rodata : ALIGN(4096)
    {
        *(.rodata.*)
    }

    .text : ALIGN(4096)
    {
        *(.text.*);
    }

    .data : ALIGN(4096)
    {
        *(.data.*);
    }

    .bss : ALIGN(4096)
    {
        *(.data.*);
    }
    __KERNEL_END__ = .;
}

