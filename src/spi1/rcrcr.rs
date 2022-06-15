#[doc = "Register `RCRCR` reader"]
pub struct R(crate::R<RCRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXCRC` reader - Rx CRC register"]
pub type RXCRC_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Rx CRC register"]
    #[inline(always)]
    pub fn rxcrc(&self) -> RXCRC_R {
        RXCRC_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "RX CRC register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcrcr](index.html) module"]
pub struct RCRCR_SPEC;
impl crate::RegisterSpec for RCRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcrcr::R](R) reader structure"]
impl crate::Readable for RCRCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RCRCR to value 0"]
impl crate::Resettable for RCRCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}