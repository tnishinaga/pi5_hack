use core::arch::asm;

use tock_registers::{
    interfaces::{ReadWriteable, Readable, Writeable},
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite, WriteOnly},
};

register_structs! {
    pub Pl011UartPeripheral {
        (0x0000 => pub data: ReadWrite<u32>),
        (0x0004 => pub receive_status_error_clear: ReadWrite<u32>),
        (0x0008 => _reserved0008),
        (0x0018 => pub flag: ReadOnly<u32,Flag::Register>),
        (0x001c => _reserved001c),
        (0x0020 => pub irda_low_power_counter: ReadWrite<u32>),
        (0x0024 => pub integer_baudrate_divisor: ReadWrite<u32>),
        (0x0028 => pub fractional_baudrate_divisor: ReadWrite<u32>),
        (0x002c => pub line_control_h: ReadWrite<u32,LineControlH::Register>),
        (0x0030 => pub control: ReadWrite<u32,Control::Register>),
        (0x0034 => _reserved0034),
        (0x0038 => pub interrupt_mask_set_clear: ReadWrite<u32>),
        (0x003c => pub raw_interrupt_status: ReadOnly<u32>),
        (0x0040 => pub masked_interrupt_status: ReadOnly<u32>),
        (0x0044 => pub interrupt_clear: WriteOnly<u32>),
        (0x0048 => pub dma_control: ReadWrite<u32>),
        (0x004c => _reserved004c),
        (0x0fe0 => pub peripheral_id: [ReadOnly<u32>;4]),
        (0x0ff0 => pub pcell_id: [ReadOnly<u32>;4]),
        (0x1000 => @END),
    }
}

register_bitfields!(
    u32,
    pub Control [
        UARTEN  OFFSET(0) NUMBITS(1) [],
        TXE  OFFSET(8) NUMBITS(1) [],
        RXE  OFFSET(9) NUMBITS(1) [],
    ],
    pub LineControlH [
        WORD_LEN  OFFSET(5) NUMBITS(2) [
             BIT8 = 0b11,
             BIT7 = 0b10,
             BIT6 = 0b01,
             BIT5 = 0b00,
        ],
        ENABLE_FIFO OFFSET(4) NUMBITS(1) [],
    ],
    pub Flag [
        TX_EMPTY    OFFSET(7) NUMBITS(1) [],
        RX_FULL    OFFSET(6) NUMBITS(1) [],
        TX_FULL    OFFSET(5) NUMBITS(1) [],
        RX_EMPTY    OFFSET(4) NUMBITS(1) [],
        BUSY    OFFSET(3) NUMBITS(1) [],
    ]
);

impl Pl011UartPeripheral {
    pub unsafe fn new_ref(address: *mut u32) -> &'static mut Pl011UartPeripheral {
        let uart: &'static mut Pl011UartPeripheral =
            unsafe { &mut *(address as *mut Pl011UartPeripheral) };
        uart
    }
}

pub struct Pl011Uart {
    uart: &'static mut Pl011UartPeripheral,
}
impl Pl011Uart {
    pub fn new(address: *mut u32) -> Self {
        let uart = unsafe { Pl011UartPeripheral::new_ref(address) };
        Self { uart }
    }

    pub fn set_baudrate(&self, peripheral_clock_hz: u64, baudrate: u64) {
        let div_x1000 = peripheral_clock_hz * 1000 / 16 / baudrate;
        let integer_part = div_x1000 / 1000;
        let fractional_part = ((div_x1000 % 1000) * 64 + 500) / 1000;
        self.uart.integer_baudrate_divisor.set(integer_part as u32);
        self.uart
            .fractional_baudrate_divisor
            .set(fractional_part as u32);
    }

    pub fn init(&self, peripheral_clock_hz: u64, baudrate: u64) {
        self.disable();

        // set TX/RX enable
        self.uart
            .control
            .set((Control::TXE::SET + Control::RXE::SET).value);
        self.uart
            .line_control_h
            .set((LineControlH::ENABLE_FIFO::SET + LineControlH::WORD_LEN::BIT8).value);

        // disable(mask) all interrupts
        self.uart.interrupt_mask_set_clear.set(0);
        self.uart.interrupt_clear.set(0xFFFF_FFFF);

        self.set_baudrate(peripheral_clock_hz, baudrate);
    }

    pub fn enable(&self) {
        self.uart.control.modify(Control::UARTEN::SET);
    }

    pub fn disable(&self) {
        self.uart.control.modify(Control::UARTEN::CLEAR);
        // wait for end transmit
        while self.uart.flag.is_set(Flag::BUSY) {
            unsafe { asm!("nop") }
        }

        // flush FIFO
        self.uart
            .line_control_h
            .modify(LineControlH::ENABLE_FIFO::CLEAR);
    }

    pub fn write(&self, data: &[u8]) -> usize {
        for (i, d) in data.iter().enumerate() {
            if self.uart.flag.is_set(Flag::TX_FULL) {
                return i;
            }
            self.uart.data.set(*d as u32);
        }
        data.len()
    }

    pub fn read(&self, data: &mut [u8]) -> usize {
        for (i, d) in data.iter_mut().enumerate() {
            if self.uart.flag.is_set(Flag::RX_EMPTY) {
                return i;
            }
            *d = self.uart.data.get() as u8;
        }
        data.len()
    }
}
