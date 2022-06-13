#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt mask register (EXTI_INTENR)"]
    pub intenr: crate::Reg<intenr::INTENR_SPEC>,
    #[doc = "0x04 - Event mask register (EXTI_EVENR)"]
    pub evenr: crate::Reg<evenr::EVENR_SPEC>,
    #[doc = "0x08 - Rising Trigger selection register (EXTI_RTENR)"]
    pub rtenr: crate::Reg<rtenr::RTENR_SPEC>,
    #[doc = "0x0c - Falling Trigger selection register (EXTI_FTENR)"]
    pub ftenr: crate::Reg<ftenr::FTENR_SPEC>,
    #[doc = "0x10 - Software interrupt event register (EXTI_SWIEVR)"]
    pub swievr: crate::Reg<swievr::SWIEVR_SPEC>,
    #[doc = "0x14 - Pending register (EXTI_INTFR)"]
    pub intfr: crate::Reg<intfr::INTFR_SPEC>,
}

#[doc = "INTENR register accessor: an alias for `Reg<INTENR_SPEC>`"]
pub type INTENR = crate::Reg<intenr::INTENR_SPEC>;
#[doc = "Interrupt mask register (EXTI_INTENR)"]
pub mod intenr;

#[doc = "EVENR register accessor: an alias for `Reg<EVENR_SPEC>`"]
pub type EVENR = crate::Reg<evenr::EVENR_SPEC>;
#[doc = "Event mask register (EXTI_EVENR)"]
pub mod evenr;

#[doc = "RTENR register accessor: an alias for `Reg<RTENR_SPEC>`"]
pub type RTENR = crate::Reg<rtenr::RTENR_SPEC>;
#[doc = "Rising Trigger selection register (EXTI_RTENR)"]
pub mod rtenr;

#[doc = "FTENR register accessor: an alias for `Reg<FTENR_SPEC>`"]
pub type FTENR = crate::Reg<ftenr::FTENR_SPEC>;
#[doc = "Falling Trigger selection register (EXTI_FTENR)"]
pub mod ftenr;

#[doc = "SWIEVR register accessor: an alias for `Reg<SWIEVR_SPEC>`"]
pub type SWIEVR = crate::Reg<swievr::SWIEVR_SPEC>;
#[doc = "Software interrupt event register (EXTI_SWIEVR)"]
pub mod swievr;

#[doc = "INTFR register accessor: an alias for `Reg<INTFR_SPEC>`"]
pub type INTFR = crate::Reg<intfr::INTFR_SPEC>;
#[doc = "Pending register (EXTI_INTFR)"]
pub mod intfr;