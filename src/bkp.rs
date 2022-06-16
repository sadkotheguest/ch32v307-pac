#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Backup data register (BKP_DR)"]
    pub datar1: crate::Reg<datar1::DATAR1_SPEC>,
    #[doc = "0x04 - Backup data register (BKP_DR)"]
    pub datar2: crate::Reg<datar2::DATAR2_SPEC>,
    #[doc = "0x08 - Backup data register (BKP_DR)"]
    pub datar3: crate::Reg<datar3::DATAR3_SPEC>,
    #[doc = "0x0c - Backup data register (BKP_DR)"]
    pub datar4: crate::Reg<datar4::DATAR4_SPEC>,
    #[doc = "0x10 - Backup data register (BKP_DR)"]
    pub datar5: crate::Reg<datar5::DATAR5_SPEC>,
    #[doc = "0x14 - Backup data register (BKP_DR)"]
    pub datar6: crate::Reg<datar6::DATAR6_SPEC>,
    #[doc = "0x18 - Backup data register (BKP_DR)"]
    pub datar7: crate::Reg<datar7::DATAR7_SPEC>,
    #[doc = "0x1c - Backup data register (BKP_DR)"]
    pub datar8: crate::Reg<datar8::DATAR8_SPEC>,
    #[doc = "0x20 - Backup data register (BKP_DR)"]
    pub datar9: crate::Reg<datar9::DATAR9_SPEC>,
    #[doc = "0x24 - Backup data register (BKP_DR)"]
    pub datar10: crate::Reg<datar10::DATAR10_SPEC>,
    #[doc = "0x28 - RTC clock calibration register (BKP_OCTLR)"]
    pub octlr: crate::Reg<octlr::OCTLR_SPEC>,
    #[doc = "0x2c - Backup control register (BKP_TPCTLR)"]
    pub tpctlr: crate::Reg<tpctlr::TPCTLR_SPEC>,
    #[doc = "0x30 - BKP_TPCSR control/status register (BKP_CSR)"]
    pub tpcsr: crate::Reg<tpcsr::TPCSR_SPEC>,
    _reserved13: [u8; 0x08],
    #[doc = "0x3c - Backup data register (BKP_DR)"]
    pub datar11: crate::Reg<datar11::DATAR11_SPEC>,
    #[doc = "0x40 - Backup data register (BKP_DR)"]
    pub datar12: crate::Reg<datar12::DATAR12_SPEC>,
    #[doc = "0x44 - Backup data register (BKP_DR)"]
    pub datar13: crate::Reg<datar13::DATAR13_SPEC>,
    #[doc = "0x48 - Backup data register (BKP_DR)"]
    pub datar14: crate::Reg<datar14::DATAR14_SPEC>,
    #[doc = "0x4c - Backup data register (BKP_DR)"]
    pub datar15: crate::Reg<datar15::DATAR15_SPEC>,
    #[doc = "0x50 - Backup data register (BKP_DR)"]
    pub datar16: crate::Reg<datar16::DATAR16_SPEC>,
    #[doc = "0x54 - Backup data register (BKP_DR)"]
    pub datar17: crate::Reg<datar17::DATAR17_SPEC>,
    #[doc = "0x58 - Backup data register (BKP_DR)"]
    pub datar18: crate::Reg<datar18::DATAR18_SPEC>,
    #[doc = "0x5c - Backup data register (BKP_DR)"]
    pub datar19: crate::Reg<datar19::DATAR19_SPEC>,
    #[doc = "0x60 - Backup data register (BKP_DR)"]
    pub datar20: crate::Reg<datar20::DATAR20_SPEC>,
    #[doc = "0x64 - Backup data register (BKP_DR)"]
    pub datar21: crate::Reg<datar21::DATAR21_SPEC>,
    #[doc = "0x68 - Backup data register (BKP_DR)"]
    pub datar22: crate::Reg<datar22::DATAR22_SPEC>,
    #[doc = "0x6c - Backup data register (BKP_DR)"]
    pub datar23: crate::Reg<datar23::DATAR23_SPEC>,
    #[doc = "0x70 - Backup data register (BKP_DR)"]
    pub datar24: crate::Reg<datar24::DATAR24_SPEC>,
    #[doc = "0x74 - Backup data register (BKP_DR)"]
    pub datar25: crate::Reg<datar25::DATAR25_SPEC>,
    #[doc = "0x78 - Backup data register (BKP_DR)"]
    pub datar26: crate::Reg<datar26::DATAR26_SPEC>,
    #[doc = "0x7c - Backup data register (BKP_DR)"]
    pub datar27: crate::Reg<datar27::DATAR27_SPEC>,
    #[doc = "0x80 - Backup data register (BKP_DR)"]
    pub datar28: crate::Reg<datar28::DATAR28_SPEC>,
    #[doc = "0x84 - Backup data register (BKP_DR)"]
    pub datar29: crate::Reg<datar29::DATAR29_SPEC>,
    #[doc = "0x88 - Backup data register (BKP_DR)"]
    pub datar30: crate::Reg<datar30::DATAR30_SPEC>,
    #[doc = "0x8c - Backup data register (BKP_DR)"]
    pub datar31: crate::Reg<datar31::DATAR31_SPEC>,
    #[doc = "0x90 - Backup data register (BKP_DR)"]
    pub datar32: crate::Reg<datar32::DATAR32_SPEC>,
    #[doc = "0x94 - Backup data register (BKP_DR)"]
    pub datar33: crate::Reg<datar33::DATAR33_SPEC>,
    #[doc = "0x98 - Backup data register (BKP_DR)"]
    pub datar34: crate::Reg<datar34::DATAR34_SPEC>,
    #[doc = "0x9c - Backup data register (BKP_DR)"]
    pub datar35: crate::Reg<datar35::DATAR35_SPEC>,
    #[doc = "0xa0 - Backup data register (BKP_DR)"]
    pub datar36: crate::Reg<datar36::DATAR36_SPEC>,
    #[doc = "0xa4 - Backup data register (BKP_DR)"]
    pub datar37: crate::Reg<datar37::DATAR37_SPEC>,
    #[doc = "0xa8 - Backup data register (BKP_DR)"]
    pub datar38: crate::Reg<datar38::DATAR38_SPEC>,
    #[doc = "0xac - Backup data register (BKP_DR)"]
    pub datar39: crate::Reg<datar39::DATAR39_SPEC>,
    #[doc = "0xb0 - Backup data register (BKP_DR)"]
    pub datar40: crate::Reg<datar40::DATAR40_SPEC>,
    #[doc = "0xb4 - Backup data register (BKP_DR)"]
    pub datar41: crate::Reg<datar41::DATAR41_SPEC>,
    #[doc = "0xb8 - Backup data register (BKP_DR)"]
    pub datar42: crate::Reg<datar42::DATAR42_SPEC>,
}

#[doc = "DATAR1 register accessor: an alias for `Reg<DATAR1_SPEC>`"]
pub type DATAR1 = crate::Reg<datar1::DATAR1_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar1;

#[doc = "DATAR2 register accessor: an alias for `Reg<DATAR2_SPEC>`"]
pub type DATAR2 = crate::Reg<datar2::DATAR2_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar2;

#[doc = "DATAR3 register accessor: an alias for `Reg<DATAR3_SPEC>`"]
pub type DATAR3 = crate::Reg<datar3::DATAR3_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar3;

#[doc = "DATAR4 register accessor: an alias for `Reg<DATAR4_SPEC>`"]
pub type DATAR4 = crate::Reg<datar4::DATAR4_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar4;

#[doc = "DATAR5 register accessor: an alias for `Reg<DATAR5_SPEC>`"]
pub type DATAR5 = crate::Reg<datar5::DATAR5_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar5;

#[doc = "DATAR6 register accessor: an alias for `Reg<DATAR6_SPEC>`"]
pub type DATAR6 = crate::Reg<datar6::DATAR6_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar6;

#[doc = "DATAR7 register accessor: an alias for `Reg<DATAR7_SPEC>`"]
pub type DATAR7 = crate::Reg<datar7::DATAR7_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar7;

#[doc = "DATAR8 register accessor: an alias for `Reg<DATAR8_SPEC>`"]
pub type DATAR8 = crate::Reg<datar8::DATAR8_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar8;

#[doc = "DATAR9 register accessor: an alias for `Reg<DATAR9_SPEC>`"]
pub type DATAR9 = crate::Reg<datar9::DATAR9_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar9;

#[doc = "DATAR10 register accessor: an alias for `Reg<DATAR10_SPEC>`"]
pub type DATAR10 = crate::Reg<datar10::DATAR10_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar10;

#[doc = "DATAR11 register accessor: an alias for `Reg<DATAR11_SPEC>`"]
pub type DATAR11 = crate::Reg<datar11::DATAR11_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar11;

#[doc = "DATAR12 register accessor: an alias for `Reg<DATAR12_SPEC>`"]
pub type DATAR12 = crate::Reg<datar12::DATAR12_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar12;

#[doc = "DATAR13 register accessor: an alias for `Reg<DATAR13_SPEC>`"]
pub type DATAR13 = crate::Reg<datar13::DATAR13_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar13;

#[doc = "DATAR14 register accessor: an alias for `Reg<DATAR14_SPEC>`"]
pub type DATAR14 = crate::Reg<datar14::DATAR14_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar14;

#[doc = "DATAR15 register accessor: an alias for `Reg<DATAR15_SPEC>`"]
pub type DATAR15 = crate::Reg<datar15::DATAR15_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar15;

#[doc = "DATAR16 register accessor: an alias for `Reg<DATAR16_SPEC>`"]
pub type DATAR16 = crate::Reg<datar16::DATAR16_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar16;

#[doc = "DATAR17 register accessor: an alias for `Reg<DATAR17_SPEC>`"]
pub type DATAR17 = crate::Reg<datar17::DATAR17_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar17;

#[doc = "DATAR18 register accessor: an alias for `Reg<DATAR18_SPEC>`"]
pub type DATAR18 = crate::Reg<datar18::DATAR18_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar18;

#[doc = "DATAR19 register accessor: an alias for `Reg<DATAR19_SPEC>`"]
pub type DATAR19 = crate::Reg<datar19::DATAR19_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar19;

#[doc = "DATAR20 register accessor: an alias for `Reg<DATAR20_SPEC>`"]
pub type DATAR20 = crate::Reg<datar20::DATAR20_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar20;

#[doc = "DATAR21 register accessor: an alias for `Reg<DATAR21_SPEC>`"]
pub type DATAR21 = crate::Reg<datar21::DATAR21_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar21;

#[doc = "DATAR22 register accessor: an alias for `Reg<DATAR22_SPEC>`"]
pub type DATAR22 = crate::Reg<datar22::DATAR22_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar22;

#[doc = "DATAR23 register accessor: an alias for `Reg<DATAR23_SPEC>`"]
pub type DATAR23 = crate::Reg<datar23::DATAR23_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar23;

#[doc = "DATAR24 register accessor: an alias for `Reg<DATAR24_SPEC>`"]
pub type DATAR24 = crate::Reg<datar24::DATAR24_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar24;

#[doc = "DATAR25 register accessor: an alias for `Reg<DATAR25_SPEC>`"]
pub type DATAR25 = crate::Reg<datar25::DATAR25_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar25;

#[doc = "DATAR26 register accessor: an alias for `Reg<DATAR26_SPEC>`"]
pub type DATAR26 = crate::Reg<datar26::DATAR26_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar26;

#[doc = "DATAR27 register accessor: an alias for `Reg<DATAR27_SPEC>`"]
pub type DATAR27 = crate::Reg<datar27::DATAR27_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar27;

#[doc = "DATAR28 register accessor: an alias for `Reg<DATAR28_SPEC>`"]
pub type DATAR28 = crate::Reg<datar28::DATAR28_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar28;

#[doc = "DATAR29 register accessor: an alias for `Reg<DATAR29_SPEC>`"]
pub type DATAR29 = crate::Reg<datar29::DATAR29_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar29;

#[doc = "DATAR30 register accessor: an alias for `Reg<DATAR30_SPEC>`"]
pub type DATAR30 = crate::Reg<datar30::DATAR30_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar30;

#[doc = "DATAR31 register accessor: an alias for `Reg<DATAR31_SPEC>`"]
pub type DATAR31 = crate::Reg<datar31::DATAR31_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar31;

#[doc = "DATAR32 register accessor: an alias for `Reg<DATAR32_SPEC>`"]
pub type DATAR32 = crate::Reg<datar32::DATAR32_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar32;

#[doc = "DATAR33 register accessor: an alias for `Reg<DATAR33_SPEC>`"]
pub type DATAR33 = crate::Reg<datar33::DATAR33_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar33;

#[doc = "DATAR34 register accessor: an alias for `Reg<DATAR34_SPEC>`"]
pub type DATAR34 = crate::Reg<datar34::DATAR34_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar34;

#[doc = "DATAR35 register accessor: an alias for `Reg<DATAR35_SPEC>`"]
pub type DATAR35 = crate::Reg<datar35::DATAR35_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar35;

#[doc = "DATAR36 register accessor: an alias for `Reg<DATAR36_SPEC>`"]
pub type DATAR36 = crate::Reg<datar36::DATAR36_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar36;

#[doc = "DATAR37 register accessor: an alias for `Reg<DATAR37_SPEC>`"]
pub type DATAR37 = crate::Reg<datar37::DATAR37_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar37;

#[doc = "DATAR38 register accessor: an alias for `Reg<DATAR38_SPEC>`"]
pub type DATAR38 = crate::Reg<datar38::DATAR38_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar38;

#[doc = "DATAR39 register accessor: an alias for `Reg<DATAR39_SPEC>`"]
pub type DATAR39 = crate::Reg<datar39::DATAR39_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar39;

#[doc = "DATAR40 register accessor: an alias for `Reg<DATAR40_SPEC>`"]
pub type DATAR40 = crate::Reg<datar40::DATAR40_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar40;

#[doc = "DATAR41 register accessor: an alias for `Reg<DATAR41_SPEC>`"]
pub type DATAR41 = crate::Reg<datar41::DATAR41_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar41;

#[doc = "DATAR42 register accessor: an alias for `Reg<DATAR42_SPEC>`"]
pub type DATAR42 = crate::Reg<datar42::DATAR42_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod datar42;

#[doc = "OCTLR register accessor: an alias for `Reg<OCTLR_SPEC>`"]
pub type OCTLR = crate::Reg<octlr::OCTLR_SPEC>;
#[doc = "RTC clock calibration register (BKP_OCTLR)"]
pub mod octlr;

#[doc = "TPCTLR register accessor: an alias for `Reg<TPCTLR_SPEC>`"]
pub type TPCTLR = crate::Reg<tpctlr::TPCTLR_SPEC>;
#[doc = "Backup control register (BKP_TPCTLR)"]
pub mod tpctlr;

#[doc = "TPCSR register accessor: an alias for `Reg<TPCSR_SPEC>`"]
pub type TPCSR = crate::Reg<tpcsr::TPCSR_SPEC>;
#[doc = "BKP_TPCSR control/status register (BKP_CSR)"]
pub mod tpcsr;