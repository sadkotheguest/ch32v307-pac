#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DBGMCU_CFGR1"]
    pub cfgr1: crate::Reg<cfgr1::CFGR1_SPEC>,
    #[doc = "0x04 - DBGMCU_CFGR2"]
    pub cfgr2: crate::Reg<cfgr2::CFGR2_SPEC>,
}
#[doc = "CFGR1 register accessor: an alias for `Reg<CFGR1_SPEC>`"]
pub type CFGR1 = crate::Reg<cfgr1::CFGR1_SPEC>;
#[doc = "DBGMCU_CFGR1"]
pub mod cfgr1;

#[doc = "CFGR2 register accessor: an alias for `Reg<CFGR2_SPEC>`"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
#[doc = "DBGMCU_CFGR2"]
pub mod cfgr2;