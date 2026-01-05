#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

#[unsafe(no_mangle)]
#[unsafe(link_section = ".startup")]
fn stage2() -> ! {
    puts(b"\r\n[ Stage1: OK ] Second Stage bootloaded at 0x1000");
    puts(b"\r\n[ Stage1: OK ] Second Stage bootloaded at 0x1000");
    puts(b"\r\n[ Stage1: OK ] Second Stage bootloaded at 0x1000");

    loop {};
}

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
            in("bx") 0x0007 as u16,
            options(nostack)
        );
    }
}


#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
