#[doc = "Register `IPR2` reader"]
pub struct R(crate::R<IPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PENDSTA` reader - PENDSTA"]
pub type PENDSTA_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - PENDSTA"]
    #[inline(always)]
    pub fn pendsta(&self) -> PENDSTA_R {
        PENDSTA_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Interrupt Pending Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipr2](index.html) module"]
pub struct IPR2_SPEC;
impl crate::RegisterSpec for IPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipr2::R](R) reader structure"]
impl crate::Readable for IPR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IPR2 to value 0"]
impl crate::Resettable for IPR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}