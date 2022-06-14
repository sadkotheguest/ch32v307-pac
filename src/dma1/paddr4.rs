#[doc = "Register `PADDR4` reader"]
pub struct R(crate::R<PADDR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PADDR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PADDR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PADDR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PADDR4` writer"]
pub struct W(crate::W<PADDR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PADDR4_SPEC>;
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
impl From<crate::W<PADDR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PADDR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PA` reader - Peripheral address"]
pub type PA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PA` writer - Peripheral address"]
pub type PA_W<'a> = crate::FieldWriter<'a, u32, PADDR4_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Peripheral address"]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Peripheral address"]
    #[inline(always)]
    pub fn pa(&mut self) -> PA_W {
        PA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA channel 4 peripheral address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [paddr4](index.html) module"]
pub struct PADDR4_SPEC;
impl crate::RegisterSpec for PADDR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [paddr4::R](R) reader structure"]
impl crate::Readable for PADDR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [paddr4::W](W) writer structure"]
impl crate::Writable for PADDR4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PADDR4 to value 0"]
impl crate::Resettable for PADDR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}