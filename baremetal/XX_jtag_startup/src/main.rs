#![no_std]
#![no_main]

global_asm!(include_str!("startup.s"));

use core::{
    arch::{asm, global_asm},
    panic::PanicInfo,
};

// https://doc.rust-lang.org/nomicon/panic-handler.html
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe { asm!("wfe") };
    }
}

#[no_mangle]
fn main() -> ! {
    loop {
        unsafe { asm!("wfe") };
    }
}
