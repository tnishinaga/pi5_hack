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
use tock_registers::interfaces::{ReadWriteable, Readable, Writeable};

const NOP_DELAY_CYCLE: u64 = 1_000_000;

// https://doc.rust-lang.org/nomicon/panic-handler.html
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe { asm!("wfe") };
    }
}

fn nop_delay(count: u64) {
    for _i in 0..count {
        unsafe {
            asm!("nop");
        };
    }
}

struct SystemCounter;

impl SystemCounter {
    pub fn get_current_frequency() -> u32 {
        // get frequency from CNTFRQ_EL0
        let mut current_frequency = 0u64;
        unsafe {
            asm!("mrs {current_frequency}, CNTFRQ_EL0", current_frequency = out(reg) current_frequency)
        };

        current_frequency as u32
    }

    pub fn set_current_frequency(frequency: u32) {
        let mut current_frequency: u64 = frequency.into();
        unsafe {
            asm!("msr CNTFRQ_EL0, {current_frequency}", current_frequency = in(reg) current_frequency)
        };
    }

    pub fn get_physical_counter_value() -> u64 {
        let mut count = 0u64;
        unsafe {
            asm!("
            isb
            mrs {count}, CNTPCT_EL0
            ", count = out(reg) count)
        };
        count
    }
}

fn wait(duration: core::time::Duration) {
    let frequency = SystemCounter::get_current_frequency();
    let frequency_1us = frequency / 1000 / 1000;
    // TODO: frequencyの値に応じて解像度の加減を決める
    let duration_us = duration.as_micros();
    let wait_count = u128::from(frequency_1us) * duration_us;

    let alerm_count = u128::from(SystemCounter::get_physical_counter_value()) + wait_count;
    while (u128::from(SystemCounter::get_physical_counter_value()) < alerm_count) {
        unsafe { asm!("nop") };
    }
}

#[no_mangle]
fn main() -> ! {
    let mut frequency = SystemCounter::get_current_frequency();

    debug!("current frequency {}", frequency);

    loop {
        wait(core::time::Duration::from_secs(10));
        debug!("Hello world!");
    }
}
