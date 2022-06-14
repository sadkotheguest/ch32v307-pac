#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register (DAC_CR)"]
    pub ctlr: crate::Reg<ctlr::CTLR_SPEC>,
    #[doc = "0x04 - DAC software trigger register (DAC_SWTRIGR)"]
    pub swtr: crate::Reg<swtr::SWTR_SPEC>,
    #[doc = "0x08 - DAC channel1 12-bit right-aligned data holding register(DAC_DHR12R1)"]
    pub r12bdhr1: crate::Reg<r12bdhr1::R12BDHR1_SPEC>,
    #[doc = "0x0c - DAC channel1 12-bit left aligned data holding register (DAC_DHR12L1)"]
    pub l12bdhr1: crate::Reg<l12bdhr1::L12BDHR1_SPEC>,
    #[doc = "0x10 - DAC channel1 8-bit right aligned data holding register (DAC_DHR8R1)"]
    pub r8bdhr1: crate::Reg<r8bdhr1::R8BDHR1_SPEC>,
    #[doc = "0x14 - DAC channel2 12-bit right aligned data holding register (DAC_DHR12R2)"]
    pub r12bdhr2: crate::Reg<r12bdhr2::R12BDHR2_SPEC>,
    #[doc = "0x18 - DAC channel2 12-bit left aligned data holding register (DAC_DHR12L2)"]
    pub l12bdhr2: crate::Reg<l12bdhr2::L12BDHR2_SPEC>,
    #[doc = "0x1c - DAC channel2 8-bit right-aligned data holding register (DAC_DHR8R2)"]
    pub r8bdhr2: crate::Reg<r8bdhr2::R8BDHR2_SPEC>,
    #[doc = "0x20 - Dual DAC 12-bit right-aligned data holding register (DAC_DHR12RD), Bits 31:28 Reserved, Bits 15:12 Reserved"]
    pub rd12bdhr: crate::Reg<rd12bdhr::RD12BDHR_SPEC>,
    #[doc = "0x24 - DUAL DAC 12-bit left aligned data holding register (DAC_DHR12LD), Bits 19:16 Reserved, Bits 3:0 Reserved"]
    pub ld12bdhr: crate::Reg<ld12bdhr::LD12BDHR_SPEC>,
    #[doc = "0x28 - DUAL DAC 8-bit right aligned data holding register (DAC_DHR8RD), Bits 31:16 Reserved"]
    pub rd8bdhr: crate::Reg<rd8bdhr::RD8BDHR_SPEC>,
    #[doc = "0x2c - DAC channel1 data output register (DAC_DOR1)"]
    pub dor1: crate::Reg<dor1::DOR1_SPEC>,
    #[doc = "0x30 - DAC channel2 data output register (DAC_DOR2)"]
    pub dor2: crate::Reg<dor2::DOR2_SPEC>,
}

#[doc = "CTLR register accessor: an alias for `Reg<CTLR_SPEC>`"]
pub type CTLR = crate::Reg<ctlr::CTLR_SPEC>;
#[doc = "Control register (DAC_CR)"]
pub mod ctlr;

#[doc = "SWTR register accessor: an alias for `Reg<SWTR_SPEC>`"]
pub type SWTR = crate::Reg<swtr::SWTR_SPEC>;
#[doc = "DAC software trigger register (DAC_SWTRIGR)"]
pub mod swtr;

#[doc = "R12BDHR1 register accessor: an alias for `Reg<R12BDHR1_SPEC>`"]
pub type R12BDHR1 = crate::Reg<r12bdhr1::R12BDHR1_SPEC>;
#[doc = "DAC channel1 12-bit right-aligned data holding register(DAC_DHR12R1)"]
pub mod r12bdhr1;

#[doc = "L12BDHR1 register accessor: an alias for `Reg<L12BDHR1_SPEC>`"]
pub type L12BDHR1 = crate::Reg<l12bdhr1::L12BDHR1_SPEC>;
#[doc = "DAC channel1 12-bit left aligned data holding register (DAC_DHR12L1)"]
pub mod l12bdhr1;

#[doc = "R8BDHR1 register accessor: an alias for `Reg<R8BDHR1_SPEC>`"]
pub type R8BDHR1 = crate::Reg<r8bdhr1::R8BDHR1_SPEC>;
#[doc = "DAC channel1 8-bit right aligned data holding register (DAC_DHR8R1)"]
pub mod r8bdhr1;

#[doc = "R12BDHR2 register accessor: an alias for `Reg<R12BDHR2_SPEC>`"]
pub type R12BDHR2 = crate::Reg<r12bdhr2::R12BDHR2_SPEC>;
#[doc = "DAC channel2 12-bit right aligned data holding register (DAC_DHR12R2)"]
pub mod r12bdhr2;

#[doc = "L12BDHR2 register accessor: an alias for `Reg<L12BDHR2_SPEC>`"]
pub type L12BDHR2 = crate::Reg<l12bdhr2::L12BDHR2_SPEC>;
#[doc = "DAC channel2 12-bit left aligned data holding register (DAC_DHR12L2)"]
pub mod l12bdhr2;

#[doc = "R8BDHR2 register accessor: an alias for `Reg<R8BDHR2_SPEC>`"]
pub type R8BDHR2 = crate::Reg<r8bdhr2::R8BDHR2_SPEC>;
#[doc = "DAC channel2 8-bit right-aligned data holding register (DAC_DHR8R2)"]
pub mod r8bdhr2;

#[doc = "RD12BDHR register accessor: an alias for `Reg<RD12BDHR_SPEC>`"]
pub type RD12BDHR = crate::Reg<rd12bdhr::RD12BDHR_SPEC>;
#[doc = "Dual DAC 12-bit right-aligned data holding register (DAC_DHR12RD), Bits 31:28 Reserved, Bits 15:12 Reserved"]
pub mod rd12bdhr;

#[doc = "LD12BDHR register accessor: an alias for `Reg<LD12BDHR_SPEC>`"]
pub type LD12BDHR = crate::Reg<ld12bdhr::LD12BDHR_SPEC>;
#[doc = "DUAL DAC 12-bit left aligned data holding register (DAC_DHR12LD), Bits 19:16 Reserved, Bits 3:0 Reserved"]
pub mod ld12bdhr;

#[doc = "RD8BDHR register accessor: an alias for `Reg<RD8BDHR_SPEC>`"]
pub type RD8BDHR = crate::Reg<rd8bdhr::RD8BDHR_SPEC>;
#[doc = "DUAL DAC 8-bit right aligned data holding register (DAC_DHR8RD), Bits 31:16 Reserved"]
pub mod rd8bdhr;

#[doc = "DOR1 register accessor: an alias for `Reg<DOR1_SPEC>`"]
pub type DOR1 = crate::Reg<dor1::DOR1_SPEC>;
#[doc = "DAC channel1 data output register (DAC_DOR1)"]
pub mod dor1;

#[doc = "DOR2 register accessor: an alias for `Reg<DOR2_SPEC>`"]
pub type DOR2 = crate::Reg<dor2::DOR2_SPEC>;
#[doc = "DAC channel2 data output register (DAC_DOR2)"]
pub mod dor2;