#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 1"]
    pub ctlr1: crate::Reg<ctlr1::CTLR1_SPEC>,
    #[doc = "0x04 - control register 2"]
    pub ctlr2: crate::Reg<ctlr2::CTLR2_SPEC>,
    #[doc = "0x08 - slave mode control register"]
    pub smcfgr: crate::Reg<smcfgr::SMCFGR_SPEC>,
    #[doc = "0x0c - DMA/Interrupt enable register"]
    pub dmaintenr: crate::Reg<dmaintenr::DMAINTENR_SPEC>,
    #[doc = "0x10 - status register"]
    pub intfr: crate::Reg<intfr::INTFR_SPEC>,
    #[doc = "0x14 - event generation register"]
    pub swevgr: crate::Reg<swevgr::SWEVGR_SPEC>,
    _reserved_6_chctlr1: [u8; 0x04],
    _reserved_7_chctlr2: [u8; 0x04],
    #[doc = "0x20 - capture/compare enable register"]
    pub ccer: crate::Reg<ccer::CCER_SPEC>,
    #[doc = "0x24 - counter"]
    pub cnt: crate::Reg<cnt::CNT_SPEC>,
    #[doc = "0x28 - prescaler"]
    pub psc: crate::Reg<psc::PSC_SPEC>,
    #[doc = "0x2c - auto-reload register"]
    pub atrlr: crate::Reg<atrlr::ATRLR_SPEC>,
    #[doc = "0x30 - repetition counter register"]
    pub rptcr: crate::Reg<rptcr::RPTCR_SPEC>,
    #[doc = "0x34 - capture/compare register 1"]
    pub ch1cvr: crate::Reg<ch1cvr::CH1CVR_SPEC>,
    #[doc = "0x38 - capture/compare register 2"]
    pub ch2cvr: crate::Reg<ch2cvr::CH2CVR_SPEC>,
    #[doc = "0x3c - capture/compare register 3"]
    pub ch3cvr: crate::Reg<ch3cvr::CH3CVR_SPEC>,
    #[doc = "0x40 - capture/compare register 4"]
    pub ch4cvr: crate::Reg<ch4cvr::CH4CVR_SPEC>,
    #[doc = "0x44 - break and dead-time register"]
    pub bdtr: crate::Reg<bdtr::BDTR_SPEC>,
    #[doc = "0x48 - DMA control register"]
    pub dmacfgr: crate::Reg<dmacfgr::DMACFGR_SPEC>,
    #[doc = "0x4c - DMA address for full transfer"]
    pub dmaadr: crate::Reg<dmaadr::DMAADR_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x18 - capture/compare mode register 1 (input mode)"]
    #[inline(always)]
    pub fn chctlr1_input(&self) -> &crate::Reg<chctlr1_input::CHCTLR1_INPUT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(24usize)
                as *const crate::Reg<chctlr1_input::CHCTLR1_INPUT_SPEC>)
        }
    }
    #[doc = "0x18 - capture/compare mode register (output mode)"]
    #[inline(always)]
    pub fn chctlr1_output(&self) -> &crate::Reg<chctlr1_output::CHCTLR1_OUTPUT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(24usize)
                as *const crate::Reg<chctlr1_output::CHCTLR1_OUTPUT_SPEC>)
        }
    }
    #[doc = "0x1c - capture/compare mode register 2 (input mode)"]
    #[inline(always)]
    pub fn chctlr2_input(&self) -> &crate::Reg<chctlr2_input::CHCTLR2_INPUT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(28usize)
                as *const crate::Reg<chctlr2_input::CHCTLR2_INPUT_SPEC>)
        }
    }
    #[doc = "0x1c - capture/compare mode register (output mode)"]
    #[inline(always)]
    pub fn chctlr2_output(&self) -> &crate::Reg<chctlr2_output::CHCTLR2_OUTPUT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(28usize)
                as *const crate::Reg<chctlr2_output::CHCTLR2_OUTPUT_SPEC>)
        }
    }
}

#[doc = "CTLR1 register accessor: an alias for `Reg<CTLR1_SPEC>`"]
pub type CTLR1 = crate::Reg<ctlr1::CTLR1_SPEC>;
#[doc = "control register 1"]
pub mod ctlr1;

#[doc = "CTLR2 register accessor: an alias for `Reg<CTLR2_SPEC>`"]
pub type CTLR2 = crate::Reg<ctlr2::CTLR2_SPEC>;
#[doc = "control register 2"]
pub mod ctlr2;

#[doc = "SMCFGR register accessor: an alias for `Reg<SMCFGR_SPEC>`"]
pub type SMCFGR = crate::Reg<smcfgr::SMCFGR_SPEC>;
#[doc = "slave mode control register"]
pub mod smcfgr;

#[doc = "DMAINTENR register accessor: an alias for `Reg<DMAINTENR_SPEC>`"]
pub type DMAINTENR = crate::Reg<dmaintenr::DMAINTENR_SPEC>;
#[doc = "DMA/Interrupt enable register"]
pub mod dmaintenr;

#[doc = "INTFR register accessor: an alias for `Reg<INTFR_SPEC>`"]
pub type INTFR = crate::Reg<intfr::INTFR_SPEC>;
#[doc = "status register"]
pub mod intfr;

#[doc = "SWEVGR register accessor: an alias for `Reg<SWEVGR_SPEC>`"]
pub type SWEVGR = crate::Reg<swevgr::SWEVGR_SPEC>;
#[doc = "event generation register"]
pub mod swevgr;

#[doc = "CHCTLR1_Output register accessor: an alias for `Reg<CHCTLR1_OUTPUT_SPEC>`"]
pub type CHCTLR1_OUTPUT = crate::Reg<chctlr1_output::CHCTLR1_OUTPUT_SPEC>;
#[doc = "capture/compare mode register (output mode)"]
pub mod chctlr1_output;

#[doc = "CHCTLR1_Input register accessor: an alias for `Reg<CHCTLR1_INPUT_SPEC>`"]
pub type CHCTLR1_INPUT = crate::Reg<chctlr1_input::CHCTLR1_INPUT_SPEC>;
#[doc = "capture/compare mode register 1 (input mode)"]
pub mod chctlr1_input;

#[doc = "CHCTLR2_Output register accessor: an alias for `Reg<CHCTLR2_OUTPUT_SPEC>`"]
pub type CHCTLR2_OUTPUT = crate::Reg<chctlr2_output::CHCTLR2_OUTPUT_SPEC>;
#[doc = "capture/compare mode register (output mode)"]
pub mod chctlr2_output;

#[doc = "CHCTLR2_Input register accessor: an alias for `Reg<CHCTLR2_INPUT_SPEC>`"]
pub type CHCTLR2_INPUT = crate::Reg<chctlr2_input::CHCTLR2_INPUT_SPEC>;
#[doc = "capture/compare mode register 2 (input mode)"]
pub mod chctlr2_input;

#[doc = "CCER register accessor: an alias for `Reg<CCER_SPEC>`"]
pub type CCER = crate::Reg<ccer::CCER_SPEC>;
#[doc = "capture/compare enable register"]
pub mod ccer;

#[doc = "CNT register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "counter"]
pub mod cnt;

#[doc = "PSC register accessor: an alias for `Reg<PSC_SPEC>`"]
pub type PSC = crate::Reg<psc::PSC_SPEC>;
#[doc = "prescaler"]
pub mod psc;

#[doc = "ATRLR register accessor: an alias for `Reg<ATRLR_SPEC>`"]
pub type ATRLR = crate::Reg<atrlr::ATRLR_SPEC>;
#[doc = "auto-reload register"]
pub mod atrlr;

#[doc = "RPTCR register accessor: an alias for `Reg<RPTCR_SPEC>`"]
pub type RPTCR = crate::Reg<rptcr::RPTCR_SPEC>;
#[doc = "repetition counter register"]
pub mod rptcr;

#[doc = "CH1CVR register accessor: an alias for `Reg<CH1CVR_SPEC>`"]
pub type CH1CVR = crate::Reg<ch1cvr::CH1CVR_SPEC>;
#[doc = "capture/compare register 1"]
pub mod ch1cvr;

#[doc = "CH2CVR register accessor: an alias for `Reg<CH2CVR_SPEC>`"]
pub type CH2CVR = crate::Reg<ch2cvr::CH2CVR_SPEC>;
#[doc = "capture/compare register 2"]
pub mod ch2cvr;

#[doc = "CH3CVR register accessor: an alias for `Reg<CH3CVR_SPEC>`"]
pub type CH3CVR = crate::Reg<ch3cvr::CH3CVR_SPEC>;
#[doc = "capture/compare register 3"]
pub mod ch3cvr;

#[doc = "CH4CVR register accessor: an alias for `Reg<CH4CVR_SPEC>`"]
pub type CH4CVR = crate::Reg<ch4cvr::CH4CVR_SPEC>;
#[doc = "capture/compare register 4"]
pub mod ch4cvr;

#[doc = "BDTR register accessor: an alias for `Reg<BDTR_SPEC>`"]
pub type BDTR = crate::Reg<bdtr::BDTR_SPEC>;
#[doc = "break and dead-time register"]
pub mod bdtr;

#[doc = "DMACFGR register accessor: an alias for `Reg<DMACFGR_SPEC>`"]
pub type DMACFGR = crate::Reg<dmacfgr::DMACFGR_SPEC>;
#[doc = "DMA control register"]
pub mod dmacfgr;

#[doc = "DMAADR register accessor: an alias for `Reg<DMAADR_SPEC>`"]
pub type DMAADR = crate::Reg<dmaadr::DMAADR_SPEC>;
#[doc = "DMA address for full transfer"]
pub mod dmaadr;