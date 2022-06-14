#[doc = "Register `CFGR9` reader"]
pub struct R(crate::R<CFGR9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR9` writer"]
pub struct W(crate::W<CFGR9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR9_SPEC>;
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
impl From<crate::W<CFGR9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Channel enable"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Channel enable"]
pub type EN_W<'a> = crate::BitWriter<'a, u32, CFGR9_SPEC, bool, 0>;
#[doc = "Field `TCIE` reader - Transfer complete interrupt enable"]
pub type TCIE_R = crate::BitReader<bool>;
#[doc = "Field `TCIE` writer - Transfer complete interrupt enable"]
pub type TCIE_W<'a> = crate::BitWriter<'a, u32, CFGR9_SPEC, bool, 1>;
#[doc = "Field `HTIE` reader - Half Transfer interrupt enable"]
pub type HTIE_R = crate::BitReader<bool>;
#[doc = "Field `HTIE` writer - Half Transfer interrupt enable"]
pub type HTIE_W<'a> = crate::BitWriter<'a, u32, CFGR9_SPEC, bool, 2>;
#[doc = "Field `TEIE` reader - Transfer error interrupt enable"]
pub type TEIE_R = crate::BitReader<bool>;
#[doc = "Field `TEIE` writer - Transfer error interrupt enable"]
pub type TEIE_W<'a> = crate::BitWriter<'a, u32, CFGR9_SPEC, bool, 3>;
#[doc = "Field `DIR` reader - Data transfer direction"]
pub type DIR_R = crate::BitReader<bool>;
#[doc = "Field `DIR` writer - Data transfer direction"]
pub type DIR_W<'a> = crate::BitWriter<'a, u32, CFGR9_SPEC, bool, 4>;
#[doc = "Field `CIRC` reader - Circular mode"]
pub type CIRC_R = crate::BitReader<bool>;
#[doc = "Field `CIRC` writer - Circular mode"]
pub type CIRC_W<'a> = crate::BitWriter<'a, u32, CFGR9_SPEC, bool, 5>;
#[doc = "Field `PINC` reader - Peripheral increment mode"]
pub type PINC_R = crate::BitReader<bool>;
#[doc = "Field `PINC` writer - Peripheral increment mode"]
pub type PINC_W<'a> = crate::BitWriter<'a, u32, CFGR9_SPEC, bool, 6>;
#[doc = "Field `MINC` reader - Memory increment mode"]
pub type MINC_R = crate::BitReader<bool>;
#[doc = "Field `MINC` writer - Memory increment mode"]
pub type MINC_W<'a> = crate::BitWriter<'a, u32, CFGR9_SPEC, bool, 7>;
#[doc = "Field `PSIZE` reader - Peripheral size"]
pub type PSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PSIZE` writer - Peripheral size"]
pub type PSIZE_W<'a> = crate::FieldWriter<'a, u32, CFGR9_SPEC, u8, u8, 2, 8>;
#[doc = "Field `MSIZE` reader - Memory size"]
pub type MSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MSIZE` writer - Memory size"]
pub type MSIZE_W<'a> = crate::FieldWriter<'a, u32, CFGR9_SPEC, u8, u8, 2, 10>;
#[doc = "Field `PL` reader - Channel Priority level"]
pub type PL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PL` writer - Channel Priority level"]
pub type PL_W<'a> = crate::FieldWriter<'a, u32, CFGR9_SPEC, u8, u8, 2, 12>;
#[doc = "Field `MEM2MEM` reader - Memory to memory mode"]
pub type MEM2MEM_R = crate::BitReader<bool>;
#[doc = "Field `MEM2MEM` writer - Memory to memory mode"]
pub type MEM2MEM_W<'a> = crate::BitWriter<'a, u32, CFGR9_SPEC, bool, 14>;
impl R {
    #[doc = "Bit 0 - Channel enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Half Transfer interrupt enable"]
    #[inline(always)]
    pub fn htie(&self) -> HTIE_R {
        HTIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transfer error interrupt enable"]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data transfer direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Circular mode"]
    #[inline(always)]
    pub fn circ(&self) -> CIRC_R {
        CIRC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Peripheral increment mode"]
    #[inline(always)]
    pub fn pinc(&self) -> PINC_R {
        PINC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Memory increment mode"]
    #[inline(always)]
    pub fn minc(&self) -> MINC_R {
        MINC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Peripheral size"]
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Memory size"]
    #[inline(always)]
    pub fn msize(&self) -> MSIZE_R {
        MSIZE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Channel Priority level"]
    #[inline(always)]
    pub fn pl(&self) -> PL_R {
        PL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Memory to memory mode"]
    #[inline(always)]
    pub fn mem2mem(&self) -> MEM2MEM_R {
        MEM2MEM_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Transfer complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W {
        TCIE_W::new(self)
    }
    #[doc = "Bit 2 - Half Transfer interrupt enable"]
    #[inline(always)]
    pub fn htie(&mut self) -> HTIE_W {
        HTIE_W::new(self)
    }
    #[doc = "Bit 3 - Transfer error interrupt enable"]
    #[inline(always)]
    pub fn teie(&mut self) -> TEIE_W {
        TEIE_W::new(self)
    }
    #[doc = "Bit 4 - Data transfer direction"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W::new(self)
    }
    #[doc = "Bit 5 - Circular mode"]
    #[inline(always)]
    pub fn circ(&mut self) -> CIRC_W {
        CIRC_W::new(self)
    }
    #[doc = "Bit 6 - Peripheral increment mode"]
    #[inline(always)]
    pub fn pinc(&mut self) -> PINC_W {
        PINC_W::new(self)
    }
    #[doc = "Bit 7 - Memory increment mode"]
    #[inline(always)]
    pub fn minc(&mut self) -> MINC_W {
        MINC_W::new(self)
    }
    #[doc = "Bits 8:9 - Peripheral size"]
    #[inline(always)]
    pub fn psize(&mut self) -> PSIZE_W {
        PSIZE_W::new(self)
    }
    #[doc = "Bits 10:11 - Memory size"]
    #[inline(always)]
    pub fn msize(&mut self) -> MSIZE_W {
        MSIZE_W::new(self)
    }
    #[doc = "Bits 12:13 - Channel Priority level"]
    #[inline(always)]
    pub fn pl(&mut self) -> PL_W {
        PL_W::new(self)
    }
    #[doc = "Bit 14 - Memory to memory mode"]
    #[inline(always)]
    pub fn mem2mem(&mut self) -> MEM2MEM_W {
        MEM2MEM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA channel configuration register (DMA_CFGR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr9](index.html) module"]
pub struct CFGR9_SPEC;
impl crate::RegisterSpec for CFGR9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr9::R](R) reader structure"]
impl crate::Readable for CFGR9_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr9::W](W) writer structure"]
impl crate::Writable for CFGR9_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGR9 to value 0"]
impl crate::Resettable for CFGR9_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}