#![no_std]
#![no_main]

global_asm!(include_str!("startup.s"));

// ref: https://interrupt.memfault.com/blog/zero-to-main-rust-1
extern "C" {
    static mut _BSS_START: u64;
    static mut _BSS_END: u64;
}

mod bcm2712_pcie;
mod brcmstb_gpio;
mod critical_section;
mod pcie_generic;
mod pl011_uart;
mod rp1_gpio;
mod systimer;

use bcm2712_pcie::{BcmStbPcie, RcInOutboundParam};
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

use crate::pcie_generic::{BistHeaderTypeTimerCache, CommandAndStatus};

// > UART interfaces have an independent baud clock (clk_uart), typically 48MHz.
// > Raspberry Pi RP1 Peripherals 2.5.5. Other Peripheral clocks pp.13
const UART_CLOCK: u64 = 48 * 1000 * 1000;
const RP1_PERIPHERAL_BASE: u64 = 0x1f_0000_0000u64;
const UART0_ADDRESS: *mut u32 = (RP1_PERIPHERAL_BASE + 0x0003_0000) as *mut u32;
const RP1_GPIO_ADDRESS: *mut u32 = (RP1_PERIPHERAL_BASE + 0x000d_0000) as *mut u32;
const RP1_PAD_ADDRESS: *mut u32 = (RP1_PERIPHERAL_BASE + 0x000f_0000) as *mut u32;

// https://doc.rust-lang.org/nomicon/panic-handler.html
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe { asm!("wfe") };
    }
}

fn id_found(bus: u8, slot: u8, func: u8, id: u32) {
    debug!(
        "bus {:#04x}, slot:{:#04x}, func: {:#04x}, id {:#010x}",
        bus, slot, func, id
    );
}

register_bitfields![
    u32,
    // ref: https://en.wikipedia.org/wiki/PCI_configuration_space
    pub PciBar [
        REGION_TYPE OFFSET(0) NUMBITS(1) [
            MEMORY = 0,
            IO = 1,
        ],
        LOCATABLE   OFFSET(1) NUMBITS(2) [
            ANY_32BIT = 0,
            LESS_THAN_1MB = 1,
            ANY_64BIT = 2,
        ],
        PREFETCHABLE OFFSET(3) NUMBITS(1) [],
        BASE_ADDRESS OFFSET(4) NUMBITS(28) [],
    ]
];

fn config_dump(dump: &[u32]) {
    debug!("config dump: {:?}", dump);
}

fn print_chip_id(id: u32) {
    debug!("id: {:#010x}", id);
}

fn print_address(addr: u64) {
    debug!("addr: {:#018x}", addr);
}

fn print_data(data: u32) {
    debug!("data: {:#010x}", data);
}

fn print_pci_header_type(t: u32) {
    debug!("header type: {:#010x}", t);
}

fn setup_pcie_bridge(bcmstb_pcie: &mut BcmStbPcie) {
    // set rc bar2
    // bcmstb_pcie.pcie.rc_pci_config[0x04 / 4].set(0x00100006);
    bcmstb_pcie.rc_pci_header_type1().command_and_status.set(
        (CommandAndStatus::CMD_MEMORY_SPACE::SET
            + CommandAndStatus::CMD_BUS_MUSTER::SET
            + CommandAndStatus::STATUS_CAPABILITIES_LIST::SET)
            .value,
    );
    bcmstb_pcie.rc_pci_header_type1().set_bus_number(0, 1, 1);
    bcmstb_pcie
        .rc_pci_header_type1()
        .io_and_secondary_status
        .set(0);

    // 0x0000_0000 - 0x005F_FFFF のmemory transactionは通す設定
    // ref: https://www.macnica.co.jp/business/semiconductor/articles/microchip/140354/
    bcmstb_pcie
        .rc_pci_header_type1()
        .set_memory_base_limit(0x0000_0000, 0x005F_FFFF);

    // RP1のMPSが256Bなので、bridgeも256Bにする
    bcmstb_pcie
        .rc_pcie_cap()
        .device_control_status
        .modify(pcie_generic::PCIeDeviceControlStatus::MAX_PAYLOAD_SIZE::SIZE_256B);
}

fn setup_rp1(bcmstb_pcie: &mut BcmStbPcie) {
    // setup ECAM window
    // TODO: refactor and lock
    let _ = bcmstb_pcie.read_pci_config_data(1, 0, 0, 0);
    let rp1_pci_header: &'static mut pcie_generic::GenericPciHeaderType0 = unsafe {
        pcie_generic::GenericPciHeaderType0::new_ref(
            bcmstb_pcie.pcie.config_data.as_mut_ptr() as *mut u32
        )
    };

    // set command
    rp1_pci_header
        .command_and_status
        .modify(CommandAndStatus::CMD_MEMORY_SPACE::SET + CommandAndStatus::CMD_BUS_MUSTER::SET);
    // RP1 memory map(from linux dmesg)
    // BAR 1: assigned [mem 0x1f00000000-0x1f003fffff]
    // BAR 2: assigned [mem 0x1f00400000-0x1f0040ffff]
    // BAR 0: assigned [mem 0x1f00410000-0x1f00413fff]
    // set BAR0(MSI, 16KiB)
    rp1_pci_header.base_addres_registers[0].set(0x00410000);
    // set BAR1(PR1 pcie address 0x4000_0000(APB0) - 0x4040_0000(AXI))
    rp1_pci_header.base_addres_registers[1].set(0);
    // set BAR2(64KiB) SRAM
    rp1_pci_header.base_addres_registers[2].set(0x00400000);

    // select RP1 pci config space
    // TODO: use lock
    let _ = bcmstb_pcie.read_pci_config_data(1, 0, 0, 0);
    const RP1_PCIE_CAP_OFFSET: usize = 0x70;
    let pcie_cap =
        (&mut bcmstb_pcie.pcie.config_data[RP1_PCIE_CAP_OFFSET / 4..]).as_mut_ptr() as *mut u32;
    let rp1_pcie_cap = unsafe { pcie_generic::PCIeCapabilityStructure::new_ref(pcie_cap) };

    // set MPS
    rp1_pcie_cap.device_control_status.modify(
        pcie_generic::PCIeDeviceControlStatus::MAX_PAYLOAD_SIZE::SIZE_256B
            + pcie_generic::PCIeDeviceControlStatus::MAXIMUM_READ_REQUEST_SIZE.val(4),
    );
    rp1_pcie_cap
        .link_control_status
        .modify(pcie_generic::PCIeLinkControlStatus::COMMON_CLOCK_CONFIGURATION::SET);
}

#[no_mangle]
fn main() -> ! {
    let mut bcmstb_pcie = crate::bcm2712_pcie::BcmStbPcie::new(0x10_0012_0000u64);

    // ref: https://github.com/raspberrypi/linux/blob/rpi-6.6.y/arch/arm/boot/dts/broadcom/bcm2712.dtsi#L1110-L1125
    // https://elinux.org/Device_Tree_Usage#PCI_Address_Translation
    bcmstb_pcie.reset(
        // 32bit outbound window
        &RcInOutboundParam::new(0x1f_0000_0000, 0x00_0000_0000, 0xfffffffc),
        // 64bit outbound window
        &RcInOutboundParam::new(0x1c_0000_0000, 0x04_0000_0000, 0x3_0000_0000),
        // PCIe devices are able to free access to CPU memory
        &RcInOutboundParam::new(0, 0, 64 * 1024 * 1024 * 1024 /* 64GiB */),
    );
    bcmstb_pcie.enable().unwrap();

    // setup bridge
    setup_pcie_bridge(&mut bcmstb_pcie);
    // setup rp1
    setup_rp1(&mut bcmstb_pcie);

    {
        // read RP1 chip id
        let chip_id_ptr = 0x1F_0000_0000 as *const u32;
        let chip_id = unsafe { core::ptr::read_volatile(chip_id_ptr) };
        print_chip_id(chip_id);
    }

    // enable RP1's uart
    let pl011_uart = Pl011Uart::new(UART0_ADDRESS);
    {
        // setup uart
        pl011_uart.init(UART_CLOCK, 115200);
        pl011_uart.enable();

        let rp1_gpio = RP1Gpio::new(RP1_GPIO_ADDRESS, RP1_PAD_ADDRESS);
        // set gpio function to uart
        rp1_gpio.set_function(14, 4);
        rp1_gpio.set_function(15, 4);
        // enable output
        rp1_gpio.set_output_enable(14, true);
        rp1_gpio.set_output_enable(15, true);
    }

    let mut loop_count = 0;
    loop {
        pl011_uart.write("Hello World\r\n".as_bytes());
        debug!("Hello world! {}", loop_count);
        wait(time::Duration::from_secs(1));
        loop_count += 1;
    }
}
