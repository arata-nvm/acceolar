use core::arch::asm;

pub fn outb(port: u16, data: u8) {
    unsafe {
        asm!("out dx, al",
            in("dx") port,
            in("al") data,
        );
    }
}
