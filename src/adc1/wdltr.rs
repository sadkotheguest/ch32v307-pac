#[doc = "Register `WDLTR` reader"]
pub struct R(crate::R<WDLTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDLTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDLTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDLTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDLTR` writer"]
pub struct W(crate::W<WDLTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDLTR_SPEC>;
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
impl From<crate::W<WDLTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDLTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LT` reader - Analog watchdog lower threshold"]
pub type LT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LT` writer - Analog watchdog lower threshold"]
pub type LT_W<'a> = crate::FieldWriter<'a, u32, WDLTR_SPEC, u16, u16, 12, 0>;
impl R {
    #[doc = "Bits 0:11 - Analog watchdog lower threshold"]
    #[inline(always)]
    pub fn lt(&self) -> LT_R {
        LT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Analog watchdog lower threshold"]
    #[inline(always)]
    pub fn lt(&mut self) -> LT_W {
        LT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "watchdog lower threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdltr](index.html) module"]
pub struct WDLTR_SPEC;
impl crate::RegisterSpec for WDLTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdltr::R](R) reader structure"]
impl crate::Readable for WDLTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdltr::W](W) writer structure"]
impl crate::Writable for WDLTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDLTR to value 0"]
impl crate::Resettable for WDLTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}