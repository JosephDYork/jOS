ENTRY(stage2)

SECTIONS {
    . = 0x2000;
    .text   : { *(.startup) *(.text*) }
    .rodata : { *(.rodata*) }
    .data   : { *(.data*) }
    .bss    : { *(.bss*) }
    /DISCARD/ : { *(.eh_frame*) }
}
