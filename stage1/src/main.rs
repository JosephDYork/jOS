#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;
use jos_shared::{DiskAddressPacket, enable_a20, load_gdt, puth, puts, readdsk_ext};

#[unsafe(no_mangle)]
#[unsafe(link_section = ".startup")]
fn stage1() -> ! {
    let stage2_ptr = 0x2000 as *const ();
    let dap = DiskAddressPacket::new(2, 1, 0x2000, 0x0000);
    let gdt_ptr = load_gdt();

    puts(b"[ Stage1: OK ] Second Stage bootloaded at 0x1000\r\n");
    readdsk_ext(&dap);
    puts(b"[ Stage1: OK ] Stage2 loaded at 0x");
    puth(stage2_ptr as usize as u32);
    puts(b"\r\n");
    puts(b"[ Stage1: OK ] GDT loaded at 0x");
    puth(gdt_ptr as usize as u32);
    puts(b"\r\n");
    enable_a20();

    unsafe {
        asm!(
            "mov eax, cr0",
            "or eax, 0x1",
            "mov cr0, eax",
            ".byte 0x66, 0xEA",
            ".long 0x2000",
            ".word 0x0008",
            options(nostack, noreturn)
        );
    }
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
