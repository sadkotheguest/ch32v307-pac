#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Digital Video control register (DVP_CR0)"]
    pub cr0: crate::Reg<cr0::CR0_SPEC>,
    _reserved_1_cr1: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x04 - Digital Video STATUS register (DVP_STATUS)"]
    #[inline(always)]
    pub fn status(&self) -> &crate::Reg<status::STATUS_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize)
                as *const crate::Reg<status::STATUS_SPEC>)
        }
    }
    #[doc = "0x04 - Digital Video Flag register (DVP_IFR)"]
    #[inline(always)]
    pub fn ifr(&self) -> &crate::Reg<ifr::IFR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize)
                as *const crate::Reg<ifr::IFR_SPEC>)
        }
    }
    #[doc = "0x04 - Digital Video Interrupt register (DVP_IER)"]
    #[inline(always)]
    pub fn ier(&self) -> &crate::Reg<ier::IER_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize)
                as *const crate::Reg<ier::IER_SPEC>)
        }
    }
    #[doc = "0x04 - Digital Video control register (DVP_CR1)"]
    #[inline(always)]
    pub fn cr1(&self) -> &crate::Reg<cr1::CR1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize)
                as *const crate::Reg<cr1::CR1_SPEC>)
        }
    }
}

#[doc = "CR0 register accessor: an alias for `Reg<CR0_SPEC>`"]
pub type CR0 = crate::Reg<cr0::CR0_SPEC>;
#[doc = "Digital Video control register (DVP_CR0)"]
pub mod cr0;

#[doc = "CR1 register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "Digital Video control register (DVP_CR1)"]
pub mod cr1;

#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Digital Video Interrupt register (DVP_IER)"]
pub mod ier;

#[doc = "IFR register accessor: an alias for `Reg<IFR_SPEC>`"]
pub type IFR = crate::Reg<ifr::IFR_SPEC>;
#[doc = "Digital Video Flag register (DVP_IFR)"]
pub mod ifr;

#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Digital Video STATUS register (DVP_STATUS)"]
pub mod status;