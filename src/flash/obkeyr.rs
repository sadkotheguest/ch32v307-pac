#[doc = "Register `OBKEYR` writer"]
pub struct W(crate::W<OBKEYR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OBKEYR_SPEC>;
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
impl From<crate::W<OBKEYR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OBKEYR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPTKEY` writer - Option byte key"]
pub type OPTKEY_W<'a> = crate::FieldWriter<'a, u32, OBKEYR_SPEC, u32, u32, 32, 0>;
impl W {
    #[doc = "Bits 0:31 - Option byte key"]
    #[inline(always)]
    pub fn optkey(&mut self) -> OPTKEY_W {
        OPTKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash option key register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [obkeyr](index.html) module"]
pub struct OBKEYR_SPEC;
impl crate::RegisterSpec for OBKEYR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [obkeyr::W](W) writer structure"]
impl crate::Writable for OBKEYR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OBKEYR to value 0"]
impl crate::Resettable for OBKEYR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}