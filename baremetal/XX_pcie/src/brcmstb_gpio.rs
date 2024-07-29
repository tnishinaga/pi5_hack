use tock_registers::registers::{ReadOnly, ReadWrite, WriteOnly};
use tock_registers::{register_bitfields, register_structs};

// called brcmstb-gpio in linux
// https://github.com/raspberrypi/linux/blob/5931a01e0017d719af578fac8475442668807a97/drivers/gpio/gpio-brcmstb.c

// gio_aon: gpio@7d517c00 (size 0x40)
// aon gpio mappad at 0x10_7d51_7c00
register_structs! {
    pub BrcmStbGpioBank {
        (0x00 => pub opendrain_enable: ReadOnly<u32>),
        (0x04 => pub data: ReadOnly<u32>),
        (0x08 => pub io_direction: ReadOnly<u32>),
        (0x0c => pub ec: ReadOnly<u32>),
        (0x10 => pub ei: ReadOnly<u32>),
        (0x14 => pub mask: ReadOnly<u32>),
        (0x18 => pub level: ReadOnly<u32>),
        (0x1c => pub stat: ReadOnly<u32>),
        (0x20 => @END),
    }
}
