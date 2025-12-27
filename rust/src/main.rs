#![no_std]
#![no_main]

use core::arch::naked_asm;
use core::panic::PanicInfo;

#[unsafe(naked)]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".startup")]
pub unsafe extern "C" fn _start() -> ! {
    naked_asm!(
        // Real-mode entry
        "cli",
        "xor ax, ax",
        "mov ds, ax",
        "mov es, ax",
        "mov ss, ax",
        "mov sp, 0x7C00",

        // Rust Entry
        "call {rust_main}",

        rust_main = sym rust_main,
    );
}

#[unsafe(naked)]
#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    // BIOS teletype: print 'R'
    naked_asm!(
        "mov ah, 0x0E",
        "mov al, '!'",
        "int 0x10",

        "cli",
        "hlt"
    );
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

