/* based on https://wiki.osdev.org/Linker_Scripts */

ENTRY(_start)

SECTIONS
{
    . = 0x200000;
    .text.boot : ALIGN(4) { 
        *(.text.boot) 
    }
    .text : ALIGN(4) {
        *(.text.*)
    }
    .data : ALIGN(4) {
        *(.data*)
    }
    .bss : ALIGN(8) { 
        _BSS_START = .;
        *(.bss*) 
        _BSS_END = .;
    }
    /* 64MB space */
    . = 0x4000000;
    STACK_POINTER = .;
}
