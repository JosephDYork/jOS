#![no_std]
#![no_main]

use core::arch::asm;
use jos::{cscrn, puts};

#[unsafe(no_mangle)]
#[unsafe(link_section = ".startup")]
pub unsafe extern "C" fn _start() -> ! {
    unsafe {
        asm!(
            "mov ax, cs",
            "mov ds, ax",
            "mov es, ax",
            "mov ss, ax",
            "mov sp, 0x7C00",
            options(nostack, nomem)
        );

        rust_main();
    }
}

fn rust_main() -> ! {
    cscrn();
    puts(b"[ Stage0: OK ] Bootsector Loaded at 0x07C00");

    loop {}
}

