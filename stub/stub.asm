MB_ALIGN    equ  1 << 0            ; align loaded modules on page boundaries
MB_MEMINFO  equ  1 << 1            ; provide memory map
MB_FLAGS    equ  MB_ALIGN | MB_MEMINFO ; this is the Multiboot 'flag' field
MB_MAGIC    equ  0x1BADB002        ; 'magic number' lets bootloader find the header
MB_CHECKSUM equ -(MB_MAGIC + MB_FLAGS)   ; checksum of above, to prove we are multiboot

section .multiboot
align 4
    global multiboot_header
    multiboot_header:
    dd MB_MAGIC
    dd MB_FLAGS
    dd MB_CHECKSUM

section .bss
align 16
    stack_bottom: resb 65536
    stack_top:
    multiboot_info: resd 1

section .text
global _start
_start:
    cli
    mov esp, stack_top
    mov [multiboot_info], ebx
    sub esp, 4
    and esp, 0xfffffff0
    mov eax, [multiboot_info]
    mov [esp], eax

    extern kmain
    call kmain
    cli

.halt:
    hlt
    jmp .halt


