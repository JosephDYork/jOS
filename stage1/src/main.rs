#![no_std]
#![no_main]

use core::panic::PanicInfo;
use jos_shared::puts;

#[unsafe(no_mangle)]
#[unsafe(link_section = ".startup")]
fn stage2() -> ! {
    puts(b"\r\n[ Stage1: OK ] Second Stage bootloaded at 0x1000");

    loop {};
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
