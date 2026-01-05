ENTRY(stage2)
EXTERN(PARAM);
SECTIONS {
    . = 0x1000;
    .table  :   { KEEP(*(.table)) } 
    .text   :   { *(.startup) *(.text*) }
    .rodata :   { *(.rodata*) }
    .data   :   { *(.data*) }
    .bss    :   { *(.bss*) }
    /DISCARD/ : { *(.eh_frame*) }
}
