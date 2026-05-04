use core::arch::asm;

pub fn puts(st: &[u8]) {
    for &c in st {
        putc(c);
    }
}

pub fn putc(ch: u8) {
    unsafe {
        asm!(
            "int 0x10",
            in("ax") ch as u16 | 0x0e00,
            options(nostack)
        );
    }
}

pub fn puth(n: u32) {
    let hex_chars = b"0123456789ABCDEF";

    if n == 0 {
        putc(b'0');
        return;
    }

    let mut started = false;
    for i in (0..8).rev() {
        let nibble = ((n >> (i * 4)) & 0xF) as usize;
        if nibble != 0 || started {
            started = true;
            putc(hex_chars[nibble]);
        }
    }
}

pub fn puti(mut n: u32) {
    let mut i = 10;
    let mut buf = [0u8; 10];

    if n == 0 {
        putc(b'0');
        return;
    }

    while n > 0 {
        i -= 1;
        buf[i] = (n % 10) as u8 + b'0';
        n /= 10;
    }

    for &c in &buf[i..] {
        putc(c);
    }
}
