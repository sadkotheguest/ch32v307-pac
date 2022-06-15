#[doc = "Register `CR1` reader"]
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR1` writer"]
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_DVP_DMA_EN` reader - DVP dma enable"]
pub type RB_DVP_DMA_EN_R = crate::BitReader<bool>;
#[doc = "Field `RB_DVP_DMA_EN` writer - DVP dma enable"]
pub type RB_DVP_DMA_EN_W<'a> = crate::BitWriter<'a, u32, CR1_SPEC, bool, 0>;
#[doc = "Field `RB_DVP_ALL_CLR` reader - DVP all clear"]
pub type RB_DVP_ALL_CLR_R = crate::BitReader<bool>;
#[doc = "Field `RB_DVP_ALL_CLR` writer - DVP all clear"]
pub type RB_DVP_ALL_CLR_W<'a> = crate::BitWriter<'a, u32, CR1_SPEC, bool, 1>;
#[doc = "Field `RB_DVP_RCV_CLR` reader - DVP receive logic clear"]
pub type RB_DVP_RCV_CLR_R = crate::BitReader<bool>;
#[doc = "Field `RB_DVP_RCV_CLR` writer - DVP receive logic clear"]
pub type RB_DVP_RCV_CLR_W<'a> = crate::BitWriter<'a, u32, CR1_SPEC, bool, 2>;
#[doc = "Field `RB_DVP_BUF_TOG` reader - DVP bug toggle by software"]
pub type RB_DVP_BUF_TOG_R = crate::BitReader<bool>;
#[doc = "Field `RB_DVP_BUF_TOG` writer - DVP bug toggle by software"]
pub type RB_DVP_BUF_TOG_W<'a> = crate::BitWriter<'a, u32, CR1_SPEC, bool, 3>;
#[doc = "Field `RB_DVP_CM` reader - DVP capture mode"]
pub type RB_DVP_CM_R = crate::BitReader<bool>;
#[doc = "Field `RB_DVP_CM` writer - DVP capture mode"]
pub type RB_DVP_CM_W<'a> = crate::BitWriter<'a, u32, CR1_SPEC, bool, 4>;
#[doc = "Field `RB_DVP_CROP` reader - DVP Crop feature enable"]
pub type RB_DVP_CROP_R = crate::BitReader<bool>;
#[doc = "Field `RB_DVP_CROP` writer - DVP Crop feature enable"]
pub type RB_DVP_CROP_W<'a> = crate::BitWriter<'a, u32, CR1_SPEC, bool, 5>;
#[doc = "Field `RB_DVP_FCRC` reader - DVP frame capture rate control"]
pub type RB_DVP_FCRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RB_DVP_FCRC` writer - DVP frame capture rate control"]
pub type RB_DVP_FCRC_W<'a> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 2, 6>;
impl R {
    #[doc = "Bit 0 - DVP dma enable"]
    #[inline(always)]
    pub fn rb_dvp_dma_en(&self) -> RB_DVP_DMA_EN_R {
        RB_DVP_DMA_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DVP all clear"]
    #[inline(always)]
    pub fn rb_dvp_all_clr(&self) -> RB_DVP_ALL_CLR_R {
        RB_DVP_ALL_CLR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DVP receive logic clear"]
    #[inline(always)]
    pub fn rb_dvp_rcv_clr(&self) -> RB_DVP_RCV_CLR_R {
        RB_DVP_RCV_CLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DVP bug toggle by software"]
    #[inline(always)]
    pub fn rb_dvp_buf_tog(&self) -> RB_DVP_BUF_TOG_R {
        RB_DVP_BUF_TOG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DVP capture mode"]
    #[inline(always)]
    pub fn rb_dvp_cm(&self) -> RB_DVP_CM_R {
        RB_DVP_CM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DVP Crop feature enable"]
    #[inline(always)]
    pub fn rb_dvp_crop(&self) -> RB_DVP_CROP_R {
        RB_DVP_CROP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - DVP frame capture rate control"]
    #[inline(always)]
    pub fn rb_dvp_fcrc(&self) -> RB_DVP_FCRC_R {
        RB_DVP_FCRC_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DVP dma enable"]
    #[inline(always)]
    pub fn rb_dvp_dma_en(&mut self) -> RB_DVP_DMA_EN_W {
        RB_DVP_DMA_EN_W::new(self)
    }
    #[doc = "Bit 1 - DVP all clear"]
    #[inline(always)]
    pub fn rb_dvp_all_clr(&mut self) -> RB_DVP_ALL_CLR_W {
        RB_DVP_ALL_CLR_W::new(self)
    }
    #[doc = "Bit 2 - DVP receive logic clear"]
    #[inline(always)]
    pub fn rb_dvp_rcv_clr(&mut self) -> RB_DVP_RCV_CLR_W {
        RB_DVP_RCV_CLR_W::new(self)
    }
    #[doc = "Bit 3 - DVP bug toggle by software"]
    #[inline(always)]
    pub fn rb_dvp_buf_tog(&mut self) -> RB_DVP_BUF_TOG_W {
        RB_DVP_BUF_TOG_W::new(self)
    }
    #[doc = "Bit 4 - DVP capture mode"]
    #[inline(always)]
    pub fn rb_dvp_cm(&mut self) -> RB_DVP_CM_W {
        RB_DVP_CM_W::new(self)
    }
    #[doc = "Bit 5 - DVP Crop feature enable"]
    #[inline(always)]
    pub fn rb_dvp_crop(&mut self) -> RB_DVP_CROP_W {
        RB_DVP_CROP_W::new(self)
    }
    #[doc = "Bits 6:7 - DVP frame capture rate control"]
    #[inline(always)]
    pub fn rb_dvp_fcrc(&mut self) -> RB_DVP_FCRC_W {
        RB_DVP_FCRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Digital Video control register (DVP_CR1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](index.html) module"]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr1::R](R) reader structure"]
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr1::W](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR1 to value 0x06"]
impl crate::Resettable for CR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x06
    }
}