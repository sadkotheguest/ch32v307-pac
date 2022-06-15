#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 1"]
    pub ctlr1: crate::Reg<ctlr1::CTLR1_SPEC>,
    #[doc = "0x04 - control register 2"]
    pub ctlr2: crate::Reg<ctlr2::CTLR2_SPEC>,
    #[doc = "0x08 - status register"]
    pub statr: crate::Reg<statr::STATR_SPEC>,
    #[doc = "0x0c - data register"]
    pub datar: crate::Reg<datar::DATAR_SPEC>,
    #[doc = "0x10 - CRCR polynomial register"]
    pub crcr: crate::Reg<crcr::CRCR_SPEC>,
    #[doc = "0x14 - RX CRC register"]
    pub rcrcr: crate::Reg<rcrcr::RCRCR_SPEC>,
    #[doc = "0x18 - TX CRC register"]
    pub tcrcr: crate::Reg<tcrcr::TCRCR_SPEC>,
    #[doc = "0x1c - I2S configuration register"]
    pub i2scfgr: crate::Reg<i2scfgr::I2SCFGR_SPEC>,
    #[doc = "0x20 - I2S prescaler register"]
    pub i2spr: crate::Reg<i2spr::I2SPR_SPEC>,
}

#[doc = "CTLR1 register accessor: an alias for `Reg<CTLR1_SPEC>`"]
pub type CTLR1 = crate::Reg<ctlr1::CTLR1_SPEC>;
#[doc = "control register 1"]
pub mod ctlr1;

#[doc = "CTLR2 register accessor: an alias for `Reg<CTLR2_SPEC>`"]
pub type CTLR2 = crate::Reg<ctlr2::CTLR2_SPEC>;
#[doc = "control register 2"]
pub mod ctlr2;

#[doc = "STATR register accessor: an alias for `Reg<STATR_SPEC>`"]
pub type STATR = crate::Reg<statr::STATR_SPEC>;
#[doc = "status register"]
pub mod statr;

#[doc = "DATAR register accessor: an alias for `Reg<DATAR_SPEC>`"]
pub type DATAR = crate::Reg<datar::DATAR_SPEC>;
#[doc = "data register"]
pub mod datar;

#[doc = "CRCR register accessor: an alias for `Reg<CRCR_SPEC>`"]
pub type CRCR = crate::Reg<crcr::CRCR_SPEC>;
#[doc = "CRCR polynomial register"]
pub mod crcr;

#[doc = "RCRCR register accessor: an alias for `Reg<RCRCR_SPEC>`"]
pub type RCRCR = crate::Reg<rcrcr::RCRCR_SPEC>;
#[doc = "RX CRC register"]
pub mod rcrcr;

#[doc = "TCRCR register accessor: an alias for `Reg<TCRCR_SPEC>`"]
pub type TCRCR = crate::Reg<tcrcr::TCRCR_SPEC>;
#[doc = "TX CRC register"]
pub mod tcrcr;

#[doc = "I2SCFGR register accessor: an alias for `Reg<I2SCFGR_SPEC>`"]
pub type I2SCFGR = crate::Reg<i2scfgr::I2SCFGR_SPEC>;
#[doc = "I2S configuration register"]
pub mod i2scfgr;

#[doc = "I2SPR register accessor: an alias for `Reg<I2SPR_SPEC>`"]
pub type I2SPR = crate::Reg<i2spr::I2SPR_SPEC>;
#[doc = "I2S prescaler register"]
pub mod i2spr;