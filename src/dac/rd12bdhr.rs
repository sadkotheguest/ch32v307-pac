#[doc = "Register `RD12BDHR` reader"]
pub struct R(crate::R<RD12BDHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD12BDHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD12BDHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD12BDHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RD12BDHR` writer"]
pub struct W(crate::W<RD12BDHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RD12BDHR_SPEC>;
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
impl From<crate::W<RD12BDHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RD12BDHR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DACC1DHR` reader - DAC channel1 12-bit right-aligned data"]
pub type DACC1DHR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DACC1DHR` writer - DAC channel1 12-bit right-aligned data"]
pub type DACC1DHR_W<'a> = crate::FieldWriter<'a, u32, RD12BDHR_SPEC, u16, u16, 12, 0>;
#[doc = "Field `DACC2DHR` reader - DAC channel2 12-bit right-aligned data"]
pub type DACC2DHR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DACC2DHR` writer - DAC channel2 12-bit right-aligned data"]
pub type DACC2DHR_W<'a> = crate::FieldWriter<'a, u32, RD12BDHR_SPEC, u16, u16, 12, 16>;
impl R {
    #[doc = "Bits 0:11 - DAC channel1 12-bit right-aligned data"]
    #[inline(always)]
    pub fn dacc1dhr(&self) -> DACC1DHR_R {
        DACC1DHR_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - DAC channel2 12-bit right-aligned data"]
    #[inline(always)]
    pub fn dacc2dhr(&self) -> DACC2DHR_R {
        DACC2DHR_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC channel1 12-bit right-aligned data"]
    #[inline(always)]
    pub fn dacc1dhr(&mut self) -> DACC1DHR_W {
        DACC1DHR_W::new(self)
    }
    #[doc = "Bits 16:27 - DAC channel2 12-bit right-aligned data"]
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
#[doc = "Dual DAC 12-bit right-aligned data holding register (DAC_DHR12RD), Bits 31:28 Reserved, Bits 15:12 Reserved\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd12bdhr](index.html) module"]
pub struct RD12BDHR_SPEC;
impl crate::RegisterSpec for RD12BDHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd12bdhr::R](R) reader structure"]
impl crate::Readable for RD12BDHR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rd12bdhr::W](W) writer structure"]
impl crate::Writable for RD12BDHR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RD12BDHR to value 0"]
impl crate::Resettable for RD12BDHR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}