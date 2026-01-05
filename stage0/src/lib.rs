#![no_std]

use core::arch::asm;
use core::panic::PanicInfo;

//Text Functions
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

//Screen Functions
pub fn cscrn() {
    unsafe {
        asm!(
            "int 0x10",
            "mov ah, 0x02",
            "xor bh, bh",
            "xor dx, dx",
            "int 0x10",

            in("ax") 0x0700 as u16,
            in("bh") 0x07 as u8,
            in("cx") 0x0000 as u16,
            in("dx") 0x184F as u16
        );
    }
}

//Disk Functions
pub fn readdsk(addr: *const ()) {
    unsafe {
        asm!(
        "push si",
        "mov es, dx",
        "int 0x13",
        "pop si",

        in("bx") addr,
        in("ax") 0x0201 as u16,
        in("cx") 0x0002 as u16,
        in("dx") 0x0000 as u16,
        )
    }
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
