#[doc = "Register `R12BDHR2` reader"]
pub struct R(crate::R<R12BDHR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R12BDHR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R12BDHR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R12BDHR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R12BDHR2` writer"]
pub struct W(crate::W<R12BDHR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R12BDHR2_SPEC>;
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
impl From<crate::W<R12BDHR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R12BDHR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DACC2DHR` reader - DAC channel2 12-bit right-aligned data"]
pub type DACC2DHR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DACC2DHR` writer - DAC channel2 12-bit right-aligned data"]
pub type DACC2DHR_W<'a> = crate::FieldWriter<'a, u32, R12BDHR2_SPEC, u16, u16, 12, 0>;
impl R {
    #[doc = "Bits 0:11 - DAC channel2 12-bit right-aligned data"]
    #[inline(always)]
    pub fn dacc2dhr(&self) -> DACC2DHR_R {
        DACC2DHR_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC channel2 12-bit right-aligned data"]
    #[inline(always)]
    pub fn dacc2dhr(&mut self) -> DACC2DHR_W {
        DACC2DHR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC channel2 12-bit right aligned data holding register (DAC_DHR12R2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r12bdhr2](index.html) module"]
pub struct R12BDHR2_SPEC;
impl crate::RegisterSpec for R12BDHR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [r12bdhr2::R](R) reader structure"]
impl crate::Readable for R12BDHR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r12bdhr2::W](W) writer structure"]
impl crate::Writable for R12BDHR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R12BDHR2 to value 0"]
impl crate::Resettable for R12BDHR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}