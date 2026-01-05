MEMORY {
    body    :   ORIGIN = 31744, LENGTH = 442
    size    :   ORIGIN = 32186, LENGTH = 2
    slba    :   ORIGIN = 32188, LENGTH = 2
}

ENTRY(_start);

SECTIONS {
    .text   :   { *(.startup) *(.text*) }   > body      /* Excutable code */
    .rodata :   { *(.rodata*) }             > body      /* Constants (R/O) */
    .data   :   { *(.data*) }               > body      /* Initialized data */
    .bss    :   { *(.bss*) }                > body      /* Unitnitialized data */
    .size   :   { KEEP(*(.size*)) }         > size      /* Boot parameter table */
    .slba   :   { KEEP(*(.slba*)) }         > slba
    /DISCARD/ : { *(.eh_frame*) }
}
