use crate::{pcie_generic, systimer::wait};
use defmt::debug;
use tock_registers::{
    interfaces::{ReadWriteable, Readable, Writeable},
    registers::{InMemoryRegister, ReadOnly, ReadWrite},
};
use tock_registers::{register_bitfields, register_structs};

register_bitfields![
    u32,
    pub MdioPacket [
        CMD            OFFSET(20) NUMBITS(12) [],
        PORT            OFFSET(16) NUMBITS(4) [],
        REG_ADDRESS     OFFSET(0) NUMBITS(16) [],
    ],
    pub MdioRW [
        DONE        OFFSET(31) NUMBITS(1) [],
        DATA        OFFSET(0) NUMBITS(31) [],
    ]
];

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum MdioCmd {
    Read = 1,
    Write = 0,
}

impl From<MdioCmd> for u32 {
    fn from(value: MdioCmd) -> Self {
        value as u32
    }
}

register_bitfields![
    u64,
    pub RcBar [
        // ref: https://github.com/raspberrypi/linux/blob/ae8a4ce56fcac6cfd2bf9c3bbbdd939725c1ae45/drivers/pci/controller/pcie-brcmstb.c#L370-L371
        ADDRESS                OFFSET(5) NUMBITS(64-5) [],
        SIZE                OFFSET(0) NUMBITS(5) [],
    ],
    pub UbusBarRemap [
        ADDRESS OFFSET(12) NUMBITS(28) [],
        ENABLE  OFFSET(0) NUMBITS(1) [],
    ],
];

register_bitfields! [
    u32,
    pub BridgeLinkState [
        LINK_STATE    OFFSET(16) NUMBITS(4) [
            GT8_0 = 3,
            GT5_0 = 2,
            GT2_5 = 1,
            NOT_UP = 0,
        ],
    ],
    pub BridgeState [
        PORT    OFFSET(7) NUMBITS(1) [],
        PORT_2712    OFFSET(6) NUMBITS(1) [],
        DL_ACTIVE    OFFSET(5) NUMBITS(1) [],
        PHY_LINK_STATE    OFFSET(4) NUMBITS(1) [
            UP = 1,
            DOWN = 0,
        ],
    ],
    pub HardwareRevision [
        REVISION    OFFSET(0) NUMBITS(16) [],
    ],
    pub BridgeSerdesMode [
        CLKREQ_DEBUG_ENABLE OFFSET(1) NUMBITS(1) [],
        PERST_ASSERT OFFSET(7) NUMBITS(1) [],
        CLKREQ_L1SS_ENABLE OFFSET(21) NUMBITS(1) [],
        SERDES_IDDQ OFFSET(27) NUMBITS(1) [],
    ],
    pub ConfigAddress [
        PCI_BUS OFFSET(20) NUMBITS(8) [],
        PCI_SLOT OFFSET(15) NUMBITS(5) [],
        PCI_FUNC OFFSET(12) NUMBITS(3) [],
        PCI_REG OFFSET(0) NUMBITS(12) [],
    ],
    pub PhyControl15 [
        PM_CLOCK_PERIOD OFFSET(0) NUMBITS(8) [],
    ],
    pub PcieControl [
        PERSTB OFFSET(2) NUMBITS(1) [],
        L223_REQUEST OFFSET(0) NUMBITS(1) [],
    ],
    pub DmaConfig [
        PCIE_RCB_64B_MODE   OFFSET(7) NUMBITS(1) [],
        PCIE_RCB_MPS_MODE   OFFSET(10) NUMBITS(1) [],
        SCB_ACCES_ENABLE    OFFSET(12) NUMBITS(1) [],
        CFG_READ_UR_MODE    OFFSET(13) NUMBITS(1) [],
        MAX_BURST_SIZE      OFFSET(20) NUMBITS(2) [
            SIZE_128 = 1,
            SIZE_256 = 2,
            SIZE_512 = 3,
        ],
        SCB0_SIZE OFFSET(27) NUMBITS(5) [],
        SCB1_SIZE OFFSET(22) NUMBITS(5) [],
        SCB2_SIZE OFFSET(0) NUMBITS(5) [],
    ],
    pub UbusControl [
        PCIE_REPLY_ERROR_DISABLE OFFSET(13) NUMBITS(1) [],
        PCIE_REPLY_DECERR_DISABLE OFFSET(19) NUMBITS(1) [],
    ],
    pub AxiIntfControl[
        AXI_REQFIFO_EN_QOS_PROPAGATION  OFFSET(7) NUMBITS(1) [],
    ],
    pub MiscControl [
        OUTBOUND_NO_SNOOP       OFFSET(3)   NUMBITS(1) [],
        OUTBOUND_RO             OFFSET(4)   NUMBITS(1) [],
        EN_VDM_QOS_CONTROL      OFFSET(5)   NUMBITS(1) [],
    ],
    pub VdmControl0 [
        ENABLED             OFFSET(16) NUMBITS(1) [],
        IGNORE_TAG          OFFSET(17) NUMBITS(1) [],
        IGNORE_VENDOR_ID    OFFSET(18) NUMBITS(1) [],
    ],
    pub OutboundCpuWindowLimit [
        LIMIT OFFSET(20) NUMBITS(12) [],
        BASE  OFFSET(4) NUMBITS(12) [],
    ],
    pub ConfigPrivate1IdVal3 [
        BASE_CLASS              OFFSET(16)  NUMBITS(8) [],
        SUB_CLASS               OFFSET(8)   NUMBITS(8) [],
        PROGRAMMING_INTERFACE   OFFSET(0)   NUMBITS(8) [],
    ],
    pub VendorSpecificReg1 [
        ENDIAN_MODE_BAR2    OFFSET(2) NUMBITS(3) [
            LITTLE = 0,
        ],
    ],
    pub RcBarLow [
        // ref: https://github.com/raspberrypi/linux/blob/ae8a4ce56fcac6cfd2bf9c3bbbdd939725c1ae45/drivers/pci/controller/pcie-brcmstb.c#L370-L371
        ADDRESS_LOWER_32    OFFSET(5) NUMBITS(32-5) [],
        SIZE                OFFSET(0) NUMBITS(5) [],
    ],
    pub OutboundCpuWindow [
        ADDR_HIGHER_32BIT OFFSET(0) NUMBITS(8) [],
    ],
    pub RcCfgPriv1LinkCapability [
        ASPM_SUPPORT OFFSET(10) NUMBITS(2) [],
    ],
];

register_structs! {
    pub Bcm2712PcieRegisters {
        // pci configuration space of root complex(rc)
        (0x0000 => pub rc_pci_config: [ReadWrite<u32>; 0x40/4]),
        (0x0040 => _reserved0040),
        // 0xac - 0xe0(52bytes): PCI Express Capability Structure space
        // ref: https://www.intel.com/content/www/us/en/docs/programmable/683686/20-4/pci-express-capability-structure.html
        (0x00ac => pub rc_pcie_capability_structure: [ReadWrite<u32>;13]),
        (0x00e0 => _reserved00e0),
        (0x0188 => pub vendor_specific_reg1: ReadWrite<u32,VendorSpecificReg1::Register>),
        (0x018c => _reserved018c),
        (0x043c => pub config_private_1_id_val3: ReadWrite<u32, ConfigPrivate1IdVal3::Register>),
        (0x0440=> _reserved0440),
        (0x04dc=> pub rc_cfg_priv1_link_capability: ReadWrite<u32, RcCfgPriv1LinkCapability::Register>),
        (0x04e0=> _reserved04e0),
        (0x0a0c => pub vdm_control1: ReadWrite<u32>),
        (0x0a10 => _reserved0a10),
        (0x0a20 => pub vdm_control0: ReadWrite<u32, VdmControl0::Register>),
        (0x0a24 => _reserved0a24),
        // 0x0000 - 0x0FFF: PCIe extended config space?
        (0x1100 => pub mdio_address: ReadWrite<u32,MdioPacket::Register>),
        (0x1104 => pub mdio_data_write: ReadWrite<u32,MdioRW::Register>),
        (0x1108 => pub mdio_data_read: ReadOnly<u32,MdioRW::Register>),
        (0x110c => _reserved110c),
        (0x184c => pub phy_control_15: ReadWrite<u32, PhyControl15::Register>),
        (0x1850 => _reserved1850),
        (0x4008 => pub dma_config: ReadWrite<u32,DmaConfig::Register>),
        (0x400c => pub outbound_bus_window: [[ReadWrite<u32>;2]; 4]), // PCIE_MISC_CPU_2_PCIE_MEM_WIN0_*
        (0x402c => pub inbound_bar_gisb_window_low: ReadWrite<u32,RcBarLow::Register>),     // RC_BAR1
        (0x4030 => pub inbound_bar_gisb_window_high: ReadWrite<u32>),     // RC_BAR1
        (0x4034 => pub inbount_bar_dma_window_low: ReadWrite<u32,RcBarLow::Register>),             // RC_BAR2
        (0x4038 => pub inbount_bar_dma_window_high: ReadWrite<u32>),             // RC_BAR2
        (0x403c => pub inbount_bar_scb_window_low: ReadWrite<u32,RcBarLow::Register>),            // RC_BAR3
        (0x4040 => pub inbount_bar_scb_window_high: ReadWrite<u32>),            // RC_BAR3
        (0x4044 => _reserved4044),
        (0x405c => pub crs_retry_timeout: ReadWrite<u32>),
        (0x4060 => _reserved4060),
        (0x4064 => pub pcie_control: ReadWrite<u32,PcieControl::Register>),
        (0x4068 => pub bridge_state: ReadOnly<u32,BridgeState::Register>),
        (0x406c => pub hardware_revision: ReadOnly<u32>),
        (0x4070 => pub outbound_cpu_window_limit: [ReadWrite<u32, OutboundCpuWindowLimit::Register>; 4]),
        // TODO: u64でのアクセスができるか確認する
        (0x4080 => pub outbound_cpu_window: [[ReadWrite<u32,OutboundCpuWindow::Register>;2]; 4]),
        (0x40a0 => pub misc_control: ReadWrite<u32, MiscControl::Register>),
        (0x40a4 => pub ubus_control: ReadWrite<u32,UbusControl::Register>),
        (0x40a8 => pub ubus_timeout: ReadWrite<u32>),
        (0x40ac => _reserved40ac),
        (0x40b4 => pub ubus_bar2_remap_config: [ReadWrite<u32>; 2]),
        (0x40bc => _reserved40bc),
        // RC_BAR4 ~ RC_BAR10
        (0x40d4 => pub rc_bar4to10: [[ReadWrite<u32>;2];7]),
        (0x410c => pub ubus_bar4to10_remap_config: [[ReadWrite<u32>;2];7]),
        (0x4144 => _reserved4144),
        (0x4164 => pub vdm_priority_to_qos_map_high: ReadWrite<u32>),
        (0x4168 => pub vdm_priority_to_qos_map_lo: ReadWrite<u32>),
        (0x416c => pub axi_intf_control: ReadWrite<u32,AxiIntfControl::Register>),
        (0x4170 => pub axi_read_error_data: ReadWrite<u32>),
        (0x4174 => _reserved4174),
        (0x4304 => pub bridge_serdes_mode: ReadWrite<u32, BridgeSerdesMode::Register>),
        (0x4308 => _reserved4308),
        (0x4410 => pub cpu_interrupt_mask_set: ReadWrite<u32>),
        (0x4414 => pub cpu_interrupt_mask_clear: ReadWrite<u32>),
        (0x4418 => _reserved4018),
        (0x8000 => pub config_data: [ReadWrite<u32>; 0x1000/4]),
        (0x9000 => pub config_address: ReadWrite<u32, ConfigAddress::Register>),
        (0x9004 => _reserved9004),
        (0x9310 => @END),
    }
}

pub fn log2_u64(value: u64) -> u8 {
    for shift in (0..64).rev() {
        let mask = 1 << shift;
        if (value & mask) != 0 {
            return shift;
        }
    }
    return 0;
}

pub fn encode_inbound_window_size(window_size: u64) -> u8 {
    let window_size_log_2 = log2_u64(window_size);
    match window_size_log_2 {
        12..=15 => {
            // 4KiB - 32KiB
            (window_size_log_2 - 12) + 0x1c
        }
        16..=36 => {
            // 64KiB - 64GiB
            window_size_log_2 - 15
        }
        _ => 0,
    }
}

pub struct RcInOutboundParam {
    cpu_base_address: u64,
    bus_base_address: u64,
    size: u64,
}
impl RcInOutboundParam {
    pub fn new(cpu_base_address: u64, bus_base_address: u64, size: u64) -> Self {
        Self {
            cpu_base_address,
            bus_base_address,
            size,
        }
    }
}

pub struct BcmStbPcie {
    pub pcie: &'static mut Bcm2712PcieRegisters,
    enable: bool,
}

impl BcmStbPcie {
    pub fn new(pcie_base_addr: u64) -> Self {
        let pcie: &'static mut Bcm2712PcieRegisters =
            unsafe { &mut *(pcie_base_addr as *mut Bcm2712PcieRegisters) };
        Self {
            pcie,
            enable: false,
        }
    }

    pub fn rc_pci_header_type1(&mut self) -> &'static mut pcie_generic::GenericPciHeaderType1 {
        let pci_header: &'static mut pcie_generic::GenericPciHeaderType1 = unsafe {
            pcie_generic::GenericPciHeaderType1::new_ref(
                self.pcie.rc_pci_config.as_mut_ptr() as *mut u32
            )
        };
        pci_header
    }

    pub fn rc_pcie_cap(&mut self) -> &'static mut pcie_generic::PCIeCapabilityStructure {
        let pcie_cap: &'static mut pcie_generic::PCIeCapabilityStructure = unsafe {
            &mut *(self.pcie.rc_pcie_capability_structure.as_mut_ptr()
                as *mut pcie_generic::PCIeCapabilityStructure)
        };
        pcie_cap
    }

    fn mdio_inner(&mut self, address: u32, cmd: MdioCmd, data: u32) -> Result<u32, ()> {
        // create mdio packet
        let mdio_packet: InMemoryRegister<u32, MdioPacket::Register> = InMemoryRegister::new(0);
        mdio_packet.modify(MdioPacket::CMD.val(cmd.into()));
        mdio_packet.modify(MdioPacket::PORT.val(0));
        mdio_packet.modify(MdioPacket::REG_ADDRESS.val(address));
        // write mdio packet
        self.pcie.mdio_address.set(mdio_packet.get());
        // read mdio
        let address_verify = self.pcie.mdio_address.get();
        debug_assert_eq!(mdio_packet.get(), address_verify);

        if matches!(cmd, MdioCmd::Write) {
            let mdio_write_packet: InMemoryRegister<u32, MdioRW::Register> =
                InMemoryRegister::new(0);
            mdio_write_packet.modify(MdioRW::DONE::SET);
            mdio_write_packet.modify(MdioRW::DATA.val(data));
            self.pcie.mdio_data_write.set(mdio_write_packet.get());
        }

        const MAX_RETRY: usize = 10;
        for _i in 0..MAX_RETRY {
            if matches!(cmd, MdioCmd::Write) {
                let data = self.pcie.mdio_data_write.extract();
                if !data.is_set(MdioRW::DONE) {
                    return Ok(0);
                }
            } else {
                // read
                let data = self.pcie.mdio_data_read.extract();
                if data.is_set(MdioRW::DONE) {
                    return Ok(data.read(MdioRW::DATA));
                }
            };
            wait(core::time::Duration::from_millis(10));
        }
        Err(())
    }

    pub fn mdio_read_u32(&mut self, address: u32) -> Result<u32, ()> {
        self.mdio_inner(address, MdioCmd::Read, 0)
    }

    pub fn mdio_write_u32(&mut self, address: u32, data: u32) -> Result<(), ()> {
        self.mdio_inner(address, MdioCmd::Write, data)?;
        Ok(())
    }

    fn assert_perst(&mut self) {
        self.pcie.pcie_control.modify(PcieControl::PERSTB::CLEAR);
    }

    fn deassert_perst(&mut self) {
        self.pcie.pcie_control.modify(PcieControl::PERSTB::SET);
    }

    /// setup PCI->CPU(inbound) memory window
    /// >  This encodes the inbound window showing the system memory to the controller.
    /// > ref: freebsd sys/arm/broadcom/bcm2835/bcm2838_pci.c:657
    fn setup_inbound_window(
        &mut self,
        index: usize,
        cpu_memory_address: u64,
        pcie_base_address: u64,
        window_size: u64,
    ) -> Result<(), ()> {
        match index {
            0..=7 => {
                // use bar2(0) and bar4 ~ bar10
                ()
            }
            _ => {
                // invalid range
                return Err(());
            }
        }
        // ref: FreeBSD sys/arm/broadcom/bcm2835/bcm2838_pci.c:656
        let window_size = encode_inbound_window_size(window_size);
        let rc_val =
            (pcie_base_address & RcBar::ADDRESS.mask) | RcBar::SIZE.val(window_size.into()).value;
        let ubus_rc_val =
            (cpu_memory_address & UbusBarRemap::ADDRESS.mask) | UbusBarRemap::ENABLE::SET.value;

        // setup RC and UBUS window
        if index == 0 {
            // use bar2
            self.pcie.inbount_bar_dma_window_low.set(rc_val as u32);
            self.pcie
                .inbount_bar_dma_window_high
                .set((rc_val >> 32) as u32);
            self.pcie.ubus_bar2_remap_config[0].set(ubus_rc_val as u32);
            self.pcie.ubus_bar2_remap_config[1].set((ubus_rc_val >> 32) as u32);
            self.pcie
                .dma_config
                .modify(DmaConfig::SCB0_SIZE.val(window_size.into()));
        } else {
            // use bar4~bar10
            let index = index - 1;
            self.pcie.rc_bar4to10[index][0].set(rc_val as u32);
            self.pcie.rc_bar4to10[index][1].set((rc_val >> 32) as u32);
            self.pcie.ubus_bar4to10_remap_config[index][0].set(ubus_rc_val as u32);
            self.pcie.ubus_bar4to10_remap_config[index][1].set((ubus_rc_val >> 32) as u32);
        }
        Ok(())
    }

    /// CPU -> PCI memory window
    fn setup_outbound_window(
        &mut self,
        index: usize,
        cpu_memory_address: u64,
        pcie_base_address: u64,
        window_size: u64,
    ) -> Result<(), ()> {
        if 4 < index {
            return Err(());
        }

        // convert address(byte) to MegaBytes
        const SIZE_1MB: u64 = 1024 * 1024;
        let cpu_address_start_mb = cpu_memory_address / SIZE_1MB;
        let cpu_address_end_mb = (cpu_memory_address + window_size - 1) / SIZE_1MB;

        // set limit
        let limit_window = self.pcie.outbound_cpu_window_limit.get_mut(index).unwrap();
        limit_window.set(
            OutboundCpuWindowLimit::LIMIT
                .val(cpu_address_end_mb as u32)
                .value
                | OutboundCpuWindowLimit::BASE
                    .val(cpu_address_start_mb as u32)
                    .value,
        );

        // set window range
        // (address / 4GiB) as u8(4 ~ 1024GiB)
        let start = OutboundCpuWindow::ADDR_HIGHER_32BIT.val((cpu_memory_address >> 32) as u32);
        let end = OutboundCpuWindow::ADDR_HIGHER_32BIT
            .val(((cpu_memory_address + window_size - 1) >> 32) as u32);
        self.pcie.outbound_cpu_window[index][0].set(start.value);
        self.pcie.outbound_cpu_window[index][1].set(end.value);

        // set bus window
        self.pcie.outbound_bus_window[index][0].set(pcie_base_address as u32);
        self.pcie.outbound_bus_window[index][1].set((pcie_base_address >> 32) as u32);

        Ok(())
    }

    fn setup_axi_qos_priority(&mut self, vdm_to_qos_map: Option<u32>) {
        let vdm_to_qos_map = match vdm_to_qos_map {
            Some(x) => x,
            None => return,
        };

        self.pcie
            .axi_intf_control
            .modify(AxiIntfControl::AXI_REQFIFO_EN_QOS_PROPAGATION::CLEAR);

        if vdm_to_qos_map == 0 {
            self.pcie
                .misc_control
                .modify(MiscControl::EN_VDM_QOS_CONTROL::CLEAR);
        }

        self.pcie
            .misc_control
            .modify(MiscControl::EN_VDM_QOS_CONTROL::SET);
        // set vdm_to_qos_map
        // ref: https://github.com/raspberrypi/linux/blob/ae8a4ce56fcac6cfd2bf9c3bbbdd939725c1ae45/drivers/pci/controller/pcie-brcmstb.c#L590-L609
        self.pcie.vdm_priority_to_qos_map_high.set(vdm_to_qos_map);
        self.pcie.vdm_priority_to_qos_map_lo.set(vdm_to_qos_map);

        self.pcie.vdm_control1.set(0);
        let mut tmp = self.pcie.vdm_control0.extract();
        tmp.modify(VdmControl0::ENABLED::SET);
        tmp.modify(VdmControl0::IGNORE_TAG::SET);
        tmp.modify(VdmControl0::IGNORE_VENDOR_ID::SET);
        self.pcie.vdm_control0.set(tmp.into());
    }

    fn reset_phy(&mut self) {
        // enable ShaDes
        self.pcie
            .bridge_serdes_mode
            .modify(BridgeSerdesMode::SERDES_IDDQ::CLEAR);

        wait(core::time::Duration::from_millis(100));

        // clock setup //
        let data = [
            (0x1f, 0x1600),
            (0x16, 0x50b9),
            (0x17, 0xbda1),
            (0x18, 0x0094),
            (0x19, 0x97b4),
            (0x1b, 0x5030),
            (0x1c, 0x5030),
            (0x1e, 0x0007),
        ];
        for (a, d) in data {
            self.mdio_write_u32(a, d).unwrap();
        }
        wait(core::time::Duration::from_millis(100));

        // Tweak PM period for 54MHz clock
        // ref: EDK2 code
        // Silicon/Broadcom/Bcm27xx/Library/Bcm2712PciHostBridgeLib/Bcm2712PciHostBridge.c:103
        self.pcie
            .phy_control_15
            .modify(PhyControl15::PM_CLOCK_PERIOD.val(
                18, // ns
            ));

        // dma config
        self.pcie.dma_config.set(
            DmaConfig::MAX_BURST_SIZE::SIZE_128.value
                | DmaConfig::SCB_ACCES_ENABLE::SET.value
                | DmaConfig::CFG_READ_UR_MODE::SET.value,
        );

        // suppress axi error
        // ref: https://github.com/raspberrypi/linux/blob/ae8a4ce56fcac6cfd2bf9c3bbbdd939725c1ae45/drivers/pci/controller/pcie-brcmstb.c#L1249-L1270
        self.pcie.ubus_control.set(
            UbusControl::PCIE_REPLY_ERROR_DISABLE::SET.value
                | UbusControl::PCIE_REPLY_DECERR_DISABLE::SET.value,
        );
        self.pcie.axi_read_error_data.set(0xFFFF_FFFF);
        // set ubus timeout to ~250ms
        // set CRS timeout to ~240ms
        self.pcie.ubus_timeout.set(0x0b2d_0000);
        self.pcie.crs_retry_timeout.set(0x0aba_0000);

        // setup AXI QoS priority
        // TODO: 設定値を取ってくるようにする
        self.setup_axi_qos_priority(Some(0xbbaa9888));

        // disable PCIe->GISB window(RC_BAR1)
        self.pcie
            .inbound_bar_gisb_window_low
            .modify(RcBarLow::SIZE::CLEAR);
        // disable PCIe->SCB window(RC_BAR3)
        self.pcie
            .inbount_bar_scb_window_low
            .modify(RcBarLow::SIZE::CLEAR);
    }

    pub fn set_link_speed(&mut self, speed: pcie_generic::LinkSpeedGen) {
        let (link_speed, max_speed) = match speed {
            pcie_generic::LinkSpeedGen::Gen1 => (
                pcie_generic::PCIeLinkControlStatus2::TARGET_LINK_SPEED::Gen1,
                pcie_generic::PCIeLinkCapabilities::MAX_SPEED::GT2_5,
            ),
            pcie_generic::LinkSpeedGen::Gen2 => (
                pcie_generic::PCIeLinkControlStatus2::TARGET_LINK_SPEED::Gen2,
                pcie_generic::PCIeLinkCapabilities::MAX_SPEED::GT5_0,
            ),
            pcie_generic::LinkSpeedGen::Gen3 => (
                pcie_generic::PCIeLinkControlStatus2::TARGET_LINK_SPEED::Gen3,
                pcie_generic::PCIeLinkCapabilities::MAX_SPEED::GT8_0,
            ),
        };
        self.rc_pcie_cap().link_control_status2.modify(link_speed);
        self.rc_pcie_cap().link_capabilities.modify(max_speed);
    }

    fn set_aspm(&mut self) {
        self.pcie.bridge_serdes_mode.modify(
            // BridgeSerdesMode::CLKREQ_DEBUG_ENABLE::SET +
            BridgeSerdesMode::CLKREQ_L1SS_ENABLE::CLEAR,
        );

        // ref
        // https://www.intel.com/content/www/us/en/docs/programmable/683488/16-0/pci-express-capability-structure.html
        // no l0s state for pcie2
        self.rc_pcie_cap().link_capabilities.modify(
            pcie_generic::PCIeLinkCapabilities::ASPM_SUPPORT_FOR_L0S_STATE::CLEAR
                + pcie_generic::PCIeLinkCapabilities::ASPM_SUPPORT_FOR_L1_STATE::SET,
        );
    }

    pub fn reset(
        &mut self,
        outbound32: &RcInOutboundParam,
        outbound64: &RcInOutboundParam,
        inbound: &RcInOutboundParam,
    ) {
        // assert PERST
        self.assert_perst();
        wait(core::time::Duration::from_millis(100));

        // reset phy
        self.reset_phy();

        // setup inbound(CPU <- PCIe) window
        self.setup_inbound_window(
            0,
            inbound.cpu_base_address,
            inbound.bus_base_address,
            inbound.size,
        )
        .unwrap();

        // setup outbound window
        self.setup_outbound_window(
            0,
            outbound32.cpu_base_address,
            outbound32.bus_base_address,
            outbound32.size,
        )
        .unwrap();

        // setup outbound window
        self.setup_outbound_window(
            1,
            outbound64.cpu_base_address,
            outbound64.bus_base_address,
            outbound64.size,
        )
        .unwrap();

        // clear and disable interrupt
        self.pcie.cpu_interrupt_mask_clear.set(0xFFFF_FFFF);
        self.pcie.cpu_interrupt_mask_set.set(0xFFFF_FFFF);

        // program root port class to "PCI to PCI bridge"
        // ref: https://pcisig.com/sites/default/files/files/PCI_Code-ID_r_1_11__v24_Jan_2019.pdf
        let mut tmp = self.pcie.config_private_1_id_val3.extract();
        tmp.modify(ConfigPrivate1IdVal3::BASE_CLASS.val(0x06));
        tmp.modify(ConfigPrivate1IdVal3::SUB_CLASS.val(0x04));
        tmp.modify(ConfigPrivate1IdVal3::PROGRAMMING_INTERFACE.val(0x00));
        self.pcie.config_private_1_id_val3.set(tmp.into());

        // set litte-endian mode for BAR
        // PCIe -> SCB
        self.pcie
            .vendor_specific_reg1
            .modify(VendorSpecificReg1::ENDIAN_MODE_BAR2::LITTLE);

        // set link speed
        // WIP: assume Gen2
        self.set_link_speed(pcie_generic::LinkSpeedGen::Gen2);

        // skip ASPM setting
        // MEMO: 設定しなくても動きはしそうなのでskipしてみる
        self.set_aspm();

        // self.pcie
        //     .dma_config
        //     .modify(DmaConfig::PCIE_RCB_MPS_MODE::SET);
    }

    pub fn enable(&mut self) -> Result<(), ()> {
        // enable phase
        self.deassert_perst();
        wait(core::time::Duration::from_millis(1000));

        // wait for contorller to start
        const PCIE_RETRY_COUNT: usize = 10;
        let mut retry_counter = 0;
        let mut bridge_state = self.pcie.bridge_state.extract();
        while !(bridge_state.is_set(BridgeState::PHY_LINK_STATE)
            && bridge_state.is_set(BridgeState::DL_ACTIVE))
        {
            if PCIE_RETRY_COUNT < retry_counter {
                debug!("failed to start pcie controller");
                return Err(());
            }
            retry_counter += 1;
            debug!("pcie controller not started. please wait...");
            // debug!("bridge_state: {:#x}", bridge_state);
            wait(core::time::Duration::from_millis(500));
            bridge_state = self.pcie.bridge_state.extract();
        }

        // TODO: check link states

        // if !self
        //     .pcie
        //     .bridge_link_state
        //     .is_set(BridgeLinkState::LINK_STATE)
        // {
        //     debug!("failed to link");
        //     return Err(());
        // }

        // debug!(
        //     "bridge_link_state: {:#x}",
        //     self.pcie.bridge_link_state.get()
        // );

        // // TODO: check link speed

        self.enable = true;
        Ok(())
    }

    pub fn hardware_revision(&mut self) -> Result<u16, ()> {
        let rev = self.pcie.hardware_revision.get();
        Ok(rev as u16)
    }

    fn set_pci_address(&mut self, bus: u8, slot: u8, func: u8, reg: u16) {
        // ConfigAddress
        let address = ConfigAddress::PCI_BUS.val(bus.into())
            + ConfigAddress::PCI_SLOT.val(slot.into())
            + ConfigAddress::PCI_FUNC.val(func.into())
            + ConfigAddress::PCI_REG.val(reg.into());
        self.pcie.config_address.set(address.into());
    }

    pub fn read_pci_config_data(&mut self, bus: u8, slot: u8, func: u8, reg: u16) -> u32 {
        if bus == 0 && slot == 0 && func == 0 {
            // special case for root devicer
            // ref: FreeBSD sys/arm/broadcom/bcm2835/bcm2838_pci.c
            // TODO: rc_pci_configの範囲を拡張して、もともとのコードは*_pci_config_dataを使うようにする
            self.pcie.rc_pci_config[usize::from(reg)].get()
        } else {
            self.set_pci_address(bus, slot, func, 0);
            self.pcie.config_data[usize::from(reg)].get()
        }
    }

    pub fn write_pci_config_data(
        &mut self,
        bus: u8,
        slot: u8,
        func: u8,
        reg: u16,
        data: u32,
    ) -> () {
        if bus == 0 && slot == 0 && func == 0 {
            // special case for root devicer
            // ref: FreeBSD sys/arm/broadcom/bcm2835/bcm2838_pci.c
            // TODO: rc_pci_configの範囲を拡張して、もともとのコードは*_pci_config_dataを使うようにする
            self.pcie.rc_pci_config[usize::from(reg)].set(data)
        } else {
            // TODO: unalign access
            self.set_pci_address(bus, slot, func, 0);
            self.pcie.config_data[usize::from(reg)].set(data)
        }
    }
}
