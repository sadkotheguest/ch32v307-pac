#[doc = "Register `WPR` reader"]
pub struct R(crate::R<WPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WRP` reader - Write protect"]
pub type WRP_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Write protect"]
    #[inline(always)]
    pub fn wrp(&self) -> WRP_R {
        WRP_R::new(self.bits)
    }
}
#[doc = "Write protection register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpr](index.html) module"]
pub struct WPR_SPEC;
impl crate::RegisterSpec for WPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wpr::R](R) reader structure"]
impl crate::Readable for WPR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WPR to value 0xffff_ffff"]
impl crate::Resettable for WPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}