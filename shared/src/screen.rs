use core::arch::asm;

pub fn cscrn() {
    unsafe {
        asm!(
            "int 0x10",
            "mov ah, 0x02",
            "xor bh, bh",
            "xor dx, dx",
            "int 0x10",
            in("ax") 0x0700_u16,
            in("bh") 0x07_u8,
            in("cx") 0x0000_u16,
            in("dx") 0x184F_u16
        );
    }
}
