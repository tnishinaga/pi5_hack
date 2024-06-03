#![no_std]
#![no_main]

global_asm!(include_str!("startup.s"));

// ref: https://interrupt.memfault.com/blog/zero-to-main-rust-1
extern "C" {
    static mut _BSS_START: u64;
    static mut _BSS_END: u64;
}

mod critical_section;
mod pl011_uart;
mod rp1_gpio;
mod systimer;

use core::{
    arch::{asm, global_asm},
    fmt::{self, Write},
    panic::PanicInfo,
    time,
};
use pl011_uart::{Pl011Uart, Pl011UartPeripheral};
use rp1_gpio::RP1Gpio;
use systimer::wait;

use tock_registers::{
    interfaces::{ReadWriteable, Readable, Writeable},
    register_bitfields,
    registers::InMemoryRegister,
};

// https://doc.rust-lang.org/nomicon/panic-handler.html
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut writer = EarlyUart {};
    writer.write_fmt(format_args!("{info}"));

    loop {
        unsafe { asm!("wfe") };
    }
}

const BCM2712_EARLY_UART_DR: *mut u32 = 0x10_7d00_1000 as *mut u32;
const BCM2712_EARLY_UART_FLAG: *mut u32 = (0x10_7d00_1000u64 + 0x18u64) as *mut u32;
// ref: https://tomoyuki-nakabayashi.github.io/embedded-rust-techniques/03-bare-metal/print.html
struct EarlyUart;
impl Write for EarlyUart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c)?;
        }
        Ok(())
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        while {
            let flag = unsafe { core::ptr::read_volatile(BCM2712_EARLY_UART_FLAG as *const u32) };
            flag & (1u32 << 5) != 0
        } {}
        unsafe { core::ptr::write_volatile(BCM2712_EARLY_UART_DR, c as u32) };
        Ok(())
    }
}

#[no_mangle]
fn main() -> ! {
    let mut writer = EarlyUart {};
    loop {
        writer.write_str("HOGEHOGE\n").unwrap();
        wait(time::Duration::from_secs(1));
    }
}
