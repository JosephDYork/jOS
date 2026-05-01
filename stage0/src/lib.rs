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

pub fn read_sectors(dap: &DiskAddressPacket) {
    unsafe {
        asm!(
            "push ds",
            "push si",
            "mov si, cx",
            "mov ds, bx",
            "int 0x13",
            "pop si",
            "pop ds",
            in("ah") 0x42 as u8,
            in("dl") 0x80 as u8,
            in("cx") dap as *const _ as u16,
            in("bx") 0,
            options(nostack)
        );
    }
}

#[repr(C, packed)]
pub struct DiskAddressPacket {
    size: u8,
    reserved: u8,
    sector_cnt: u16,
    buffer_off: u16,
    buffer_seg: u16,
    lba: u64,
}

impl DiskAddressPacket {
    pub fn new(lba: u64, sectors: u16, offset: u16, segment: u16) -> Self {
        Self {
            size: 0x10,
            reserved: 0,
            sector_cnt: sectors,
            buffer_off: offset,
            buffer_seg: segment,
            lba,
        }
    }
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
