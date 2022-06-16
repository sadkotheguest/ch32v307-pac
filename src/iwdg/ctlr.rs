#[doc = "Register `CTLR` writer"]
pub struct W(crate::W<CTLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTLR_SPEC>;
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
impl From<crate::W<CTLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY` writer - Key value"]
pub type KEY_W<'a> = crate::FieldWriter<'a, u32, CTLR_SPEC, u16, u16, 16, 0>;
impl W {
    #[doc = "Bits 0:15 - Key value"]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Key register (IWDG_CTLR)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctlr](index.html) module"]
pub struct CTLR_SPEC;
impl crate::RegisterSpec for CTLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ctlr::W](W) writer structure"]
impl crate::Writable for CTLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTLR to value 0"]
impl crate::Resettable for CTLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}