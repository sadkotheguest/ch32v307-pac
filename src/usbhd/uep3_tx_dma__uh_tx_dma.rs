#[doc = "Register `UEP3_TX_DMA__UH_TX_DMA` reader"]
pub struct R(crate::R<UEP3_TX_DMA__UH_TX_DMA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UEP3_TX_DMA__UH_TX_DMA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UEP3_TX_DMA__UH_TX_DMA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UEP3_TX_DMA__UH_TX_DMA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UEP3_TX_DMA__UH_TX_DMA` writer"]
pub struct W(crate::W<UEP3_TX_DMA__UH_TX_DMA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UEP3_TX_DMA__UH_TX_DMA_SPEC>;
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
impl From<crate::W<UEP3_TX_DMA__UH_TX_DMA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UEP3_TX_DMA__UH_TX_DMA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UEP3_TX_DMA__UH_TX_DMA` reader - endpoint 3 DMA buffer address"]
pub type UEP3_TX_DMA__UH_TX_DMA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `UEP3_TX_DMA__UH_TX_DMA` writer - endpoint 3 DMA buffer address"]
pub type UEP3_TX_DMA__UH_TX_DMA_W<'a> =
    crate::FieldWriter<'a, u32, UEP3_TX_DMA__UH_TX_DMA_SPEC, u32, u32, 17, 0>;
impl R {
    #[doc = "Bits 0:16 - endpoint 3 DMA buffer address"]
    #[inline(always)]
    pub fn uep3_tx_dma__uh_tx_dma(&self) -> UEP3_TX_DMA__UH_TX_DMA_R {
        UEP3_TX_DMA__UH_TX_DMA_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:16 - endpoint 3 DMA buffer address"]
    #[inline(always)]
    pub fn uep3_tx_dma__uh_tx_dma(&mut self) -> UEP3_TX_DMA__UH_TX_DMA_W {
        UEP3_TX_DMA__UH_TX_DMA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "endpoint 3 DMA TX buffer address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uep3_tx_dma__uh_tx_dma](index.html) module"]
pub struct UEP3_TX_DMA__UH_TX_DMA_SPEC;
impl crate::RegisterSpec for UEP3_TX_DMA__UH_TX_DMA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uep3_tx_dma__uh_tx_dma::R](R) reader structure"]
impl crate::Readable for UEP3_TX_DMA__UH_TX_DMA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uep3_tx_dma__uh_tx_dma::W](W) writer structure"]
impl crate::Writable for UEP3_TX_DMA__UH_TX_DMA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UEP3_TX_DMA__UH_TX_DMA to value 0"]
impl crate::Resettable for UEP3_TX_DMA__UH_TX_DMA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}