#[doc = "Register `UEP15_RX_DMA` reader"]
pub struct R(crate::R<UEP15_RX_DMA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UEP15_RX_DMA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UEP15_RX_DMA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UEP15_RX_DMA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UEP15_RX_DMA` writer"]
pub struct W(crate::W<UEP15_RX_DMA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UEP15_RX_DMA_SPEC>;
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
impl From<crate::W<UEP15_RX_DMA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UEP15_RX_DMA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UEP15_RX_DMA` reader - endpoint 15 DMA buffer address"]
pub type UEP15_RX_DMA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `UEP15_RX_DMA` writer - endpoint 15 DMA buffer address"]
pub type UEP15_RX_DMA_W<'a> =
    crate::FieldWriter<'a, u32, UEP15_RX_DMA_SPEC, u32, u32, 17, 0>;
impl R {
    #[doc = "Bits 0:16 - endpoint 15 DMA buffer address"]
    #[inline(always)]
    pub fn uep15_rx_dma(&self) -> UEP15_RX_DMA_R {
        UEP15_RX_DMA_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:16 - endpoint 15 DMA buffer address"]
    #[inline(always)]
    pub fn uep15_rx_dma(&mut self) -> UEP15_RX_DMA_W {
        UEP15_RX_DMA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "endpoint 15 DMA RX buffer address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uep15_rx_dma](index.html) module"]
pub struct UEP15_RX_DMA_SPEC;
impl crate::RegisterSpec for UEP15_RX_DMA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uep15_rx_dma::R](R) reader structure"]
impl crate::Readable for UEP15_RX_DMA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uep15_rx_dma::W](W) writer structure"]
impl crate::Writable for UEP15_RX_DMA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UEP15_RX_DMA to value 0"]
impl crate::Resettable for UEP15_RX_DMA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}