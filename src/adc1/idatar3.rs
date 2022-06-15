#[doc = "Register `IDATAR3` reader"]
pub struct R(crate::R<IDATAR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDATAR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDATAR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDATAR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IDATA` reader - Injected data"]
pub type IDATA_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Injected data"]
    #[inline(always)]
    pub fn idata(&self) -> IDATA_R {
        IDATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "injected data register x\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idatar3](index.html) module"]
pub struct IDATAR3_SPEC;
impl crate::RegisterSpec for IDATAR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idatar3::R](R) reader structure"]
impl crate::Readable for IDATAR3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IDATAR3 to value 0"]
impl crate::Resettable for IDATAR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}