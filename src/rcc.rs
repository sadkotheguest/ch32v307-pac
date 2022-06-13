#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock control register"]
    pub ctlr: crate::Reg<ctlr::CTLR_SPEC>,
    #[doc = "0x04 - Clock configuration register (RCC_CFGR0)"]
    pub cfgr0: crate::Reg<cfgr0::CFGR0_SPEC>,
    #[doc = "0x08 - Clock interrupt register (RCC_INTR)"]
    pub intr: crate::Reg<intr::INTR_SPEC>,
    #[doc = "0x0c - APB2 peripheral reset register (RCC_APB2PRSTR)"]
    pub apb2prstr: crate::Reg<apb2prstr::APB2PRSTR_SPEC>,
    #[doc = "0x10 - APB1 peripheral reset register (RCC_APB1PRSTR)"]
    pub apb1prstr: crate::Reg<apb1prstr::APB1PRSTR_SPEC>,
    #[doc = "0x14 - AHB Peripheral Clock enable register (RCC_AHBPCENR)"]
    pub ahbpcenr: crate::Reg<ahbpcenr::AHBPCENR_SPEC>,
    #[doc = "0x18 - APB2 peripheral clock enable register (RCC_APB2PCENR)"]
    pub apb2pcenr: crate::Reg<apb2pcenr::APB2PCENR_SPEC>,
    #[doc = "0x1c - APB1 peripheral clock enable register (RCC_APB1PCENR)"]
    pub apb1pcenr: crate::Reg<apb1pcenr::APB1PCENR_SPEC>,
    #[doc = "0x20 - Backup domain control register (RCC_BDCTLR)"]
    pub bdctlr: crate::Reg<bdctlr::BDCTLR_SPEC>,
    #[doc = "0x24 - Control/status register (RCC_RSTSCKR)"]
    pub rstsckr: crate::Reg<rstsckr::RSTSCKR_SPEC>,
    #[doc = "0x28 - AHB reset register (RCC_APHBRSTR)"]
    pub ahbrstr: crate::Reg<ahbrstr::AHBRSTR_SPEC>,
    #[doc = "0x2c - Clock configuration register2 (RCC_CFGR2)"]
    pub cfgr2: crate::Reg<cfgr2::CFGR2_SPEC>,
}

#[doc = "CTLR register accessor: an alias for `Reg<CTLR_SPEC>`"]
pub type CTLR = crate::Reg<ctlr::CTLR_SPEC>;
#[doc = "Clock control register"]
pub mod ctlr;

#[doc = "CFGR0 register accessor: an alias for `Reg<CFGR0_SPEC>`"]
pub type CFGR0 = crate::Reg<cfgr0::CFGR0_SPEC>;
#[doc = "Clock configuration register (RCC_CFGR0)"]
pub mod cfgr0;

#[doc = "INTR register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Clock interrupt register (RCC_INTR)"]
pub mod intr;

#[doc = "APB2PRSTR register accessor: an alias for `Reg<APB2PRSTR_SPEC>`"]
pub type APB2PRSTR = crate::Reg<apb2prstr::APB2PRSTR_SPEC>;
#[doc = "APB2 peripheral reset register (RCC_APB2PRSTR)"]
pub mod apb2prstr;

#[doc = "APB1PRSTR register accessor: an alias for `Reg<APB1PRSTR_SPEC>`"]
pub type APB1PRSTR = crate::Reg<apb1prstr::APB1PRSTR_SPEC>;
#[doc = "APB1 peripheral reset register (RCC_APB1PRSTR)"]
pub mod apb1prstr;

#[doc = "AHBPCENR register accessor: an alias for `Reg<AHBPCENR_SPEC>`"]
pub type AHBPCENR = crate::Reg<ahbpcenr::AHBPCENR_SPEC>;
#[doc = "AHB Peripheral Clock enable register (RCC_AHBPCENR)"]
pub mod ahbpcenr;

#[doc = "APB2PCENR register accessor: an alias for `Reg<APB2PCENR_SPEC>`"]
pub type APB2PCENR = crate::Reg<apb2pcenr::APB2PCENR_SPEC>;
#[doc = "APB2 peripheral clock enable register (RCC_APB2PCENR)"]
pub mod apb2pcenr;

#[doc = "APB1PCENR register accessor: an alias for `Reg<APB1PCENR_SPEC>`"]
pub type APB1PCENR = crate::Reg<apb1pcenr::APB1PCENR_SPEC>;
#[doc = "APB1 peripheral clock enable register (RCC_APB1PCENR)"]
pub mod apb1pcenr;

#[doc = "BDCTLR register accessor: an alias for `Reg<BDCTLR_SPEC>`"]
pub type BDCTLR = crate::Reg<bdctlr::BDCTLR_SPEC>;
#[doc = "Backup domain control register (RCC_BDCTLR)"]
pub mod bdctlr;

#[doc = "RSTSCKR register accessor: an alias for `Reg<RSTSCKR_SPEC>`"]
pub type RSTSCKR = crate::Reg<rstsckr::RSTSCKR_SPEC>;
#[doc = "Control/status register (RCC_RSTSCKR)"]
pub mod rstsckr;

#[doc = "AHBRSTR register accessor: an alias for `Reg<AHBRSTR_SPEC>`"]
pub type AHBRSTR = crate::Reg<ahbrstr::AHBRSTR_SPEC>;
#[doc = "AHB reset register (RCC_APHBRSTR)"]
pub mod ahbrstr;

#[doc = "CFGR2 register accessor: an alias for `Reg<CFGR2_SPEC>`"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
#[doc = "Clock configuration register2 (RCC_CFGR2)"]
pub mod cfgr2;
