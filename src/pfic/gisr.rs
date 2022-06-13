#[doc = "Register `GISR` reader"]
pub struct R(crate::R<GISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NESTSTA` reader - NESTSTA"]
pub type NESTSTA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GACTSTA` reader - GACTSTA"]
pub type GACTSTA_R = crate::BitReader<bool>;
#[doc = "Field `GPENDSTA` reader - GPENDSTA"]
pub type GPENDSTA_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:7 - NESTSTA"]
    #[inline(always)]
    pub fn neststa(&self) -> NESTSTA_R {
        NESTSTA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - GACTSTA"]
    #[inline(always)]
    pub fn gactsta(&self) -> GACTSTA_R {
        GACTSTA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPENDSTA"]
    #[inline(always)]
    pub fn gpendsta(&self) -> GPENDSTA_R {
        GPENDSTA_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Interrupt Global Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gisr](index.html) module"]
pub struct GISR_SPEC;
impl crate::RegisterSpec for GISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gisr::R](R) reader structure"]
impl crate::Readable for GISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GISR to value 0"]
impl crate::Resettable for GISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}