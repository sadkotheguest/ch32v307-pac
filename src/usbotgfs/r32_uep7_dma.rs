#[doc = "Register `R32_UEP7_DMA` reader"]
pub struct R(crate::R<R32_UEP7_DMA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R32_UEP7_DMA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R32_UEP7_DMA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R32_UEP7_DMA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R32_UEP7_DMA` writer"]
pub struct W(crate::W<R32_UEP7_DMA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R32_UEP7_DMA_SPEC>;
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
impl From<crate::W<R32_UEP7_DMA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R32_UEP7_DMA_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "endpoint 7 DMA buffer address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r32_uep7_dma](index.html) module"]
pub struct R32_UEP7_DMA_SPEC;
impl crate::RegisterSpec for R32_UEP7_DMA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [r32_uep7_dma::R](R) reader structure"]
impl crate::Readable for R32_UEP7_DMA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r32_uep7_dma::W](W) writer structure"]
impl crate::Writable for R32_UEP7_DMA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R32_UEP7_DMA to value 0"]
impl crate::Resettable for R32_UEP7_DMA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}