#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

#[unsafe(no_mangle)]
#[unsafe(link_section = ".startup")]
pub extern "C" fn stage2() -> ! {
    unsafe {
        asm!(
            "mov ax, 0x10",
            "mov ds, ax",
            "mov es, ax",
            "mov fs, ax",
            "mov gs, ax",
            "mov ss, ax",
            "mov esp, 0x9000",
            options(nostack, preserves_flags)
        );

        rust_32_main();
    }
}


fn rust_32_main() -> ! {
    let address = 0xB8000 as *mut u8;

    unsafe {
        *address.offset((16 * 40) + 0) = b'H';
        *address.offset((16 * 40) + 1) = 0x03;
        *address.offset((16 * 40) + 2) = b'e';
        *address.offset((16 * 40) + 3) = 0x03;
        *address.offset((16 * 40) + 4) = b'l';
        *address.offset((16 * 40) + 5) = 0x03;
        *address.offset((16 * 40) + 6) = b'l';
        *address.offset((16 * 40) + 7) = 0x03;
        *address.offset((16 * 40) + 8) = b'o';
        *address.offset((16 * 40) + 9) = 0x03;
    }

    loop {}
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
