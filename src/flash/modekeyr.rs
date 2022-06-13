#[doc = "Register `MODEKEYR` writer"]
pub struct W(crate::W<MODEKEYR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODEKEYR_SPEC>;
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
impl From<crate::W<MODEKEYR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODEKEYR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODEKEYR` writer - Mode select"]
pub type MODEKEYR_W<'a> = crate::FieldWriter<'a, u32, MODEKEYR_SPEC, u32, u32, 32, 0>;
impl W {
    #[doc = "Bits 0:31 - Mode select"]
    #[inline(always)]
    pub fn modekeyr(&mut self) -> MODEKEYR_W {
        MODEKEYR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode select register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modekeyr](index.html) module"]
pub struct MODEKEYR_SPEC;
impl crate::RegisterSpec for MODEKEYR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [modekeyr::W](W) writer structure"]
impl crate::Writable for MODEKEYR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MODEKEYR to value 0"]
impl crate::Resettable for MODEKEYR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}