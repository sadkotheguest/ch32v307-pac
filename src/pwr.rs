#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power control register (PWR_CTRL)"]
    pub ctlr: crate::Reg<ctlr::CTLR_SPEC>,
    #[doc = "0x04 - Power control register (PWR_CSR)"]
    pub csr: crate::Reg<csr::CSR_SPEC>,
}

#[doc = "CTLR register accessor: an alias for `Reg<CTLR_SPEC>`"]
pub type CTLR = crate::Reg<ctlr::CTLR_SPEC>;
#[doc = "Power control register (PWR_CTRL)"]
pub mod ctlr;

#[doc = "CSR register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "Power control register (PWR_CSR)"]
pub mod csr;