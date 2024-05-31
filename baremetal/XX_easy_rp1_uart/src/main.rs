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
    panic::PanicInfo,
    time,
};
use defmt::debug;
use defmt_rtt as _;
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
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe { asm!("wfe") };
    }
}

#[no_mangle]
fn main() -> ! {
    // cpu memory address 0x1f_0000_0000 is mapped to RP1 proc address 0x4000_0000
    const RP1_PERIPHERAL_BASE: u64 = 0x1f_0000_0000u64;

    // > UART interfaces have an independent baud clock (clk_uart), typically 48MHz.
    // > Raspberry Pi RP1 Peripherals 2.5.5. Other Peripheral clocks pp.13
    const UART_CLOCK: u64 = 48 * 1000 * 1000;
    const UART0_ADDRESS: *mut u32 = (RP1_PERIPHERAL_BASE + 0x0003_0000u64) as *mut u32;

    let pl011_uart = Pl011Uart::new(UART0_ADDRESS);
    pl011_uart.init(UART_CLOCK, 115200);
    pl011_uart.enable();

    // setup gpio
    const RP1_GPIO_ADDRESS: *mut u32 = (RP1_PERIPHERAL_BASE + 0x000d_0000u64) as *mut u32;
    const RP1_PAD_ADDRESS: *mut u32 = (RP1_PERIPHERAL_BASE + 0x000f_0000u64) as *mut u32;
    let rp1_gpio = RP1Gpio::new(RP1_GPIO_ADDRESS, RP1_PAD_ADDRESS);
    rp1_gpio.set_function(14, 4);
    rp1_gpio.set_function(15, 4);
    rp1_gpio.set_output_enable(14, true);
    rp1_gpio.set_output_enable(15, true);

    let mut loop_count = 0;
    loop {
        pl011_uart.write(&[b'A']);
        debug!("Hello world! {}", loop_count);
        wait(time::Duration::from_secs(1));
        loop_count += 1;
    }
}
