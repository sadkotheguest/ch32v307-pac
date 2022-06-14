#[doc = "Register `INTFCR` writer"]
pub struct W(crate::W<INTFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTFCR_SPEC>;
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
impl From<crate::W<INTFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CGIF1` writer - Channel 1 Global interrupt clear"]
pub type CGIF1_W<'a> = crate::BitWriter<'a, u32, INTFCR_SPEC, bool, 0>;
#[doc = "Field `CGIF2` writer - Channel 2 Global interrupt clear"]
pub type CGIF2_W<'a> = crate::BitWriter<'a, u32, INTFCR_SPEC, bool, 4>;
#[doc = "Field `CGIF3` writer - Channel 3 Global interrupt clear"]
pub type CGIF3_W<'a> = crate::BitWriter<'a, u32, INTFCR_SPEC, bool, 8>;
#[doc = "Field `CGIF4` writer - Channel 4 Global interrupt clear"]
pub type CGIF4_W<'a> = crate::BitWriter<'a, u32, INTFCR_SPEC, bool, 12>;
#[doc = "Field `CGIF5` writer - Channel 5 Global interrupt clear"]
pub type CGIF5_W<'a> = crate::BitWriter<'a, u32, INTFCR_SPEC, bool, 16>;
#[doc = "Field `CGIF6` writer - Channel 6 Global interrupt clear"]
pub type CGIF6_W<'a> = crate::BitWriter<'a, u32, INTFCR_SPEC, bool, 20>;
#[doc = "Field `CGIF7` writer - Channel 7 Global interrupt clear"]
pub type CGIF7_W<'a> = crate::BitWriter<'a, u32, INTFCR_SPEC, bool, 24>;
#[doc = "Field `CTCIF1` writer - Channel 1 Transfer Complete clear"]
pub type CTCIF1_W<'a> = crate::BitWriter<'a, u32, INTFCR_SPEC, bool, 1>;
#[doc = "Field `CTCIF2` writer - Channel 2 Transfer Complete clear"]
pub type CTCIF2_W<'a> = crate::BitWriter<'a, u32, INTFCR_SPEC, bool, 5>;
#[doc = "Field `CTCIF3` writer - Channel 3 Transfer Complete clear"]
pub type CTCIF3_W<'a> = crate::BitWriter<'a, u32, INTFCR_SPEC, bool, 9>;
#[doc = "Field `CTCIF4` writer - Channel 4 Transfer Complete clear"]
pub type CTCIF4_W<'a> = crate::BitWriter<'a, u32, INTFCR_SPEC, bool, 13>;
#[doc = "Field `CTCIF5` writer - Channel 5 Transfer Complete clear"]
pub type CTCIF5_W<'a> = crate::BitWriter<'a, u32, INTFCR_SPEC, bool, 17>;
#[doc = "Field `CTCIF6` writer - Channel 6 Transfer Complete clear"]
pub type CTCIF6_W<'a> = crate::BitWriter<'a, u32, INTFCR_SPEC, bool, 21>;
#[doc = "Field `CTCIF7` writer - Channel 7 Transfer Complete clear"]
pub type CTCIF7_W<'a> = crate::BitWriter<'a, u32, INTFCR_SPEC, bool, 25>;
#[doc = "Field `CHTIF1` writer - Channel 1 Half Transfer clear"]
pub type CHTIF1_W<'a> = crate::BitWriter<'a, u32, INTFCR_SPEC, bool, 2>;
#[doc = "Field `CHTIF2` writer - Channel 2 Half Transfer clear"]
pub type CHTIF2_W<'a> = crate::BitWriter<'a, u32, INTFCR_SPEC, bool, 6>;
#[doc = "Field `CHTIF3` writer - Channel 3 Half Transfer clear"]
pub type CHTIF3_W<'a> = crate::BitWriter<'a, u32, INTFCR_SPEC, bool, 10>;
#[doc = "Field `CHTIF4` writer - Channel 4 Half Transfer clear"]
pub type CHTIF4_W<'a> = crate::BitWriter<'a, u32, INTFCR_SPEC, bool, 14>;
#[doc = "Field `CHTIF5` writer - Channel 5 Half Transfer clear"]
pub type CHTIF5_W<'a> = crate::BitWriter<'a, u32, INTFCR_SPEC, bool, 18>;
#[doc = "Field `CHTIF6` writer - Channel 6 Half Transfer clear"]
pub type CHTIF6_W<'a> = crate::BitWriter<'a, u32, INTFCR_SPEC, bool, 22>;
#[doc = "Field `CHTIF7` writer - Channel 7 Half Transfer clear"]
pub type CHTIF7_W<'a> = crate::BitWriter<'a, u32, INTFCR_SPEC, bool, 26>;
#[doc = "Field `CTEIF1` writer - Channel 1 Transfer Error clear"]
pub type CTEIF1_W<'a> = crate::BitWriter<'a, u32, INTFCR_SPEC, bool, 3>;
#[doc = "Field `CTEIF2` writer - Channel 2 Transfer Error clear"]
pub type CTEIF2_W<'a> = crate::BitWriter<'a, u32, INTFCR_SPEC, bool, 7>;
#[doc = "Field `CTEIF3` writer - Channel 3 Transfer Error clear"]
pub type CTEIF3_W<'a> = crate::BitWriter<'a, u32, INTFCR_SPEC, bool, 11>;
#[doc = "Field `CTEIF4` writer - Channel 4 Transfer Error clear"]
pub type CTEIF4_W<'a> = crate::BitWriter<'a, u32, INTFCR_SPEC, bool, 15>;
#[doc = "Field `CTEIF5` writer - Channel 5 Transfer Error clear"]
pub type CTEIF5_W<'a> = crate::BitWriter<'a, u32, INTFCR_SPEC, bool, 19>;
#[doc = "Field `CTEIF6` writer - Channel 6 Transfer Error clear"]
pub type CTEIF6_W<'a> = crate::BitWriter<'a, u32, INTFCR_SPEC, bool, 23>;
#[doc = "Field `CTEIF7` writer - Channel 7 Transfer Error clear"]
pub type CTEIF7_W<'a> = crate::BitWriter<'a, u32, INTFCR_SPEC, bool, 27>;
impl W {
    #[doc = "Bit 0 - Channel 1 Global interrupt clear"]
    #[inline(always)]
    pub fn cgif1(&mut self) -> CGIF1_W {
        CGIF1_W::new(self)
    }
    #[doc = "Bit 4 - Channel 2 Global interrupt clear"]
    #[inline(always)]
    pub fn cgif2(&mut self) -> CGIF2_W {
        CGIF2_W::new(self)
    }
    #[doc = "Bit 8 - Channel 3 Global interrupt clear"]
    #[inline(always)]
    pub fn cgif3(&mut self) -> CGIF3_W {
        CGIF3_W::new(self)
    }
    #[doc = "Bit 12 - Channel 4 Global interrupt clear"]
    #[inline(always)]
    pub fn cgif4(&mut self) -> CGIF4_W {
        CGIF4_W::new(self)
    }
    #[doc = "Bit 16 - Channel 5 Global interrupt clear"]
    #[inline(always)]
    pub fn cgif5(&mut self) -> CGIF5_W {
        CGIF5_W::new(self)
    }
    #[doc = "Bit 20 - Channel 6 Global interrupt clear"]
    #[inline(always)]
    pub fn cgif6(&mut self) -> CGIF6_W {
        CGIF6_W::new(self)
    }
    #[doc = "Bit 24 - Channel 7 Global interrupt clear"]
    #[inline(always)]
    pub fn cgif7(&mut self) -> CGIF7_W {
        CGIF7_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Transfer Complete clear"]
    #[inline(always)]
    pub fn ctcif1(&mut self) -> CTCIF1_W {
        CTCIF1_W::new(self)
    }
    #[doc = "Bit 5 - Channel 2 Transfer Complete clear"]
    #[inline(always)]
    pub fn ctcif2(&mut self) -> CTCIF2_W {
        CTCIF2_W::new(self)
    }
    #[doc = "Bit 9 - Channel 3 Transfer Complete clear"]
    #[inline(always)]
    pub fn ctcif3(&mut self) -> CTCIF3_W {
        CTCIF3_W::new(self)
    }
    #[doc = "Bit 13 - Channel 4 Transfer Complete clear"]
    #[inline(always)]
    pub fn ctcif4(&mut self) -> CTCIF4_W {
        CTCIF4_W::new(self)
    }
    #[doc = "Bit 17 - Channel 5 Transfer Complete clear"]
    #[inline(always)]
    pub fn ctcif5(&mut self) -> CTCIF5_W {
        CTCIF5_W::new(self)
    }
    #[doc = "Bit 21 - Channel 6 Transfer Complete clear"]
    #[inline(always)]
    pub fn ctcif6(&mut self) -> CTCIF6_W {
        CTCIF6_W::new(self)
    }
    #[doc = "Bit 25 - Channel 7 Transfer Complete clear"]
    #[inline(always)]
    pub fn ctcif7(&mut self) -> CTCIF7_W {
        CTCIF7_W::new(self)
    }
    #[doc = "Bit 2 - Channel 1 Half Transfer clear"]
    #[inline(always)]
    pub fn chtif1(&mut self) -> CHTIF1_W {
        CHTIF1_W::new(self)
    }
    #[doc = "Bit 6 - Channel 2 Half Transfer clear"]
    #[inline(always)]
    pub fn chtif2(&mut self) -> CHTIF2_W {
        CHTIF2_W::new(self)
    }
    #[doc = "Bit 10 - Channel 3 Half Transfer clear"]
    #[inline(always)]
    pub fn chtif3(&mut self) -> CHTIF3_W {
        CHTIF3_W::new(self)
    }
    #[doc = "Bit 14 - Channel 4 Half Transfer clear"]
    #[inline(always)]
    pub fn chtif4(&mut self) -> CHTIF4_W {
        CHTIF4_W::new(self)
    }
    #[doc = "Bit 18 - Channel 5 Half Transfer clear"]
    #[inline(always)]
    pub fn chtif5(&mut self) -> CHTIF5_W {
        CHTIF5_W::new(self)
    }
    #[doc = "Bit 22 - Channel 6 Half Transfer clear"]
    #[inline(always)]
    pub fn chtif6(&mut self) -> CHTIF6_W {
        CHTIF6_W::new(self)
    }
    #[doc = "Bit 26 - Channel 7 Half Transfer clear"]
    #[inline(always)]
    pub fn chtif7(&mut self) -> CHTIF7_W {
        CHTIF7_W::new(self)
    }
    #[doc = "Bit 3 - Channel 1 Transfer Error clear"]
    #[inline(always)]
    pub fn cteif1(&mut self) -> CTEIF1_W {
        CTEIF1_W::new(self)
    }
    #[doc = "Bit 7 - Channel 2 Transfer Error clear"]
    #[inline(always)]
    pub fn cteif2(&mut self) -> CTEIF2_W {
        CTEIF2_W::new(self)
    }
    #[doc = "Bit 11 - Channel 3 Transfer Error clear"]
    #[inline(always)]
    pub fn cteif3(&mut self) -> CTEIF3_W {
        CTEIF3_W::new(self)
    }
    #[doc = "Bit 15 - Channel 4 Transfer Error clear"]
    #[inline(always)]
    pub fn cteif4(&mut self) -> CTEIF4_W {
        CTEIF4_W::new(self)
    }
    #[doc = "Bit 19 - Channel 5 Transfer Error clear"]
    #[inline(always)]
    pub fn cteif5(&mut self) -> CTEIF5_W {
        CTEIF5_W::new(self)
    }
    #[doc = "Bit 23 - Channel 6 Transfer Error clear"]
    #[inline(always)]
    pub fn cteif6(&mut self) -> CTEIF6_W {
        CTEIF6_W::new(self)
    }
    #[doc = "Bit 27 - Channel 7 Transfer Error clear"]
    #[inline(always)]
    pub fn cteif7(&mut self) -> CTEIF7_W {
        CTEIF7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA interrupt flag clear register (DMA_INTFCR)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intfcr](index.html) module"]
pub struct INTFCR_SPEC;
impl crate::RegisterSpec for INTFCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [intfcr::W](W) writer structure"]
impl crate::Writable for INTFCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTFCR to value 0"]
impl crate::Resettable for INTFCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}