#[doc = "Register `DMABMR` reader"]
pub struct R(crate::R<DMABMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMABMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMABMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMABMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMABMR` writer"]
pub struct W(crate::W<DMABMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMABMR_SPEC>;
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
impl From<crate::W<DMABMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMABMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SR` reader - Software reset"]
pub type SR_R = crate::BitReader<bool>;
#[doc = "Field `SR` writer - Software reset"]
pub type SR_W<'a> = crate::BitWriter<'a, u32, DMABMR_SPEC, bool, 0>;
#[doc = "Field `DA` reader - DMA Arbitration"]
pub type DA_R = crate::BitReader<bool>;
#[doc = "Field `DA` writer - DMA Arbitration"]
pub type DA_W<'a> = crate::BitWriter<'a, u32, DMABMR_SPEC, bool, 1>;
#[doc = "Field `DSL` reader - Descriptor skip length"]
pub type DSL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DSL` writer - Descriptor skip length"]
pub type DSL_W<'a> = crate::FieldWriter<'a, u32, DMABMR_SPEC, u8, u8, 5, 2>;
#[doc = "Field `PBL` reader - Programmable burst length"]
pub type PBL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PBL` writer - Programmable burst length"]
pub type PBL_W<'a> = crate::FieldWriter<'a, u32, DMABMR_SPEC, u8, u8, 6, 8>;
#[doc = "Field `RTPR` reader - Rx Tx priority ratio"]
pub type RTPR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTPR` writer - Rx Tx priority ratio"]
pub type RTPR_W<'a> = crate::FieldWriter<'a, u32, DMABMR_SPEC, u8, u8, 2, 14>;
#[doc = "Field `FB` reader - Fixed burst"]
pub type FB_R = crate::BitReader<bool>;
#[doc = "Field `FB` writer - Fixed burst"]
pub type FB_W<'a> = crate::BitWriter<'a, u32, DMABMR_SPEC, bool, 16>;
#[doc = "Field `RDP` reader - Rx DMA PBL"]
pub type RDP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDP` writer - Rx DMA PBL"]
pub type RDP_W<'a> = crate::FieldWriter<'a, u32, DMABMR_SPEC, u8, u8, 6, 17>;
#[doc = "Field `USP` reader - Use separate PBL"]
pub type USP_R = crate::BitReader<bool>;
#[doc = "Field `USP` writer - Use separate PBL"]
pub type USP_W<'a> = crate::BitWriter<'a, u32, DMABMR_SPEC, bool, 23>;
#[doc = "Field `FPM` reader - 4xPBL mode"]
pub type FPM_R = crate::BitReader<bool>;
#[doc = "Field `FPM` writer - 4xPBL mode"]
pub type FPM_W<'a> = crate::BitWriter<'a, u32, DMABMR_SPEC, bool, 24>;
#[doc = "Field `AAB` reader - Address-aligned beats"]
pub type AAB_R = crate::BitReader<bool>;
#[doc = "Field `AAB` writer - Address-aligned beats"]
pub type AAB_W<'a> = crate::BitWriter<'a, u32, DMABMR_SPEC, bool, 25>;
impl R {
    #[doc = "Bit 0 - Software reset"]
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Arbitration"]
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - Descriptor skip length"]
    #[inline(always)]
    pub fn dsl(&self) -> DSL_R {
        DSL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 8:13 - Programmable burst length"]
    #[inline(always)]
    pub fn pbl(&self) -> PBL_R {
        PBL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - Rx Tx priority ratio"]
    #[inline(always)]
    pub fn rtpr(&self) -> RTPR_R {
        RTPR_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Fixed burst"]
    #[inline(always)]
    pub fn fb(&self) -> FB_R {
        FB_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:22 - Rx DMA PBL"]
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 17) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - Use separate PBL"]
    #[inline(always)]
    pub fn usp(&self) -> USP_R {
        USP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 4xPBL mode"]
    #[inline(always)]
    pub fn fpm(&self) -> FPM_R {
        FPM_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Address-aligned beats"]
    #[inline(always)]
    pub fn aab(&self) -> AAB_R {
        AAB_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software reset"]
    #[inline(always)]
    pub fn sr(&mut self) -> SR_W {
        SR_W::new(self)
    }
    #[doc = "Bit 1 - DMA Arbitration"]
    #[inline(always)]
    pub fn da(&mut self) -> DA_W {
        DA_W::new(self)
    }
    #[doc = "Bits 2:6 - Descriptor skip length"]
    #[inline(always)]
    pub fn dsl(&mut self) -> DSL_W {
        DSL_W::new(self)
    }
    #[doc = "Bits 8:13 - Programmable burst length"]
    #[inline(always)]
    pub fn pbl(&mut self) -> PBL_W {
        PBL_W::new(self)
    }
    #[doc = "Bits 14:15 - Rx Tx priority ratio"]
    #[inline(always)]
    pub fn rtpr(&mut self) -> RTPR_W {
        RTPR_W::new(self)
    }
    #[doc = "Bit 16 - Fixed burst"]
    #[inline(always)]
    pub fn fb(&mut self) -> FB_W {
        FB_W::new(self)
    }
    #[doc = "Bits 17:22 - Rx DMA PBL"]
    #[inline(always)]
    pub fn rdp(&mut self) -> RDP_W {
        RDP_W::new(self)
    }
    #[doc = "Bit 23 - Use separate PBL"]
    #[inline(always)]
    pub fn usp(&mut self) -> USP_W {
        USP_W::new(self)
    }
    #[doc = "Bit 24 - 4xPBL mode"]
    #[inline(always)]
    pub fn fpm(&mut self) -> FPM_W {
        FPM_W::new(self)
    }
    #[doc = "Bit 25 - Address-aligned beats"]
    #[inline(always)]
    pub fn aab(&mut self) -> AAB_W {
        AAB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet DMA bus mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmabmr](index.html) module"]
pub struct DMABMR_SPEC;
impl crate::RegisterSpec for DMABMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmabmr::R](R) reader structure"]
impl crate::Readable for DMABMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmabmr::W](W) writer structure"]
impl crate::Writable for DMABMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMABMR to value 0x0002_0101"]
impl crate::Resettable for DMABMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0002_0101
    }
}