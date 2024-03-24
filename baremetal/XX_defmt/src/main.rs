#![no_std]
#![no_main]

global_asm!(include_str!("startup.s"));

// ref: https://interrupt.memfault.com/blog/zero-to-main-rust-1
extern "C" {
    static mut _BSS_START: u64;
    static mut _BSS_END: u64;
}

mod critical_section;
use core::{
    arch::{asm, global_asm},
    panic::PanicInfo,
};
use defmt::debug;
use defmt_rtt as _;

// https://doc.rust-lang.org/nomicon/panic-handler.html
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe { asm!("wfe") };
    }
}

fn nop_delay(_count: u64) {
    for _i in 0..count {
        unsafe {
            asm!("nop");
        };
    }
}

#[no_mangle]
fn main() -> ! {
    let mut loop_count = 0;
    loop {
        debug!("Hello world! {}", loop_count);
        nop_delay(100_000_000);
        loop_count += 1;
    }
}
