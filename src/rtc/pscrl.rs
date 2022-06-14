#[doc = "Register `PSCRL` writer"]
pub struct W(crate::W<PSCRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSCRL_SPEC>;
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
impl From<crate::W<PSCRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSCRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRLL` writer - RTC Prescaler Divider Register Low"]
pub type PRLL_W<'a> = crate::FieldWriter<'a, u32, PSCRL_SPEC, u16, u16, 16, 0>;
impl W {
    #[doc = "Bits 0:15 - RTC Prescaler Divider Register Low"]
    #[inline(always)]
    pub fn prll(&mut self) -> PRLL_W {
        PRLL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Prescaler Load Register Low\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pscrl](index.html) module"]
pub struct PSCRL_SPEC;
impl crate::RegisterSpec for PSCRL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pscrl::W](W) writer structure"]
impl crate::Writable for PSCRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PSCRL to value 0x8000"]
impl crate::Resettable for PSCRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000
    }
}