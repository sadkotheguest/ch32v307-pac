#[doc = "Register `AHBPCENR` reader"]
pub struct R(crate::R<AHBPCENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBPCENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBPCENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBPCENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBPCENR` writer"]
pub struct W(crate::W<AHBPCENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBPCENR_SPEC>;
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
impl From<crate::W<AHBPCENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBPCENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA1EN` reader - DMA clock enable"]
pub type DMA1EN_R = crate::BitReader<bool>;
#[doc = "Field `DMA1EN` writer - DMA clock enable"]
pub type DMA1EN_W<'a> = crate::BitWriter<'a, u32, AHBPCENR_SPEC, bool, 0>;
#[doc = "Field `DMA2EN` reader - DMA2 clock enable"]
pub type DMA2EN_R = crate::BitReader<bool>;
#[doc = "Field `DMA2EN` writer - DMA2 clock enable"]
pub type DMA2EN_W<'a> = crate::BitWriter<'a, u32, AHBPCENR_SPEC, bool, 1>;
#[doc = "Field `SRAMEN` reader - SRAM interface clock enable"]
pub type SRAMEN_R = crate::BitReader<bool>;
#[doc = "Field `SRAMEN` writer - SRAM interface clock enable"]
pub type SRAMEN_W<'a> = crate::BitWriter<'a, u32, AHBPCENR_SPEC, bool, 2>;
#[doc = "Field `FLITFEN` reader - FLITF clock enable"]
pub type FLITFEN_R = crate::BitReader<bool>;
#[doc = "Field `FLITFEN` writer - FLITF clock enable"]
pub type FLITFEN_W<'a> = crate::BitWriter<'a, u32, AHBPCENR_SPEC, bool, 4>;
#[doc = "Field `CRCEN` reader - CRC clock enable"]
pub type CRCEN_R = crate::BitReader<bool>;
#[doc = "Field `CRCEN` writer - CRC clock enable"]
pub type CRCEN_W<'a> = crate::BitWriter<'a, u32, AHBPCENR_SPEC, bool, 6>;
#[doc = "Field `FSMCEN` reader - FSMC clock enable"]
pub type FSMCEN_R = crate::BitReader<bool>;
#[doc = "Field `FSMCEN` writer - FSMC clock enable"]
pub type FSMCEN_W<'a> = crate::BitWriter<'a, u32, AHBPCENR_SPEC, bool, 8>;
#[doc = "Field `TRNG_EN` reader - TRNG clock enable"]
pub type TRNG_EN_R = crate::BitReader<bool>;
#[doc = "Field `TRNG_EN` writer - TRNG clock enable"]
pub type TRNG_EN_W<'a> = crate::BitWriter<'a, u32, AHBPCENR_SPEC, bool, 9>;
#[doc = "Field `SDIOEN` reader - SDIO clock enable"]
pub type SDIOEN_R = crate::BitReader<bool>;
#[doc = "Field `SDIOEN` writer - SDIO clock enable"]
pub type SDIOEN_W<'a> = crate::BitWriter<'a, u32, AHBPCENR_SPEC, bool, 10>;
#[doc = "Field `USBHS_EN` reader - USBHS clock enable"]
pub type USBHS_EN_R = crate::BitReader<bool>;
#[doc = "Field `USBHS_EN` writer - USBHS clock enable"]
pub type USBHS_EN_W<'a> = crate::BitWriter<'a, u32, AHBPCENR_SPEC, bool, 11>;
#[doc = "Field `OTG_EN` reader - OTG clock enable"]
pub type OTG_EN_R = crate::BitReader<bool>;
#[doc = "Field `OTG_EN` writer - OTG clock enable"]
pub type OTG_EN_W<'a> = crate::BitWriter<'a, u32, AHBPCENR_SPEC, bool, 12>;
#[doc = "Field `DVP_EN` reader - DVP clock enable"]
pub type DVP_EN_R = crate::BitReader<bool>;
#[doc = "Field `DVP_EN` writer - DVP clock enable"]
pub type DVP_EN_W<'a> = crate::BitWriter<'a, u32, AHBPCENR_SPEC, bool, 13>;
#[doc = "Field `ETHMACEN` reader - Ethernet MAC clock enable"]
pub type ETHMACEN_R = crate::BitReader<bool>;
#[doc = "Field `ETHMACEN` writer - Ethernet MAC clock enable"]
pub type ETHMACEN_W<'a> = crate::BitWriter<'a, u32, AHBPCENR_SPEC, bool, 14>;
#[doc = "Field `ETHMACTXEN` reader - Ethernet MAC TX clock enable"]
pub type ETHMACTXEN_R = crate::BitReader<bool>;
#[doc = "Field `ETHMACTXEN` writer - Ethernet MAC TX clock enable"]
pub type ETHMACTXEN_W<'a> = crate::BitWriter<'a, u32, AHBPCENR_SPEC, bool, 15>;
#[doc = "Field `ETHMACRXEN` reader - Ethernet MAC RX clock enable"]
pub type ETHMACRXEN_R = crate::BitReader<bool>;
#[doc = "Field `ETHMACRXEN` writer - Ethernet MAC RX clock enable"]
pub type ETHMACRXEN_W<'a> = crate::BitWriter<'a, u32, AHBPCENR_SPEC, bool, 16>;
impl R {
    #[doc = "Bit 0 - DMA clock enable"]
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 clock enable"]
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SRAM interface clock enable"]
    #[inline(always)]
    pub fn sramen(&self) -> SRAMEN_R {
        SRAMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - FLITF clock enable"]
    #[inline(always)]
    pub fn flitfen(&self) -> FLITFEN_R {
        FLITFEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - FSMC clock enable"]
    #[inline(always)]
    pub fn fsmcen(&self) -> FSMCEN_R {
        FSMCEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TRNG clock enable"]
    #[inline(always)]
    pub fn trng_en(&self) -> TRNG_EN_R {
        TRNG_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SDIO clock enable"]
    #[inline(always)]
    pub fn sdioen(&self) -> SDIOEN_R {
        SDIOEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USBHS clock enable"]
    #[inline(always)]
    pub fn usbhs_en(&self) -> USBHS_EN_R {
        USBHS_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - OTG clock enable"]
    #[inline(always)]
    pub fn otg_en(&self) -> OTG_EN_R {
        OTG_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DVP clock enable"]
    #[inline(always)]
    pub fn dvp_en(&self) -> DVP_EN_R {
        DVP_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Ethernet MAC clock enable"]
    #[inline(always)]
    pub fn ethmacen(&self) -> ETHMACEN_R {
        ETHMACEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Ethernet MAC TX clock enable"]
    #[inline(always)]
    pub fn ethmactxen(&self) -> ETHMACTXEN_R {
        ETHMACTXEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Ethernet MAC RX clock enable"]
    #[inline(always)]
    pub fn ethmacrxen(&self) -> ETHMACRXEN_R {
        ETHMACRXEN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA clock enable"]
    #[inline(always)]
    pub fn dma1en(&mut self) -> DMA1EN_W {
        DMA1EN_W::new(self)
    }
    #[doc = "Bit 1 - DMA2 clock enable"]
    #[inline(always)]
    pub fn dma2en(&mut self) -> DMA2EN_W {
        DMA2EN_W::new(self)
    }
    #[doc = "Bit 2 - SRAM interface clock enable"]
    #[inline(always)]
    pub fn sramen(&mut self) -> SRAMEN_W {
        SRAMEN_W::new(self)
    }
    #[doc = "Bit 4 - FLITF clock enable"]
    #[inline(always)]
    pub fn flitfen(&mut self) -> FLITFEN_W {
        FLITFEN_W::new(self)
    }
    #[doc = "Bit 6 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W {
        CRCEN_W::new(self)
    }
    #[doc = "Bit 8 - FSMC clock enable"]
    #[inline(always)]
    pub fn fsmcen(&mut self) -> FSMCEN_W {
        FSMCEN_W::new(self)
    }
    #[doc = "Bit 9 - TRNG clock enable"]
    #[inline(always)]
    pub fn trng_en(&mut self) -> TRNG_EN_W {
        TRNG_EN_W::new(self)
    }
    #[doc = "Bit 10 - SDIO clock enable"]
    #[inline(always)]
    pub fn sdioen(&mut self) -> SDIOEN_W {
        SDIOEN_W::new(self)
    }
    #[doc = "Bit 11 - USBHS clock enable"]
    #[inline(always)]
    pub fn usbhs_en(&mut self) -> USBHS_EN_W {
        USBHS_EN_W::new(self)
    }
    #[doc = "Bit 12 - OTG clock enable"]
    #[inline(always)]
    pub fn otg_en(&mut self) -> OTG_EN_W {
        OTG_EN_W::new(self)
    }
    #[doc = "Bit 13 - DVP clock enable"]
    #[inline(always)]
    pub fn dvp_en(&mut self) -> DVP_EN_W {
        DVP_EN_W::new(self)
    }
    #[doc = "Bit 14 - Ethernet MAC clock enable"]
    #[inline(always)]
    pub fn ethmacen(&mut self) -> ETHMACEN_W {
        ETHMACEN_W::new(self)
    }
    #[doc = "Bit 15 - Ethernet MAC TX clock enable"]
    #[inline(always)]
    pub fn ethmactxen(&mut self) -> ETHMACTXEN_W {
        ETHMACTXEN_W::new(self)
    }
    #[doc = "Bit 16 - Ethernet MAC RX clock enable"]
    #[inline(always)]
    pub fn ethmacrxen(&mut self) -> ETHMACRXEN_W {
        ETHMACRXEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB Peripheral Clock enable register (RCC_AHBPCENR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbpcenr](index.html) module"]
pub struct AHBPCENR_SPEC;
impl crate::RegisterSpec for AHBPCENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahbpcenr::R](R) reader structure"]
impl crate::Readable for AHBPCENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahbpcenr::W](W) writer structure"]
impl crate::Writable for AHBPCENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHBPCENR to value 0x14"]
impl crate::Resettable for AHBPCENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x14
    }
}
