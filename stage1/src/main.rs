#![no_std]
#![no_main]

use core::panic::PanicInfo;
use jos_shared::{load_gdt, puth, puts, enable_a20};

#[unsafe(no_mangle)]
#[unsafe(link_section = ".startup")]
fn stage2() -> ! {
    let gdt_ptr = load_gdt();

    puts(b"[ Stage1: OK ] Second Stage bootloaded at 0x1000\r\n");
    puts(b"[ Stage1: OK ] GDT loaded at 0x");
    puth(gdt_ptr as usize as u32);
    enable_a20();

    loop {};
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
