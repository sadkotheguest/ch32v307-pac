#[doc = "Register `CTLR2` reader"]
pub struct R(crate::R<CTLR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTLR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTLR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTLR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTLR2` writer"]
pub struct W(crate::W<CTLR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTLR2_SPEC>;
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
impl From<crate::W<CTLR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTLR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSVREFE` reader - Temperature sensor and VREFINT enable"]
pub type TSVREFE_R = crate::BitReader<bool>;
#[doc = "Field `TSVREFE` writer - Temperature sensor and VREFINT enable"]
pub type TSVREFE_W<'a> = crate::BitWriter<'a, u32, CTLR2_SPEC, bool, 23>;
#[doc = "Field `RSWSTART` reader - Start conversion of regular channels"]
pub type RSWSTART_R = crate::BitReader<bool>;
#[doc = "Field `RSWSTART` writer - Start conversion of regular channels"]
pub type RSWSTART_W<'a> = crate::BitWriter<'a, u32, CTLR2_SPEC, bool, 22>;
#[doc = "Field `ISWSTART` reader - Start conversion of injected channels"]
pub type ISWSTART_R = crate::BitReader<bool>;
#[doc = "Field `ISWSTART` writer - Start conversion of injected channels"]
pub type ISWSTART_W<'a> = crate::BitWriter<'a, u32, CTLR2_SPEC, bool, 21>;
#[doc = "Field `EXTTRIG` reader - External trigger conversion mode for regular channels"]
pub type EXTTRIG_R = crate::BitReader<bool>;
#[doc = "Field `EXTTRIG` writer - External trigger conversion mode for regular channels"]
pub type EXTTRIG_W<'a> = crate::BitWriter<'a, u32, CTLR2_SPEC, bool, 20>;
#[doc = "Field `EXTSEL` reader - External event select for regular group"]
pub type EXTSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTSEL` writer - External event select for regular group"]
pub type EXTSEL_W<'a> = crate::FieldWriter<'a, u32, CTLR2_SPEC, u8, u8, 3, 17>;
#[doc = "Field `IEXTTRIG` reader - External trigger conversion mode for injected channels"]
pub type IEXTTRIG_R = crate::BitReader<bool>;
#[doc = "Field `IEXTTRIG` writer - External trigger conversion mode for injected channels"]
pub type IEXTTRIG_W<'a> = crate::BitWriter<'a, u32, CTLR2_SPEC, bool, 15>;
#[doc = "Field `IEXTSEL` reader - External event select for injected group"]
pub type IEXTSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IEXTSEL` writer - External event select for injected group"]
pub type IEXTSEL_W<'a> = crate::FieldWriter<'a, u32, CTLR2_SPEC, u8, u8, 3, 12>;
#[doc = "Field `ALIGN` reader - Data alignment"]
pub type ALIGN_R = crate::BitReader<bool>;
#[doc = "Field `ALIGN` writer - Data alignment"]
pub type ALIGN_W<'a> = crate::BitWriter<'a, u32, CTLR2_SPEC, bool, 11>;
#[doc = "Field `DMA` reader - Direct memory access mode"]
pub type DMA_R = crate::BitReader<bool>;
#[doc = "Field `DMA` writer - Direct memory access mode"]
pub type DMA_W<'a> = crate::BitWriter<'a, u32, CTLR2_SPEC, bool, 8>;
#[doc = "Field `RSTCAL` reader - Reset calibration"]
pub type RSTCAL_R = crate::BitReader<bool>;
#[doc = "Field `RSTCAL` writer - Reset calibration"]
pub type RSTCAL_W<'a> = crate::BitWriter<'a, u32, CTLR2_SPEC, bool, 3>;
#[doc = "Field `CAL` reader - A/D calibration"]
pub type CAL_R = crate::BitReader<bool>;
#[doc = "Field `CAL` writer - A/D calibration"]
pub type CAL_W<'a> = crate::BitWriter<'a, u32, CTLR2_SPEC, bool, 2>;
#[doc = "Field `CONT` reader - Continuous conversion"]
pub type CONT_R = crate::BitReader<bool>;
#[doc = "Field `CONT` writer - Continuous conversion"]
pub type CONT_W<'a> = crate::BitWriter<'a, u32, CTLR2_SPEC, bool, 1>;
#[doc = "Field `ADON` reader - A/D converter ON / OFF"]
pub type ADON_R = crate::BitReader<bool>;
#[doc = "Field `ADON` writer - A/D converter ON / OFF"]
pub type ADON_W<'a> = crate::BitWriter<'a, u32, CTLR2_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 23 - Temperature sensor and VREFINT enable"]
    #[inline(always)]
    pub fn tsvrefe(&self) -> TSVREFE_R {
        TSVREFE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 22 - Start conversion of regular channels"]
    #[inline(always)]
    pub fn rswstart(&self) -> RSWSTART_R {
        RSWSTART_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 21 - Start conversion of injected channels"]
    #[inline(always)]
    pub fn iswstart(&self) -> ISWSTART_R {
        ISWSTART_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 20 - External trigger conversion mode for regular channels"]
    #[inline(always)]
    pub fn exttrig(&self) -> EXTTRIG_R {
        EXTTRIG_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 17:19 - External event select for regular group"]
    #[inline(always)]
    pub fn extsel(&self) -> EXTSEL_R {
        EXTSEL_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 15 - External trigger conversion mode for injected channels"]
    #[inline(always)]
    pub fn iexttrig(&self) -> IEXTTRIG_R {
        IEXTTRIG_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 12:14 - External event select for injected group"]
    #[inline(always)]
    pub fn iextsel(&self) -> IEXTSEL_R {
        IEXTSEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    pub fn align(&self) -> ALIGN_R {
        ALIGN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 8 - Direct memory access mode"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 3 - Reset calibration"]
    #[inline(always)]
    pub fn rstcal(&self) -> RSTCAL_R {
        RSTCAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - A/D calibration"]
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Continuous conversion"]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - A/D converter ON / OFF"]
    #[inline(always)]
    pub fn adon(&self) -> ADON_R {
        ADON_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - Temperature sensor and VREFINT enable"]
    #[inline(always)]
    pub fn tsvrefe(&mut self) -> TSVREFE_W {
        TSVREFE_W::new(self)
    }
    #[doc = "Bit 22 - Start conversion of regular channels"]
    #[inline(always)]
    pub fn rswstart(&mut self) -> RSWSTART_W {
        RSWSTART_W::new(self)
    }
    #[doc = "Bit 21 - Start conversion of injected channels"]
    #[inline(always)]
    pub fn iswstart(&mut self) -> ISWSTART_W {
        ISWSTART_W::new(self)
    }
    #[doc = "Bit 20 - External trigger conversion mode for regular channels"]
    #[inline(always)]
    pub fn exttrig(&mut self) -> EXTTRIG_W {
        EXTTRIG_W::new(self)
    }
    #[doc = "Bits 17:19 - External event select for regular group"]
    #[inline(always)]
    pub fn extsel(&mut self) -> EXTSEL_W {
        EXTSEL_W::new(self)
    }
    #[doc = "Bit 15 - External trigger conversion mode for injected channels"]
    #[inline(always)]
    pub fn iexttrig(&mut self) -> IEXTTRIG_W {
        IEXTTRIG_W::new(self)
    }
    #[doc = "Bits 12:14 - External event select for injected group"]
    #[inline(always)]
    pub fn iextsel(&mut self) -> IEXTSEL_W {
        IEXTSEL_W::new(self)
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    pub fn align(&mut self) -> ALIGN_W {
        ALIGN_W::new(self)
    }
    #[doc = "Bit 8 - Direct memory access mode"]
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W {
        DMA_W::new(self)
    }
    #[doc = "Bit 3 - Reset calibration"]
    #[inline(always)]
    pub fn rstcal(&mut self) -> RSTCAL_W {
        RSTCAL_W::new(self)
    }
    #[doc = "Bit 2 - A/D calibration"]
    #[inline(always)]
    pub fn cal(&mut self) -> CAL_W {
        CAL_W::new(self)
    }
    #[doc = "Bit 1 - Continuous conversion"]
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W {
        CONT_W::new(self)
    }
    #[doc = "Bit 0 - A/D converter ON / OFF"]
    #[inline(always)]
    pub fn adon(&mut self) -> ADON_W {
        ADON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctlr2](index.html) module"]
pub struct CTLR2_SPEC;
impl crate::RegisterSpec for CTLR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctlr2::R](R) reader structure"]
impl crate::Readable for CTLR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctlr2::W](W) writer structure"]
impl crate::Writable for CTLR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTLR2 to value 0"]
impl crate::Resettable for CTLR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}