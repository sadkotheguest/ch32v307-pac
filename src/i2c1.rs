#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register 1"]
    pub ctlr1: crate::Reg<ctlr1::CTLR1_SPEC>,
    #[doc = "0x04 - Control register 2"]
    pub ctlr2: crate::Reg<ctlr2::CTLR2_SPEC>,
    #[doc = "0x08 - Own address register 1"]
    pub oaddr1: crate::Reg<oaddr1::OADDR1_SPEC>,
    #[doc = "0x0c - Own address register 2"]
    pub oaddr2: crate::Reg<oaddr2::OADDR2_SPEC>,
    #[doc = "0x10 - Data register"]
    pub datar: crate::Reg<datar::DATAR_SPEC>,
    #[doc = "0x14 - Status register 1"]
    pub star1: crate::Reg<star1::STAR1_SPEC>,
    #[doc = "0x18 - Status register 2"]
    pub star2: crate::Reg<star2::STAR2_SPEC>,
    #[doc = "0x1c - Clock control register"]
    pub ckcfgr: crate::Reg<ckcfgr::CKCFGR_SPEC>,
    #[doc = "0x20 - Raise time register"]
    pub rtr: crate::Reg<rtr::RTR_SPEC>,
}

#[doc = "CTLR1 register accessor: an alias for `Reg<CTLR1_SPEC>`"]
pub type CTLR1 = crate::Reg<ctlr1::CTLR1_SPEC>;
#[doc = "Control register 1"]
pub mod ctlr1;

#[doc = "CTLR2 register accessor: an alias for `Reg<CTLR2_SPEC>`"]
pub type CTLR2 = crate::Reg<ctlr2::CTLR2_SPEC>;
#[doc = "Control register 2"]
pub mod ctlr2;

#[doc = "OADDR1 register accessor: an alias for `Reg<OADDR1_SPEC>`"]
pub type OADDR1 = crate::Reg<oaddr1::OADDR1_SPEC>;
#[doc = "Own address register 1"]
pub mod oaddr1;

#[doc = "OADDR2 register accessor: an alias for `Reg<OADDR2_SPEC>`"]
pub type OADDR2 = crate::Reg<oaddr2::OADDR2_SPEC>;
#[doc = "Own address register 2"]
pub mod oaddr2;

#[doc = "DATAR register accessor: an alias for `Reg<DATAR_SPEC>`"]
pub type DATAR = crate::Reg<datar::DATAR_SPEC>;
#[doc = "Data register"]
pub mod datar;

#[doc = "STAR1 register accessor: an alias for `Reg<STAR1_SPEC>`"]
pub type STAR1 = crate::Reg<star1::STAR1_SPEC>;
#[doc = "Status register 1"]
pub mod star1;

#[doc = "STAR2 register accessor: an alias for `Reg<STAR2_SPEC>`"]
pub type STAR2 = crate::Reg<star2::STAR2_SPEC>;
#[doc = "Status register 2"]
pub mod star2;

#[doc = "CKCFGR register accessor: an alias for `Reg<CKCFGR_SPEC>`"]
pub type CKCFGR = crate::Reg<ckcfgr::CKCFGR_SPEC>;
#[doc = "Clock control register"]
pub mod ckcfgr;

#[doc = "RTR register accessor: an alias for `Reg<RTR_SPEC>`"]
pub type RTR = crate::Reg<rtr::RTR_SPEC>;
#[doc = "Raise time register"]
pub mod rtr;