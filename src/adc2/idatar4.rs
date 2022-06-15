#[doc = "Register `IDATAR4` reader"]
pub struct R(crate::R<IDATAR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDATAR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDATAR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDATAR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `JDATA` reader - Injected data"]
pub type JDATA_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Injected data"]
    #[inline(always)]
    pub fn jdata(&self) -> JDATA_R {
        JDATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "injected data register x\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idatar4](index.html) module"]
pub struct IDATAR4_SPEC;
impl crate::RegisterSpec for IDATAR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idatar4::R](R) reader structure"]
impl crate::Readable for IDATAR4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IDATAR4 to value 0"]
impl crate::Resettable for IDATAR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}