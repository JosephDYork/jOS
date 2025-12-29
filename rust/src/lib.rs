#![no_std]

use core::arch::asm;
use core::panic::PanicInfo;

// Screen Utilities

pub fn puts(st: &[u8]) {
    for &c in st {
        putc(c);
    }
}

fn putc(ch: u8) {
    unsafe {
        asm!(
            "int 0x10",
            in("ax") ch as u16 | 0x0e00,
            options(nostack)
        );
    }
}

pub fn cscrn() {
    unsafe {
        asm!(
            "mov dx, 0x184F",
            "xor cx, cx",
            "mov bh, 0x07",
            "mov ax, 0x0700",
            "int 0x10",
            "mov ah, 0x02",
            "xor bh, bh",
            "xor dx, dx",
            "int 0x10",
        );
    }
}

// 

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
