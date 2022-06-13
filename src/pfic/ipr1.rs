#[doc = "Register `IPR1` reader"]
pub struct R(crate::R<IPR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PENDSTA` reader - PENDSTA"]
pub type PENDSTA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 12:31 - PENDSTA"]
    #[inline(always)]
    pub fn pendsta(&self) -> PENDSTA_R {
        PENDSTA_R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
}
#[doc = "Interrupt Pending Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipr1](index.html) module"]
pub struct IPR1_SPEC;
impl crate::RegisterSpec for IPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipr1::R](R) reader structure"]
impl crate::Readable for IPR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IPR1 to value 0"]
impl crate::Resettable for IPR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}