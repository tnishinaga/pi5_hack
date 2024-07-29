use tock_registers::{
    interfaces::{ReadWriteable, Readable, Writeable},
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite, WriteOnly},
};

register_structs! {
    pub Rp1IoBank0 {
        (0x0000 => pub data: [(ReadOnly<u32,GpioStatus::Register>,ReadWrite<u32,GpioControl::Register>);28]),
        (0x00e0 => _reserved00e0),
        (0x0100 => pub raw_interrupt: ReadOnly<u32>),
        // (PROC*_INTE, PROC*_INTF, PROC*_INTS)
        (0x0104 => pub proc_interrupt: [(ReadWrite<u32>,ReadWrite<u32>,ReadOnly<u32>);2]),
        (0x011c => pub pcie: (ReadWrite<u32>,ReadWrite<u32>,ReadWrite<u32>)),
        (0x0128 => _reserved0128),
        (0x1000 => @END),
    }
}

register_bitfields!(
    u32,
    pub GpioStatus [
        IRQ_TO_PROC   OFFSET(29) NUMBITS(1) [],
        IRQ_COMBINED    OFFSET(28) NUMBITS(1) [],
        EVENT_DB_LEVEL_HIGH    OFFSET(27) NUMBITS(1) [],
        EVENT_DB_LEVEL_LOW    OFFSET(26) NUMBITS(1) [],
        EVENT_F_EDGE_HIGH    OFFSET(25) NUMBITS(1) [],
        EVENT_F_EDGE_LOW    OFFSET(24) NUMBITS(1) [],
        EVENT_LEVEL_HIGH    OFFSET(23) NUMBITS(1) [],
        EVENT_LEVEL_LOW    OFFSET(22) NUMBITS(1) [],
        EVENT_EDGE_HIGH    OFFSET(21) NUMBITS(1) [],
        EVENT_EDGE_LOW    OFFSET(20) NUMBITS(1) [],
        INT_TO_PERI    OFFSET(19) NUMBITS(1) [],
        INT_FILTERD    OFFSET(18) NUMBITS(1) [],
        IN_FROM_PAD    OFFSET(17) NUMBITS(1) [],
        IN_IS_DIRECT    OFFSET(16) NUMBITS(1) [],
        OE_TO_PAD       OFFSET(13) NUMBITS(1) [],
        OE_FROM_PERI       OFFSET(12) NUMBITS(1) [],
        OUT_TO_PAD       OFFSET(9) NUMBITS(1) [],
        OUT_FROM_PERI       OFFSET(8) NUMBITS(1) [],

    ],
    pub GpioControl [
        IRQ_OVER  OFFSET(30) NUMBITS(2) [],
        IRQ_RESET  OFFSET(28) NUMBITS(1) [],
        IRQ_MASK_DB_LEVEL_HIGH  OFFSET(27) NUMBITS(1) [],
        IRQ_MASK_DB_LEVEL_LOW  OFFSET(26) NUMBITS(1) [],
        IRQMASK_F_EDGE_HIGH  OFFSET(25) NUMBITS(1) [],
        IRQMASK_F_EDGE_LOW  OFFSET(24) NUMBITS(1) [],
        IRQMASK_LEVEL_HIGH  OFFSET(23) NUMBITS(1) [],
        IRQMASK_LEVEL_LOW  OFFSET(22) NUMBITS(1) [],
        IRQMASK_EDGE_HIGH  OFFSET(21) NUMBITS(1) [],
        IRQMASK_EDGE_LOW  OFFSET(20) NUMBITS(1) [],
        INPUT_OVER  OFFSET(16) NUMBITS(2) [
            DONT_INVERT_INPUT = 0,
            INVERT_INPUT = 1,
            DRIVE_INPUT_LOW = 2,
            DRIVE_INPUT_HIGH = 3,
        ],
        OUTPUT_ENABLE_OVER  OFFSET(14) NUMBITS(2) [
            ENABLE_OUTPUT_SIGNAL = 0,
            ENABLE_OUTPUT_SIGNAL_INVERT = 1,
            DISABLE_OUTPUT = 2,
            ENABLE_OUTPUT = 3,
        ],
        OUT_OVER  OFFSET(12) NUMBITS(2) [
            DRIVE_OUTPUT_SIGNAL = 0,
            DRIVE_OUTPUT_SIGNAL_INVERT = 1,
            DRIVE_OUTPUT_LOW = 2,
            DRIVE_OUTPUT_HIGH = 3,
        ],
        F_M OFFSET(5) NUMBITS(7) [],
        FUNCSEL OFFSET(0) NUMBITS(5) [],
    ],
);

impl Rp1IoBank0 {
    pub unsafe fn new_ref(address: *mut u32) -> &'static mut Rp1IoBank0 {
        let x: &'static mut Rp1IoBank0 = unsafe { &mut *(address as *mut Rp1IoBank0) };
        x
    }
}

register_structs! {
    pub Rp1PadsBank0 {
        (0x0000 => pub voltage_select: ReadOnly<u32,VoltageSelect::Register>),
        (0x0004 => pub gpio: [ReadWrite<u32,PadControl::Register>;28]),
        (0x0074 => _reserved0128),
        (0x1000 => @END),
    }
}

register_bitfields!(
    u32,
    pub VoltageSelect [
        VOLTAGE_SELECT OFFSET(0) NUMBITS(1) [
            VOLTAGE_3V3 = 0,
            VOLTAGE_1V8 = 1,
        ]
    ],
    pub PadControl [
        OUTPUT_DISABLE   OFFSET(7) NUMBITS(1) [],
        INPUT_ENABLE   OFFSET(6) NUMBITS(1) [],
        DRIVE_STRENGTH   OFFSET(4) NUMBITS(2) [
            STRENGTH_2mA = 0,
            STRENGTH_4mA = 1,
            STRENGTH_8mA = 2,
            STRENGTH_12mA = 3,
        ],
        PULLUP_ENABLE   OFFSET(3) NUMBITS(1) [],
        PULLDOWN_ENABLE   OFFSET(2) NUMBITS(1) [],
        SHUMITT_TRIGER_ENABLE   OFFSET(1) NUMBITS(1) [],
        SLEW_RATE   OFFSET(0) NUMBITS(1) [
            Fast = 1,
            Slow = 0,
        ],

    ],

);

impl Rp1PadsBank0 {
    pub unsafe fn new_ref(address: *mut u32) -> &'static mut Self {
        let x: &'static mut Self = unsafe { &mut *(address as *mut Self) };
        x
    }
}

pub struct RP1Gpio {
    iobank0: &'static mut Rp1IoBank0,
    padsbank0: &'static mut Rp1PadsBank0,
}

impl RP1Gpio {
    pub fn new(iobank_address: *mut u32, padsbank_address: *mut u32) -> Self {
        let iobank0 = unsafe { Rp1IoBank0::new_ref(iobank_address) };
        let padsbank0 = unsafe { Rp1PadsBank0::new_ref(padsbank_address) };
        Self { iobank0, padsbank0 }
    }

    pub fn set_function(&self, gpio_num: usize, function: u8) {
        match self.iobank0.data.get(gpio_num) {
            Some(gpio) => {
                // set gpio function
                gpio.1.modify(
                    GpioControl::FUNCSEL.val(u32::from(function))
                        + GpioControl::OUTPUT_ENABLE_OVER::ENABLE_OUTPUT_SIGNAL
                        + GpioControl::OUT_OVER::DRIVE_OUTPUT_SIGNAL,
                );
            }
            None => (),
        }
    }

    pub fn set_output_enable(&self, gpio_num: usize, output_enable: bool) {
        match self.padsbank0.gpio.get(gpio_num) {
            Some(gpio) => {
                // set gpio function
                if output_enable {
                    gpio.modify(PadControl::OUTPUT_DISABLE::CLEAR);
                } else {
                    gpio.modify(PadControl::OUTPUT_DISABLE::SET);
                }
            }
            None => (),
        }
    }
}
