#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - status register"]
    pub statr: crate::Reg<statr::STATR_SPEC>,
    #[doc = "0x04 - control register 1"]
    pub ctlr1: crate::Reg<ctlr1::CTLR1_SPEC>,
    #[doc = "0x08 - control register 2"]
    pub ctlr2: crate::Reg<ctlr2::CTLR2_SPEC>,
    #[doc = "0x0c - sample time register 1"]
    pub samptr1: crate::Reg<samptr1::SAMPTR1_SPEC>,
    #[doc = "0x10 - sample time register 2"]
    pub samptr2: crate::Reg<samptr2::SAMPTR2_SPEC>,
    #[doc = "0x14 - injected channel data offset register x"]
    pub iofr1: crate::Reg<iofr1::IOFR1_SPEC>,
    #[doc = "0x18 - injected channel data offset register x"]
    pub iofr2: crate::Reg<iofr2::IOFR2_SPEC>,
    #[doc = "0x1c - injected channel data offset register x"]
    pub iofr3: crate::Reg<iofr3::IOFR3_SPEC>,
    #[doc = "0x20 - injected channel data offset register x"]
    pub iofr4: crate::Reg<iofr4::IOFR4_SPEC>,
    #[doc = "0x24 - watchdog higher threshold register"]
    pub wdhtr: crate::Reg<wdhtr::WDHTR_SPEC>,
    #[doc = "0x28 - watchdog lower threshold register"]
    pub wdltr: crate::Reg<wdltr::WDLTR_SPEC>,
    #[doc = "0x2c - regular sequence register 1"]
    pub rsqr1: crate::Reg<rsqr1::RSQR1_SPEC>,
    #[doc = "0x30 - regular sequence register 2"]
    pub rsqr2: crate::Reg<rsqr2::RSQR2_SPEC>,
    #[doc = "0x34 - regular sequence register 3"]
    pub rsqr3: crate::Reg<rsqr3::RSQR3_SPEC>,
    #[doc = "0x38 - injected sequence register"]
    pub isqr: crate::Reg<isqr::ISQR_SPEC>,
    #[doc = "0x3c - injected data register x"]
    pub idatar1: crate::Reg<idatar1::IDATAR1_SPEC>,
    #[doc = "0x40 - injected data register x"]
    pub idatar2: crate::Reg<idatar2::IDATAR2_SPEC>,
    #[doc = "0x44 - injected data register x"]
    pub idatar3: crate::Reg<idatar3::IDATAR3_SPEC>,
    #[doc = "0x48 - injected data register x"]
    pub idatar4: crate::Reg<idatar4::IDATAR4_SPEC>,
    #[doc = "0x4c - regular data register"]
    pub rdatar: crate::Reg<rdatar::RDATAR_SPEC>,
}

#[doc = "STATR register accessor: an alias for `Reg<STATR_SPEC>`"]
pub type STATR = crate::Reg<statr::STATR_SPEC>;
#[doc = "status register"]
pub mod statr;

#[doc = "CTLR1 register accessor: an alias for `Reg<CTLR1_SPEC>`"]
pub type CTLR1 = crate::Reg<ctlr1::CTLR1_SPEC>;
#[doc = "control register 1"]
pub mod ctlr1;

#[doc = "CTLR2 register accessor: an alias for `Reg<CTLR2_SPEC>`"]
pub type CTLR2 = crate::Reg<ctlr2::CTLR2_SPEC>;
#[doc = "control register 2"]
pub mod ctlr2;

#[doc = "SAMPTR1 register accessor: an alias for `Reg<SAMPTR1_SPEC>`"]
pub type SAMPTR1 = crate::Reg<samptr1::SAMPTR1_SPEC>;
#[doc = "sample time register 1"]
pub mod samptr1;

#[doc = "SAMPTR2 register accessor: an alias for `Reg<SAMPTR2_SPEC>`"]
pub type SAMPTR2 = crate::Reg<samptr2::SAMPTR2_SPEC>;
#[doc = "sample time register 2"]
pub mod samptr2;

#[doc = "IOFR1 register accessor: an alias for `Reg<IOFR1_SPEC>`"]
pub type IOFR1 = crate::Reg<iofr1::IOFR1_SPEC>;
#[doc = "injected channel data offset register x"]
pub mod iofr1;

#[doc = "IOFR2 register accessor: an alias for `Reg<IOFR2_SPEC>`"]
pub type IOFR2 = crate::Reg<iofr2::IOFR2_SPEC>;
#[doc = "injected channel data offset register x"]
pub mod iofr2;

#[doc = "IOFR3 register accessor: an alias for `Reg<IOFR3_SPEC>`"]
pub type IOFR3 = crate::Reg<iofr3::IOFR3_SPEC>;
#[doc = "injected channel data offset register x"]
pub mod iofr3;

#[doc = "IOFR4 register accessor: an alias for `Reg<IOFR4_SPEC>`"]
pub type IOFR4 = crate::Reg<iofr4::IOFR4_SPEC>;
#[doc = "injected channel data offset register x"]
pub mod iofr4;

#[doc = "WDHTR register accessor: an alias for `Reg<WDHTR_SPEC>`"]
pub type WDHTR = crate::Reg<wdhtr::WDHTR_SPEC>;
#[doc = "watchdog higher threshold register"]
pub mod wdhtr;

#[doc = "WDLTR register accessor: an alias for `Reg<WDLTR_SPEC>`"]
pub type WDLTR = crate::Reg<wdltr::WDLTR_SPEC>;
#[doc = "watchdog lower threshold register"]
pub mod wdltr;

#[doc = "RSQR1 register accessor: an alias for `Reg<RSQR1_SPEC>`"]
pub type RSQR1 = crate::Reg<rsqr1::RSQR1_SPEC>;
#[doc = "regular sequence register 1"]
pub mod rsqr1;

#[doc = "RSQR2 register accessor: an alias for `Reg<RSQR2_SPEC>`"]
pub type RSQR2 = crate::Reg<rsqr2::RSQR2_SPEC>;
#[doc = "regular sequence register 2"]
pub mod rsqr2;

#[doc = "RSQR3 register accessor: an alias for `Reg<RSQR3_SPEC>`"]
pub type RSQR3 = crate::Reg<rsqr3::RSQR3_SPEC>;
#[doc = "regular sequence register 3"]
pub mod rsqr3;

#[doc = "ISQR register accessor: an alias for `Reg<ISQR_SPEC>`"]
pub type ISQR = crate::Reg<isqr::ISQR_SPEC>;
#[doc = "injected sequence register"]
pub mod isqr;

#[doc = "IDATAR1 register accessor: an alias for `Reg<IDATAR1_SPEC>`"]
pub type IDATAR1 = crate::Reg<idatar1::IDATAR1_SPEC>;
#[doc = "injected data register x"]
pub mod idatar1;

#[doc = "IDATAR2 register accessor: an alias for `Reg<IDATAR2_SPEC>`"]
pub type IDATAR2 = crate::Reg<idatar2::IDATAR2_SPEC>;
#[doc = "injected data register x"]
pub mod idatar2;

#[doc = "IDATAR3 register accessor: an alias for `Reg<IDATAR3_SPEC>`"]
pub type IDATAR3 = crate::Reg<idatar3::IDATAR3_SPEC>;
#[doc = "injected data register x"]
pub mod idatar3;

#[doc = "IDATAR4 register accessor: an alias for `Reg<IDATAR4_SPEC>`"]
pub type IDATAR4 = crate::Reg<idatar4::IDATAR4_SPEC>;
#[doc = "injected data register x"]
pub mod idatar4;

#[doc = "RDATAR register accessor: an alias for `Reg<RDATAR_SPEC>`"]
pub type RDATAR = crate::Reg<rdatar::RDATAR_SPEC>;
#[doc = "regular data register"]
pub mod rdatar;