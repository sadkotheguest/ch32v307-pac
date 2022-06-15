#[doc = "Register `WDHTR` reader"]
pub struct R(crate::R<WDHTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDHTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDHTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDHTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDHTR` writer"]
pub struct W(crate::W<WDHTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDHTR_SPEC>;
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
impl From<crate::W<WDHTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDHTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HT` reader - Analog watchdog higher threshold"]
pub type HT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HT` writer - Analog watchdog higher threshold"]
pub type HT_W<'a> = crate::FieldWriter<'a, u32, WDHTR_SPEC, u16, u16, 12, 0>;
impl R {
    #[doc = "Bits 0:11 - Analog watchdog higher threshold"]
    #[inline(always)]
    pub fn ht(&self) -> HT_R {
        HT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Analog watchdog higher threshold"]
    #[inline(always)]
    pub fn ht(&mut self) -> HT_W {
        HT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "watchdog higher threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdhtr](index.html) module"]
pub struct WDHTR_SPEC;
impl crate::RegisterSpec for WDHTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdhtr::R](R) reader structure"]
impl crate::Readable for WDHTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdhtr::W](W) writer structure"]
impl crate::Writable for WDHTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDHTR to value 0"]
impl crate::Resettable for WDHTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}