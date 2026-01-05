#![no_std]
#![no_main]

use core::arch::asm;
use jos::{cscrn, readdsk, puts};

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
    let ptr = 0x1000 as *const ();
    let stage2: fn() -> ! = unsafe { core::mem::transmute(ptr) };
    cscrn();
    puts(b"[ Stage0: OK ] Bootsector Loaded at 0x07C00");
    readdsk(ptr); // Reads disk to 0
    stage2(); // Jump to the next stage
}

