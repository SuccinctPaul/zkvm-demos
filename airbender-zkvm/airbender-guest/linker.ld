ENTRY(_start)

MEMORY {
    /* Airbender memory layout assumption: Code starts at 0x0 */
    ram (rwx) : ORIGIN = 0x00000000, LENGTH = 0x10000000 /* 256MB */
}

SECTIONS {
    .text : {
        *(.init)
        *(.text*)
    } > ram

    .rodata : {
        *(.rodata*)
    } > ram

    .data : {
        *(.data*)
    } > ram

    .bss : {
        *(.bss*)
    } > ram

    /* Stack pointer initialization */
    /* Place stack at 16MB */
    _sstack = 0x01000000;
}

