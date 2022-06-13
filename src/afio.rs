#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Event Control Register (AFIO_ECR)"]
    pub ecr: crate::Reg<ecr::ECR_SPEC>,
    #[doc = "0x04 - AF remap and debug I/O configuration register (AFIO_PCFR)"]
    pub pcfr: crate::Reg<pcfr::PCFR_SPEC>,
    #[doc = "0x08 - External interrupt configuration register 1 (AFIO_EXTICR1)"]
    pub exticr1: crate::Reg<exticr1::EXTICR1_SPEC>,
    #[doc = "0x0c - External interrupt configuration register 2 (AFIO_EXTICR2)"]
    pub exticr2: crate::Reg<exticr2::EXTICR2_SPEC>,
    #[doc = "0x10 - External interrupt configuration register 3 (AFIO_EXTICR3)"]
    pub exticr3: crate::Reg<exticr3::EXTICR3_SPEC>,
    #[doc = "0x14 - External interrupt configuration register 4 (AFIO_EXTICR4)"]
    pub exticr4: crate::Reg<exticr4::EXTICR4_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - AF remap and debug I/O configuration register (AFIO_PCFR2)"]
    pub pcfr2: crate::Reg<pcfr2::PCFR2_SPEC>,
}

#[doc = "ECR register accessor: an alias for `Reg<ECR_SPEC>`"]
pub type ECR = crate::Reg<ecr::ECR_SPEC>;
#[doc = "Event Control Register (AFIO_ECR)"]
pub mod ecr;

#[doc = "PCFR register accessor: an alias for `Reg<PCFR_SPEC>`"]
pub type PCFR = crate::Reg<pcfr::PCFR_SPEC>;
#[doc = "AF remap and debug I/O configuration register (AFIO_PCFR)"]
pub mod pcfr;

#[doc = "EXTICR1 register accessor: an alias for `Reg<EXTICR1_SPEC>`"]
pub type EXTICR1 = crate::Reg<exticr1::EXTICR1_SPEC>;
#[doc = "External interrupt configuration register 1 (AFIO_EXTICR1)"]
pub mod exticr1;

#[doc = "EXTICR2 register accessor: an alias for `Reg<EXTICR2_SPEC>`"]
pub type EXTICR2 = crate::Reg<exticr2::EXTICR2_SPEC>;
#[doc = "External interrupt configuration register 2 (AFIO_EXTICR2)"]
pub mod exticr2;

#[doc = "EXTICR3 register accessor: an alias for `Reg<EXTICR3_SPEC>`"]
pub type EXTICR3 = crate::Reg<exticr3::EXTICR3_SPEC>;
#[doc = "External interrupt configuration register 3 (AFIO_EXTICR3)"]
pub mod exticr3;

#[doc = "EXTICR4 register accessor: an alias for `Reg<EXTICR4_SPEC>`"]
pub type EXTICR4 = crate::Reg<exticr4::EXTICR4_SPEC>;
#[doc = "External interrupt configuration register 4 (AFIO_EXTICR4)"]
pub mod exticr4;

#[doc = "PCFR2 register accessor: an alias for `Reg<PCFR2_SPEC>`"]
pub type PCFR2 = crate::Reg<pcfr2::PCFR2_SPEC>;
#[doc = "AF remap and debug I/O configuration register (AFIO_PCFR2)"]
pub mod pcfr2;