#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Status register"]
    pub statr: crate::Reg<statr::STATR_SPEC>,
    #[doc = "0x04 - Data register"]
    pub datar: crate::Reg<datar::DATAR_SPEC>,
    #[doc = "0x08 - Baud rate register"]
    pub brr: crate::Reg<brr::BRR_SPEC>,
    #[doc = "0x0c - Control register 1"]
    pub ctlr1: crate::Reg<ctlr1::CTLR1_SPEC>,
    #[doc = "0x10 - Control register 2"]
    pub ctlr2: crate::Reg<ctlr2::CTLR2_SPEC>,
    #[doc = "0x14 - Control register 3"]
    pub ctlr3: crate::Reg<ctlr3::CTLR3_SPEC>,
    #[doc = "0x18 - Guard time and prescaler register"]
    pub gpr: crate::Reg<gpr::GPR_SPEC>,
}

#[doc = "STATR register accessor: an alias for `Reg<STATR_SPEC>`"]
pub type STATR = crate::Reg<statr::STATR_SPEC>;
#[doc = "Status register"]
pub mod statr;

#[doc = "DATAR register accessor: an alias for `Reg<DATAR_SPEC>`"]
pub type DATAR = crate::Reg<datar::DATAR_SPEC>;
#[doc = "Data register"]
pub mod datar;

#[doc = "BRR register accessor: an alias for `Reg<BRR_SPEC>`"]
pub type BRR = crate::Reg<brr::BRR_SPEC>;
#[doc = "Baud rate register"]
pub mod brr;

#[doc = "CTLR1 register accessor: an alias for `Reg<CTLR1_SPEC>`"]
pub type CTLR1 = crate::Reg<ctlr1::CTLR1_SPEC>;
#[doc = "Control register 1"]
pub mod ctlr1;

#[doc = "CTLR2 register accessor: an alias for `Reg<CTLR2_SPEC>`"]
pub type CTLR2 = crate::Reg<ctlr2::CTLR2_SPEC>;
#[doc = "Control register 2"]
pub mod ctlr2;

#[doc = "CTLR3 register accessor: an alias for `Reg<CTLR3_SPEC>`"]
pub type CTLR3 = crate::Reg<ctlr3::CTLR3_SPEC>;
#[doc = "Control register 3"]
pub mod ctlr3;

#[doc = "GPR register accessor: an alias for `Reg<GPR_SPEC>`"]
pub type GPR = crate::Reg<gpr::GPR_SPEC>;
#[doc = "Guard time and prescaler register"]
pub mod gpr;