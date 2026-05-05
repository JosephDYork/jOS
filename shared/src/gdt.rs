use core::arch::asm;

pub type GdtEntry = u64;

#[repr(C, packed)]
pub struct GdtDescriptor {
    pub limit: u16,
    pub base: u32,
}

// important to remember...--JY 5/4/2026
// Flat 32-bit segment base = 0x00000000 and limit = 0x000FFFFF 
// Granularity is one so 4 KiB means 4 GiB - 1
// D/B=1 for 32-bit segments
pub const GDT_TABLE: [GdtEntry; 3] = [
    0x0000_0000_0000_0000, // So this is the null entry, it's required
    0x00CF_9A00_0000_FFFF, // Bootloader text section ring0
    0x00CF_9200_0000_FFFF, // Bootloader data section ring0
];

pub fn load_gdt() -> *const GdtEntry {
    let gdtr = GdtDescriptor {
        limit: (core::mem::size_of_val(&GDT_TABLE) - 1) as u16,
        base: GDT_TABLE.as_ptr() as u32,
    };

    unsafe {
        asm!(
            "lgdt [{}]",
            in(reg) &gdtr,
            options(readonly, nostack)
        );
    }

    GDT_TABLE.as_ptr()
}
