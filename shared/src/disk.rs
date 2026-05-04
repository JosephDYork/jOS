use core::arch::asm;

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

pub fn readdsk_chs(addr: *const ()) {
    unsafe {
        asm!(
            "push si",
            "mov es, dx",
            "int 0x13",
            "pop si",
            in("bx") addr,
            in("ax") 0x0201_u16,
            in("cx") 0x0002_u16,
            in("dx") 0x0000_u16,
        )
    }
}

pub fn readdsk_ext(dap: &DiskAddressPacket) {
    unsafe {
        asm!(
            "push ds",
            "push si",
            "mov si, cx",
            "mov ds, bx",
            "int 0x13",
            "pop si",
            "pop ds",
            in("ah") 0x42_u8,
            in("dl") 0x80_u8,
            in("cx") dap as *const _ as u16,
            in("bx") 0,
            options(nostack)
        );
    }
}

pub fn readdsk(addr: *const ()) {
    readdsk_chs(addr);
}

pub fn read_sectors(dap: &DiskAddressPacket) {
    readdsk_ext(dap);
}
