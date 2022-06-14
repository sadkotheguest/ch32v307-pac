#[doc = "Register `MADDR` reader"]
pub struct R(crate::R<MADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MADDR` writer"]
pub struct W(crate::W<MADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MADDR_SPEC>;
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
impl From<crate::W<MADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MA` reader - Memory address"]
pub type MA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MA` writer - Memory address"]
pub type MA_W<'a> = crate::FieldWriter<'a, u32, MADDR_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Memory address"]
    #[inline(always)]
    pub fn ma(&self) -> MA_R {
        MA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory address"]
    #[inline(always)]
    pub fn ma(&mut self) -> MA_W {
        MA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA channel 1 memory address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maddr](index.html) module"]
pub struct MADDR_SPEC;
impl crate::RegisterSpec for MADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [maddr::R](R) reader structure"]
impl crate::Readable for MADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [maddr::W](W) writer structure"]
impl crate::Writable for MADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MADDR to value 0"]
impl crate::Resettable for MADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}