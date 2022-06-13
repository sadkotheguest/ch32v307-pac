#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port configuration register low (GPIOn_CFGLR)"]
    pub cfglr: crate::Reg<cfglr::CFGLR_SPEC>,
    #[doc = "0x04 - Port configuration register high (GPIOn_CFGHR)"]
    pub cfghr: crate::Reg<cfghr::CFGHR_SPEC>,
    #[doc = "0x08 - Port input data register (GPIOn_INDR)"]
    pub indr: crate::Reg<indr::INDR_SPEC>,
    #[doc = "0x0c - Port output data register (GPIOn_OUTDR)"]
    pub outdr: crate::Reg<outdr::OUTDR_SPEC>,
    #[doc = "0x10 - Port bit set/reset register (GPIOn_BSHR)"]
    pub bshr: crate::Reg<bshr::BSHR_SPEC>,
    #[doc = "0x14 - Port bit reset register (GPIOn_BCR)"]
    pub bcr: crate::Reg<bcr::BCR_SPEC>,
    #[doc = "0x18 - Port configuration lock register"]
    pub lckr: crate::Reg<lckr::LCKR_SPEC>,
}

#[doc = "CFGLR register accessor: an alias for `Reg<CFGLR_SPEC>`"]
pub type CFGLR = crate::Reg<cfglr::CFGLR_SPEC>;
#[doc = "Port configuration register low (GPIOn_CFGLR)"]
pub mod cfglr;

#[doc = "CFGHR register accessor: an alias for `Reg<CFGHR_SPEC>`"]
pub type CFGHR = crate::Reg<cfghr::CFGHR_SPEC>;
#[doc = "Port configuration register high (GPIOn_CFGHR)"]
pub mod cfghr;

#[doc = "INDR register accessor: an alias for `Reg<INDR_SPEC>`"]
pub type INDR = crate::Reg<indr::INDR_SPEC>;
#[doc = "Port input data register (GPIOn_INDR)"]
pub mod indr;

#[doc = "OUTDR register accessor: an alias for `Reg<OUTDR_SPEC>`"]
pub type OUTDR = crate::Reg<outdr::OUTDR_SPEC>;
#[doc = "Port output data register (GPIOn_OUTDR)"]
pub mod outdr;

#[doc = "BSHR register accessor: an alias for `Reg<BSHR_SPEC>`"]
pub type BSHR = crate::Reg<bshr::BSHR_SPEC>;
#[doc = "Port bit set/reset register (GPIOn_BSHR)"]
pub mod bshr;

#[doc = "BCR register accessor: an alias for `Reg<BCR_SPEC>`"]
pub type BCR = crate::Reg<bcr::BCR_SPEC>;
#[doc = "Port bit reset register (GPIOn_BCR)"]
pub mod bcr;

#[doc = "LCKR register accessor: an alias for `Reg<LCKR_SPEC>`"]
pub type LCKR = crate::Reg<lckr::LCKR_SPEC>;
#[doc = "Port configuration lock register"]
pub mod lckr;
