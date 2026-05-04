use core::arch::asm;

pub fn enable_a20() {
    unsafe {
        asm!(
            "int 0x15",
            in("ax") 0x2401_u16,
            options(nostack)
        );
    }
}
