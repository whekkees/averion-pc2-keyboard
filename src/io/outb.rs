use std::arch::asm;

pub unsafe fn outb (port: u16, val: u8) {
    asm!(
        "out dx, al",
        in("dx") port,
        in("al") val,
        options(nostack,preserves_flags),
    );

}