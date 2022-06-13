#[doc = "Register `RXMIR1` reader"]
pub struct R(crate::R<RXMIR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXMIR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXMIR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXMIR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STID` reader - STID"]
pub type STID_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EXID` reader - EXID"]
pub type EXID_R = crate::FieldReader<u32, u32>;
#[doc = "Field `IDE` reader - IDE"]
pub type IDE_R = crate::BitReader<bool>;
#[doc = "Field `RTR` reader - RTR"]
pub type RTR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 21:31 - STID"]
    #[inline(always)]
    pub fn stid(&self) -> STID_R {
        STID_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
    #[doc = "Bits 3:20 - EXID"]
    #[inline(always)]
    pub fn exid(&self) -> EXID_R {
        EXID_R::new(((self.bits >> 3) & 0x0003_ffff) as u32)
    }
    #[doc = "Bit 2 - IDE"]
    #[inline(always)]
    pub fn ide(&self) -> IDE_R {
        IDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - RTR"]
    #[inline(always)]
    pub fn rtr(&self) -> RTR_R {
        RTR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "RXMIR1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxmir1](index.html) module"]
pub struct RXMIR1_SPEC;
impl crate::RegisterSpec for RXMIR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxmir1::R](R) reader structure"]
impl crate::Readable for RXMIR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXMIR1 to value 0"]
impl crate::Resettable for RXMIR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}