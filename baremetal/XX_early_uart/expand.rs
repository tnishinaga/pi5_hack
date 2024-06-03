#![feature(prelude_import)]
#![no_std]
#![no_main]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
extern "C" {
    static mut _BSS_START: u64;
    static mut _BSS_END: u64;
}
mod critical_section {
    #[cfg(not(feature = "std"))]
    mod no_std {
        use critical_section::RawRestoreState;
        struct MyCriticalSection;
        #[no_mangle]
        unsafe fn _critical_section_1_0_acquire() -> ::critical_section::RawRestoreState {
            <MyCriticalSection as ::critical_section::Impl>::acquire()
        }
        #[no_mangle]
        unsafe fn _critical_section_1_0_release(
            restore_state: ::critical_section::RawRestoreState,
        ) {
            <MyCriticalSection as ::critical_section::Impl>::release(restore_state)
        }
        unsafe impl critical_section::Impl for MyCriticalSection {
            unsafe fn acquire() -> RawRestoreState {}
            unsafe fn release(_token: RawRestoreState) {}
        }
    }
}
mod pl011_uart {
    use tock_registers::{
        interfaces::{ReadWriteable, Readable, Writeable},
        register_bitfields, register_structs, registers::{ReadOnly, ReadWrite, WriteOnly},
    };
    #[repr(C)]
    pub struct Pl011UartPeripheral {
        pub data: ReadWrite<u32>,
        pub receive_status_error_clear: ReadWrite<u32>,
        _reserved0008: [u8; 0x0018 - 0x0008],
        pub flag: ReadOnly<u32, Flag::Register>,
        _reserved001c: [u8; 0x0020 - 0x001c],
        pub irda_low_power_counter: ReadWrite<u32>,
        pub integer_baudrate_divisor: ReadWrite<u32>,
        pub fractional_baudrate_divisor: ReadWrite<u32>,
        pub line_control_h: ReadWrite<u32, LineControlH::Register>,
        pub control: ReadWrite<u32, Control::Register>,
        _reserved0034: [u8; 0x0038 - 0x0034],
        pub interrupt_mask_set_clear: ReadWrite<u32>,
        pub raw_interrupt_status: ReadOnly<u32>,
        pub masked_interrupt_status: ReadOnly<u32>,
        pub interrupt_clear: WriteOnly<u32>,
        pub dma_control: ReadWrite<u32>,
        _reserved004c: [u8; 0x0fe0 - 0x004c],
        pub peripheral_id: [ReadOnly<u32>; 4],
        pub pcell_id: [ReadOnly<u32>; 4],
    }
    const _: () = {
        const SUM_MAX_ALIGN: (usize, usize) = {
            const SUM_MAX_ALIGN: (usize, usize) = {
                const SUM_MAX_ALIGN: (usize, usize) = {
                    const SUM_MAX_ALIGN: (usize, usize) = {
                        const SUM_MAX_ALIGN: (usize, usize) = {
                            const SUM_MAX_ALIGN: (usize, usize) = {
                                const SUM_MAX_ALIGN: (usize, usize) = {
                                    const SUM_MAX_ALIGN: (usize, usize) = {
                                        const SUM_MAX_ALIGN: (usize, usize) = {
                                            const SUM_MAX_ALIGN: (usize, usize) = {
                                                const SUM_MAX_ALIGN: (usize, usize) = {
                                                    const SUM_MAX_ALIGN: (usize, usize) = {
                                                        const SUM_MAX_ALIGN: (usize, usize) = {
                                                            const SUM_MAX_ALIGN: (usize, usize) = {
                                                                const SUM_MAX_ALIGN: (usize, usize) = {
                                                                    const SUM_MAX_ALIGN: (usize, usize) = {
                                                                        const SUM_MAX_ALIGN: (usize, usize) = {
                                                                            const SUM_MAX_ALIGN: (usize, usize) = {
                                                                                const SUM_MAX_ALIGN: (usize, usize) = {
                                                                                    const SUM_MAX_ALIGN: (usize, usize) = (0, 0);
                                                                                    const SUM: usize = SUM_MAX_ALIGN.0;
                                                                                    const MAX_ALIGN: usize = SUM_MAX_ALIGN.1;
                                                                                    if !(SUM == 0x0000) {
                                                                                        {
                                                                                            #[cold]
                                                                                            #[track_caller]
                                                                                            #[inline(never)]
                                                                                            #[rustc_const_panic_str]
                                                                                            #[rustc_do_not_const_check]
                                                                                            const fn panic_cold_display<T: ::core::fmt::Display>(
                                                                                                arg: &T,
                                                                                            ) -> ! {
                                                                                                ::core::panicking::panic_display(arg)
                                                                                            }
                                                                                            panic_cold_display(
                                                                                                &"Invalid start offset for field data (expected 0 but actual value differs)",
                                                                                            );
                                                                                        }
                                                                                    }
                                                                                    const ALIGN: usize = core::mem::align_of::<
                                                                                        ReadWrite<u32>,
                                                                                    >();
                                                                                    #[allow(clippy::bad_bit_mask)]
                                                                                    {
                                                                                        if !(SUM & (ALIGN - 1) == 0) {
                                                                                            {
                                                                                                #[cold]
                                                                                                #[track_caller]
                                                                                                #[inline(never)]
                                                                                                #[rustc_const_panic_str]
                                                                                                #[rustc_do_not_const_check]
                                                                                                const fn panic_cold_display<T: ::core::fmt::Display>(
                                                                                                    arg: &T,
                                                                                                ) -> ! {
                                                                                                    ::core::panicking::panic_display(arg)
                                                                                                }
                                                                                                panic_cold_display(
                                                                                                    &"Invalid alignment for field data (offset differs from expected)",
                                                                                                );
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    const NEW_SUM: usize = SUM
                                                                                        + core::mem::size_of::<ReadWrite<u32>>();
                                                                                    if !(NEW_SUM == 0x0004) {
                                                                                        {
                                                                                            #[cold]
                                                                                            #[track_caller]
                                                                                            #[inline(never)]
                                                                                            #[rustc_const_panic_str]
                                                                                            #[rustc_do_not_const_check]
                                                                                            const fn panic_cold_display<T: ::core::fmt::Display>(
                                                                                                arg: &T,
                                                                                            ) -> ! {
                                                                                                ::core::panicking::panic_display(arg)
                                                                                            }
                                                                                            panic_cold_display(
                                                                                                &"Invalid end offset for field data (expected 4 but actual value differs)",
                                                                                            );
                                                                                        }
                                                                                    }
                                                                                    const NEW_MAX_ALIGN: usize = if ALIGN > MAX_ALIGN {
                                                                                        ALIGN
                                                                                    } else {
                                                                                        MAX_ALIGN
                                                                                    };
                                                                                    (NEW_SUM, NEW_MAX_ALIGN)
                                                                                };
                                                                                const SUM: usize = SUM_MAX_ALIGN.0;
                                                                                const MAX_ALIGN: usize = SUM_MAX_ALIGN.1;
                                                                                if !(SUM == 0x0004) {
                                                                                    {
                                                                                        #[cold]
                                                                                        #[track_caller]
                                                                                        #[inline(never)]
                                                                                        #[rustc_const_panic_str]
                                                                                        #[rustc_do_not_const_check]
                                                                                        const fn panic_cold_display<T: ::core::fmt::Display>(
                                                                                            arg: &T,
                                                                                        ) -> ! {
                                                                                            ::core::panicking::panic_display(arg)
                                                                                        }
                                                                                        panic_cold_display(
                                                                                            &"Invalid start offset for field receive_status_error_clear (expected 4 but actual value differs)",
                                                                                        );
                                                                                    }
                                                                                }
                                                                                const ALIGN: usize = core::mem::align_of::<
                                                                                    ReadWrite<u32>,
                                                                                >();
                                                                                #[allow(clippy::bad_bit_mask)]
                                                                                {
                                                                                    if !(SUM & (ALIGN - 1) == 0) {
                                                                                        {
                                                                                            #[cold]
                                                                                            #[track_caller]
                                                                                            #[inline(never)]
                                                                                            #[rustc_const_panic_str]
                                                                                            #[rustc_do_not_const_check]
                                                                                            const fn panic_cold_display<T: ::core::fmt::Display>(
                                                                                                arg: &T,
                                                                                            ) -> ! {
                                                                                                ::core::panicking::panic_display(arg)
                                                                                            }
                                                                                            panic_cold_display(
                                                                                                &"Invalid alignment for field receive_status_error_clear (offset differs from expected)",
                                                                                            );
                                                                                        }
                                                                                    }
                                                                                }
                                                                                const NEW_SUM: usize = SUM
                                                                                    + core::mem::size_of::<ReadWrite<u32>>();
                                                                                if !(NEW_SUM == 0x0008) {
                                                                                    {
                                                                                        #[cold]
                                                                                        #[track_caller]
                                                                                        #[inline(never)]
                                                                                        #[rustc_const_panic_str]
                                                                                        #[rustc_do_not_const_check]
                                                                                        const fn panic_cold_display<T: ::core::fmt::Display>(
                                                                                            arg: &T,
                                                                                        ) -> ! {
                                                                                            ::core::panicking::panic_display(arg)
                                                                                        }
                                                                                        panic_cold_display(
                                                                                            &"Invalid end offset for field receive_status_error_clear (expected 8 but actual value differs)",
                                                                                        );
                                                                                    }
                                                                                }
                                                                                const NEW_MAX_ALIGN: usize = if ALIGN > MAX_ALIGN {
                                                                                    ALIGN
                                                                                } else {
                                                                                    MAX_ALIGN
                                                                                };
                                                                                (NEW_SUM, NEW_MAX_ALIGN)
                                                                            };
                                                                            const SUM: usize = SUM_MAX_ALIGN.0;
                                                                            const MAX_ALIGN: usize = SUM_MAX_ALIGN.1;
                                                                            if !(SUM == 0x0008) {
                                                                                {
                                                                                    ::core::panicking::panic_fmt(
                                                                                        format_args!(
                                                                                            "Invalid start offset for padding _reserved0008 (expected 8 but actual value differs)"
                                                                                        ),
                                                                                    );
                                                                                }
                                                                            }
                                                                            (0x0018, MAX_ALIGN)
                                                                        };
                                                                        const SUM: usize = SUM_MAX_ALIGN.0;
                                                                        const MAX_ALIGN: usize = SUM_MAX_ALIGN.1;
                                                                        if !(SUM == 0x0018) {
                                                                            {
                                                                                #[cold]
                                                                                #[track_caller]
                                                                                #[inline(never)]
                                                                                #[rustc_const_panic_str]
                                                                                #[rustc_do_not_const_check]
                                                                                const fn panic_cold_display<T: ::core::fmt::Display>(
                                                                                    arg: &T,
                                                                                ) -> ! {
                                                                                    ::core::panicking::panic_display(arg)
                                                                                }
                                                                                panic_cold_display(
                                                                                    &"Invalid start offset for field flag (expected 24 but actual value differs)",
                                                                                );
                                                                            }
                                                                        }
                                                                        const ALIGN: usize = core::mem::align_of::<
                                                                            ReadOnly<u32, Flag::Register>,
                                                                        >();
                                                                        #[allow(clippy::bad_bit_mask)]
                                                                        {
                                                                            if !(SUM & (ALIGN - 1) == 0) {
                                                                                {
                                                                                    #[cold]
                                                                                    #[track_caller]
                                                                                    #[inline(never)]
                                                                                    #[rustc_const_panic_str]
                                                                                    #[rustc_do_not_const_check]
                                                                                    const fn panic_cold_display<T: ::core::fmt::Display>(
                                                                                        arg: &T,
                                                                                    ) -> ! {
                                                                                        ::core::panicking::panic_display(arg)
                                                                                    }
                                                                                    panic_cold_display(
                                                                                        &"Invalid alignment for field flag (offset differs from expected)",
                                                                                    );
                                                                                }
                                                                            }
                                                                        }
                                                                        const NEW_SUM: usize = SUM
                                                                            + core::mem::size_of::<ReadOnly<u32, Flag::Register>>();
                                                                        if !(NEW_SUM == 0x001c) {
                                                                            {
                                                                                #[cold]
                                                                                #[track_caller]
                                                                                #[inline(never)]
                                                                                #[rustc_const_panic_str]
                                                                                #[rustc_do_not_const_check]
                                                                                const fn panic_cold_display<T: ::core::fmt::Display>(
                                                                                    arg: &T,
                                                                                ) -> ! {
                                                                                    ::core::panicking::panic_display(arg)
                                                                                }
                                                                                panic_cold_display(
                                                                                    &"Invalid end offset for field flag (expected 28 but actual value differs)",
                                                                                );
                                                                            }
                                                                        }
                                                                        const NEW_MAX_ALIGN: usize = if ALIGN > MAX_ALIGN {
                                                                            ALIGN
                                                                        } else {
                                                                            MAX_ALIGN
                                                                        };
                                                                        (NEW_SUM, NEW_MAX_ALIGN)
                                                                    };
                                                                    const SUM: usize = SUM_MAX_ALIGN.0;
                                                                    const MAX_ALIGN: usize = SUM_MAX_ALIGN.1;
                                                                    if !(SUM == 0x001c) {
                                                                        {
                                                                            ::core::panicking::panic_fmt(
                                                                                format_args!(
                                                                                    "Invalid start offset for padding _reserved001c (expected 28 but actual value differs)"
                                                                                ),
                                                                            );
                                                                        }
                                                                    }
                                                                    (0x0020, MAX_ALIGN)
                                                                };
                                                                const SUM: usize = SUM_MAX_ALIGN.0;
                                                                const MAX_ALIGN: usize = SUM_MAX_ALIGN.1;
                                                                if !(SUM == 0x0020) {
                                                                    {
                                                                        #[cold]
                                                                        #[track_caller]
                                                                        #[inline(never)]
                                                                        #[rustc_const_panic_str]
                                                                        #[rustc_do_not_const_check]
                                                                        const fn panic_cold_display<T: ::core::fmt::Display>(
                                                                            arg: &T,
                                                                        ) -> ! {
                                                                            ::core::panicking::panic_display(arg)
                                                                        }
                                                                        panic_cold_display(
                                                                            &"Invalid start offset for field irda_low_power_counter (expected 32 but actual value differs)",
                                                                        );
                                                                    }
                                                                }
                                                                const ALIGN: usize = core::mem::align_of::<
                                                                    ReadWrite<u32>,
                                                                >();
                                                                #[allow(clippy::bad_bit_mask)]
                                                                {
                                                                    if !(SUM & (ALIGN - 1) == 0) {
                                                                        {
                                                                            #[cold]
                                                                            #[track_caller]
                                                                            #[inline(never)]
                                                                            #[rustc_const_panic_str]
                                                                            #[rustc_do_not_const_check]
                                                                            const fn panic_cold_display<T: ::core::fmt::Display>(
                                                                                arg: &T,
                                                                            ) -> ! {
                                                                                ::core::panicking::panic_display(arg)
                                                                            }
                                                                            panic_cold_display(
                                                                                &"Invalid alignment for field irda_low_power_counter (offset differs from expected)",
                                                                            );
                                                                        }
                                                                    }
                                                                }
                                                                const NEW_SUM: usize = SUM
                                                                    + core::mem::size_of::<ReadWrite<u32>>();
                                                                if !(NEW_SUM == 0x0024) {
                                                                    {
                                                                        #[cold]
                                                                        #[track_caller]
                                                                        #[inline(never)]
                                                                        #[rustc_const_panic_str]
                                                                        #[rustc_do_not_const_check]
                                                                        const fn panic_cold_display<T: ::core::fmt::Display>(
                                                                            arg: &T,
                                                                        ) -> ! {
                                                                            ::core::panicking::panic_display(arg)
                                                                        }
                                                                        panic_cold_display(
                                                                            &"Invalid end offset for field irda_low_power_counter (expected 36 but actual value differs)",
                                                                        );
                                                                    }
                                                                }
                                                                const NEW_MAX_ALIGN: usize = if ALIGN > MAX_ALIGN {
                                                                    ALIGN
                                                                } else {
                                                                    MAX_ALIGN
                                                                };
                                                                (NEW_SUM, NEW_MAX_ALIGN)
                                                            };
                                                            const SUM: usize = SUM_MAX_ALIGN.0;
                                                            const MAX_ALIGN: usize = SUM_MAX_ALIGN.1;
                                                            if !(SUM == 0x0024) {
                                                                {
                                                                    #[cold]
                                                                    #[track_caller]
                                                                    #[inline(never)]
                                                                    #[rustc_const_panic_str]
                                                                    #[rustc_do_not_const_check]
                                                                    const fn panic_cold_display<T: ::core::fmt::Display>(
                                                                        arg: &T,
                                                                    ) -> ! {
                                                                        ::core::panicking::panic_display(arg)
                                                                    }
                                                                    panic_cold_display(
                                                                        &"Invalid start offset for field integer_baudrate_divisor (expected 36 but actual value differs)",
                                                                    );
                                                                }
                                                            }
                                                            const ALIGN: usize = core::mem::align_of::<
                                                                ReadWrite<u32>,
                                                            >();
                                                            #[allow(clippy::bad_bit_mask)]
                                                            {
                                                                if !(SUM & (ALIGN - 1) == 0) {
                                                                    {
                                                                        #[cold]
                                                                        #[track_caller]
                                                                        #[inline(never)]
                                                                        #[rustc_const_panic_str]
                                                                        #[rustc_do_not_const_check]
                                                                        const fn panic_cold_display<T: ::core::fmt::Display>(
                                                                            arg: &T,
                                                                        ) -> ! {
                                                                            ::core::panicking::panic_display(arg)
                                                                        }
                                                                        panic_cold_display(
                                                                            &"Invalid alignment for field integer_baudrate_divisor (offset differs from expected)",
                                                                        );
                                                                    }
                                                                }
                                                            }
                                                            const NEW_SUM: usize = SUM
                                                                + core::mem::size_of::<ReadWrite<u32>>();
                                                            if !(NEW_SUM == 0x0028) {
                                                                {
                                                                    #[cold]
                                                                    #[track_caller]
                                                                    #[inline(never)]
                                                                    #[rustc_const_panic_str]
                                                                    #[rustc_do_not_const_check]
                                                                    const fn panic_cold_display<T: ::core::fmt::Display>(
                                                                        arg: &T,
                                                                    ) -> ! {
                                                                        ::core::panicking::panic_display(arg)
                                                                    }
                                                                    panic_cold_display(
                                                                        &"Invalid end offset for field integer_baudrate_divisor (expected 40 but actual value differs)",
                                                                    );
                                                                }
                                                            }
                                                            const NEW_MAX_ALIGN: usize = if ALIGN > MAX_ALIGN {
                                                                ALIGN
                                                            } else {
                                                                MAX_ALIGN
                                                            };
                                                            (NEW_SUM, NEW_MAX_ALIGN)
                                                        };
                                                        const SUM: usize = SUM_MAX_ALIGN.0;
                                                        const MAX_ALIGN: usize = SUM_MAX_ALIGN.1;
                                                        if !(SUM == 0x0028) {
                                                            {
                                                                #[cold]
                                                                #[track_caller]
                                                                #[inline(never)]
                                                                #[rustc_const_panic_str]
                                                                #[rustc_do_not_const_check]
                                                                const fn panic_cold_display<T: ::core::fmt::Display>(
                                                                    arg: &T,
                                                                ) -> ! {
                                                                    ::core::panicking::panic_display(arg)
                                                                }
                                                                panic_cold_display(
                                                                    &"Invalid start offset for field fractional_baudrate_divisor (expected 40 but actual value differs)",
                                                                );
                                                            }
                                                        }
                                                        const ALIGN: usize = core::mem::align_of::<
                                                            ReadWrite<u32>,
                                                        >();
                                                        #[allow(clippy::bad_bit_mask)]
                                                        {
                                                            if !(SUM & (ALIGN - 1) == 0) {
                                                                {
                                                                    #[cold]
                                                                    #[track_caller]
                                                                    #[inline(never)]
                                                                    #[rustc_const_panic_str]
                                                                    #[rustc_do_not_const_check]
                                                                    const fn panic_cold_display<T: ::core::fmt::Display>(
                                                                        arg: &T,
                                                                    ) -> ! {
                                                                        ::core::panicking::panic_display(arg)
                                                                    }
                                                                    panic_cold_display(
                                                                        &"Invalid alignment for field fractional_baudrate_divisor (offset differs from expected)",
                                                                    );
                                                                }
                                                            }
                                                        }
                                                        const NEW_SUM: usize = SUM
                                                            + core::mem::size_of::<ReadWrite<u32>>();
                                                        if !(NEW_SUM == 0x002c) {
                                                            {
                                                                #[cold]
                                                                #[track_caller]
                                                                #[inline(never)]
                                                                #[rustc_const_panic_str]
                                                                #[rustc_do_not_const_check]
                                                                const fn panic_cold_display<T: ::core::fmt::Display>(
                                                                    arg: &T,
                                                                ) -> ! {
                                                                    ::core::panicking::panic_display(arg)
                                                                }
                                                                panic_cold_display(
                                                                    &"Invalid end offset for field fractional_baudrate_divisor (expected 44 but actual value differs)",
                                                                );
                                                            }
                                                        }
                                                        const NEW_MAX_ALIGN: usize = if ALIGN > MAX_ALIGN {
                                                            ALIGN
                                                        } else {
                                                            MAX_ALIGN
                                                        };
                                                        (NEW_SUM, NEW_MAX_ALIGN)
                                                    };
                                                    const SUM: usize = SUM_MAX_ALIGN.0;
                                                    const MAX_ALIGN: usize = SUM_MAX_ALIGN.1;
                                                    if !(SUM == 0x002c) {
                                                        {
                                                            #[cold]
                                                            #[track_caller]
                                                            #[inline(never)]
                                                            #[rustc_const_panic_str]
                                                            #[rustc_do_not_const_check]
                                                            const fn panic_cold_display<T: ::core::fmt::Display>(
                                                                arg: &T,
                                                            ) -> ! {
                                                                ::core::panicking::panic_display(arg)
                                                            }
                                                            panic_cold_display(
                                                                &"Invalid start offset for field line_control_h (expected 44 but actual value differs)",
                                                            );
                                                        }
                                                    }
                                                    const ALIGN: usize = core::mem::align_of::<
                                                        ReadWrite<u32, LineControlH::Register>,
                                                    >();
                                                    #[allow(clippy::bad_bit_mask)]
                                                    {
                                                        if !(SUM & (ALIGN - 1) == 0) {
                                                            {
                                                                #[cold]
                                                                #[track_caller]
                                                                #[inline(never)]
                                                                #[rustc_const_panic_str]
                                                                #[rustc_do_not_const_check]
                                                                const fn panic_cold_display<T: ::core::fmt::Display>(
                                                                    arg: &T,
                                                                ) -> ! {
                                                                    ::core::panicking::panic_display(arg)
                                                                }
                                                                panic_cold_display(
                                                                    &"Invalid alignment for field line_control_h (offset differs from expected)",
                                                                );
                                                            }
                                                        }
                                                    }
                                                    const NEW_SUM: usize = SUM
                                                        + core::mem::size_of::<
                                                            ReadWrite<u32, LineControlH::Register>,
                                                        >();
                                                    if !(NEW_SUM == 0x0030) {
                                                        {
                                                            #[cold]
                                                            #[track_caller]
                                                            #[inline(never)]
                                                            #[rustc_const_panic_str]
                                                            #[rustc_do_not_const_check]
                                                            const fn panic_cold_display<T: ::core::fmt::Display>(
                                                                arg: &T,
                                                            ) -> ! {
                                                                ::core::panicking::panic_display(arg)
                                                            }
                                                            panic_cold_display(
                                                                &"Invalid end offset for field line_control_h (expected 48 but actual value differs)",
                                                            );
                                                        }
                                                    }
                                                    const NEW_MAX_ALIGN: usize = if ALIGN > MAX_ALIGN {
                                                        ALIGN
                                                    } else {
                                                        MAX_ALIGN
                                                    };
                                                    (NEW_SUM, NEW_MAX_ALIGN)
                                                };
                                                const SUM: usize = SUM_MAX_ALIGN.0;
                                                const MAX_ALIGN: usize = SUM_MAX_ALIGN.1;
                                                if !(SUM == 0x0030) {
                                                    {
                                                        #[cold]
                                                        #[track_caller]
                                                        #[inline(never)]
                                                        #[rustc_const_panic_str]
                                                        #[rustc_do_not_const_check]
                                                        const fn panic_cold_display<T: ::core::fmt::Display>(
                                                            arg: &T,
                                                        ) -> ! {
                                                            ::core::panicking::panic_display(arg)
                                                        }
                                                        panic_cold_display(
                                                            &"Invalid start offset for field control (expected 48 but actual value differs)",
                                                        );
                                                    }
                                                }
                                                const ALIGN: usize = core::mem::align_of::<
                                                    ReadWrite<u32, Control::Register>,
                                                >();
                                                #[allow(clippy::bad_bit_mask)]
                                                {
                                                    if !(SUM & (ALIGN - 1) == 0) {
                                                        {
                                                            #[cold]
                                                            #[track_caller]
                                                            #[inline(never)]
                                                            #[rustc_const_panic_str]
                                                            #[rustc_do_not_const_check]
                                                            const fn panic_cold_display<T: ::core::fmt::Display>(
                                                                arg: &T,
                                                            ) -> ! {
                                                                ::core::panicking::panic_display(arg)
                                                            }
                                                            panic_cold_display(
                                                                &"Invalid alignment for field control (offset differs from expected)",
                                                            );
                                                        }
                                                    }
                                                }
                                                const NEW_SUM: usize = SUM
                                                    + core::mem::size_of::<ReadWrite<u32, Control::Register>>();
                                                if !(NEW_SUM == 0x0034) {
                                                    {
                                                        #[cold]
                                                        #[track_caller]
                                                        #[inline(never)]
                                                        #[rustc_const_panic_str]
                                                        #[rustc_do_not_const_check]
                                                        const fn panic_cold_display<T: ::core::fmt::Display>(
                                                            arg: &T,
                                                        ) -> ! {
                                                            ::core::panicking::panic_display(arg)
                                                        }
                                                        panic_cold_display(
                                                            &"Invalid end offset for field control (expected 52 but actual value differs)",
                                                        );
                                                    }
                                                }
                                                const NEW_MAX_ALIGN: usize = if ALIGN > MAX_ALIGN {
                                                    ALIGN
                                                } else {
                                                    MAX_ALIGN
                                                };
                                                (NEW_SUM, NEW_MAX_ALIGN)
                                            };
                                            const SUM: usize = SUM_MAX_ALIGN.0;
                                            const MAX_ALIGN: usize = SUM_MAX_ALIGN.1;
                                            if !(SUM == 0x0034) {
                                                {
                                                    ::core::panicking::panic_fmt(
                                                        format_args!(
                                                            "Invalid start offset for padding _reserved0034 (expected 52 but actual value differs)"
                                                        ),
                                                    );
                                                }
                                            }
                                            (0x0038, MAX_ALIGN)
                                        };
                                        const SUM: usize = SUM_MAX_ALIGN.0;
                                        const MAX_ALIGN: usize = SUM_MAX_ALIGN.1;
                                        if !(SUM == 0x0038) {
                                            {
                                                #[cold]
                                                #[track_caller]
                                                #[inline(never)]
                                                #[rustc_const_panic_str]
                                                #[rustc_do_not_const_check]
                                                const fn panic_cold_display<T: ::core::fmt::Display>(
                                                    arg: &T,
                                                ) -> ! {
                                                    ::core::panicking::panic_display(arg)
                                                }
                                                panic_cold_display(
                                                    &"Invalid start offset for field interrupt_mask_set_clear (expected 56 but actual value differs)",
                                                );
                                            }
                                        }
                                        const ALIGN: usize = core::mem::align_of::<
                                            ReadWrite<u32>,
                                        >();
                                        #[allow(clippy::bad_bit_mask)]
                                        {
                                            if !(SUM & (ALIGN - 1) == 0) {
                                                {
                                                    #[cold]
                                                    #[track_caller]
                                                    #[inline(never)]
                                                    #[rustc_const_panic_str]
                                                    #[rustc_do_not_const_check]
                                                    const fn panic_cold_display<T: ::core::fmt::Display>(
                                                        arg: &T,
                                                    ) -> ! {
                                                        ::core::panicking::panic_display(arg)
                                                    }
                                                    panic_cold_display(
                                                        &"Invalid alignment for field interrupt_mask_set_clear (offset differs from expected)",
                                                    );
                                                }
                                            }
                                        }
                                        const NEW_SUM: usize = SUM
                                            + core::mem::size_of::<ReadWrite<u32>>();
                                        if !(NEW_SUM == 0x003c) {
                                            {
                                                #[cold]
                                                #[track_caller]
                                                #[inline(never)]
                                                #[rustc_const_panic_str]
                                                #[rustc_do_not_const_check]
                                                const fn panic_cold_display<T: ::core::fmt::Display>(
                                                    arg: &T,
                                                ) -> ! {
                                                    ::core::panicking::panic_display(arg)
                                                }
                                                panic_cold_display(
                                                    &"Invalid end offset for field interrupt_mask_set_clear (expected 60 but actual value differs)",
                                                );
                                            }
                                        }
                                        const NEW_MAX_ALIGN: usize = if ALIGN > MAX_ALIGN {
                                            ALIGN
                                        } else {
                                            MAX_ALIGN
                                        };
                                        (NEW_SUM, NEW_MAX_ALIGN)
                                    };
                                    const SUM: usize = SUM_MAX_ALIGN.0;
                                    const MAX_ALIGN: usize = SUM_MAX_ALIGN.1;
                                    if !(SUM == 0x003c) {
                                        {
                                            #[cold]
                                            #[track_caller]
                                            #[inline(never)]
                                            #[rustc_const_panic_str]
                                            #[rustc_do_not_const_check]
                                            const fn panic_cold_display<T: ::core::fmt::Display>(
                                                arg: &T,
                                            ) -> ! {
                                                ::core::panicking::panic_display(arg)
                                            }
                                            panic_cold_display(
                                                &"Invalid start offset for field raw_interrupt_status (expected 60 but actual value differs)",
                                            );
                                        }
                                    }
                                    const ALIGN: usize = core::mem::align_of::<ReadOnly<u32>>();
                                    #[allow(clippy::bad_bit_mask)]
                                    {
                                        if !(SUM & (ALIGN - 1) == 0) {
                                            {
                                                #[cold]
                                                #[track_caller]
                                                #[inline(never)]
                                                #[rustc_const_panic_str]
                                                #[rustc_do_not_const_check]
                                                const fn panic_cold_display<T: ::core::fmt::Display>(
                                                    arg: &T,
                                                ) -> ! {
                                                    ::core::panicking::panic_display(arg)
                                                }
                                                panic_cold_display(
                                                    &"Invalid alignment for field raw_interrupt_status (offset differs from expected)",
                                                );
                                            }
                                        }
                                    }
                                    const NEW_SUM: usize = SUM
                                        + core::mem::size_of::<ReadOnly<u32>>();
                                    if !(NEW_SUM == 0x0040) {
                                        {
                                            #[cold]
                                            #[track_caller]
                                            #[inline(never)]
                                            #[rustc_const_panic_str]
                                            #[rustc_do_not_const_check]
                                            const fn panic_cold_display<T: ::core::fmt::Display>(
                                                arg: &T,
                                            ) -> ! {
                                                ::core::panicking::panic_display(arg)
                                            }
                                            panic_cold_display(
                                                &"Invalid end offset for field raw_interrupt_status (expected 64 but actual value differs)",
                                            );
                                        }
                                    }
                                    const NEW_MAX_ALIGN: usize = if ALIGN > MAX_ALIGN {
                                        ALIGN
                                    } else {
                                        MAX_ALIGN
                                    };
                                    (NEW_SUM, NEW_MAX_ALIGN)
                                };
                                const SUM: usize = SUM_MAX_ALIGN.0;
                                const MAX_ALIGN: usize = SUM_MAX_ALIGN.1;
                                if !(SUM == 0x0040) {
                                    {
                                        #[cold]
                                        #[track_caller]
                                        #[inline(never)]
                                        #[rustc_const_panic_str]
                                        #[rustc_do_not_const_check]
                                        const fn panic_cold_display<T: ::core::fmt::Display>(
                                            arg: &T,
                                        ) -> ! {
                                            ::core::panicking::panic_display(arg)
                                        }
                                        panic_cold_display(
                                            &"Invalid start offset for field masked_interrupt_status (expected 64 but actual value differs)",
                                        );
                                    }
                                }
                                const ALIGN: usize = core::mem::align_of::<ReadOnly<u32>>();
                                #[allow(clippy::bad_bit_mask)]
                                {
                                    if !(SUM & (ALIGN - 1) == 0) {
                                        {
                                            #[cold]
                                            #[track_caller]
                                            #[inline(never)]
                                            #[rustc_const_panic_str]
                                            #[rustc_do_not_const_check]
                                            const fn panic_cold_display<T: ::core::fmt::Display>(
                                                arg: &T,
                                            ) -> ! {
                                                ::core::panicking::panic_display(arg)
                                            }
                                            panic_cold_display(
                                                &"Invalid alignment for field masked_interrupt_status (offset differs from expected)",
                                            );
                                        }
                                    }
                                }
                                const NEW_SUM: usize = SUM
                                    + core::mem::size_of::<ReadOnly<u32>>();
                                if !(NEW_SUM == 0x0044) {
                                    {
                                        #[cold]
                                        #[track_caller]
                                        #[inline(never)]
                                        #[rustc_const_panic_str]
                                        #[rustc_do_not_const_check]
                                        const fn panic_cold_display<T: ::core::fmt::Display>(
                                            arg: &T,
                                        ) -> ! {
                                            ::core::panicking::panic_display(arg)
                                        }
                                        panic_cold_display(
                                            &"Invalid end offset for field masked_interrupt_status (expected 68 but actual value differs)",
                                        );
                                    }
                                }
                                const NEW_MAX_ALIGN: usize = if ALIGN > MAX_ALIGN {
                                    ALIGN
                                } else {
                                    MAX_ALIGN
                                };
                                (NEW_SUM, NEW_MAX_ALIGN)
                            };
                            const SUM: usize = SUM_MAX_ALIGN.0;
                            const MAX_ALIGN: usize = SUM_MAX_ALIGN.1;
                            if !(SUM == 0x0044) {
                                {
                                    #[cold]
                                    #[track_caller]
                                    #[inline(never)]
                                    #[rustc_const_panic_str]
                                    #[rustc_do_not_const_check]
                                    const fn panic_cold_display<T: ::core::fmt::Display>(
                                        arg: &T,
                                    ) -> ! {
                                        ::core::panicking::panic_display(arg)
                                    }
                                    panic_cold_display(
                                        &"Invalid start offset for field interrupt_clear (expected 68 but actual value differs)",
                                    );
                                }
                            }
                            const ALIGN: usize = core::mem::align_of::<WriteOnly<u32>>();
                            #[allow(clippy::bad_bit_mask)]
                            {
                                if !(SUM & (ALIGN - 1) == 0) {
                                    {
                                        #[cold]
                                        #[track_caller]
                                        #[inline(never)]
                                        #[rustc_const_panic_str]
                                        #[rustc_do_not_const_check]
                                        const fn panic_cold_display<T: ::core::fmt::Display>(
                                            arg: &T,
                                        ) -> ! {
                                            ::core::panicking::panic_display(arg)
                                        }
                                        panic_cold_display(
                                            &"Invalid alignment for field interrupt_clear (offset differs from expected)",
                                        );
                                    }
                                }
                            }
                            const NEW_SUM: usize = SUM
                                + core::mem::size_of::<WriteOnly<u32>>();
                            if !(NEW_SUM == 0x0048) {
                                {
                                    #[cold]
                                    #[track_caller]
                                    #[inline(never)]
                                    #[rustc_const_panic_str]
                                    #[rustc_do_not_const_check]
                                    const fn panic_cold_display<T: ::core::fmt::Display>(
                                        arg: &T,
                                    ) -> ! {
                                        ::core::panicking::panic_display(arg)
                                    }
                                    panic_cold_display(
                                        &"Invalid end offset for field interrupt_clear (expected 72 but actual value differs)",
                                    );
                                }
                            }
                            const NEW_MAX_ALIGN: usize = if ALIGN > MAX_ALIGN {
                                ALIGN
                            } else {
                                MAX_ALIGN
                            };
                            (NEW_SUM, NEW_MAX_ALIGN)
                        };
                        const SUM: usize = SUM_MAX_ALIGN.0;
                        const MAX_ALIGN: usize = SUM_MAX_ALIGN.1;
                        if !(SUM == 0x0048) {
                            {
                                #[cold]
                                #[track_caller]
                                #[inline(never)]
                                #[rustc_const_panic_str]
                                #[rustc_do_not_const_check]
                                const fn panic_cold_display<T: ::core::fmt::Display>(
                                    arg: &T,
                                ) -> ! {
                                    ::core::panicking::panic_display(arg)
                                }
                                panic_cold_display(
                                    &"Invalid start offset for field dma_control (expected 72 but actual value differs)",
                                );
                            }
                        }
                        const ALIGN: usize = core::mem::align_of::<ReadWrite<u32>>();
                        #[allow(clippy::bad_bit_mask)]
                        {
                            if !(SUM & (ALIGN - 1) == 0) {
                                {
                                    #[cold]
                                    #[track_caller]
                                    #[inline(never)]
                                    #[rustc_const_panic_str]
                                    #[rustc_do_not_const_check]
                                    const fn panic_cold_display<T: ::core::fmt::Display>(
                                        arg: &T,
                                    ) -> ! {
                                        ::core::panicking::panic_display(arg)
                                    }
                                    panic_cold_display(
                                        &"Invalid alignment for field dma_control (offset differs from expected)",
                                    );
                                }
                            }
                        }
                        const NEW_SUM: usize = SUM
                            + core::mem::size_of::<ReadWrite<u32>>();
                        if !(NEW_SUM == 0x004c) {
                            {
                                #[cold]
                                #[track_caller]
                                #[inline(never)]
                                #[rustc_const_panic_str]
                                #[rustc_do_not_const_check]
                                const fn panic_cold_display<T: ::core::fmt::Display>(
                                    arg: &T,
                                ) -> ! {
                                    ::core::panicking::panic_display(arg)
                                }
                                panic_cold_display(
                                    &"Invalid end offset for field dma_control (expected 76 but actual value differs)",
                                );
                            }
                        }
                        const NEW_MAX_ALIGN: usize = if ALIGN > MAX_ALIGN {
                            ALIGN
                        } else {
                            MAX_ALIGN
                        };
                        (NEW_SUM, NEW_MAX_ALIGN)
                    };
                    const SUM: usize = SUM_MAX_ALIGN.0;
                    const MAX_ALIGN: usize = SUM_MAX_ALIGN.1;
                    if !(SUM == 0x004c) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "Invalid start offset for padding _reserved004c (expected 76 but actual value differs)"
                                ),
                            );
                        }
                    }
                    (0x0fe0, MAX_ALIGN)
                };
                const SUM: usize = SUM_MAX_ALIGN.0;
                const MAX_ALIGN: usize = SUM_MAX_ALIGN.1;
                if !(SUM == 0x0fe0) {
                    {
                        #[cold]
                        #[track_caller]
                        #[inline(never)]
                        #[rustc_const_panic_str]
                        #[rustc_do_not_const_check]
                        const fn panic_cold_display<T: ::core::fmt::Display>(
                            arg: &T,
                        ) -> ! {
                            ::core::panicking::panic_display(arg)
                        }
                        panic_cold_display(
                            &"Invalid start offset for field peripheral_id (expected 4064 but actual value differs)",
                        );
                    }
                }
                const ALIGN: usize = core::mem::align_of::<[ReadOnly<u32>; 4]>();
                #[allow(clippy::bad_bit_mask)]
                {
                    if !(SUM & (ALIGN - 1) == 0) {
                        {
                            #[cold]
                            #[track_caller]
                            #[inline(never)]
                            #[rustc_const_panic_str]
                            #[rustc_do_not_const_check]
                            const fn panic_cold_display<T: ::core::fmt::Display>(
                                arg: &T,
                            ) -> ! {
                                ::core::panicking::panic_display(arg)
                            }
                            panic_cold_display(
                                &"Invalid alignment for field peripheral_id (offset differs from expected)",
                            );
                        }
                    }
                }
                const NEW_SUM: usize = SUM + core::mem::size_of::<[ReadOnly<u32>; 4]>();
                if !(NEW_SUM == 0x0ff0) {
                    {
                        #[cold]
                        #[track_caller]
                        #[inline(never)]
                        #[rustc_const_panic_str]
                        #[rustc_do_not_const_check]
                        const fn panic_cold_display<T: ::core::fmt::Display>(
                            arg: &T,
                        ) -> ! {
                            ::core::panicking::panic_display(arg)
                        }
                        panic_cold_display(
                            &"Invalid end offset for field peripheral_id (expected 4080 but actual value differs)",
                        );
                    }
                }
                const NEW_MAX_ALIGN: usize = if ALIGN > MAX_ALIGN {
                    ALIGN
                } else {
                    MAX_ALIGN
                };
                (NEW_SUM, NEW_MAX_ALIGN)
            };
            const SUM: usize = SUM_MAX_ALIGN.0;
            const MAX_ALIGN: usize = SUM_MAX_ALIGN.1;
            if !(SUM == 0x0ff0) {
                {
                    #[cold]
                    #[track_caller]
                    #[inline(never)]
                    #[rustc_const_panic_str]
                    #[rustc_do_not_const_check]
                    const fn panic_cold_display<T: ::core::fmt::Display>(arg: &T) -> ! {
                        ::core::panicking::panic_display(arg)
                    }
                    panic_cold_display(
                        &"Invalid start offset for field pcell_id (expected 4080 but actual value differs)",
                    );
                }
            }
            const ALIGN: usize = core::mem::align_of::<[ReadOnly<u32>; 4]>();
            #[allow(clippy::bad_bit_mask)]
            {
                if !(SUM & (ALIGN - 1) == 0) {
                    {
                        #[cold]
                        #[track_caller]
                        #[inline(never)]
                        #[rustc_const_panic_str]
                        #[rustc_do_not_const_check]
                        const fn panic_cold_display<T: ::core::fmt::Display>(
                            arg: &T,
                        ) -> ! {
                            ::core::panicking::panic_display(arg)
                        }
                        panic_cold_display(
                            &"Invalid alignment for field pcell_id (offset differs from expected)",
                        );
                    }
                }
            }
            const NEW_SUM: usize = SUM + core::mem::size_of::<[ReadOnly<u32>; 4]>();
            if !(NEW_SUM == 0x1000) {
                {
                    #[cold]
                    #[track_caller]
                    #[inline(never)]
                    #[rustc_const_panic_str]
                    #[rustc_do_not_const_check]
                    const fn panic_cold_display<T: ::core::fmt::Display>(arg: &T) -> ! {
                        ::core::panicking::panic_display(arg)
                    }
                    panic_cold_display(
                        &"Invalid end offset for field pcell_id (expected 4096 but actual value differs)",
                    );
                }
            }
            const NEW_MAX_ALIGN: usize = if ALIGN > MAX_ALIGN {
                ALIGN
            } else {
                MAX_ALIGN
            };
            (NEW_SUM, NEW_MAX_ALIGN)
        };
        const SUM: usize = SUM_MAX_ALIGN.0;
        const MAX_ALIGN: usize = SUM_MAX_ALIGN.1;
        if !(SUM == 0x1000) {
            ::core::panicking::panic("assertion failed: SUM == 0x1000")
        }
        const STRUCT_SIZE: usize = core::mem::size_of::<Pl011UartPeripheral>();
        const ALIGNMENT_CORRECTED_SIZE: usize = if 0x1000 % MAX_ALIGN != 0 {
            0x1000 + (MAX_ALIGN - (0x1000 % MAX_ALIGN))
        } else {
            0x1000
        };
        if !(STRUCT_SIZE == ALIGNMENT_CORRECTED_SIZE) {
            {
                #[cold]
                #[track_caller]
                #[inline(never)]
                #[rustc_const_panic_str]
                #[rustc_do_not_const_check]
                const fn panic_cold_display<T: ::core::fmt::Display>(arg: &T) -> ! {
                    ::core::panicking::panic_display(arg)
                }
                panic_cold_display(
                    &"Invalid size for struct Pl011UartPeripheral (expected 4096, actual struct size differs)",
                );
            }
        }
    };
    #[allow(non_snake_case)]
    pub mod Control {
        pub struct Register;
        #[automatically_derived]
        impl ::core::clone::Clone for Register {
            #[inline]
            fn clone(&self) -> Register {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Register {}
        impl ::tock_registers::RegisterLongName for Register {}
        use ::tock_registers::fields::Field;
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const UARTEN: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 0);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod UARTEN {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{FieldValue, TryFromValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
                0,
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 0, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {}
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(_v: u32) -> Option<Self::EnumType> {
                    Option::None
                }
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const TXE: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 8);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod TXE {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{FieldValue, TryFromValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
                8,
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 8, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {}
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(_v: u32) -> Option<Self::EnumType> {
                    Option::None
                }
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const RXE: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 9);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod RXE {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{FieldValue, TryFromValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
                9,
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 9, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {}
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(_v: u32) -> Option<Self::EnumType> {
                    Option::None
                }
            }
        }
    }
    #[allow(non_snake_case)]
    pub mod LineControlH {
        pub struct Register;
        #[automatically_derived]
        impl ::core::clone::Clone for Register {
            #[inline]
            fn clone(&self) -> Register {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Register {}
        impl ::tock_registers::RegisterLongName for Register {}
        use ::tock_registers::fields::Field;
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const WORD_LEN: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (2 - 1)) + ((1 << (2 - 1)) - 1), 5);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod WORD_LEN {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{TryFromValue, FieldValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const BIT8: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (2 - 1)) + ((1 << (2 - 1)) - 1), 5, 0b11);
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const BIT7: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (2 - 1)) + ((1 << (2 - 1)) - 1), 5, 0b10);
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const BIT6: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (2 - 1)) + ((1 << (2 - 1)) - 1), 5, 0b01);
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const BIT5: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (2 - 1)) + ((1 << (2 - 1)) - 1), 5, 0b00);
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (2 - 1)) + ((1 << (2 - 1)) - 1),
                5,
                (1 << (2 - 1)) + ((1 << (2 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (2 - 1)) + ((1 << (2 - 1)) - 1), 5, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            #[repr(u32)]
            pub enum Value {
                BIT8 = 0b11,
                BIT7 = 0b10,
                BIT6 = 0b01,
                BIT5 = 0b00,
            }
            #[automatically_derived]
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            impl ::core::marker::Copy for Value {}
            #[automatically_derived]
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            impl ::core::clone::Clone for Value {
                #[inline]
                fn clone(&self) -> Value {
                    *self
                }
            }
            #[automatically_derived]
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            impl ::core::fmt::Debug for Value {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            Value::BIT8 => "BIT8",
                            Value::BIT7 => "BIT7",
                            Value::BIT6 => "BIT6",
                            Value::BIT5 => "BIT5",
                        },
                    )
                }
            }
            #[automatically_derived]
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            impl ::core::cmp::Eq for Value {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {}
            }
            #[automatically_derived]
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            impl ::core::marker::StructuralPartialEq for Value {}
            #[automatically_derived]
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            impl ::core::cmp::PartialEq for Value {
                #[inline]
                fn eq(&self, other: &Value) -> bool {
                    let __self_tag = ::core::intrinsics::discriminant_value(self);
                    let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                    __self_tag == __arg1_tag
                }
            }
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(v: u32) -> Option<Self::EnumType> {
                    match v {
                        x if x == Value::BIT8 as u32 => Some(Value::BIT8),
                        x if x == Value::BIT7 as u32 => Some(Value::BIT7),
                        x if x == Value::BIT6 as u32 => Some(Value::BIT6),
                        x if x == Value::BIT5 as u32 => Some(Value::BIT5),
                        _ => Option::None,
                    }
                }
            }
            impl From<Value> for FieldValue<u32, Register> {
                fn from(v: Value) -> Self {
                    Self::new((1 << (2 - 1)) + ((1 << (2 - 1)) - 1), 5, v as u32)
                }
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const ENABLE_FIFO: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 4);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod ENABLE_FIFO {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{FieldValue, TryFromValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
                4,
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 4, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {}
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(_v: u32) -> Option<Self::EnumType> {
                    Option::None
                }
            }
        }
    }
    #[allow(non_snake_case)]
    pub mod Flag {
        pub struct Register;
        #[automatically_derived]
        impl ::core::clone::Clone for Register {
            #[inline]
            fn clone(&self) -> Register {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Register {}
        impl ::tock_registers::RegisterLongName for Register {}
        use ::tock_registers::fields::Field;
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const TX_EMPTY: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 7);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod TX_EMPTY {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{FieldValue, TryFromValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
                7,
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 7, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {}
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(_v: u32) -> Option<Self::EnumType> {
                    Option::None
                }
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const RX_FULL: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 6);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod RX_FULL {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{FieldValue, TryFromValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
                6,
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 6, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {}
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(_v: u32) -> Option<Self::EnumType> {
                    Option::None
                }
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const TX_FULL: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 5);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod TX_FULL {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{FieldValue, TryFromValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
                5,
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 5, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {}
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(_v: u32) -> Option<Self::EnumType> {
                    Option::None
                }
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const RX_EMPTY: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 4);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod RX_EMPTY {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{FieldValue, TryFromValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
                4,
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 4, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {}
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(_v: u32) -> Option<Self::EnumType> {
                    Option::None
                }
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const BUSY: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 3);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod BUSY {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{FieldValue, TryFromValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
                3,
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 3, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {}
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(_v: u32) -> Option<Self::EnumType> {
                    Option::None
                }
            }
        }
    }
    impl Pl011UartPeripheral {
        pub unsafe fn new_ref(address: *mut u32) -> &'static mut Pl011UartPeripheral {
            let uart: &'static mut Pl011UartPeripheral = unsafe {
                &mut *(address as *mut Pl011UartPeripheral)
            };
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
            self.uart.fractional_baudrate_divisor.set(fractional_part as u32);
        }
        pub fn init(&self, peripheral_clock_hz: u64, baudrate: u64) {
            self.disable();
            self.uart.control.set((Control::TXE::SET + Control::RXE::SET).value);
            self.uart
                .line_control_h
                .set(
                    (LineControlH::ENABLE_FIFO::SET + LineControlH::WORD_LEN::BIT8).value,
                );
            self.uart.interrupt_mask_set_clear.set(0);
            self.uart.interrupt_clear.set(0xFFFF_FFFF);
            self.set_baudrate(peripheral_clock_hz, baudrate);
        }
        pub fn enable(&self) {
            self.uart.control.modify(Control::UARTEN::SET);
        }
        pub fn disable(&self) {
            self.uart.control.modify(Control::UARTEN::CLEAR);
            self.uart.line_control_h.modify(LineControlH::ENABLE_FIFO::CLEAR);
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
}
mod rp1_gpio {
    use tock_registers::{
        interfaces::{ReadWriteable, Readable, Writeable},
        register_bitfields, register_structs, registers::{ReadOnly, ReadWrite, WriteOnly},
    };
    #[repr(C)]
    pub struct Rp1IoBank0 {
        pub data: [(
            ReadOnly<u32, GpioStatus::Register>,
            ReadWrite<u32, GpioControl::Register>,
        ); 28],
        _reserved00e0: [u8; 0x0100 - 0x00e0],
        pub raw_interrupt: ReadOnly<u32>,
        pub proc_interrupt: [(ReadWrite<u32>, ReadWrite<u32>, ReadOnly<u32>); 2],
        pub pcie: (ReadWrite<u32>, ReadWrite<u32>, ReadWrite<u32>),
        _reserved0128: [u8; 0x1000 - 0x0128],
    }
    const _: () = {
        const SUM_MAX_ALIGN: (usize, usize) = {
            const SUM_MAX_ALIGN: (usize, usize) = {
                const SUM_MAX_ALIGN: (usize, usize) = {
                    const SUM_MAX_ALIGN: (usize, usize) = {
                        const SUM_MAX_ALIGN: (usize, usize) = {
                            const SUM_MAX_ALIGN: (usize, usize) = {
                                const SUM_MAX_ALIGN: (usize, usize) = (0, 0);
                                const SUM: usize = SUM_MAX_ALIGN.0;
                                const MAX_ALIGN: usize = SUM_MAX_ALIGN.1;
                                if !(SUM == 0x0000) {
                                    {
                                        #[cold]
                                        #[track_caller]
                                        #[inline(never)]
                                        #[rustc_const_panic_str]
                                        #[rustc_do_not_const_check]
                                        const fn panic_cold_display<T: ::core::fmt::Display>(
                                            arg: &T,
                                        ) -> ! {
                                            ::core::panicking::panic_display(arg)
                                        }
                                        panic_cold_display(
                                            &"Invalid start offset for field data (expected 0 but actual value differs)",
                                        );
                                    }
                                }
                                const ALIGN: usize = core::mem::align_of::<
                                    [(
                                        ReadOnly<u32, GpioStatus::Register>,
                                        ReadWrite<u32, GpioControl::Register>,
                                    ); 28],
                                >();
                                #[allow(clippy::bad_bit_mask)]
                                {
                                    if !(SUM & (ALIGN - 1) == 0) {
                                        {
                                            #[cold]
                                            #[track_caller]
                                            #[inline(never)]
                                            #[rustc_const_panic_str]
                                            #[rustc_do_not_const_check]
                                            const fn panic_cold_display<T: ::core::fmt::Display>(
                                                arg: &T,
                                            ) -> ! {
                                                ::core::panicking::panic_display(arg)
                                            }
                                            panic_cold_display(
                                                &"Invalid alignment for field data (offset differs from expected)",
                                            );
                                        }
                                    }
                                }
                                const NEW_SUM: usize = SUM
                                    + core::mem::size_of::<
                                        [(
                                            ReadOnly<u32, GpioStatus::Register>,
                                            ReadWrite<u32, GpioControl::Register>,
                                        ); 28],
                                    >();
                                if !(NEW_SUM == 0x00e0) {
                                    {
                                        #[cold]
                                        #[track_caller]
                                        #[inline(never)]
                                        #[rustc_const_panic_str]
                                        #[rustc_do_not_const_check]
                                        const fn panic_cold_display<T: ::core::fmt::Display>(
                                            arg: &T,
                                        ) -> ! {
                                            ::core::panicking::panic_display(arg)
                                        }
                                        panic_cold_display(
                                            &"Invalid end offset for field data (expected 224 but actual value differs)",
                                        );
                                    }
                                }
                                const NEW_MAX_ALIGN: usize = if ALIGN > MAX_ALIGN {
                                    ALIGN
                                } else {
                                    MAX_ALIGN
                                };
                                (NEW_SUM, NEW_MAX_ALIGN)
                            };
                            const SUM: usize = SUM_MAX_ALIGN.0;
                            const MAX_ALIGN: usize = SUM_MAX_ALIGN.1;
                            if !(SUM == 0x00e0) {
                                {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "Invalid start offset for padding _reserved00e0 (expected 224 but actual value differs)"
                                        ),
                                    );
                                }
                            }
                            (0x0100, MAX_ALIGN)
                        };
                        const SUM: usize = SUM_MAX_ALIGN.0;
                        const MAX_ALIGN: usize = SUM_MAX_ALIGN.1;
                        if !(SUM == 0x0100) {
                            {
                                #[cold]
                                #[track_caller]
                                #[inline(never)]
                                #[rustc_const_panic_str]
                                #[rustc_do_not_const_check]
                                const fn panic_cold_display<T: ::core::fmt::Display>(
                                    arg: &T,
                                ) -> ! {
                                    ::core::panicking::panic_display(arg)
                                }
                                panic_cold_display(
                                    &"Invalid start offset for field raw_interrupt (expected 256 but actual value differs)",
                                );
                            }
                        }
                        const ALIGN: usize = core::mem::align_of::<ReadOnly<u32>>();
                        #[allow(clippy::bad_bit_mask)]
                        {
                            if !(SUM & (ALIGN - 1) == 0) {
                                {
                                    #[cold]
                                    #[track_caller]
                                    #[inline(never)]
                                    #[rustc_const_panic_str]
                                    #[rustc_do_not_const_check]
                                    const fn panic_cold_display<T: ::core::fmt::Display>(
                                        arg: &T,
                                    ) -> ! {
                                        ::core::panicking::panic_display(arg)
                                    }
                                    panic_cold_display(
                                        &"Invalid alignment for field raw_interrupt (offset differs from expected)",
                                    );
                                }
                            }
                        }
                        const NEW_SUM: usize = SUM
                            + core::mem::size_of::<ReadOnly<u32>>();
                        if !(NEW_SUM == 0x0104) {
                            {
                                #[cold]
                                #[track_caller]
                                #[inline(never)]
                                #[rustc_const_panic_str]
                                #[rustc_do_not_const_check]
                                const fn panic_cold_display<T: ::core::fmt::Display>(
                                    arg: &T,
                                ) -> ! {
                                    ::core::panicking::panic_display(arg)
                                }
                                panic_cold_display(
                                    &"Invalid end offset for field raw_interrupt (expected 260 but actual value differs)",
                                );
                            }
                        }
                        const NEW_MAX_ALIGN: usize = if ALIGN > MAX_ALIGN {
                            ALIGN
                        } else {
                            MAX_ALIGN
                        };
                        (NEW_SUM, NEW_MAX_ALIGN)
                    };
                    const SUM: usize = SUM_MAX_ALIGN.0;
                    const MAX_ALIGN: usize = SUM_MAX_ALIGN.1;
                    if !(SUM == 0x0104) {
                        {
                            #[cold]
                            #[track_caller]
                            #[inline(never)]
                            #[rustc_const_panic_str]
                            #[rustc_do_not_const_check]
                            const fn panic_cold_display<T: ::core::fmt::Display>(
                                arg: &T,
                            ) -> ! {
                                ::core::panicking::panic_display(arg)
                            }
                            panic_cold_display(
                                &"Invalid start offset for field proc_interrupt (expected 260 but actual value differs)",
                            );
                        }
                    }
                    const ALIGN: usize = core::mem::align_of::<
                        [(ReadWrite<u32>, ReadWrite<u32>, ReadOnly<u32>); 2],
                    >();
                    #[allow(clippy::bad_bit_mask)]
                    {
                        if !(SUM & (ALIGN - 1) == 0) {
                            {
                                #[cold]
                                #[track_caller]
                                #[inline(never)]
                                #[rustc_const_panic_str]
                                #[rustc_do_not_const_check]
                                const fn panic_cold_display<T: ::core::fmt::Display>(
                                    arg: &T,
                                ) -> ! {
                                    ::core::panicking::panic_display(arg)
                                }
                                panic_cold_display(
                                    &"Invalid alignment for field proc_interrupt (offset differs from expected)",
                                );
                            }
                        }
                    }
                    const NEW_SUM: usize = SUM
                        + core::mem::size_of::<
                            [(ReadWrite<u32>, ReadWrite<u32>, ReadOnly<u32>); 2],
                        >();
                    if !(NEW_SUM == 0x011c) {
                        {
                            #[cold]
                            #[track_caller]
                            #[inline(never)]
                            #[rustc_const_panic_str]
                            #[rustc_do_not_const_check]
                            const fn panic_cold_display<T: ::core::fmt::Display>(
                                arg: &T,
                            ) -> ! {
                                ::core::panicking::panic_display(arg)
                            }
                            panic_cold_display(
                                &"Invalid end offset for field proc_interrupt (expected 284 but actual value differs)",
                            );
                        }
                    }
                    const NEW_MAX_ALIGN: usize = if ALIGN > MAX_ALIGN {
                        ALIGN
                    } else {
                        MAX_ALIGN
                    };
                    (NEW_SUM, NEW_MAX_ALIGN)
                };
                const SUM: usize = SUM_MAX_ALIGN.0;
                const MAX_ALIGN: usize = SUM_MAX_ALIGN.1;
                if !(SUM == 0x011c) {
                    {
                        #[cold]
                        #[track_caller]
                        #[inline(never)]
                        #[rustc_const_panic_str]
                        #[rustc_do_not_const_check]
                        const fn panic_cold_display<T: ::core::fmt::Display>(
                            arg: &T,
                        ) -> ! {
                            ::core::panicking::panic_display(arg)
                        }
                        panic_cold_display(
                            &"Invalid start offset for field pcie (expected 284 but actual value differs)",
                        );
                    }
                }
                const ALIGN: usize = core::mem::align_of::<
                    (ReadWrite<u32>, ReadWrite<u32>, ReadWrite<u32>),
                >();
                #[allow(clippy::bad_bit_mask)]
                {
                    if !(SUM & (ALIGN - 1) == 0) {
                        {
                            #[cold]
                            #[track_caller]
                            #[inline(never)]
                            #[rustc_const_panic_str]
                            #[rustc_do_not_const_check]
                            const fn panic_cold_display<T: ::core::fmt::Display>(
                                arg: &T,
                            ) -> ! {
                                ::core::panicking::panic_display(arg)
                            }
                            panic_cold_display(
                                &"Invalid alignment for field pcie (offset differs from expected)",
                            );
                        }
                    }
                }
                const NEW_SUM: usize = SUM
                    + core::mem::size_of::<
                        (ReadWrite<u32>, ReadWrite<u32>, ReadWrite<u32>),
                    >();
                if !(NEW_SUM == 0x0128) {
                    {
                        #[cold]
                        #[track_caller]
                        #[inline(never)]
                        #[rustc_const_panic_str]
                        #[rustc_do_not_const_check]
                        const fn panic_cold_display<T: ::core::fmt::Display>(
                            arg: &T,
                        ) -> ! {
                            ::core::panicking::panic_display(arg)
                        }
                        panic_cold_display(
                            &"Invalid end offset for field pcie (expected 296 but actual value differs)",
                        );
                    }
                }
                const NEW_MAX_ALIGN: usize = if ALIGN > MAX_ALIGN {
                    ALIGN
                } else {
                    MAX_ALIGN
                };
                (NEW_SUM, NEW_MAX_ALIGN)
            };
            const SUM: usize = SUM_MAX_ALIGN.0;
            const MAX_ALIGN: usize = SUM_MAX_ALIGN.1;
            if !(SUM == 0x0128) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "Invalid start offset for padding _reserved0128 (expected 296 but actual value differs)"
                        ),
                    );
                }
            }
            (0x1000, MAX_ALIGN)
        };
        const SUM: usize = SUM_MAX_ALIGN.0;
        const MAX_ALIGN: usize = SUM_MAX_ALIGN.1;
        if !(SUM == 0x1000) {
            ::core::panicking::panic("assertion failed: SUM == 0x1000")
        }
        const STRUCT_SIZE: usize = core::mem::size_of::<Rp1IoBank0>();
        const ALIGNMENT_CORRECTED_SIZE: usize = if 0x1000 % MAX_ALIGN != 0 {
            0x1000 + (MAX_ALIGN - (0x1000 % MAX_ALIGN))
        } else {
            0x1000
        };
        if !(STRUCT_SIZE == ALIGNMENT_CORRECTED_SIZE) {
            {
                #[cold]
                #[track_caller]
                #[inline(never)]
                #[rustc_const_panic_str]
                #[rustc_do_not_const_check]
                const fn panic_cold_display<T: ::core::fmt::Display>(arg: &T) -> ! {
                    ::core::panicking::panic_display(arg)
                }
                panic_cold_display(
                    &"Invalid size for struct Rp1IoBank0 (expected 4096, actual struct size differs)",
                );
            }
        }
    };
    #[allow(non_snake_case)]
    pub mod GpioStatus {
        pub struct Register;
        #[automatically_derived]
        impl ::core::clone::Clone for Register {
            #[inline]
            fn clone(&self) -> Register {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Register {}
        impl ::tock_registers::RegisterLongName for Register {}
        use ::tock_registers::fields::Field;
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const IRQ_TO_PROC: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 29);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod IRQ_TO_PROC {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{FieldValue, TryFromValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
                29,
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 29, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {}
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(_v: u32) -> Option<Self::EnumType> {
                    Option::None
                }
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const IRQ_COMBINED: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 28);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod IRQ_COMBINED {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{FieldValue, TryFromValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
                28,
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 28, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {}
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(_v: u32) -> Option<Self::EnumType> {
                    Option::None
                }
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const EVENT_DB_LEVEL_HIGH: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 27);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod EVENT_DB_LEVEL_HIGH {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{FieldValue, TryFromValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
                27,
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 27, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {}
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(_v: u32) -> Option<Self::EnumType> {
                    Option::None
                }
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const EVENT_DB_LEVEL_LOW: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 26);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod EVENT_DB_LEVEL_LOW {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{FieldValue, TryFromValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
                26,
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 26, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {}
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(_v: u32) -> Option<Self::EnumType> {
                    Option::None
                }
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const EVENT_F_EDGE_HIGH: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 25);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod EVENT_F_EDGE_HIGH {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{FieldValue, TryFromValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
                25,
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 25, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {}
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(_v: u32) -> Option<Self::EnumType> {
                    Option::None
                }
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const EVENT_F_EDGE_LOW: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 24);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod EVENT_F_EDGE_LOW {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{FieldValue, TryFromValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
                24,
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 24, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {}
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(_v: u32) -> Option<Self::EnumType> {
                    Option::None
                }
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const EVENT_LEVEL_HIGH: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 23);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod EVENT_LEVEL_HIGH {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{FieldValue, TryFromValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
                23,
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 23, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {}
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(_v: u32) -> Option<Self::EnumType> {
                    Option::None
                }
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const EVENT_LEVEL_LOW: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 22);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod EVENT_LEVEL_LOW {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{FieldValue, TryFromValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
                22,
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 22, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {}
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(_v: u32) -> Option<Self::EnumType> {
                    Option::None
                }
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const EVENT_EDGE_HIGH: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 21);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod EVENT_EDGE_HIGH {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{FieldValue, TryFromValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
                21,
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 21, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {}
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(_v: u32) -> Option<Self::EnumType> {
                    Option::None
                }
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const EVENT_EDGE_LOW: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 20);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod EVENT_EDGE_LOW {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{FieldValue, TryFromValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
                20,
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 20, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {}
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(_v: u32) -> Option<Self::EnumType> {
                    Option::None
                }
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const INT_TO_PERI: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 19);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod INT_TO_PERI {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{FieldValue, TryFromValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
                19,
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 19, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {}
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(_v: u32) -> Option<Self::EnumType> {
                    Option::None
                }
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const INT_FILTERD: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 18);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod INT_FILTERD {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{FieldValue, TryFromValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
                18,
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 18, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {}
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(_v: u32) -> Option<Self::EnumType> {
                    Option::None
                }
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const IN_FROM_PAD: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 17);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod IN_FROM_PAD {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{FieldValue, TryFromValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
                17,
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 17, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {}
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(_v: u32) -> Option<Self::EnumType> {
                    Option::None
                }
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const IN_IS_DIRECT: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 16);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod IN_IS_DIRECT {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{FieldValue, TryFromValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
                16,
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 16, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {}
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(_v: u32) -> Option<Self::EnumType> {
                    Option::None
                }
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const OE_TO_PAD: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 13);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod OE_TO_PAD {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{FieldValue, TryFromValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
                13,
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 13, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {}
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(_v: u32) -> Option<Self::EnumType> {
                    Option::None
                }
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const OE_FROM_PERI: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 12);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod OE_FROM_PERI {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{FieldValue, TryFromValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
                12,
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 12, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {}
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(_v: u32) -> Option<Self::EnumType> {
                    Option::None
                }
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const OUT_TO_PAD: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 9);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod OUT_TO_PAD {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{FieldValue, TryFromValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
                9,
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 9, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {}
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(_v: u32) -> Option<Self::EnumType> {
                    Option::None
                }
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const OUT_FROM_PERI: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 8);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod OUT_FROM_PERI {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{FieldValue, TryFromValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
                8,
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 8, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {}
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(_v: u32) -> Option<Self::EnumType> {
                    Option::None
                }
            }
        }
    }
    #[allow(non_snake_case)]
    pub mod GpioControl {
        pub struct Register;
        #[automatically_derived]
        impl ::core::clone::Clone for Register {
            #[inline]
            fn clone(&self) -> Register {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Register {}
        impl ::tock_registers::RegisterLongName for Register {}
        use ::tock_registers::fields::Field;
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const IRQ_OVER: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (2 - 1)) + ((1 << (2 - 1)) - 1), 30);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod IRQ_OVER {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{FieldValue, TryFromValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (2 - 1)) + ((1 << (2 - 1)) - 1),
                30,
                (1 << (2 - 1)) + ((1 << (2 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (2 - 1)) + ((1 << (2 - 1)) - 1), 30, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {}
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(_v: u32) -> Option<Self::EnumType> {
                    Option::None
                }
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const IRQ_RESET: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 28);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod IRQ_RESET {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{FieldValue, TryFromValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
                28,
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 28, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {}
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(_v: u32) -> Option<Self::EnumType> {
                    Option::None
                }
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const IRQ_MASK_DB_LEVEL_HIGH: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 27);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod IRQ_MASK_DB_LEVEL_HIGH {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{FieldValue, TryFromValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
                27,
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 27, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {}
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(_v: u32) -> Option<Self::EnumType> {
                    Option::None
                }
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const IRQ_MASK_DB_LEVEL_LOW: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 26);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod IRQ_MASK_DB_LEVEL_LOW {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{FieldValue, TryFromValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
                26,
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 26, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {}
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(_v: u32) -> Option<Self::EnumType> {
                    Option::None
                }
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const IRQMASK_F_EDGE_HIGH: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 25);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod IRQMASK_F_EDGE_HIGH {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{FieldValue, TryFromValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
                25,
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 25, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {}
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(_v: u32) -> Option<Self::EnumType> {
                    Option::None
                }
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const IRQMASK_F_EDGE_LOW: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 24);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod IRQMASK_F_EDGE_LOW {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{FieldValue, TryFromValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
                24,
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 24, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {}
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(_v: u32) -> Option<Self::EnumType> {
                    Option::None
                }
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const IRQMASK_LEVEL_HIGH: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 23);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod IRQMASK_LEVEL_HIGH {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{FieldValue, TryFromValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
                23,
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 23, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {}
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(_v: u32) -> Option<Self::EnumType> {
                    Option::None
                }
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const IRQMASK_LEVEL_LOW: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 22);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod IRQMASK_LEVEL_LOW {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{FieldValue, TryFromValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
                22,
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 22, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {}
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(_v: u32) -> Option<Self::EnumType> {
                    Option::None
                }
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const IRQMASK_EDGE_HIGH: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 21);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod IRQMASK_EDGE_HIGH {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{FieldValue, TryFromValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
                21,
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 21, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {}
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(_v: u32) -> Option<Self::EnumType> {
                    Option::None
                }
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const IRQMASK_EDGE_LOW: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 20);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod IRQMASK_EDGE_LOW {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{FieldValue, TryFromValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
                20,
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 20, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {}
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(_v: u32) -> Option<Self::EnumType> {
                    Option::None
                }
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const INPUT_OVER: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (2 - 1)) + ((1 << (2 - 1)) - 1), 16);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod INPUT_OVER {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{TryFromValue, FieldValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const DONT_INVERT_INPUT: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (2 - 1)) + ((1 << (2 - 1)) - 1), 16, 0);
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const INVERT_INPUT: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (2 - 1)) + ((1 << (2 - 1)) - 1), 16, 1);
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const DRIVE_INPUT_LOW: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (2 - 1)) + ((1 << (2 - 1)) - 1), 16, 2);
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const DRIVE_INPUT_HIGH: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (2 - 1)) + ((1 << (2 - 1)) - 1), 16, 3);
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (2 - 1)) + ((1 << (2 - 1)) - 1),
                16,
                (1 << (2 - 1)) + ((1 << (2 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (2 - 1)) + ((1 << (2 - 1)) - 1), 16, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            #[repr(u32)]
            pub enum Value {
                DONT_INVERT_INPUT = 0,
                INVERT_INPUT = 1,
                DRIVE_INPUT_LOW = 2,
                DRIVE_INPUT_HIGH = 3,
            }
            #[automatically_derived]
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            impl ::core::marker::Copy for Value {}
            #[automatically_derived]
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            impl ::core::clone::Clone for Value {
                #[inline]
                fn clone(&self) -> Value {
                    *self
                }
            }
            #[automatically_derived]
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            impl ::core::fmt::Debug for Value {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            Value::DONT_INVERT_INPUT => "DONT_INVERT_INPUT",
                            Value::INVERT_INPUT => "INVERT_INPUT",
                            Value::DRIVE_INPUT_LOW => "DRIVE_INPUT_LOW",
                            Value::DRIVE_INPUT_HIGH => "DRIVE_INPUT_HIGH",
                        },
                    )
                }
            }
            #[automatically_derived]
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            impl ::core::cmp::Eq for Value {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {}
            }
            #[automatically_derived]
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            impl ::core::marker::StructuralPartialEq for Value {}
            #[automatically_derived]
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            impl ::core::cmp::PartialEq for Value {
                #[inline]
                fn eq(&self, other: &Value) -> bool {
                    let __self_tag = ::core::intrinsics::discriminant_value(self);
                    let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                    __self_tag == __arg1_tag
                }
            }
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(v: u32) -> Option<Self::EnumType> {
                    match v {
                        x if x == Value::DONT_INVERT_INPUT as u32 => {
                            Some(Value::DONT_INVERT_INPUT)
                        }
                        x if x == Value::INVERT_INPUT as u32 => Some(Value::INVERT_INPUT),
                        x if x == Value::DRIVE_INPUT_LOW as u32 => {
                            Some(Value::DRIVE_INPUT_LOW)
                        }
                        x if x == Value::DRIVE_INPUT_HIGH as u32 => {
                            Some(Value::DRIVE_INPUT_HIGH)
                        }
                        _ => Option::None,
                    }
                }
            }
            impl From<Value> for FieldValue<u32, Register> {
                fn from(v: Value) -> Self {
                    Self::new((1 << (2 - 1)) + ((1 << (2 - 1)) - 1), 16, v as u32)
                }
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const OUTPUT_ENABLE_OVER: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (2 - 1)) + ((1 << (2 - 1)) - 1), 14);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod OUTPUT_ENABLE_OVER {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{TryFromValue, FieldValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const ENABLE_OUTPUT_SIGNAL: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (2 - 1)) + ((1 << (2 - 1)) - 1), 14, 0);
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const ENABLE_OUTPUT_SIGNAL_INVERT: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (2 - 1)) + ((1 << (2 - 1)) - 1), 14, 1);
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const DISABLE_OUTPUT: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (2 - 1)) + ((1 << (2 - 1)) - 1), 14, 2);
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const ENABLE_OUTPUT: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (2 - 1)) + ((1 << (2 - 1)) - 1), 14, 3);
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (2 - 1)) + ((1 << (2 - 1)) - 1),
                14,
                (1 << (2 - 1)) + ((1 << (2 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (2 - 1)) + ((1 << (2 - 1)) - 1), 14, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            #[repr(u32)]
            pub enum Value {
                ENABLE_OUTPUT_SIGNAL = 0,
                ENABLE_OUTPUT_SIGNAL_INVERT = 1,
                DISABLE_OUTPUT = 2,
                ENABLE_OUTPUT = 3,
            }
            #[automatically_derived]
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            impl ::core::marker::Copy for Value {}
            #[automatically_derived]
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            impl ::core::clone::Clone for Value {
                #[inline]
                fn clone(&self) -> Value {
                    *self
                }
            }
            #[automatically_derived]
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            impl ::core::fmt::Debug for Value {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            Value::ENABLE_OUTPUT_SIGNAL => "ENABLE_OUTPUT_SIGNAL",
                            Value::ENABLE_OUTPUT_SIGNAL_INVERT => {
                                "ENABLE_OUTPUT_SIGNAL_INVERT"
                            }
                            Value::DISABLE_OUTPUT => "DISABLE_OUTPUT",
                            Value::ENABLE_OUTPUT => "ENABLE_OUTPUT",
                        },
                    )
                }
            }
            #[automatically_derived]
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            impl ::core::cmp::Eq for Value {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {}
            }
            #[automatically_derived]
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            impl ::core::marker::StructuralPartialEq for Value {}
            #[automatically_derived]
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            impl ::core::cmp::PartialEq for Value {
                #[inline]
                fn eq(&self, other: &Value) -> bool {
                    let __self_tag = ::core::intrinsics::discriminant_value(self);
                    let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                    __self_tag == __arg1_tag
                }
            }
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(v: u32) -> Option<Self::EnumType> {
                    match v {
                        x if x == Value::ENABLE_OUTPUT_SIGNAL as u32 => {
                            Some(Value::ENABLE_OUTPUT_SIGNAL)
                        }
                        x if x == Value::ENABLE_OUTPUT_SIGNAL_INVERT as u32 => {
                            Some(Value::ENABLE_OUTPUT_SIGNAL_INVERT)
                        }
                        x if x == Value::DISABLE_OUTPUT as u32 => {
                            Some(Value::DISABLE_OUTPUT)
                        }
                        x if x == Value::ENABLE_OUTPUT as u32 => {
                            Some(Value::ENABLE_OUTPUT)
                        }
                        _ => Option::None,
                    }
                }
            }
            impl From<Value> for FieldValue<u32, Register> {
                fn from(v: Value) -> Self {
                    Self::new((1 << (2 - 1)) + ((1 << (2 - 1)) - 1), 14, v as u32)
                }
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const OUT_OVER: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (2 - 1)) + ((1 << (2 - 1)) - 1), 12);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod OUT_OVER {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{TryFromValue, FieldValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const DRIVE_OUTPUT_SIGNAL: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (2 - 1)) + ((1 << (2 - 1)) - 1), 12, 0);
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const DRIVE_OUTPUT_SIGNAL_INVERT: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (2 - 1)) + ((1 << (2 - 1)) - 1), 12, 1);
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const DRIVE_OUTPUT_LOW: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (2 - 1)) + ((1 << (2 - 1)) - 1), 12, 2);
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const DRIVE_OUTPUT_HIGH: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (2 - 1)) + ((1 << (2 - 1)) - 1), 12, 3);
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (2 - 1)) + ((1 << (2 - 1)) - 1),
                12,
                (1 << (2 - 1)) + ((1 << (2 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (2 - 1)) + ((1 << (2 - 1)) - 1), 12, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            #[repr(u32)]
            pub enum Value {
                DRIVE_OUTPUT_SIGNAL = 0,
                DRIVE_OUTPUT_SIGNAL_INVERT = 1,
                DRIVE_OUTPUT_LOW = 2,
                DRIVE_OUTPUT_HIGH = 3,
            }
            #[automatically_derived]
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            impl ::core::marker::Copy for Value {}
            #[automatically_derived]
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            impl ::core::clone::Clone for Value {
                #[inline]
                fn clone(&self) -> Value {
                    *self
                }
            }
            #[automatically_derived]
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            impl ::core::fmt::Debug for Value {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            Value::DRIVE_OUTPUT_SIGNAL => "DRIVE_OUTPUT_SIGNAL",
                            Value::DRIVE_OUTPUT_SIGNAL_INVERT => {
                                "DRIVE_OUTPUT_SIGNAL_INVERT"
                            }
                            Value::DRIVE_OUTPUT_LOW => "DRIVE_OUTPUT_LOW",
                            Value::DRIVE_OUTPUT_HIGH => "DRIVE_OUTPUT_HIGH",
                        },
                    )
                }
            }
            #[automatically_derived]
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            impl ::core::cmp::Eq for Value {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {}
            }
            #[automatically_derived]
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            impl ::core::marker::StructuralPartialEq for Value {}
            #[automatically_derived]
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            impl ::core::cmp::PartialEq for Value {
                #[inline]
                fn eq(&self, other: &Value) -> bool {
                    let __self_tag = ::core::intrinsics::discriminant_value(self);
                    let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                    __self_tag == __arg1_tag
                }
            }
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(v: u32) -> Option<Self::EnumType> {
                    match v {
                        x if x == Value::DRIVE_OUTPUT_SIGNAL as u32 => {
                            Some(Value::DRIVE_OUTPUT_SIGNAL)
                        }
                        x if x == Value::DRIVE_OUTPUT_SIGNAL_INVERT as u32 => {
                            Some(Value::DRIVE_OUTPUT_SIGNAL_INVERT)
                        }
                        x if x == Value::DRIVE_OUTPUT_LOW as u32 => {
                            Some(Value::DRIVE_OUTPUT_LOW)
                        }
                        x if x == Value::DRIVE_OUTPUT_HIGH as u32 => {
                            Some(Value::DRIVE_OUTPUT_HIGH)
                        }
                        _ => Option::None,
                    }
                }
            }
            impl From<Value> for FieldValue<u32, Register> {
                fn from(v: Value) -> Self {
                    Self::new((1 << (2 - 1)) + ((1 << (2 - 1)) - 1), 12, v as u32)
                }
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const F_M: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (7 - 1)) + ((1 << (7 - 1)) - 1), 5);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod F_M {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{FieldValue, TryFromValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (7 - 1)) + ((1 << (7 - 1)) - 1),
                5,
                (1 << (7 - 1)) + ((1 << (7 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (7 - 1)) + ((1 << (7 - 1)) - 1), 5, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {}
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(_v: u32) -> Option<Self::EnumType> {
                    Option::None
                }
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const FUNCSEL: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (5 - 1)) + ((1 << (5 - 1)) - 1), 0);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod FUNCSEL {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{FieldValue, TryFromValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (5 - 1)) + ((1 << (5 - 1)) - 1),
                0,
                (1 << (5 - 1)) + ((1 << (5 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (5 - 1)) + ((1 << (5 - 1)) - 1), 0, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {}
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(_v: u32) -> Option<Self::EnumType> {
                    Option::None
                }
            }
        }
    }
    impl Rp1IoBank0 {
        pub unsafe fn new_ref(address: *mut u32) -> &'static mut Rp1IoBank0 {
            let x: &'static mut Rp1IoBank0 = unsafe {
                &mut *(address as *mut Rp1IoBank0)
            };
            x
        }
    }
    #[repr(C)]
    pub struct Rp1PadsBank0 {
        pub voltage_select: ReadOnly<u32, VoltageSelect::Register>,
        pub gpio: [ReadWrite<u32, PadControl::Register>; 28],
        _reserved0128: [u8; 0x1000 - 0x0074],
    }
    const _: () = {
        const SUM_MAX_ALIGN: (usize, usize) = {
            const SUM_MAX_ALIGN: (usize, usize) = {
                const SUM_MAX_ALIGN: (usize, usize) = {
                    const SUM_MAX_ALIGN: (usize, usize) = (0, 0);
                    const SUM: usize = SUM_MAX_ALIGN.0;
                    const MAX_ALIGN: usize = SUM_MAX_ALIGN.1;
                    if !(SUM == 0x0000) {
                        {
                            #[cold]
                            #[track_caller]
                            #[inline(never)]
                            #[rustc_const_panic_str]
                            #[rustc_do_not_const_check]
                            const fn panic_cold_display<T: ::core::fmt::Display>(
                                arg: &T,
                            ) -> ! {
                                ::core::panicking::panic_display(arg)
                            }
                            panic_cold_display(
                                &"Invalid start offset for field voltage_select (expected 0 but actual value differs)",
                            );
                        }
                    }
                    const ALIGN: usize = core::mem::align_of::<
                        ReadOnly<u32, VoltageSelect::Register>,
                    >();
                    #[allow(clippy::bad_bit_mask)]
                    {
                        if !(SUM & (ALIGN - 1) == 0) {
                            {
                                #[cold]
                                #[track_caller]
                                #[inline(never)]
                                #[rustc_const_panic_str]
                                #[rustc_do_not_const_check]
                                const fn panic_cold_display<T: ::core::fmt::Display>(
                                    arg: &T,
                                ) -> ! {
                                    ::core::panicking::panic_display(arg)
                                }
                                panic_cold_display(
                                    &"Invalid alignment for field voltage_select (offset differs from expected)",
                                );
                            }
                        }
                    }
                    const NEW_SUM: usize = SUM
                        + core::mem::size_of::<ReadOnly<u32, VoltageSelect::Register>>();
                    if !(NEW_SUM == 0x0004) {
                        {
                            #[cold]
                            #[track_caller]
                            #[inline(never)]
                            #[rustc_const_panic_str]
                            #[rustc_do_not_const_check]
                            const fn panic_cold_display<T: ::core::fmt::Display>(
                                arg: &T,
                            ) -> ! {
                                ::core::panicking::panic_display(arg)
                            }
                            panic_cold_display(
                                &"Invalid end offset for field voltage_select (expected 4 but actual value differs)",
                            );
                        }
                    }
                    const NEW_MAX_ALIGN: usize = if ALIGN > MAX_ALIGN {
                        ALIGN
                    } else {
                        MAX_ALIGN
                    };
                    (NEW_SUM, NEW_MAX_ALIGN)
                };
                const SUM: usize = SUM_MAX_ALIGN.0;
                const MAX_ALIGN: usize = SUM_MAX_ALIGN.1;
                if !(SUM == 0x0004) {
                    {
                        #[cold]
                        #[track_caller]
                        #[inline(never)]
                        #[rustc_const_panic_str]
                        #[rustc_do_not_const_check]
                        const fn panic_cold_display<T: ::core::fmt::Display>(
                            arg: &T,
                        ) -> ! {
                            ::core::panicking::panic_display(arg)
                        }
                        panic_cold_display(
                            &"Invalid start offset for field gpio (expected 4 but actual value differs)",
                        );
                    }
                }
                const ALIGN: usize = core::mem::align_of::<
                    [ReadWrite<u32, PadControl::Register>; 28],
                >();
                #[allow(clippy::bad_bit_mask)]
                {
                    if !(SUM & (ALIGN - 1) == 0) {
                        {
                            #[cold]
                            #[track_caller]
                            #[inline(never)]
                            #[rustc_const_panic_str]
                            #[rustc_do_not_const_check]
                            const fn panic_cold_display<T: ::core::fmt::Display>(
                                arg: &T,
                            ) -> ! {
                                ::core::panicking::panic_display(arg)
                            }
                            panic_cold_display(
                                &"Invalid alignment for field gpio (offset differs from expected)",
                            );
                        }
                    }
                }
                const NEW_SUM: usize = SUM
                    + core::mem::size_of::<[ReadWrite<u32, PadControl::Register>; 28]>();
                if !(NEW_SUM == 0x0074) {
                    {
                        #[cold]
                        #[track_caller]
                        #[inline(never)]
                        #[rustc_const_panic_str]
                        #[rustc_do_not_const_check]
                        const fn panic_cold_display<T: ::core::fmt::Display>(
                            arg: &T,
                        ) -> ! {
                            ::core::panicking::panic_display(arg)
                        }
                        panic_cold_display(
                            &"Invalid end offset for field gpio (expected 116 but actual value differs)",
                        );
                    }
                }
                const NEW_MAX_ALIGN: usize = if ALIGN > MAX_ALIGN {
                    ALIGN
                } else {
                    MAX_ALIGN
                };
                (NEW_SUM, NEW_MAX_ALIGN)
            };
            const SUM: usize = SUM_MAX_ALIGN.0;
            const MAX_ALIGN: usize = SUM_MAX_ALIGN.1;
            if !(SUM == 0x0074) {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "Invalid start offset for padding _reserved0128 (expected 116 but actual value differs)"
                        ),
                    );
                }
            }
            (0x1000, MAX_ALIGN)
        };
        const SUM: usize = SUM_MAX_ALIGN.0;
        const MAX_ALIGN: usize = SUM_MAX_ALIGN.1;
        if !(SUM == 0x1000) {
            ::core::panicking::panic("assertion failed: SUM == 0x1000")
        }
        const STRUCT_SIZE: usize = core::mem::size_of::<Rp1PadsBank0>();
        const ALIGNMENT_CORRECTED_SIZE: usize = if 0x1000 % MAX_ALIGN != 0 {
            0x1000 + (MAX_ALIGN - (0x1000 % MAX_ALIGN))
        } else {
            0x1000
        };
        if !(STRUCT_SIZE == ALIGNMENT_CORRECTED_SIZE) {
            {
                #[cold]
                #[track_caller]
                #[inline(never)]
                #[rustc_const_panic_str]
                #[rustc_do_not_const_check]
                const fn panic_cold_display<T: ::core::fmt::Display>(arg: &T) -> ! {
                    ::core::panicking::panic_display(arg)
                }
                panic_cold_display(
                    &"Invalid size for struct Rp1PadsBank0 (expected 4096, actual struct size differs)",
                );
            }
        }
    };
    #[allow(non_snake_case)]
    pub mod VoltageSelect {
        pub struct Register;
        #[automatically_derived]
        impl ::core::clone::Clone for Register {
            #[inline]
            fn clone(&self) -> Register {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Register {}
        impl ::tock_registers::RegisterLongName for Register {}
        use ::tock_registers::fields::Field;
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const VOLTAGE_SELECT: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 0);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod VOLTAGE_SELECT {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{TryFromValue, FieldValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const VOLTAGE_3V3: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 0, 0);
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const VOLTAGE_1V8: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 0, 1);
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
                0,
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 0, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            #[repr(u32)]
            pub enum Value {
                VOLTAGE_3V3 = 0,
                VOLTAGE_1V8 = 1,
            }
            #[automatically_derived]
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            impl ::core::marker::Copy for Value {}
            #[automatically_derived]
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            impl ::core::clone::Clone for Value {
                #[inline]
                fn clone(&self) -> Value {
                    *self
                }
            }
            #[automatically_derived]
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            impl ::core::fmt::Debug for Value {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            Value::VOLTAGE_3V3 => "VOLTAGE_3V3",
                            Value::VOLTAGE_1V8 => "VOLTAGE_1V8",
                        },
                    )
                }
            }
            #[automatically_derived]
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            impl ::core::cmp::Eq for Value {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {}
            }
            #[automatically_derived]
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            impl ::core::marker::StructuralPartialEq for Value {}
            #[automatically_derived]
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            impl ::core::cmp::PartialEq for Value {
                #[inline]
                fn eq(&self, other: &Value) -> bool {
                    let __self_tag = ::core::intrinsics::discriminant_value(self);
                    let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                    __self_tag == __arg1_tag
                }
            }
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(v: u32) -> Option<Self::EnumType> {
                    match v {
                        x if x == Value::VOLTAGE_3V3 as u32 => Some(Value::VOLTAGE_3V3),
                        x if x == Value::VOLTAGE_1V8 as u32 => Some(Value::VOLTAGE_1V8),
                        _ => Option::None,
                    }
                }
            }
            impl From<Value> for FieldValue<u32, Register> {
                fn from(v: Value) -> Self {
                    Self::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 0, v as u32)
                }
            }
        }
    }
    #[allow(non_snake_case)]
    pub mod PadControl {
        pub struct Register;
        #[automatically_derived]
        impl ::core::clone::Clone for Register {
            #[inline]
            fn clone(&self) -> Register {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Register {}
        impl ::tock_registers::RegisterLongName for Register {}
        use ::tock_registers::fields::Field;
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const OUTPUT_DISABLE: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 7);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod OUTPUT_DISABLE {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{FieldValue, TryFromValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
                7,
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 7, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {}
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(_v: u32) -> Option<Self::EnumType> {
                    Option::None
                }
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const INPUT_ENABLE: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 6);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod INPUT_ENABLE {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{FieldValue, TryFromValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
                6,
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 6, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {}
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(_v: u32) -> Option<Self::EnumType> {
                    Option::None
                }
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const DRIVE_STRENGTH: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (2 - 1)) + ((1 << (2 - 1)) - 1), 4);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod DRIVE_STRENGTH {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{TryFromValue, FieldValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const STRENGTH_2mA: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (2 - 1)) + ((1 << (2 - 1)) - 1), 4, 0);
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const STRENGTH_4mA: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (2 - 1)) + ((1 << (2 - 1)) - 1), 4, 1);
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const STRENGTH_8mA: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (2 - 1)) + ((1 << (2 - 1)) - 1), 4, 2);
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const STRENGTH_12mA: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (2 - 1)) + ((1 << (2 - 1)) - 1), 4, 3);
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (2 - 1)) + ((1 << (2 - 1)) - 1),
                4,
                (1 << (2 - 1)) + ((1 << (2 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (2 - 1)) + ((1 << (2 - 1)) - 1), 4, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            #[repr(u32)]
            pub enum Value {
                STRENGTH_2mA = 0,
                STRENGTH_4mA = 1,
                STRENGTH_8mA = 2,
                STRENGTH_12mA = 3,
            }
            #[automatically_derived]
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            impl ::core::marker::Copy for Value {}
            #[automatically_derived]
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            impl ::core::clone::Clone for Value {
                #[inline]
                fn clone(&self) -> Value {
                    *self
                }
            }
            #[automatically_derived]
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            impl ::core::fmt::Debug for Value {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            Value::STRENGTH_2mA => "STRENGTH_2mA",
                            Value::STRENGTH_4mA => "STRENGTH_4mA",
                            Value::STRENGTH_8mA => "STRENGTH_8mA",
                            Value::STRENGTH_12mA => "STRENGTH_12mA",
                        },
                    )
                }
            }
            #[automatically_derived]
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            impl ::core::cmp::Eq for Value {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {}
            }
            #[automatically_derived]
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            impl ::core::marker::StructuralPartialEq for Value {}
            #[automatically_derived]
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            impl ::core::cmp::PartialEq for Value {
                #[inline]
                fn eq(&self, other: &Value) -> bool {
                    let __self_tag = ::core::intrinsics::discriminant_value(self);
                    let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                    __self_tag == __arg1_tag
                }
            }
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(v: u32) -> Option<Self::EnumType> {
                    match v {
                        x if x == Value::STRENGTH_2mA as u32 => Some(Value::STRENGTH_2mA),
                        x if x == Value::STRENGTH_4mA as u32 => Some(Value::STRENGTH_4mA),
                        x if x == Value::STRENGTH_8mA as u32 => Some(Value::STRENGTH_8mA),
                        x if x == Value::STRENGTH_12mA as u32 => {
                            Some(Value::STRENGTH_12mA)
                        }
                        _ => Option::None,
                    }
                }
            }
            impl From<Value> for FieldValue<u32, Register> {
                fn from(v: Value) -> Self {
                    Self::new((1 << (2 - 1)) + ((1 << (2 - 1)) - 1), 4, v as u32)
                }
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const PULLUP_ENABLE: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 3);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod PULLUP_ENABLE {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{FieldValue, TryFromValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
                3,
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 3, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {}
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(_v: u32) -> Option<Self::EnumType> {
                    Option::None
                }
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const PULLDOWN_ENABLE: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 2);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod PULLDOWN_ENABLE {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{FieldValue, TryFromValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
                2,
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 2, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {}
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(_v: u32) -> Option<Self::EnumType> {
                    Option::None
                }
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const SHUMITT_TRIGER_ENABLE: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 1);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod SHUMITT_TRIGER_ENABLE {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{FieldValue, TryFromValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
                1,
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 1, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            pub enum Value {}
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(_v: u32) -> Option<Self::EnumType> {
                    Option::None
                }
            }
        }
        #[allow(non_upper_case_globals)]
        #[allow(unused)]
        pub const SLEW_RATE: Field<u32, Register> = Field::<
            u32,
            Register,
        >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 0);
        #[allow(non_snake_case)]
        #[allow(unused)]
        pub mod SLEW_RATE {
            #[allow(unused_imports)]
            use ::tock_registers::fields::{TryFromValue, FieldValue};
            use super::Register;
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const Fast: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 0, 1);
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const Slow: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 0, 0);
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const SET: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new(
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
                0,
                (1 << (1 - 1)) + ((1 << (1 - 1)) - 1),
            );
            #[allow(non_upper_case_globals)]
            #[allow(unused)]
            pub const CLEAR: FieldValue<u32, Register> = FieldValue::<
                u32,
                Register,
            >::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 0, 0);
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            #[repr(u32)]
            pub enum Value {
                Fast = 1,
                Slow = 0,
            }
            #[automatically_derived]
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            impl ::core::marker::Copy for Value {}
            #[automatically_derived]
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            impl ::core::clone::Clone for Value {
                #[inline]
                fn clone(&self) -> Value {
                    *self
                }
            }
            #[automatically_derived]
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            impl ::core::fmt::Debug for Value {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            Value::Fast => "Fast",
                            Value::Slow => "Slow",
                        },
                    )
                }
            }
            #[automatically_derived]
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            impl ::core::cmp::Eq for Value {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {}
            }
            #[automatically_derived]
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            impl ::core::marker::StructuralPartialEq for Value {}
            #[automatically_derived]
            #[allow(dead_code)]
            #[allow(non_camel_case_types)]
            impl ::core::cmp::PartialEq for Value {
                #[inline]
                fn eq(&self, other: &Value) -> bool {
                    let __self_tag = ::core::intrinsics::discriminant_value(self);
                    let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                    __self_tag == __arg1_tag
                }
            }
            impl TryFromValue<u32> for Value {
                type EnumType = Value;
                fn try_from_value(v: u32) -> Option<Self::EnumType> {
                    match v {
                        x if x == Value::Fast as u32 => Some(Value::Fast),
                        x if x == Value::Slow as u32 => Some(Value::Slow),
                        _ => Option::None,
                    }
                }
            }
            impl From<Value> for FieldValue<u32, Register> {
                fn from(v: Value) -> Self {
                    Self::new((1 << (1 - 1)) + ((1 << (1 - 1)) - 1), 0, v as u32)
                }
            }
        }
    }
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
                    gpio.1
                        .modify(
                            GpioControl::FUNCSEL.val(u32::from(function))
                                + GpioControl::OUTPUT_ENABLE_OVER::ENABLE_OUTPUT_SIGNAL
                                + GpioControl::OUT_OVER::DRIVE_OUTPUT_SIGNAL,
                        );
                }
                None => {}
            }
        }
        pub fn set_output_enable(&self, gpio_num: usize, output_enable: bool) {
            match self.padsbank0.gpio.get(gpio_num) {
                Some(gpio) => {
                    if output_enable {
                        gpio.modify(PadControl::OUTPUT_DISABLE::CLEAR);
                    } else {
                        gpio.modify(PadControl::OUTPUT_DISABLE::SET);
                    }
                }
                None => {}
            }
        }
    }
}
mod systimer {
    use core::arch::asm;
    struct SystemCounter;
    impl SystemCounter {
        pub fn get_current_frequency() -> u32 {
            #[allow(unused_assignments)]
            let mut current_frequency = 0u64;
            unsafe { asm!("mrs {0}, CNTFRQ_EL0", out(reg) current_frequency) };
            current_frequency as u32
        }
        pub fn set_current_frequency(frequency: u32) {
            let current_frequency: u64 = frequency.into();
            unsafe { asm!("msr CNTFRQ_EL0, {0}", in (reg) current_frequency) };
        }
        pub fn get_physical_counter_value() -> u64 {
            let mut count = 0u64;
            unsafe {
                asm!(
                    "\n            isb\n            mrs {0}, CNTPCT_EL0\n            ",
                    out(reg) count
                )
            };
            count
        }
    }
    pub fn wait(duration: core::time::Duration) {
        let frequency = SystemCounter::get_current_frequency();
        let frequency_1us = frequency / 1000 / 1000;
        let duration_us = duration.as_micros();
        let wait_count = u128::from(frequency_1us) * duration_us;
        let alerm_count = u128::from(SystemCounter::get_physical_counter_value())
            + wait_count;
        while u128::from(SystemCounter::get_physical_counter_value()) < alerm_count {
            unsafe { asm!("nop") };
        }
    }
}
use core::{
    arch::{asm, global_asm},
    fmt::{self, Write},
    panic::PanicInfo, time,
};
use pl011_uart::{Pl011Uart, Pl011UartPeripheral};
use rp1_gpio::RP1Gpio;
use systimer::wait;
use tock_registers::{
    interfaces::{ReadWriteable, Readable, Writeable},
    register_bitfields, registers::InMemoryRegister,
};
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {
        unsafe { asm!("wfe") };
    }
}

const BCM2712_EARLY_UART_DR: *mut u32 = 0x10_7d00_1000 as *mut u32;
const BCM2712_EARLY_UART_FLAG: *mut u32 = (0x10_7d00_1000u64 + 0x18u64) as *mut u32;
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
            let flag = unsafe {
                core::ptr::read_volatile(BCM2712_EARLY_UART_FLAG as *const u32)
            };
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
        wait
    }
}
