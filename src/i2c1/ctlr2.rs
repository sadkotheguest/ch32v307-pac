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
#[doc = "Field `LAST` reader - DMA last transfer"]
pub type LAST_R = crate::BitReader<bool>;
#[doc = "Field `LAST` writer - DMA last transfer"]
pub type LAST_W<'a> = crate::BitWriter<'a, u32, CTLR2_SPEC, bool, 12>;
#[doc = "Field `DMAEN` reader - DMA requests enable"]
pub type DMAEN_R = crate::BitReader<bool>;
#[doc = "Field `DMAEN` writer - DMA requests enable"]
pub type DMAEN_W<'a> = crate::BitWriter<'a, u32, CTLR2_SPEC, bool, 11>;
#[doc = "Field `ITBUFEN` reader - Buffer interrupt enable"]
pub type ITBUFEN_R = crate::BitReader<bool>;
#[doc = "Field `ITBUFEN` writer - Buffer interrupt enable"]
pub type ITBUFEN_W<'a> = crate::BitWriter<'a, u32, CTLR2_SPEC, bool, 10>;
#[doc = "Field `ITEVTEN` reader - Event interrupt enable"]
pub type ITEVTEN_R = crate::BitReader<bool>;
#[doc = "Field `ITEVTEN` writer - Event interrupt enable"]
pub type ITEVTEN_W<'a> = crate::BitWriter<'a, u32, CTLR2_SPEC, bool, 9>;
#[doc = "Field `ITERREN` reader - Error interrupt enable"]
pub type ITERREN_R = crate::BitReader<bool>;
#[doc = "Field `ITERREN` writer - Error interrupt enable"]
pub type ITERREN_W<'a> = crate::BitWriter<'a, u32, CTLR2_SPEC, bool, 8>;
#[doc = "Field `FREQ` reader - Peripheral clock frequency"]
pub type FREQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FREQ` writer - Peripheral clock frequency"]
pub type FREQ_W<'a> = crate::FieldWriter<'a, u32, CTLR2_SPEC, u8, u8, 6, 0>;
impl R {
    #[doc = "Bit 12 - DMA last transfer"]
    #[inline(always)]
    pub fn last(&self) -> LAST_R {
        LAST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - DMA requests enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Buffer interrupt enable"]
    #[inline(always)]
    pub fn itbufen(&self) -> ITBUFEN_R {
        ITBUFEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Event interrupt enable"]
    #[inline(always)]
    pub fn itevten(&self) -> ITEVTEN_R {
        ITEVTEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Error interrupt enable"]
    #[inline(always)]
    pub fn iterren(&self) -> ITERREN_R {
        ITERREN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 0:5 - Peripheral clock frequency"]
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 12 - DMA last transfer"]
    #[inline(always)]
    pub fn last(&mut self) -> LAST_W {
        LAST_W::new(self)
    }
    #[doc = "Bit 11 - DMA requests enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W::new(self)
    }
    #[doc = "Bit 10 - Buffer interrupt enable"]
    #[inline(always)]
    pub fn itbufen(&mut self) -> ITBUFEN_W {
        ITBUFEN_W::new(self)
    }
    #[doc = "Bit 9 - Event interrupt enable"]
    #[inline(always)]
    pub fn itevten(&mut self) -> ITEVTEN_W {
        ITEVTEN_W::new(self)
    }
    #[doc = "Bit 8 - Error interrupt enable"]
    #[inline(always)]
    pub fn iterren(&mut self) -> ITERREN_W {
        ITERREN_W::new(self)
    }
    #[doc = "Bits 0:5 - Peripheral clock frequency"]
    #[inline(always)]
    pub fn freq(&mut self) -> FREQ_W {
        FREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctlr2](index.html) module"]
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