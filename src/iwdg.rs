#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Key register (IWDG_CTLR)"]
    pub ctlr: crate::Reg<ctlr::CTLR_SPEC>,
    #[doc = "0x04 - Prescaler register (IWDG_PSCR)"]
    pub pscr: crate::Reg<pscr::PSCR_SPEC>,
    #[doc = "0x08 - Reload register (IWDG_RLDR)"]
    pub rldr: crate::Reg<rldr::RLDR_SPEC>,
    #[doc = "0x0c - Status register (IWDG_SR)"]
    pub statr: crate::Reg<statr::STATR_SPEC>,
}

#[doc = "CTLR register accessor: an alias for `Reg<CTLR_SPEC>`"]
pub type CTLR = crate::Reg<ctlr::CTLR_SPEC>;
#[doc = "Key register (IWDG_CTLR)"]
pub mod ctlr;

#[doc = "PSCR register accessor: an alias for `Reg<PSCR_SPEC>`"]
pub type PSCR = crate::Reg<pscr::PSCR_SPEC>;
#[doc = "Prescaler register (IWDG_PSCR)"]
pub mod pscr;

#[doc = "RLDR register accessor: an alias for `Reg<RLDR_SPEC>`"]
pub type RLDR = crate::Reg<rldr::RLDR_SPEC>;
#[doc = "Reload register (IWDG_RLDR)"]
pub mod rldr;

#[doc = "STATR register accessor: an alias for `Reg<STATR_SPEC>`"]
pub type STATR = crate::Reg<statr::STATR_SPEC>;
#[doc = "Status register (IWDG_SR)"]
pub mod statr;