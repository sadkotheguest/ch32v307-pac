#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC Control Register High"]
    pub ctlrh: crate::Reg<ctlrh::CTLRH_SPEC>,
    #[doc = "0x04 - RTC Control Register Low"]
    pub ctlrl: crate::Reg<ctlrl::CTLRL_SPEC>,
    #[doc = "0x08 - RTC Prescaler Load Register High"]
    pub pscrh: crate::Reg<pscrh::PSCRH_SPEC>,
    #[doc = "0x0c - RTC Prescaler Load Register Low"]
    pub pscrl: crate::Reg<pscrl::PSCRL_SPEC>,
    #[doc = "0x10 - RTC Prescaler Divider Register High"]
    pub divh: crate::Reg<divh::DIVH_SPEC>,
    #[doc = "0x14 - RTC Prescaler Divider Register Low"]
    pub divl: crate::Reg<divl::DIVL_SPEC>,
    #[doc = "0x18 - RTC Counter Register High"]
    pub cnth: crate::Reg<cnth::CNTH_SPEC>,
    #[doc = "0x1c - RTC Counter Register Low"]
    pub cntl: crate::Reg<cntl::CNTL_SPEC>,
    #[doc = "0x20 - RTC Alarm Register High"]
    pub alrmh: crate::Reg<alrmh::ALRMH_SPEC>,
    #[doc = "0x24 - RTC Alarm Register Low"]
    pub alrml: crate::Reg<alrml::ALRML_SPEC>,
}

#[doc = "CTLRH register accessor: an alias for `Reg<CTLRH_SPEC>`"]
pub type CTLRH = crate::Reg<ctlrh::CTLRH_SPEC>;
#[doc = "RTC Control Register High"]
pub mod ctlrh;

#[doc = "CTLRL register accessor: an alias for `Reg<CTLRL_SPEC>`"]
pub type CTLRL = crate::Reg<ctlrl::CTLRL_SPEC>;
#[doc = "RTC Control Register Low"]
pub mod ctlrl;

#[doc = "PSCRH register accessor: an alias for `Reg<PSCRH_SPEC>`"]
pub type PSCRH = crate::Reg<pscrh::PSCRH_SPEC>;
#[doc = "RTC Prescaler Load Register High"]
pub mod pscrh;

#[doc = "PSCRL register accessor: an alias for `Reg<PSCRL_SPEC>`"]
pub type PSCRL = crate::Reg<pscrl::PSCRL_SPEC>;
#[doc = "RTC Prescaler Load Register Low"]
pub mod pscrl;

#[doc = "DIVH register accessor: an alias for `Reg<DIVH_SPEC>`"]
pub type DIVH = crate::Reg<divh::DIVH_SPEC>;
#[doc = "RTC Prescaler Divider Register High"]
pub mod divh;

#[doc = "DIVL register accessor: an alias for `Reg<DIVL_SPEC>`"]
pub type DIVL = crate::Reg<divl::DIVL_SPEC>;
#[doc = "RTC Prescaler Divider Register Low"]
pub mod divl;

#[doc = "CNTH register accessor: an alias for `Reg<CNTH_SPEC>`"]
pub type CNTH = crate::Reg<cnth::CNTH_SPEC>;
#[doc = "RTC Counter Register High"]
pub mod cnth;

#[doc = "CNTL register accessor: an alias for `Reg<CNTL_SPEC>`"]
pub type CNTL = crate::Reg<cntl::CNTL_SPEC>;
#[doc = "RTC Counter Register Low"]
pub mod cntl;

#[doc = "ALRMH register accessor: an alias for `Reg<ALRMH_SPEC>`"]
pub type ALRMH = crate::Reg<alrmh::ALRMH_SPEC>;
#[doc = "RTC Alarm Register High"]
pub mod alrmh;

#[doc = "ALRML register accessor: an alias for `Reg<ALRML_SPEC>`"]
pub type ALRML = crate::Reg<alrml::ALRML_SPEC>;
#[doc = "RTC Alarm Register Low"]
pub mod alrml;