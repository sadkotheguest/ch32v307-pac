#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EXTEND register"]
    pub extend_ctr: crate::Reg<extend_ctr::EXTEND_CTR_SPEC>,
}

#[doc = "EXTEND_CTR register accessor: an alias for `Reg<EXTEND_CTR_SPEC>`"]
pub type EXTEND_CTR = crate::Reg<extend_ctr::EXTEND_CTR_SPEC>;
#[doc = "EXTEND register"]
pub mod extend_ctr;