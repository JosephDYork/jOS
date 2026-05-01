use core::arch::asm;

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
