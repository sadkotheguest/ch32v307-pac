#[doc = "Register `RD8BDHR` reader"]
pub struct R(crate::R<RD8BDHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD8BDHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD8BDHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD8BDHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RD8BDHR` writer"]
pub struct W(crate::W<RD8BDHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RD8BDHR_SPEC>;
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
impl From<crate::W<RD8BDHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RD8BDHR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DACC1DHR` reader - DAC channel1 8-bit right-aligned data"]
pub type DACC1DHR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DACC1DHR` writer - DAC channel1 8-bit right-aligned data"]
pub type DACC1DHR_W<'a> = crate::FieldWriter<'a, u32, RD8BDHR_SPEC, u8, u8, 8, 0>;
#[doc = "Field `DACC2DHR` reader - DAC channel2 8-bit right-aligned data"]
pub type DACC2DHR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DACC2DHR` writer - DAC channel2 8-bit right-aligned data"]
pub type DACC2DHR_W<'a> = crate::FieldWriter<'a, u32, RD8BDHR_SPEC, u8, u8, 8, 8>;
impl R {
    #[doc = "Bits 0:7 - DAC channel1 8-bit right-aligned data"]
    #[inline(always)]
    pub fn dacc1dhr(&self) -> DACC1DHR_R {
        DACC1DHR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DAC channel2 8-bit right-aligned data"]
    #[inline(always)]
    pub fn dacc2dhr(&self) -> DACC2DHR_R {
        DACC2DHR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DAC channel1 8-bit right-aligned data"]
    #[inline(always)]
    pub fn dacc1dhr(&mut self) -> DACC1DHR_W {
        DACC1DHR_W::new(self)
    }
    #[doc = "Bits 8:15 - DAC channel2 8-bit right-aligned data"]
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
#[doc = "DUAL DAC 8-bit right aligned data holding register (DAC_DHR8RD), Bits 31:16 Reserved\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd8bdhr](index.html) module"]
pub struct RD8BDHR_SPEC;
impl crate::RegisterSpec for RD8BDHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd8bdhr::R](R) reader structure"]
impl crate::Readable for RD8BDHR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rd8bdhr::W](W) writer structure"]
impl crate::Writable for RD8BDHR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RD8BDHR to value 0"]
impl crate::Resettable for RD8BDHR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}