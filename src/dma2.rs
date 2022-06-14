#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA interrupt status register (DMA_INTFR)"]
    pub intfr: crate::Reg<intfr::INTFR_SPEC>,
    #[doc = "0x04 - DMA interrupt flag clear register (DMA_INTFCR)"]
    pub intfcr: crate::Reg<intfcr::INTFCR_SPEC>,
    _reserved_1_intfcr: [u8; 0x04],
    #[doc = "0x08 - DMA channel configuration register (DMA_CFGR)"]
    pub cfgr: crate::Reg<cfgr::CFGR_SPEC>,
    #[doc = "0x0c - DMA channel 1 number of data register"]
    pub cntr: crate::Reg<cntr::CNTR_SPEC>,
    #[doc = "0x10 - DMA channel 1 peripheral address register"]
    pub paddr: crate::Reg<paddr::PADDR_SPEC>,
    #[doc = "0x14 - DMA channel 1 memory address register"]
    pub maddr: crate::Reg<maddr::MADDR_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - DMA channel configuration register (DMA_CFGR)"]
    pub cfgr2: crate::Reg<cfgr2::CFGR2_SPEC>,
    #[doc = "0x20 - DMA channel 2 number of data register"]
    pub cntr2: crate::Reg<cntr2::CNTR2_SPEC>,
    #[doc = "0x24 - DMA channel 2 peripheral address register"]
    pub paddr2: crate::Reg<paddr2::PADDR2_SPEC>,
    #[doc = "0x28 - DMA channel 2 memory address register"]
    pub maddr2: crate::Reg<maddr2::MADDR2_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x30 - DMA channel configuration register (DMA_CFGR)"]
    pub cfgr3: crate::Reg<cfgr3::CFGR3_SPEC>,
    #[doc = "0x34 - DMA channel 3 number of data register"]
    pub cntr3: crate::Reg<cntr3::CNTR3_SPEC>,
    #[doc = "0x38 - DMA channel 3 peripheral address register"]
    pub paddr3: crate::Reg<paddr3::PADDR3_SPEC>,
    #[doc = "0x3c - DMA channel 3 memory address register"]
    pub maddr3: crate::Reg<maddr3::MADDR3_SPEC>,
    _reserved14: [u8; 0x04],
    #[doc = "0x44 - DMA channel configuration register (DMA_CFGR)"]
    pub cfgr4: crate::Reg<cfgr4::CFGR4_SPEC>,
    #[doc = "0x48 - DMA channel 4 number of data register"]
    pub cntr4: crate::Reg<cntr4::CNTR4_SPEC>,
    #[doc = "0x4c - DMA channel 4 peripheral address register"]
    pub paddr4: crate::Reg<paddr4::PADDR4_SPEC>,
    #[doc = "0x50 - DMA channel 4 memory address register"]
    pub maddr4: crate::Reg<maddr4::MADDR4_SPEC>,
    _reserved18: [u8; 0x04],
    #[doc = "0x58 - DMA channel configuration register (DMA_CFGR)"]
    pub cfgr5: crate::Reg<cfgr5::CFGR5_SPEC>,
    #[doc = "0x5c - DMA channel 5 number of data register"]
    pub cntr5: crate::Reg<cntr5::CNTR5_SPEC>,
    #[doc = "0x60 - DMA channel 5 peripheral address register"]
    pub paddr5: crate::Reg<paddr5::PADDR5_SPEC>,
    #[doc = "0x64 - DMA channel 5 memory address register"]
    pub maddr5: crate::Reg<maddr5::MADDR5_SPEC>,
    _reserved22: [u8; 0x04],
    #[doc = "0x6c - DMA channel configuration register (DMA_CFGR)"]
    pub cfgr6: crate::Reg<cfgr6::CFGR6_SPEC>,
    #[doc = "0x70 - DMA channel 6 number of data register"]
    pub cntr6: crate::Reg<cntr6::CNTR6_SPEC>,
    #[doc = "0x74 - DMA channel 6 peripheral address register"]
    pub paddr6: crate::Reg<paddr6::PADDR6_SPEC>,
    #[doc = "0x78 - DMA channel 6 memory address register"]
    pub maddr6: crate::Reg<maddr6::MADDR6_SPEC>,
    _reserved26: [u8; 0x04],
    #[doc = "0x80 - DMA channel configuration register (DMA_CFGR)"]
    pub cfgr7: crate::Reg<cfgr7::CFGR7_SPEC>,
    #[doc = "0x84 - DMA channel 7 number of data register"]
    pub cntr7: crate::Reg<cntr7::CNTR7_SPEC>,
    #[doc = "0x88 - DMA channel 7 peripheral address register"]
    pub paddr7: crate::Reg<paddr7::PADDR7_SPEC>,
    #[doc = "0x8c - DMA channel 7 memory address register"]
    pub maddr7: crate::Reg<maddr7::MADDR7_SPEC>,
    #[doc = "0x90 - DMA channel configuration register (DMA_CFGR)"]
    pub cfgr8: crate::Reg<cfgr8::CFGR8_SPEC>,
    #[doc = "0x94 - DMA channel 8 number of data register"]
    pub cntr8: crate::Reg<cntr8::CNTR8_SPEC>,
    #[doc = "0x98 - DMA channel 8 peripheral address register"]
    pub paddr8: crate::Reg<paddr8::PADDR8_SPEC>,
    #[doc = "0x9c - DMA channel 8 memory address register"]
    pub maddr8: crate::Reg<maddr8::MADDR8_SPEC>,
    #[doc = "0xa0 - DMA channel configuration register (DMA_CFGR)"]
    pub cfgr9: crate::Reg<cfgr9::CFGR9_SPEC>,
    #[doc = "0xa4 - DMA channel 9 number of data register"]
    pub cntr9: crate::Reg<cntr9::CNTR9_SPEC>,
    #[doc = "0xa8 - DMA channel 7 peripheral address register"]
    pub paddr9: crate::Reg<paddr9::PADDR9_SPEC>,
    #[doc = "0xac - DMA channel 9 memory address register"]
    pub maddr9: crate::Reg<maddr9::MADDR9_SPEC>,
    #[doc = "0xb0 - DMA channel configuration register (DMA_CFGR)"]
    pub cfgr10: crate::Reg<cfgr10::CFGR10_SPEC>,
    #[doc = "0xb4 - DMA channel 10 number of data register"]
    pub cntr10: crate::Reg<cntr10::CNTR10_SPEC>,
    #[doc = "0xb8 - DMA channel 10 peripheral address register"]
    pub paddr10: crate::Reg<paddr10::PADDR10_SPEC>,
    #[doc = "0xbc - DMA channel 10 memory address register"]
    pub maddr10: crate::Reg<maddr10::MADDR10_SPEC>,
    #[doc = "0xc0 - DMA channel configuration register (DMA_CFGR)"]
    pub cfgr11: crate::Reg<cfgr11::CFGR11_SPEC>,
    #[doc = "0xc4 - DMA channel 11 number of data register"]
    pub cntr11: crate::Reg<cntr11::CNTR11_SPEC>,
    #[doc = "0xc8 - DMA channel 11 peripheral address register"]
    pub paddr11: crate::Reg<paddr11::PADDR11_SPEC>,
    #[doc = "0xcc - DMA channel 11 memory address register"]
    pub maddr11: crate::Reg<maddr11::MADDR11_SPEC>,
    #[doc = "0xd0 - DMA2 EXTEN interrupt status register (DMA_INTFR)"]
    pub exten_intfr: crate::Reg<exten_intfr::EXTEN_INTFR_SPEC>,
}

#[doc = "INTFR register accessor: an alias for `Reg<INTFR_SPEC>`"]
pub type INTFR = crate::Reg<intfr::INTFR_SPEC>;
#[doc = "DMA interrupt status register (DMA_INTFR)"]
pub mod intfr;

#[doc = "INTFCR register accessor: an alias for `Reg<INTFCR_SPEC>`"]
pub type INTFCR = crate::Reg<intfcr::INTFCR_SPEC>;
#[doc = "DMA interrupt flag clear register (DMA_INTFCR)"]
pub mod intfcr;

#[doc = "CFGR register accessor: an alias for `Reg<CFGR_SPEC>`"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "DMA channel configuration register (DMA_CFGR)"]
pub mod cfgr;

#[doc = "CNTR register accessor: an alias for `Reg<CNTR_SPEC>`"]
pub type CNTR = crate::Reg<cntr::CNTR_SPEC>;
#[doc = "DMA channel 1 number of data register"]
pub mod cntr;

#[doc = "PADDR register accessor: an alias for `Reg<PADDR_SPEC>`"]
pub type PADDR = crate::Reg<paddr::PADDR_SPEC>;
#[doc = "DMA channel 1 peripheral address register"]
pub mod paddr;

#[doc = "MADDR register accessor: an alias for `Reg<MADDR_SPEC>`"]
pub type MADDR = crate::Reg<maddr::MADDR_SPEC>;
#[doc = "DMA channel 1 memory address register"]
pub mod maddr;

#[doc = "CFGR2 register accessor: an alias for `Reg<CFGR2_SPEC>`"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
#[doc = "DMA channel configuration register (DMA_CFGR)"]
pub mod cfgr2;

#[doc = "CNTR2 register accessor: an alias for `Reg<CNTR2_SPEC>`"]
pub type CNTR2 = crate::Reg<cntr2::CNTR2_SPEC>;
#[doc = "DMA channel 2 number of data register"]
pub mod cntr2;

#[doc = "PADDR2 register accessor: an alias for `Reg<PADDR2_SPEC>`"]
pub type PADDR2 = crate::Reg<paddr2::PADDR2_SPEC>;
#[doc = "DMA channel 2 peripheral address register"]
pub mod paddr2;

#[doc = "MADDR2 register accessor: an alias for `Reg<MADDR2_SPEC>`"]
pub type MADDR2 = crate::Reg<maddr2::MADDR2_SPEC>;
#[doc = "DMA channel 2 memory address register"]
pub mod maddr2;

#[doc = "CFGR3 register accessor: an alias for `Reg<CFGR3_SPEC>`"]
pub type CFGR3 = crate::Reg<cfgr3::CFGR3_SPEC>;
#[doc = "DMA channel configuration register (DMA_CFGR)"]
pub mod cfgr3;

#[doc = "CNTR3 register accessor: an alias for `Reg<CNTR3_SPEC>`"]
pub type CNTR3 = crate::Reg<cntr3::CNTR3_SPEC>;
#[doc = "DMA channel 3 number of data register"]
pub mod cntr3;

#[doc = "PADDR3 register accessor: an alias for `Reg<PADDR3_SPEC>`"]
pub type PADDR3 = crate::Reg<paddr3::PADDR3_SPEC>;
#[doc = "DMA channel 3 peripheral address register"]
pub mod paddr3;

#[doc = "MADDR3 register accessor: an alias for `Reg<MADDR3_SPEC>`"]
pub type MADDR3 = crate::Reg<maddr3::MADDR3_SPEC>;
#[doc = "DMA channel 3 memory address register"]
pub mod maddr3;

#[doc = "CFGR4 register accessor: an alias for `Reg<CFGR4_SPEC>`"]
pub type CFGR4 = crate::Reg<cfgr4::CFGR4_SPEC>;
#[doc = "DMA channel configuration register (DMA_CFGR)"]
pub mod cfgr4;

#[doc = "CNTR4 register accessor: an alias for `Reg<CNTR4_SPEC>`"]
pub type CNTR4 = crate::Reg<cntr4::CNTR4_SPEC>;
#[doc = "DMA channel 4 number of data register"]
pub mod cntr4;

#[doc = "PADDR4 register accessor: an alias for `Reg<PADDR4_SPEC>`"]
pub type PADDR4 = crate::Reg<paddr4::PADDR4_SPEC>;
#[doc = "DMA channel 4 peripheral address register"]
pub mod paddr4;

#[doc = "MADDR4 register accessor: an alias for `Reg<MADDR4_SPEC>`"]
pub type MADDR4 = crate::Reg<maddr4::MADDR4_SPEC>;
#[doc = "DMA channel 4 memory address register"]
pub mod maddr4;

#[doc = "CFGR5 register accessor: an alias for `Reg<CFGR5_SPEC>`"]
pub type CFGR5 = crate::Reg<cfgr5::CFGR5_SPEC>;
#[doc = "DMA channel configuration register (DMA_CFGR)"]
pub mod cfgr5;

#[doc = "CNTR5 register accessor: an alias for `Reg<CNTR5_SPEC>`"]
pub type CNTR5 = crate::Reg<cntr5::CNTR5_SPEC>;
#[doc = "DMA channel 5 number of data register"]
pub mod cntr5;

#[doc = "PADDR5 register accessor: an alias for `Reg<PADDR5_SPEC>`"]
pub type PADDR5 = crate::Reg<paddr5::PADDR5_SPEC>;
#[doc = "DMA channel 5 peripheral address register"]
pub mod paddr5;

#[doc = "MADDR5 register accessor: an alias for `Reg<MADDR5_SPEC>`"]
pub type MADDR5 = crate::Reg<maddr5::MADDR5_SPEC>;
#[doc = "DMA channel 5 memory address register"]
pub mod maddr5;

#[doc = "CFGR6 register accessor: an alias for `Reg<CFGR6_SPEC>`"]
pub type CFGR6 = crate::Reg<cfgr6::CFGR6_SPEC>;
#[doc = "DMA channel configuration register (DMA_CFGR)"]
pub mod cfgr6;

#[doc = "CNTR6 register accessor: an alias for `Reg<CNTR6_SPEC>`"]
pub type CNTR6 = crate::Reg<cntr6::CNTR6_SPEC>;
#[doc = "DMA channel 6 number of data register"]
pub mod cntr6;

#[doc = "PADDR6 register accessor: an alias for `Reg<PADDR6_SPEC>`"]
pub type PADDR6 = crate::Reg<paddr6::PADDR6_SPEC>;
#[doc = "DMA channel 6 peripheral address register"]
pub mod paddr6;

#[doc = "MADDR6 register accessor: an alias for `Reg<MADDR6_SPEC>`"]
pub type MADDR6 = crate::Reg<maddr6::MADDR6_SPEC>;
#[doc = "DMA channel 6 memory address register"]
pub mod maddr6;

#[doc = "CFGR7 register accessor: an alias for `Reg<CFGR7_SPEC>`"]
pub type CFGR7 = crate::Reg<cfgr7::CFGR7_SPEC>;
#[doc = "DMA channel configuration register (DMA_CFGR)"]
pub mod cfgr7;

#[doc = "CNTR7 register accessor: an alias for `Reg<CNTR7_SPEC>`"]
pub type CNTR7 = crate::Reg<cntr7::CNTR7_SPEC>;
#[doc = "DMA channel 7 number of data register"]
pub mod cntr7;

#[doc = "PADDR7 register accessor: an alias for `Reg<PADDR7_SPEC>`"]
pub type PADDR7 = crate::Reg<paddr7::PADDR7_SPEC>;
#[doc = "DMA channel 7 peripheral address register"]
pub mod paddr7;

#[doc = "MADDR7 register accessor: an alias for `Reg<MADDR7_SPEC>`"]
pub type MADDR7 = crate::Reg<maddr7::MADDR7_SPEC>;
#[doc = "DMA channel 7 memory address register"]
pub mod maddr7;

#[doc = "CFGR8 register accessor: an alias for `Reg<CFGR8_SPEC>`"]
pub type CFGR8 = crate::Reg<cfgr8::CFGR8_SPEC>;
#[doc = "DMA channel configuration register (DMA_CFGR)"]
pub mod cfgr8;

#[doc = "CNTR8 register accessor: an alias for `Reg<CNTR8_SPEC>`"]
pub type CNTR8 = crate::Reg<cntr8::CNTR8_SPEC>;
#[doc = "DMA channel 8 number of data register"]
pub mod cntr8;

#[doc = "PADDR8 register accessor: an alias for `Reg<PADDR8_SPEC>`"]
pub type PADDR8 = crate::Reg<paddr8::PADDR8_SPEC>;
#[doc = "DMA channel 8 peripheral address register"]
pub mod paddr8;

#[doc = "MADDR8 register accessor: an alias for `Reg<MADDR8_SPEC>`"]
pub type MADDR8 = crate::Reg<maddr8::MADDR8_SPEC>;
#[doc = "DMA channel 8 memory address register"]
pub mod maddr8;

#[doc = "CFGR9 register accessor: an alias for `Reg<CFGR9_SPEC>`"]
pub type CFGR9 = crate::Reg<cfgr9::CFGR9_SPEC>;
#[doc = "DMA channel configuration register (DMA_CFGR)"]
pub mod cfgr9;

#[doc = "CNTR9 register accessor: an alias for `Reg<CNTR9_SPEC>`"]
pub type CNTR9 = crate::Reg<cntr9::CNTR9_SPEC>;
#[doc = "DMA channel 9 number of data register"]
pub mod cntr9;

#[doc = "PADDR9 register accessor: an alias for `Reg<PADDR9_SPEC>`"]
pub type PADDR9 = crate::Reg<paddr9::PADDR9_SPEC>;
#[doc = "DMA channel 7 peripheral address register"]
pub mod paddr9;

#[doc = "MADDR9 register accessor: an alias for `Reg<MADDR9_SPEC>`"]
pub type MADDR9 = crate::Reg<maddr9::MADDR9_SPEC>;
#[doc = "DMA channel 9 memory address register"]
pub mod maddr9;

#[doc = "CFGR10 register accessor: an alias for `Reg<CFGR10_SPEC>`"]
pub type CFGR10 = crate::Reg<cfgr10::CFGR10_SPEC>;
#[doc = "DMA channel configuration register (DMA_CFGR)"]
pub mod cfgr10;

#[doc = "CNTR10 register accessor: an alias for `Reg<CNTR10_SPEC>`"]
pub type CNTR10 = crate::Reg<cntr10::CNTR10_SPEC>;
#[doc = "DMA channel 10 number of data register"]
pub mod cntr10;

#[doc = "PADDR10 register accessor: an alias for `Reg<PADDR10_SPEC>`"]
pub type PADDR10 = crate::Reg<paddr10::PADDR10_SPEC>;
#[doc = "DMA channel 10 peripheral address register"]
pub mod paddr10;

#[doc = "MADDR10 register accessor: an alias for `Reg<MADDR10_SPEC>`"]
pub type MADDR10 = crate::Reg<maddr10::MADDR10_SPEC>;
#[doc = "DMA channel 10 memory address register"]
pub mod maddr10;

#[doc = "CFGR11 register accessor: an alias for `Reg<CFGR11_SPEC>`"]
pub type CFGR11 = crate::Reg<cfgr11::CFGR11_SPEC>;
#[doc = "DMA channel configuration register (DMA_CFGR)"]
pub mod cfgr11;

#[doc = "CNTR11 register accessor: an alias for `Reg<CNTR11_SPEC>`"]
pub type CNTR11 = crate::Reg<cntr11::CNTR11_SPEC>;
#[doc = "DMA channel 11 number of data register"]
pub mod cntr11;

#[doc = "PADDR11 register accessor: an alias for `Reg<PADDR11_SPEC>`"]
pub type PADDR11 = crate::Reg<paddr11::PADDR11_SPEC>;
#[doc = "DMA channel 11 peripheral address register"]
pub mod paddr11;

#[doc = "MADDR11 register accessor: an alias for `Reg<MADDR11_SPEC>`"]
pub type MADDR11 = crate::Reg<maddr11::MADDR11_SPEC>;
#[doc = "DMA channel 11 memory address register"]
pub mod maddr11;

#[doc = "EXTEN_INTFR register accessor: an alias for `Reg<EXTEN_INTFR_SPEC>`"]
pub type EXTEN_INTFR = crate::Reg<exten_intfr::EXTEN_INTFR_SPEC>;
#[doc = "DMA2 EXTEN interrupt status register (DMA_INTFR)"]
pub mod exten_intfr;