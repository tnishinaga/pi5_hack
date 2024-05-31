.global _start
.extern STACK_POINTER
.extern _BSS_START
.extern _BSS_END
.section ".text.boot"

_start:
    # mov x0, #0
    # cbz x0, _start
    // Exception Level 2
    // disable all interrupt
    msr daifclr, #0
    // set stack pointer address
    ldr x0,=STACK_POINTER
    mov sp, x0
    
clear_bss:
    ldr x0, =_BSS_START
    ldr x1, =_BSS_END
clear_bss_loop:
    cmp x0, x1
    beq clear_bss_end
    str xzr, [x0], #8
    b clear_bss_loop
clear_bss_end:
    bl main
loop:
    wfe
    b loop

