#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data register"]
    pub datar: crate::Reg<datar::DATAR_SPEC>,
    #[doc = "0x04 - Independent Data register"]
    pub idatar: crate::Reg<idatar::IDATAR_SPEC>,
    #[doc = "0x08 - Control register"]
    pub ctlr: crate::Reg<ctlr::CTLR_SPEC>,
}

#[doc = "DATAR register accessor: an alias for `Reg<DATAR_SPEC>`"]
pub type DATAR = crate::Reg<datar::DATAR_SPEC>;
#[doc = "Data register"]
pub mod datar;

#[doc = "IDATAR register accessor: an alias for `Reg<IDATAR_SPEC>`"]
pub type IDATAR = crate::Reg<idatar::IDATAR_SPEC>;
#[doc = "Independent Data register"]
pub mod idatar;

#[doc = "CTLR register accessor: an alias for `Reg<CTLR_SPEC>`"]
pub type CTLR = crate::Reg<ctlr::CTLR_SPEC>;
#[doc = "Control register"]
pub mod ctlr;