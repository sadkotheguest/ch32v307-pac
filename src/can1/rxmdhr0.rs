#[doc = "Register `RXMDHR0` reader"]
pub struct R(crate::R<RXMDHR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXMDHR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXMDHR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXMDHR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA7` reader - DATA7"]
pub type DATA7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA6` reader - DATA6"]
pub type DATA6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA5` reader - DATA5"]
pub type DATA5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA4` reader - DATA4"]
pub type DATA4_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 24:31 - DATA7"]
    #[inline(always)]
    pub fn data7(&self) -> DATA7_R {
        DATA7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DATA6"]
    #[inline(always)]
    pub fn data6(&self) -> DATA6_R {
        DATA6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DATA5"]
    #[inline(always)]
    pub fn data5(&self) -> DATA5_R {
        DATA5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - DATA4"]
    #[inline(always)]
    pub fn data4(&self) -> DATA4_R {
        DATA4_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "RXMDHR0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxmdhr0](index.html) module"]
pub struct RXMDHR0_SPEC;
impl crate::RegisterSpec for RXMDHR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxmdhr0::R](R) reader structure"]
impl crate::Readable for RXMDHR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXMDHR0 to value 0"]
impl crate::Resettable for RXMDHR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}