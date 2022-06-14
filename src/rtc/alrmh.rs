#[doc = "Register `ALRMH` writer"]
pub struct W(crate::W<ALRMH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALRMH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ALRMH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALRMH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALRH` writer - RTC alarm register high"]
pub type ALRH_W<'a> = crate::FieldWriter<'a, u32, ALRMH_SPEC, u16, u16, 16, 0>;
impl W {
    #[doc = "Bits 0:15 - RTC alarm register high"]
    #[inline(always)]
    pub fn alrh(&mut self) -> ALRH_W {
        ALRH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Alarm Register High\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alrmh](index.html) module"]
pub struct ALRMH_SPEC;
impl crate::RegisterSpec for ALRMH_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [alrmh::W](W) writer structure"]
impl crate::Writable for ALRMH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALRMH to value 0xffff"]
impl crate::Resettable for ALRMH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}