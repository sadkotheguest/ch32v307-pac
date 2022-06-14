#[doc = "Register `RESP4` reader"]
pub struct R(crate::R<RESP4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESP4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESP4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESP4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CARDSTATUS4` reader - CARDSTATUS4"]
pub type CARDSTATUS4_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - CARDSTATUS4"]
    #[inline(always)]
    pub fn cardstatus4(&self) -> CARDSTATUS4_R {
        CARDSTATUS4_R::new(self.bits)
    }
}
#[doc = "Bits 31:0 = CARDSTATUS4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resp4](index.html) module"]
pub struct RESP4_SPEC;
impl crate::RegisterSpec for RESP4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [resp4::R](R) reader structure"]
impl crate::Readable for RESP4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESP4 to value 0"]
impl crate::Resettable for RESP4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}