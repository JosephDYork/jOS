ENTRY(stage1)
EXTERN(PARAM);

MEMORY {
    stage1 : ORIGIN = 0x1000, LENGTH = 512
}

SECTIONS {
    . = ORIGIN(stage1);
    .table  :   { KEEP(*(.table)) } > stage1
    .text   :   { *(.startup) *(.text*) } > stage1
    .rodata :   { *(.rodata*) } > stage1
    .data   :   { *(.data*) } > stage1
    .bss    :   { *(.bss*) } > stage1
    /DISCARD/ : { *(.eh_frame*) }
}
