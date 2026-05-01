#![no_std]
#![no_main]

use core::arch::asm;
use jos::{DiskAddressPacket, read_sectors, cscrn, puts};

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
    let dap = DiskAddressPacket::new(1, 1, 0x1000, 0x0000);

    cscrn();
    puts(b"[ Stage0: OK ] Bootsector Loaded at 0x07C00");
    read_sectors(&dap);
    stage2();
}
