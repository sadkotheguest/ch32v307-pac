#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register (WWDG_CR)"]
    pub ctlr: crate::Reg<ctlr::CTLR_SPEC>,
    #[doc = "0x04 - Configuration register (WWDG_CFR)"]
    pub cfgr: crate::Reg<cfgr::CFGR_SPEC>,
    #[doc = "0x08 - Status register (WWDG_SR)"]
    pub statr: crate::Reg<statr::STATR_SPEC>,
}

#[doc = "CTLR register accessor: an alias for `Reg<CTLR_SPEC>`"]
pub type CTLR = crate::Reg<ctlr::CTLR_SPEC>;
#[doc = "Control register (WWDG_CR)"]
pub mod ctlr;

#[doc = "CFGR register accessor: an alias for `Reg<CFGR_SPEC>`"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "Configuration register (WWDG_CFR)"]
pub mod cfgr;

#[doc = "STATR register accessor: an alias for `Reg<STATR_SPEC>`"]
pub type STATR = crate::Reg<statr::STATR_SPEC>;
#[doc = "Status register (WWDG_SR)"]
pub mod statr;