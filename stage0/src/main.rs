#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;
use jos_shared::{DiskAddressPacket, puts, readdsk_ext, cscrn};

#[unsafe(no_mangle)]
#[unsafe(link_section = ".startup")]
pub unsafe extern "C" fn _start() -> ! {
    unsafe {
        asm!(
            "cli",
            "xor ax, ax",
            "mov ds, ax",
            "mov es, ax",
            "mov ss, ax",
            "mov fs, ax",
            "mov gs, ax",
            "cld",
            "mov sp, 0x7c00",
            "sub sp, 0x0200",
            options(nostack, nomem)
        );

        rust_main();
    }
}

fn rust_main() -> ! {
    let stage1_ptr = 0x1000 as *const ();
    let stage1: fn() -> ! = unsafe { core::mem::transmute(stage1_ptr) };
    let dap = DiskAddressPacket::new(1, 1, 0x1000, 0x0000);

    cscrn();
    puts(b"[ Stage0: OK ] Bootsector stage0 started at 0x07C00\n\r");
    readdsk_ext(&dap);
    stage1();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
