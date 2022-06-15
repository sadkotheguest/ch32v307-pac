#[doc = "Register `TCRCR` reader"]
pub struct R(crate::R<TCRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXCRC` reader - Tx CRC register"]
pub type TXCRC_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Tx CRC register"]
    #[inline(always)]
    pub fn txcrc(&self) -> TXCRC_R {
        TXCRC_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "TX CRC register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcrcr](index.html) module"]
pub struct TCRCR_SPEC;
impl crate::RegisterSpec for TCRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcrcr::R](R) reader structure"]
impl crate::Readable for TCRCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TCRCR to value 0"]
impl crate::Resettable for TCRCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}