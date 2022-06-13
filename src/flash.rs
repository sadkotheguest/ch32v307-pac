#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Flash access control register"]
    pub actlr: crate::Reg<actlr::ACTLR_SPEC>,
    #[doc = "0x04 - Flash key register"]
    pub keyr: crate::Reg<keyr::KEYR_SPEC>,
    #[doc = "0x08 - Flash option key register"]
    pub obkeyr: crate::Reg<obkeyr::OBKEYR_SPEC>,
    #[doc = "0x0c - Status register"]
    pub statr: crate::Reg<statr::STATR_SPEC>,
    #[doc = "0x10 - Control register"]
    pub ctlr: crate::Reg<ctlr::CTLR_SPEC>,
    #[doc = "0x14 - Flash address register"]
    pub addr: crate::Reg<addr::ADDR_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - Option byte register"]
    pub obr: crate::Reg<obr::OBR_SPEC>,
    #[doc = "0x20 - Write protection register"]
    pub wpr: crate::Reg<wpr::WPR_SPEC>,
    #[doc = "0x24 - Mode select register"]
    pub modekeyr: crate::Reg<modekeyr::MODEKEYR_SPEC>,
}

#[doc = "ACTLR register accessor: an alias for `Reg<ACTLR_SPEC>`"]
pub type ACTLR = crate::Reg<actlr::ACTLR_SPEC>;
#[doc = "Flash access control register"]
pub mod actlr;

#[doc = "KEYR register accessor: an alias for `Reg<KEYR_SPEC>`"]
pub type KEYR = crate::Reg<keyr::KEYR_SPEC>;
#[doc = "Flash key register"]
pub mod keyr;

#[doc = "OBKEYR register accessor: an alias for `Reg<OBKEYR_SPEC>`"]
pub type OBKEYR = crate::Reg<obkeyr::OBKEYR_SPEC>;
#[doc = "Flash option key register"]
pub mod obkeyr;

#[doc = "STATR register accessor: an alias for `Reg<STATR_SPEC>`"]
pub type STATR = crate::Reg<statr::STATR_SPEC>;
#[doc = "Status register"]
pub mod statr;

#[doc = "CTLR register accessor: an alias for `Reg<CTLR_SPEC>`"]
pub type CTLR = crate::Reg<ctlr::CTLR_SPEC>;
#[doc = "Control register"]
pub mod ctlr;

#[doc = "ADDR register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "Flash address register"]
pub mod addr;

#[doc = "OBR register accessor: an alias for `Reg<OBR_SPEC>`"]
pub type OBR = crate::Reg<obr::OBR_SPEC>;
#[doc = "Option byte register"]
pub mod obr;

#[doc = "WPR register accessor: an alias for `Reg<WPR_SPEC>`"]
pub type WPR = crate::Reg<wpr::WPR_SPEC>;
#[doc = "Write protection register"]
pub mod wpr;

#[doc = "MODEKEYR register accessor: an alias for `Reg<MODEKEYR_SPEC>`"]
pub type MODEKEYR = crate::Reg<modekeyr::MODEKEYR_SPEC>;
#[doc = "Mode select register"]
pub mod modekeyr;