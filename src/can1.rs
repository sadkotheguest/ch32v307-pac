#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CTLR"]
    pub ctlr: crate::Reg<ctlr::CTLR_SPEC>,
    #[doc = "0x04 - STATR"]
    pub statr: crate::Reg<statr::STATR_SPEC>,
    #[doc = "0x08 - TSTATR"]
    pub tstatr: crate::Reg<tstatr::TSTATR_SPEC>,
    #[doc = "0x0c - RFIFO0"]
    pub rfifo0: crate::Reg<rfifo0::RFIFO0_SPEC>,
    #[doc = "0x10 - RFIFO1"]
    pub rfifo1: crate::Reg<rfifo1::RFIFO1_SPEC>,
    #[doc = "0x14 - INTENR"]
    pub intenr: crate::Reg<intenr::INTENR_SPEC>,
    #[doc = "0x18 - ERRSR"]
    pub errsr: crate::Reg<errsr::ERRSR_SPEC>,
    #[doc = "0x1c - BTIMR"]
    pub btimr: crate::Reg<btimr::BTIMR_SPEC>,
    _reserved8: [u8; 0x0160],
    #[doc = "0x180 - TXMIR0"]
    pub txmir0: crate::Reg<txmir0::TXMIR0_SPEC>,
    #[doc = "0x184 - TXMDTR0"]
    pub txmdtr0: crate::Reg<txmdtr0::TXMDTR0_SPEC>,
    #[doc = "0x188 - TXMDLR0"]
    pub txmdlr0: crate::Reg<txmdlr0::TXMDLR0_SPEC>,
    #[doc = "0x18c - TXMDHR0"]
    pub txmdhr0: crate::Reg<txmdhr0::TXMDHR0_SPEC>,
    #[doc = "0x190 - TXMIR1"]
    pub txmir1: crate::Reg<txmir1::TXMIR1_SPEC>,
    #[doc = "0x194 - TXMDTR1"]
    pub txmdtr1: crate::Reg<txmdtr1::TXMDTR1_SPEC>,
    #[doc = "0x198 - TXMDLR1"]
    pub txmdlr1: crate::Reg<txmdlr1::TXMDLR1_SPEC>,
    #[doc = "0x19c - TXMDHR1"]
    pub txmdhr1: crate::Reg<txmdhr1::TXMDHR1_SPEC>,
    #[doc = "0x1a0 - TXMIR2"]
    pub txmir2: crate::Reg<txmir2::TXMIR2_SPEC>,
    #[doc = "0x1a4 - TXMDTR2"]
    pub txmdtr2: crate::Reg<txmdtr2::TXMDTR2_SPEC>,
    #[doc = "0x1a8 - TXMDLR2"]
    pub txmdlr2: crate::Reg<txmdlr2::TXMDLR2_SPEC>,
    #[doc = "0x1ac - TXMDHR2"]
    pub txmdhr2: crate::Reg<txmdhr2::TXMDHR2_SPEC>,
    #[doc = "0x1b0 - RXMIR0"]
    pub rxmir0: crate::Reg<rxmir0::RXMIR0_SPEC>,
    #[doc = "0x1b4 - RXMDTR0"]
    pub rxmdtr0: crate::Reg<rxmdtr0::RXMDTR0_SPEC>,
    #[doc = "0x1b8 - RXMDLR0"]
    pub rxmdlr0: crate::Reg<rxmdlr0::RXMDLR0_SPEC>,
    #[doc = "0x1bc - RXMDHR0"]
    pub rxmdhr0: crate::Reg<rxmdhr0::RXMDHR0_SPEC>,
    #[doc = "0x1c0 - RXMIR1"]
    pub rxmir1: crate::Reg<rxmir1::RXMIR1_SPEC>,
    #[doc = "0x1c4 - RXMDTR1"]
    pub rxmdtr1: crate::Reg<rxmdtr1::RXMDTR1_SPEC>,
    #[doc = "0x1c8 - RXMDLR1"]
    pub rxmdlr1: crate::Reg<rxmdlr1::RXMDLR1_SPEC>,
    #[doc = "0x1cc - RXMDHR1"]
    pub rxmdhr1: crate::Reg<rxmdhr1::RXMDHR1_SPEC>,
    _reserved28: [u8; 0x30],
    #[doc = "0x200 - FCTLR"]
    pub fctlr: crate::Reg<fctlr::FCTLR_SPEC>,
    #[doc = "0x204 - FMCFGR"]
    pub fmcfgr: crate::Reg<fmcfgr::FMCFGR_SPEC>,
    _reserved30: [u8; 0x04],
    #[doc = "0x20c - FSCFGR"]
    pub fscfgr: crate::Reg<fscfgr::FSCFGR_SPEC>,
    _reserved31: [u8; 0x04],
    #[doc = "0x214 - FAFIFOR"]
    pub fafifor: crate::Reg<fafifor::FAFIFOR_SPEC>,
    _reserved32: [u8; 0x04],
    #[doc = "0x21c - FWR"]
    pub fwr: crate::Reg<fwr::FWR_SPEC>,
    _reserved33: [u8; 0x20],
    #[doc = "0x240 - Filter bank 0 register 1"]
    pub f0r1: crate::Reg<f0r1::F0R1_SPEC>,
    #[doc = "0x244 - Filter bank 0 register 2"]
    pub f0r2: crate::Reg<f0r2::F0R2_SPEC>,
    #[doc = "0x248 - Filter bank 1 register 1"]
    pub f1r1: crate::Reg<f1r1::F1R1_SPEC>,
    #[doc = "0x24c - Filter bank 1 register 2"]
    pub f1r2: crate::Reg<f1r2::F1R2_SPEC>,
    #[doc = "0x250 - Filter bank 2 register 1"]
    pub f2r1: crate::Reg<f2r1::F2R1_SPEC>,
    #[doc = "0x254 - Filter bank 2 register 2"]
    pub f2r2: crate::Reg<f2r2::F2R2_SPEC>,
    #[doc = "0x258 - Filter bank 3 register 1"]
    pub f3r1: crate::Reg<f3r1::F3R1_SPEC>,
    #[doc = "0x25c - Filter bank 3 register 2"]
    pub f3r2: crate::Reg<f3r2::F3R2_SPEC>,
    #[doc = "0x260 - Filter bank 4 register 1"]
    pub f4r1: crate::Reg<f4r1::F4R1_SPEC>,
    #[doc = "0x264 - Filter bank 4 register 2"]
    pub f4r2: crate::Reg<f4r2::F4R2_SPEC>,
    #[doc = "0x268 - Filter bank 5 register 1"]
    pub f5r1: crate::Reg<f5r1::F5R1_SPEC>,
    #[doc = "0x26c - Filter bank 5 register 2"]
    pub f5r2: crate::Reg<f5r2::F5R2_SPEC>,
    #[doc = "0x270 - Filter bank 6 register 1"]
    pub f6r1: crate::Reg<f6r1::F6R1_SPEC>,
    #[doc = "0x274 - Filter bank 6 register 2"]
    pub f6r2: crate::Reg<f6r2::F6R2_SPEC>,
    #[doc = "0x278 - Filter bank 7 register 1"]
    pub f7r1: crate::Reg<f7r1::F7R1_SPEC>,
    #[doc = "0x27c - Filter bank 7 register 2"]
    pub f7r2: crate::Reg<f7r2::F7R2_SPEC>,
    #[doc = "0x280 - Filter bank 8 register 1"]
    pub f8r1: crate::Reg<f8r1::F8R1_SPEC>,
    #[doc = "0x284 - Filter bank 8 register 2"]
    pub f8r2: crate::Reg<f8r2::F8R2_SPEC>,
    #[doc = "0x288 - Filter bank 9 register 1"]
    pub f9r1: crate::Reg<f9r1::F9R1_SPEC>,
    #[doc = "0x28c - Filter bank 9 register 2"]
    pub f9r2: crate::Reg<f9r2::F9R2_SPEC>,
    #[doc = "0x290 - Filter bank 10 register 1"]
    pub f10r1: crate::Reg<f10r1::F10R1_SPEC>,
    #[doc = "0x294 - Filter bank 10 register 2"]
    pub f10r2: crate::Reg<f10r2::F10R2_SPEC>,
    #[doc = "0x298 - Filter bank 11 register 1"]
    pub f11r1: crate::Reg<f11r1::F11R1_SPEC>,
    #[doc = "0x29c - Filter bank 11 register 2"]
    pub f11r2: crate::Reg<f11r2::F11R2_SPEC>,
    #[doc = "0x2a0 - Filter bank 4 register 1"]
    pub f12r1: crate::Reg<f12r1::F12R1_SPEC>,
    #[doc = "0x2a4 - Filter bank 12 register 2"]
    pub f12r2: crate::Reg<f12r2::F12R2_SPEC>,
    #[doc = "0x2a8 - Filter bank 13 register 1"]
    pub f13r1: crate::Reg<f13r1::F13R1_SPEC>,
    #[doc = "0x2ac - Filter bank 13 register 2"]
    pub f13r2: crate::Reg<f13r2::F13R2_SPEC>,
}

#[doc = "CTLR register accessor: an alias for `Reg<CTLR_SPEC>`"]
pub type CTLR = crate::Reg<ctlr::CTLR_SPEC>;
#[doc = "CTLR"]
pub mod ctlr;

#[doc = "STATR register accessor: an alias for `Reg<STATR_SPEC>`"]
pub type STATR = crate::Reg<statr::STATR_SPEC>;
#[doc = "STATR"]
pub mod statr;

#[doc = "TSTATR register accessor: an alias for `Reg<TSTATR_SPEC>`"]
pub type TSTATR = crate::Reg<tstatr::TSTATR_SPEC>;
#[doc = "TSTATR"]
pub mod tstatr;

#[doc = "RFIFO0 register accessor: an alias for `Reg<RFIFO0_SPEC>`"]
pub type RFIFO0 = crate::Reg<rfifo0::RFIFO0_SPEC>;
#[doc = "RFIFO0"]
pub mod rfifo0;

#[doc = "RFIFO1 register accessor: an alias for `Reg<RFIFO1_SPEC>`"]
pub type RFIFO1 = crate::Reg<rfifo1::RFIFO1_SPEC>;
#[doc = "RFIFO1"]
pub mod rfifo1;

#[doc = "INTENR register accessor: an alias for `Reg<INTENR_SPEC>`"]
pub type INTENR = crate::Reg<intenr::INTENR_SPEC>;
#[doc = "INTENR"]
pub mod intenr;

#[doc = "ERRSR register accessor: an alias for `Reg<ERRSR_SPEC>`"]
pub type ERRSR = crate::Reg<errsr::ERRSR_SPEC>;
#[doc = "ERRSR"]
pub mod errsr;

#[doc = "BTIMR register accessor: an alias for `Reg<BTIMR_SPEC>`"]
pub type BTIMR = crate::Reg<btimr::BTIMR_SPEC>;
#[doc = "BTIMR"]
pub mod btimr;

#[doc = "TXMIR0 register accessor: an alias for `Reg<TXMIR0_SPEC>`"]
pub type TXMIR0 = crate::Reg<txmir0::TXMIR0_SPEC>;
#[doc = "TXMIR0"]
pub mod txmir0;

#[doc = "TXMDTR0 register accessor: an alias for `Reg<TXMDTR0_SPEC>`"]
pub type TXMDTR0 = crate::Reg<txmdtr0::TXMDTR0_SPEC>;
#[doc = "TXMDTR0"]
pub mod txmdtr0;

#[doc = "TXMDLR0 register accessor: an alias for `Reg<TXMDLR0_SPEC>`"]
pub type TXMDLR0 = crate::Reg<txmdlr0::TXMDLR0_SPEC>;
#[doc = "TXMDLR0"]
pub mod txmdlr0;

#[doc = "TXMDHR0 register accessor: an alias for `Reg<TXMDHR0_SPEC>`"]
pub type TXMDHR0 = crate::Reg<txmdhr0::TXMDHR0_SPEC>;
#[doc = "TXMDHR0"]
pub mod txmdhr0;

#[doc = "TXMIR1 register accessor: an alias for `Reg<TXMIR1_SPEC>`"]
pub type TXMIR1 = crate::Reg<txmir1::TXMIR1_SPEC>;
#[doc = "TXMIR1"]
pub mod txmir1;

#[doc = "TXMDTR1 register accessor: an alias for `Reg<TXMDTR1_SPEC>`"]
pub type TXMDTR1 = crate::Reg<txmdtr1::TXMDTR1_SPEC>;
#[doc = "TXMDTR1"]
pub mod txmdtr1;

#[doc = "TXMDLR1 register accessor: an alias for `Reg<TXMDLR1_SPEC>`"]
pub type TXMDLR1 = crate::Reg<txmdlr1::TXMDLR1_SPEC>;
#[doc = "TXMDLR1"]
pub mod txmdlr1;

#[doc = "TXMDHR1 register accessor: an alias for `Reg<TXMDHR1_SPEC>`"]
pub type TXMDHR1 = crate::Reg<txmdhr1::TXMDHR1_SPEC>;
#[doc = "TXMDHR1"]
pub mod txmdhr1;

#[doc = "TXMIR2 register accessor: an alias for `Reg<TXMIR2_SPEC>`"]
pub type TXMIR2 = crate::Reg<txmir2::TXMIR2_SPEC>;
#[doc = "TXMIR2"]
pub mod txmir2;

#[doc = "TXMDTR2 register accessor: an alias for `Reg<TXMDTR2_SPEC>`"]
pub type TXMDTR2 = crate::Reg<txmdtr2::TXMDTR2_SPEC>;
#[doc = "TXMDTR2"]
pub mod txmdtr2;

#[doc = "TXMDLR2 register accessor: an alias for `Reg<TXMDLR2_SPEC>`"]
pub type TXMDLR2 = crate::Reg<txmdlr2::TXMDLR2_SPEC>;
#[doc = "TXMDLR2"]
pub mod txmdlr2;

#[doc = "TXMDHR2 register accessor: an alias for `Reg<TXMDHR2_SPEC>`"]
pub type TXMDHR2 = crate::Reg<txmdhr2::TXMDHR2_SPEC>;
#[doc = "TXMDHR2"]
pub mod txmdhr2;

#[doc = "RXMIR0 register accessor: an alias for `Reg<RXMIR0_SPEC>`"]
pub type RXMIR0 = crate::Reg<rxmir0::RXMIR0_SPEC>;
#[doc = "RXMIR0"]
pub mod rxmir0;

#[doc = "RXMDTR0 register accessor: an alias for `Reg<RXMDTR0_SPEC>`"]
pub type RXMDTR0 = crate::Reg<rxmdtr0::RXMDTR0_SPEC>;
#[doc = "RXMDTR0"]
pub mod rxmdtr0;

#[doc = "RXMDLR0 register accessor: an alias for `Reg<RXMDLR0_SPEC>`"]
pub type RXMDLR0 = crate::Reg<rxmdlr0::RXMDLR0_SPEC>;
#[doc = "RXMDLR0"]
pub mod rxmdlr0;

#[doc = "RXMDHR0 register accessor: an alias for `Reg<RXMDHR0_SPEC>`"]
pub type RXMDHR0 = crate::Reg<rxmdhr0::RXMDHR0_SPEC>;
#[doc = "RXMDHR0"]
pub mod rxmdhr0;

#[doc = "RXMIR1 register accessor: an alias for `Reg<RXMIR1_SPEC>`"]
pub type RXMIR1 = crate::Reg<rxmir1::RXMIR1_SPEC>;
#[doc = "RXMIR1"]
pub mod rxmir1;

#[doc = "RXMDTR1 register accessor: an alias for `Reg<RXMDTR1_SPEC>`"]
pub type RXMDTR1 = crate::Reg<rxmdtr1::RXMDTR1_SPEC>;
#[doc = "RXMDTR1"]
pub mod rxmdtr1;

#[doc = "RXMDLR1 register accessor: an alias for `Reg<RXMDLR1_SPEC>`"]
pub type RXMDLR1 = crate::Reg<rxmdlr1::RXMDLR1_SPEC>;
#[doc = "RXMDLR1"]
pub mod rxmdlr1;

#[doc = "RXMDHR1 register accessor: an alias for `Reg<RXMDHR1_SPEC>`"]
pub type RXMDHR1 = crate::Reg<rxmdhr1::RXMDHR1_SPEC>;
#[doc = "RXMDHR1"]
pub mod rxmdhr1;

#[doc = "FCTLR register accessor: an alias for `Reg<FCTLR_SPEC>`"]
pub type FCTLR = crate::Reg<fctlr::FCTLR_SPEC>;
#[doc = "FCTLR"]
pub mod fctlr;

#[doc = "FMCFGR register accessor: an alias for `Reg<FMCFGR_SPEC>`"]
pub type FMCFGR = crate::Reg<fmcfgr::FMCFGR_SPEC>;
#[doc = "FMCFGR"]
pub mod fmcfgr;

#[doc = "FSCFGR register accessor: an alias for `Reg<FSCFGR_SPEC>`"]
pub type FSCFGR = crate::Reg<fscfgr::FSCFGR_SPEC>;
#[doc = "FSCFGR"]
pub mod fscfgr;

#[doc = "FAFIFOR register accessor: an alias for `Reg<FAFIFOR_SPEC>`"]
pub type FAFIFOR = crate::Reg<fafifor::FAFIFOR_SPEC>;
#[doc = "FAFIFOR"]
pub mod fafifor;

#[doc = "FWR register accessor: an alias for `Reg<FWR_SPEC>`"]
pub type FWR = crate::Reg<fwr::FWR_SPEC>;
#[doc = "FWR"]
pub mod fwr;

#[doc = "F0R1 register accessor: an alias for `Reg<F0R1_SPEC>`"]
pub type F0R1 = crate::Reg<f0r1::F0R1_SPEC>;
#[doc = "Filter bank 0 register 1"]
pub mod f0r1;

#[doc = "F0R2 register accessor: an alias for `Reg<F0R2_SPEC>`"]
pub type F0R2 = crate::Reg<f0r2::F0R2_SPEC>;
#[doc = "Filter bank 0 register 2"]
pub mod f0r2;

#[doc = "F1R1 register accessor: an alias for `Reg<F1R1_SPEC>`"]
pub type F1R1 = crate::Reg<f1r1::F1R1_SPEC>;
#[doc = "Filter bank 1 register 1"]
pub mod f1r1;

#[doc = "F1R2 register accessor: an alias for `Reg<F1R2_SPEC>`"]
pub type F1R2 = crate::Reg<f1r2::F1R2_SPEC>;
#[doc = "Filter bank 1 register 2"]
pub mod f1r2;

#[doc = "F2R1 register accessor: an alias for `Reg<F2R1_SPEC>`"]
pub type F2R1 = crate::Reg<f2r1::F2R1_SPEC>;
#[doc = "Filter bank 2 register 1"]
pub mod f2r1;

#[doc = "F2R2 register accessor: an alias for `Reg<F2R2_SPEC>`"]
pub type F2R2 = crate::Reg<f2r2::F2R2_SPEC>;
#[doc = "Filter bank 2 register 2"]
pub mod f2r2;

#[doc = "F3R1 register accessor: an alias for `Reg<F3R1_SPEC>`"]
pub type F3R1 = crate::Reg<f3r1::F3R1_SPEC>;
#[doc = "Filter bank 3 register 1"]
pub mod f3r1;

#[doc = "F3R2 register accessor: an alias for `Reg<F3R2_SPEC>`"]
pub type F3R2 = crate::Reg<f3r2::F3R2_SPEC>;
#[doc = "Filter bank 3 register 2"]
pub mod f3r2;

#[doc = "F4R1 register accessor: an alias for `Reg<F4R1_SPEC>`"]
pub type F4R1 = crate::Reg<f4r1::F4R1_SPEC>;
#[doc = "Filter bank 4 register 1"]
pub mod f4r1;

#[doc = "F4R2 register accessor: an alias for `Reg<F4R2_SPEC>`"]
pub type F4R2 = crate::Reg<f4r2::F4R2_SPEC>;
#[doc = "Filter bank 4 register 2"]
pub mod f4r2;

#[doc = "F5R1 register accessor: an alias for `Reg<F5R1_SPEC>`"]
pub type F5R1 = crate::Reg<f5r1::F5R1_SPEC>;
#[doc = "Filter bank 5 register 1"]
pub mod f5r1;

#[doc = "F5R2 register accessor: an alias for `Reg<F5R2_SPEC>`"]
pub type F5R2 = crate::Reg<f5r2::F5R2_SPEC>;
#[doc = "Filter bank 5 register 2"]
pub mod f5r2;

#[doc = "F6R1 register accessor: an alias for `Reg<F6R1_SPEC>`"]
pub type F6R1 = crate::Reg<f6r1::F6R1_SPEC>;
#[doc = "Filter bank 6 register 1"]
pub mod f6r1;

#[doc = "F6R2 register accessor: an alias for `Reg<F6R2_SPEC>`"]
pub type F6R2 = crate::Reg<f6r2::F6R2_SPEC>;
#[doc = "Filter bank 6 register 2"]
pub mod f6r2;

#[doc = "F7R1 register accessor: an alias for `Reg<F7R1_SPEC>`"]
pub type F7R1 = crate::Reg<f7r1::F7R1_SPEC>;
#[doc = "Filter bank 7 register 1"]
pub mod f7r1;

#[doc = "F7R2 register accessor: an alias for `Reg<F7R2_SPEC>`"]
pub type F7R2 = crate::Reg<f7r2::F7R2_SPEC>;
#[doc = "Filter bank 7 register 2"]
pub mod f7r2;

#[doc = "F8R1 register accessor: an alias for `Reg<F8R1_SPEC>`"]
pub type F8R1 = crate::Reg<f8r1::F8R1_SPEC>;
#[doc = "Filter bank 8 register 1"]
pub mod f8r1;

#[doc = "F8R2 register accessor: an alias for `Reg<F8R2_SPEC>`"]
pub type F8R2 = crate::Reg<f8r2::F8R2_SPEC>;
#[doc = "Filter bank 8 register 2"]
pub mod f8r2;

#[doc = "F9R1 register accessor: an alias for `Reg<F9R1_SPEC>`"]
pub type F9R1 = crate::Reg<f9r1::F9R1_SPEC>;
#[doc = "Filter bank 9 register 1"]
pub mod f9r1;

#[doc = "F9R2 register accessor: an alias for `Reg<F9R2_SPEC>`"]
pub type F9R2 = crate::Reg<f9r2::F9R2_SPEC>;
#[doc = "Filter bank 9 register 2"]
pub mod f9r2;

#[doc = "F10R1 register accessor: an alias for `Reg<F10R1_SPEC>`"]
pub type F10R1 = crate::Reg<f10r1::F10R1_SPEC>;
#[doc = "Filter bank 10 register 1"]
pub mod f10r1;

#[doc = "F10R2 register accessor: an alias for `Reg<F10R2_SPEC>`"]
pub type F10R2 = crate::Reg<f10r2::F10R2_SPEC>;
#[doc = "Filter bank 10 register 2"]
pub mod f10r2;

#[doc = "F11R1 register accessor: an alias for `Reg<F11R1_SPEC>`"]
pub type F11R1 = crate::Reg<f11r1::F11R1_SPEC>;
#[doc = "Filter bank 11 register 1"]
pub mod f11r1;

#[doc = "F11R2 register accessor: an alias for `Reg<F11R2_SPEC>`"]
pub type F11R2 = crate::Reg<f11r2::F11R2_SPEC>;
#[doc = "Filter bank 11 register 2"]
pub mod f11r2;

#[doc = "F12R1 register accessor: an alias for `Reg<F12R1_SPEC>`"]
pub type F12R1 = crate::Reg<f12r1::F12R1_SPEC>;
#[doc = "Filter bank 4 register 1"]
pub mod f12r1;

#[doc = "F12R2 register accessor: an alias for `Reg<F12R2_SPEC>`"]
pub type F12R2 = crate::Reg<f12r2::F12R2_SPEC>;
#[doc = "Filter bank 12 register 2"]
pub mod f12r2;

#[doc = "F13R1 register accessor: an alias for `Reg<F13R1_SPEC>`"]
pub type F13R1 = crate::Reg<f13r1::F13R1_SPEC>;
#[doc = "Filter bank 13 register 1"]
pub mod f13r1;

#[doc = "F13R2 register accessor: an alias for `Reg<F13R2_SPEC>`"]
pub type F13R2 = crate::Reg<f13r2::F13R2_SPEC>;
#[doc = "Filter bank 13 register 2"]
pub mod f13r2;