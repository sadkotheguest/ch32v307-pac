#[doc = "Register `PSCRH` writer"]
pub struct W(crate::W<PSCRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSCRH_SPEC>;
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
impl From<crate::W<PSCRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSCRH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRLH` writer - RTC Prescaler Load Register High"]
pub type PRLH_W<'a> = crate::FieldWriter<'a, u32, PSCRH_SPEC, u8, u8, 4, 0>;
impl W {
    #[doc = "Bits 0:3 - RTC Prescaler Load Register High"]
    #[inline(always)]
    pub fn prlh(&mut self) -> PRLH_W {
        PRLH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Prescaler Load Register High\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pscrh](index.html) module"]
pub struct PSCRH_SPEC;
impl crate::RegisterSpec for PSCRH_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pscrh::W](W) writer structure"]
impl crate::Writable for PSCRH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PSCRH to value 0"]
impl crate::Resettable for PSCRH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}