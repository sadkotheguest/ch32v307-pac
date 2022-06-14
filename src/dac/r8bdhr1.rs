#[doc = "Register `R8BDHR1` reader"]
pub struct R(crate::R<R8BDHR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8BDHR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8BDHR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8BDHR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8BDHR1` writer"]
pub struct W(crate::W<R8BDHR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8BDHR1_SPEC>;
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
impl From<crate::W<R8BDHR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8BDHR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DACC1DHR` reader - DAC channel1 8-bit right-aligned data"]
pub type DACC1DHR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DACC1DHR` writer - DAC channel1 8-bit right-aligned data"]
pub type DACC1DHR_W<'a> = crate::FieldWriter<'a, u32, R8BDHR1_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - DAC channel1 8-bit right-aligned data"]
    #[inline(always)]
    pub fn dacc1dhr(&self) -> DACC1DHR_R {
        DACC1DHR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DAC channel1 8-bit right-aligned data"]
    #[inline(always)]
    pub fn dacc1dhr(&mut self) -> DACC1DHR_W {
        DACC1DHR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC channel1 8-bit right aligned data holding register (DAC_DHR8R1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8bdhr1](index.html) module"]
pub struct R8BDHR1_SPEC;
impl crate::RegisterSpec for R8BDHR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [r8bdhr1::R](R) reader structure"]
impl crate::Readable for R8BDHR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8bdhr1::W](W) writer structure"]
impl crate::Writable for R8BDHR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8BDHR1 to value 0"]
impl crate::Resettable for R8BDHR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}