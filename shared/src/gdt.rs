#[repr(C, packed)]
pub struct GdtDescriptor {
    limit: u16,
    base: u32,
}

#[repr(C, packed)]
pub struct GdtEntry {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    flags_limit_high: u8,
    base_high: u8,
}

impl GdtEntry {
    pub const fn new(base: u32, limit: u32, access: u8, flags: u8) -> Self {
        GdtEntry {
            limit_low: (limit & 0xFFFF) as u16,
            base_low: (base & 0xFFFF) as u16,
            base_middle: ((base >> 16) & 0xFF) as u8,
            access,
            flags_limit_high: (((limit >> 16) & 0x0F) as u8) | (flags & 0xF0),
            base_high: ((base >> 24) & 0xFF) as u8,
        }
    }
}
