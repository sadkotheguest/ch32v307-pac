#[doc = "Register `RDATAR__SDR` reader"]
pub struct R(crate::R<RDATAR__SDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDATAR__SDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDATAR__SDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDATAR__SDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDATA` reader - Regular data"]
pub type RDATA_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Regular data"]
    #[inline(always)]
    pub fn rdata(&self) -> RDATA_R {
        RDATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "regular data register;TKEY_V status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdatar__sdr](index.html) module"]
pub struct RDATAR__SDR_SPEC;
impl crate::RegisterSpec for RDATAR__SDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rdatar__sdr::R](R) reader structure"]
impl crate::Readable for RDATAR__SDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RDATAR__SDR to value 0"]
impl crate::Resettable for RDATAR__SDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}