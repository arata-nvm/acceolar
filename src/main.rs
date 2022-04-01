#![no_std]
#![no_main]

use acceolar::println;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    loop {}
}

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    println!("Hello world!");

    loop {}
}
