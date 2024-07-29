use crate::pcie_generic;
use tock_registers::{
    interfaces::Writeable,
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite, WriteOnly},
};

register_bitfields![
    u32,
    // ref: https://wiki.osdev.org/PCI
    pub CommandAndStatus [
        CMD_IO_SPACE    OFFSET(0) NUMBITS(1) [],
        CMD_MEMORY_SPACE    OFFSET(1) NUMBITS(1) [],
        CMD_BUS_MUSTER    OFFSET(2) NUMBITS(1) [],
        CMD_SPECIAL_CYCLES    OFFSET(3) NUMBITS(1) [],
        CMD_MEMORY_WRITE_AND_INVALIDATE_ENABLE    OFFSET(4) NUMBITS(1) [],
        CMD_VGA_PALLETE_SNOOP    OFFSET(5) NUMBITS(1) [],
        CMD_PARITY_ERROR_RESPONSE    OFFSET(6) NUMBITS(1) [],
        CMD_SERR_ENABLE    OFFSET(8) NUMBITS(1) [],
        CMD_FAST_BACK_TO_BACK_ENABLE    OFFSET(9) NUMBITS(1) [],
        CMD_INTERRUPT_DISABLE    OFFSET(10) NUMBITS(1) [],
        STATUS_INTERRUPT    OFFSET(19) NUMBITS(1) [],
        STATUS_CAPABILITIES_LIST    OFFSET(20) NUMBITS(1) [],
        STATUS_66MHZ_CAPABLE    OFFSET(21) NUMBITS(1) [],
        STATUS_FAST_BACK_TO_BACK_CAPABLE    OFFSET(23) NUMBITS(1) [],
        STATUS_MASTER_DATA_PARITY_ERROR    OFFSET(24) NUMBITS(1) [],
        STATUS_DEVSEL_TIMING    OFFSET(25) NUMBITS(2) [],
        STATUS_SIGNALED_TARGET_ABORT    OFFSET(27) NUMBITS(1) [],
        STATUS_RECEIVED_TARGET_ABORT    OFFSET(28) NUMBITS(1) [],
        STATUS_RECEIVED_MASTER_ABORT    OFFSET(29) NUMBITS(1) [],
        STATUS_SIGNALED_SYSTEM_ERROR    OFFSET(30) NUMBITS(1) [],
        STATUS_DETECTED_PARITY_ERROR    OFFSET(31) NUMBITS(1) [],
    ],
    pub RevisionIdAndClassCode [
        REVISION_ID    OFFSET(0) NUMBITS(8) [],
        ProgIF    OFFSET(8) NUMBITS(8) [],
        SubClass    OFFSET(16) NUMBITS(8) [],
        ClassCode    OFFSET(24) NUMBITS(8) [],
    ],
    pub BistHeaderTypeTimerCache [
        CACHE_LINE_SIZE OFFSET(0) NUMBITS(8) [],
        LATENCY_TIMER   OFFSET(8) NUMBITS(8) [],
        HEADER_TYPE    OFFSET(16) NUMBITS(8) [],
        BIST    OFFSET(24) NUMBITS(8) [],
    ]
];

register_structs! {
    pub GenericPciHeaderType0 {
        (0x00 => pub vendor_and_device_id: ReadOnly<u32>),
        (0x04 => pub command_and_status: ReadWrite<u32,CommandAndStatus::Register>),
        (0x08 => pub revision_id_and_class_code: ReadOnly<u32,RevisionIdAndClassCode::Register>),
        (0x0c => pub bist_header_type_timer_cache: ReadOnly<u32>),
        (0x10 => pub base_addres_registers: [ReadWrite<u32>;6]),
        (0x28 => pub cardbus_cis_pointer: ReadOnly<u32>),
        (0x2C => pub subsystem_and_vendor_id: ReadOnly<u32>),
        (0x30 => pub expansion_rom_base_address: ReadOnly<u32>),
        (0x34 => pub cap_pointer: ReadOnly<u32>),
        (0x38 => pub reserved: ReadOnly<u32>),
        (0x3c => pub lat_gnt_interrupt: ReadOnly<u32>),
        (0x40 => @END),
    },
    // https://www.intel.com/content/www/us/en/docs/programmable/683733/14-1/type-1-configuration-space-registers.html
    pub GenericPciHeaderType1 {
        (0x00 => pub vendor_and_device_id: ReadOnly<u32>),
        (0x04 => pub command_and_status: ReadWrite<u32,CommandAndStatus::Register>),
        (0x08 => pub revision_id_and_class_code: ReadOnly<u32,RevisionIdAndClassCode::Register>),
        (0x0c => pub bist_header_type_timer_cache: ReadOnly<u32>),
        (0x10 => pub base_addres_registers: [ReadOnly<u32>;2]),
        (0x18 => pub timer_and_bus_number_config: ReadWrite<u32,TimerAndBusNumber::Register>),
        (0x1c => pub io_and_secondary_status: ReadWrite<u32>),
        (0x20 => pub memory_limit_base: ReadWrite<u32,MemoryLimitBase::Register>),
        (0x24 => pub prefetchable_memory_limit_base: ReadWrite<u32,MemoryLimitBase::Register>),
        (0x28 => pub prefetchable_base_upper_32bit: ReadWrite<u32>),
        (0x2C => pub prefetchable_limit_upper_32bit: ReadWrite<u32>),
        (0x30 => pub io_limit_base_upper_16bit: ReadOnly<u32>),
        (0x34 => pub cap_pointer: ReadOnly<u32>),
        (0x38 => pub reserved: ReadOnly<u32>),
        (0x3c => pub lat_gnt_interrupt: ReadOnly<u32>),
        (0x40 => @END),
    }
}

impl GenericPciHeaderType1 {
    /// bridgeのbase addressとlimit addressをセットする
    /// baseの下位20bitは0固定、limitの下位20bitはF固定となる
    pub fn set_memory_base_limit(&mut self, base: u32, limit: u32) {
        let b = base >> (20 - 4) & 0xFFF0;
        let l = limit >> (20 - 4) & 0xFFF0;
        self.memory_limit_base.set(
            (pcie_generic::MemoryLimitBase::MEMORY_LIMIT.val(b)
                + pcie_generic::MemoryLimitBase::MEMORY_BASE.val(l))
            .value,
        );
    }

    pub fn set_bus_number(&mut self, primary: u8, secondary: u8, subordinate: u8) {
        self.timer_and_bus_number_config.set(
            (pcie_generic::TimerAndBusNumber::PRIMARY_BUS_NUMBER.val(primary.into())
                + pcie_generic::TimerAndBusNumber::SECONDARY_BUS_NUMBER.val(secondary.into())
                + pcie_generic::TimerAndBusNumber::SUBORDINATE_BUS_NUMBER.val(subordinate.into()))
            .value,
        );
    }
}

impl GenericPciHeaderType0 {
    pub unsafe fn new_ref(address: *mut u32) -> &'static mut GenericPciHeaderType0 {
        let header: &'static mut GenericPciHeaderType0 =
            unsafe { &mut *(address as *mut GenericPciHeaderType0) };
        header
    }
}

impl GenericPciHeaderType1 {
    pub unsafe fn new_ref(address: *mut u32) -> &'static mut GenericPciHeaderType1 {
        let header: &'static mut GenericPciHeaderType1 =
            unsafe { &mut *(address as *mut GenericPciHeaderType1) };
        header
    }
}

// PCI Type1 header
register_bitfields![
    u32,
    pub TimerAndBusNumber [
        PRIMARY_BUS_NUMBER    OFFSET(0) NUMBITS(8) [],
        SECONDARY_BUS_NUMBER    OFFSET(8) NUMBITS(8) [],
        SUBORDINATE_BUS_NUMBER    OFFSET(16) NUMBITS(8) [],
        SECONDARY_LATENCY_TIMER OFFSET(24) NUMBITS(8) [],
    ],
    pub MemoryLimitBase [
        MEMORY_BASE    OFFSET(0) NUMBITS(16) [],
        MEMORY_LIMIT    OFFSET(16) NUMBITS(16) [],
    ]
];

register_structs! {
    pub PCIeCapabilityStructure {
        (0x0000 => pub capability: ReadOnly<u32,PCIeCapabilies::Register>),
        (0x0004 => pub device_capabilities: ReadOnly<u32,PCIeDeviceCapabilies::Register>),
        (0x0008 => pub device_control_status: ReadWrite<u32,PCIeDeviceControlStatus::Register>),
        (0x000c => pub link_capabilities: ReadWrite<u32,PCIeLinkCapabilities::Register>),
        (0x0010 => pub link_control_status: ReadWrite<u32,PCIeLinkControlStatus::Register>),
        (0x0014 => pub slot_capabilities: ReadWrite<u32>),
        (0x0018 => pub slot_control_status: ReadWrite<u32>),
        (0x001c => pub root_control_and_capabilities: ReadWrite<u32,PCIeRootControlCapabilities::Register>),
        (0x0020 => pub root_status: ReadWrite<u32,PCIeRootStatus::Register>),
        (0x0024 => pub device_compatiiblities2: ReadWrite<u32,PCIeCapabilies2::Register>),
        (0x0028 => pub device_control_status2: ReadWrite<u32,PCIeDeviceControlStatus2::Register>),
        (0x002c => pub link_capabilities2: ReadWrite<u32,PCIeLinkCapabilities2::Register>),
        (0x0030 => pub link_control_status2: ReadWrite<u32,PCIeLinkControlStatus2::Register>),
        (0x0034 => @END),
    }
}

impl PCIeCapabilityStructure {
    pub unsafe fn new_ref(address: *mut u32) -> &'static mut PCIeCapabilityStructure {
        let header: &'static mut PCIeCapabilityStructure =
            unsafe { &mut *(address as *mut PCIeCapabilityStructure) };
        header
    }
}

register_bitfields!(
    u32,
    pub PCIeCapabilies [
        PCIE_CAPABILITIES_ID   OFFSET(0) NUMBITS(8) [],
        NEXT_CAPABILITIES_POINTER   OFFSET(8) NUMBITS(8) [],
        PCIE_CAPABILITIES_REGISTER   OFFSET(16) NUMBITS(16) [],
    ],
    pub PCIeDeviceCapabilies [
        MAX_PAYLOAD_SIZE   OFFSET(0) NUMBITS(3) [
            SIZE_128B = 0b000,
            SIZE_256B = 0b001,
        ],
        EXTENDED_TAG_SUPPORT   OFFSET(5) NUMBITS(1) [],
        ACCEPTABLE_L0S_LATENCY   OFFSET(6) NUMBITS(3) [],
        ACCEPTABLE_L1_LATENCY   OFFSET(9) NUMBITS(3) [],
        ROLE_BASED_ERROR_REPORTING_SUPPORTED   OFFSET(15) NUMBITS(1) [],
        CAPTURED_SLOT_POWER_LIMIT_AND_SCALE   OFFSET(18) NUMBITS(10) [],
        FLR_CAPABLE   OFFSET(28) NUMBITS(1) [],
    ],
    pub PCIeDeviceControlStatus [
        ENABLE_CORRECTABLE_ERROR_REPORTING   OFFSET(0) NUMBITS(1) [],
        ENABLE_NON_FATAL_ERROR_REPORTING   OFFSET(1) NUMBITS(1) [],
        ENABLE_FATAL_ERROR_REPORTING   OFFSET(2) NUMBITS(1) [],
        ENABLE_UNSUPPORTED_REQUEST_REPORTING   OFFSET(3) NUMBITS(1) [],
        ENABLE_RELAXED_ORDERING   OFFSET(4) NUMBITS(1) [],
        MAX_PAYLOAD_SIZE   OFFSET(5) NUMBITS(3) [
            SIZE_128B = 0,
            SIZE_256B = 1,
            SIZE_512B = 2,
        ],
        EXTENDED_TAG_FIELD_ENABLE OFFSET(8) NUMBITS(1) [],
        ENABLE_NO_SNOOP OFFSET(11) NUMBITS(1) [],
        MAXIMUM_READ_REQUEST_SIZE OFFSET(14) NUMBITS(3) [
            SIZE_128B = 0,
            SIZE_256B = 1,
            SIZE_512B = 2,
        ],
        FUNCTION_LEVEL_RESET OFFSET(15) NUMBITS(1) [],
        CORRECTABLE_ERROR_DETECTED OFFSET(16) NUMBITS(1) [],
        NON_FATABL_ERROR_DETECTED OFFSET(17) NUMBITS(1) [],
        FATABL_ERROR_DETECTED OFFSET(18) NUMBITS(1) [],
        UNSUPPORTED_REQUEST_DETECTED OFFSET(19) NUMBITS(1) [],
        TRANSACTION_PENDING OFFSET(21) NUMBITS(1) [],
    ],
    pub PCIeLinkCapabilities [
        MAX_SPEED   OFFSET(0) NUMBITS(4) [
            GT2_5 = 1,  // Gen1
            GT5_0 = 2,  // Gen2
            GT8_0 = 3,  // Gen3
        ],
        MAX_LINK_WIDTH  OFFSET(4) NUMBITS(5) [],
        ASPM_SUPPORT_FOR_L0S_STATE  OFFSET(10) NUMBITS(1) [],
        ASPM_SUPPORT_FOR_L1_STATE  OFFSET(11) NUMBITS(1) [],
        LOS_EXIT_LATENCY  OFFSET(12) NUMBITS(2) [],
        L1_EXIT_LATENCY  OFFSET(15) NUMBITS(2) [],
        ASPM_OPTIONALITY_COMPIANCE  OFFSET(22) NUMBITS(1) [],
    ],
    pub PCIeLinkControlStatus [
        ASPM_CONTROL   OFFSET(0) NUMBITS(2) [],
        READ_COMPLETION_BOUNDARY   OFFSET(3) NUMBITS(1) [],
        COMMON_CLOCK_CONFIGURATION   OFFSET(6) NUMBITS(1) [],
        EXTENDED_SYNCH   OFFSET(7) NUMBITS(1) [],
        NEGOTIATED_LINK_SPEED   OFFSET(16) NUMBITS(4) [],
        NEGOTIATED_LINK_WIDTH   OFFSET(20) NUMBITS(6) [],
        SLOT_CLOCK_CONFIGURATION   OFFSET(28) NUMBITS(1) [],
    ],
    pub PCIeCapabilies2 [
        COMPLETION_TIMEOUT_RANGES   OFFSET(0) NUMBITS(4) [],
        COMPLETION_TIMEOUT_DISABLE_SUPPORTED   OFFSET(4) NUMBITS(1) [],
    ],
    pub PCIeDeviceControlStatus2 [
        COMPLETION_TIMEOUT_VALUE   OFFSET(0) NUMBITS(4) [],
        COMPLETION_TIMEOUT_DISABLE   OFFSET(4) NUMBITS(1) [],
    ],
    pub PCIeLinkCapabilities2[
        LINK_SPEED_SUPPORTED                   OFFSET(1) NUMBITS(3) [
            Gen1 = 1,
            Gen2 = 2,
            Gen3 = 3,
        ],
    ],
    pub PCIeLinkControlStatus2[
        TARGET_LINK_SPEED                   OFFSET(0) NUMBITS(4) [
            Gen1 = 1,
            Gen2 = 2,
            Gen3 = 3,
        ],
        ENTER_COMPLIANCE                    OFFSET(4) NUMBITS(1) [],
        HARDWARE_AUTONOMOUS_SPEED_DISABLE   OFFSET(5) NUMBITS(1) [],
        SELECTABLE_DE_EMPHASIS              OFFSET(6) NUMBITS(1) [],
        TRANSMIT_MARGIN                     OFFSET(7) NUMBITS(3) [],
        ENTER_MIDIFIED_COMPLIANCE           OFFSET(10) NUMBITS(1) [],
        COMPLIANCE_SOS                      OFFSET(11) NUMBITS(1) [],
        COMPLIANCE_PRESET_DE_EMPHASIS       OFFSET(12) NUMBITS(4) [],

        CURRENT_DE_EMPHASIS_LEVEL           OFFSET(16) NUMBITS(1) [],
        EQUALIZATION_COMPLITE               OFFSET(17) NUMBITS(1) [],
        EQUALIZATION_PHASE1_SUCCESSFUL      OFFSET(18) NUMBITS(1) [],
        EQUALIZATION_PHASE2_SUCCESSFUL      OFFSET(19) NUMBITS(1) [],
        EQUALIZATION_PHASE3_SUCCESSFUL      OFFSET(20) NUMBITS(1) [],
        LINK_EQUALIZATION_REQUEST           OFFSET(21) NUMBITS(1) [],
    ],
    pub PCIeRootControlCapabilities [
        SYSTEM_ERROR_ON_CORRECTABLE_ERROR   OFFSET(0) NUMBITS(1) [],
        SYSTEM_ERROR_ON_NON_FATAL_ERROR   OFFSET(1) NUMBITS(1) [],
        SYSTEM_ERROR_ON_FATAL_ERROR   OFFSET(3) NUMBITS(1) [],
        PME_INTERRUPT_ENABLE   OFFSET(4) NUMBITS(1) [],
        CRS_SOFTWARE_VISIBILITY_ENABLE   OFFSET(5) NUMBITS(1) [],
        CRS_SOFTWARE_VISIABILITY_CAPABILITY   OFFSET(16) NUMBITS(1) [],
    ],
    pub PCIeRootStatus [
        PME_STATUS   OFFSET(16) NUMBITS(1) [],
        PME_PENDING   OFFSET(17) NUMBITS(1) [],
    ]
);

pub enum LinkSpeedGen {
    Gen1,
    Gen2,
    Gen3,
}
