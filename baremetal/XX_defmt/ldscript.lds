/* based on https://wiki.osdev.org/Linker_Scripts */

ENTRY(_start)

SECTIONS
{
    . = 0x80000;
    .text.boot : ALIGN(4) { 
        *(.text.boot) 
    }
    .text : ALIGN(8) {
        *(.text*)
        . = ALIGN(8);
    }

    .rodata : ALIGN(8) {
        *(.rodata*)
        . = ALIGN(8);
    }

    .data : ALIGN(8) {
        *(.data*)
        . = ALIGN(8);
    }

    .bss : ALIGN(8) { 
        _BSS_START = .;
        *(.bss*) 
        . = ALIGN(8);    
        _BSS_END = .;
    }
    /* 64MB space */
    . = 0x4000000;
    STACK_POINTER = .;
}
