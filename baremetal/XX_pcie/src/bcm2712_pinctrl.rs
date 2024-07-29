use tock_registers::registers::{ReadOnly, ReadWrite, WriteOnly};
use tock_registers::{register_bitfields, register_structs};

// called ,bcm2712-aon-pinctrl in linux
// drivers/pinctrl/bcm/pinctrl-bcm2712.c

// 0x10_7d51_0700: pinctrl_aon
register_structs! {
    pub Bcm2712AonPinControl {
        (0x00 => pub func_regs: [ReadWrite<u32>;6]),
        (0x10 => pub pull_config_regs: [ReadWrite<u32>;2]),
        (0x20 => @END),
    }
}
