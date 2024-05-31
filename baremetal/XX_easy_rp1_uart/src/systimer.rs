use core::arch::asm;
struct SystemCounter;

impl SystemCounter {
    pub fn get_current_frequency() -> u32 {
        // get frequency from CNTFRQ_EL0
        #[allow(unused_assignments)]
        let mut current_frequency = 0u64;
        unsafe {
            asm!("mrs {current_frequency}, CNTFRQ_EL0", current_frequency = out(reg) current_frequency)
        };

        current_frequency as u32
    }

    pub fn set_current_frequency(frequency: u32) {
        let current_frequency: u64 = frequency.into();
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

pub fn wait(duration: core::time::Duration) {
    let frequency = SystemCounter::get_current_frequency();
    let frequency_1us = frequency / 1000 / 1000;
    // TODO: frequencyの値に応じて解像度の加減を決める
    let duration_us = duration.as_micros();
    let wait_count = u128::from(frequency_1us) * duration_us;

    let alerm_count = u128::from(SystemCounter::get_physical_counter_value()) + wait_count;
    while u128::from(SystemCounter::get_physical_counter_value()) < alerm_count {
        unsafe { asm!("nop") };
    }
}
