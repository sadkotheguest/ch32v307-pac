#[doc = "Register `STK_CTLR` reader"]
pub struct R(crate::R<STK_CTLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STK_CTLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STK_CTLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STK_CTLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STK_CTLR` writer"]
pub struct W(crate::W<STK_CTLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STK_CTLR_SPEC>;
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
impl From<crate::W<STK_CTLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STK_CTLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STE` reader - System counter enable"]
pub type STE_R = crate::BitReader<bool>;
#[doc = "Field `STE` writer - System counter enable"]
pub type STE_W<'a> = crate::BitWriter<'a, u32, STK_CTLR_SPEC, bool, 0>;
#[doc = "Field `STIE` reader - System counter interrupt enable"]
pub type STIE_R = crate::BitReader<bool>;
#[doc = "Field `STIE` writer - System counter interrupt enable"]
pub type STIE_W<'a> = crate::BitWriter<'a, u32, STK_CTLR_SPEC, bool, 1>;
#[doc = "Field `STCLK` reader - System selects the clock source"]
pub type STCLK_R = crate::BitReader<bool>;
#[doc = "Field `STCLK` writer - System selects the clock source"]
pub type STCLK_W<'a> = crate::BitWriter<'a, u32, STK_CTLR_SPEC, bool, 2>;
#[doc = "Field `STRE` reader - System reload register"]
pub type STRE_R = crate::BitReader<bool>;
#[doc = "Field `STRE` writer - System reload register"]
pub type STRE_W<'a> = crate::BitWriter<'a, u32, STK_CTLR_SPEC, bool, 3>;
#[doc = "Field `MODE` reader - System Mode"]
pub type MODE_R = crate::BitReader<bool>;
#[doc = "Field `MODE` writer - System Mode"]
pub type MODE_W<'a> = crate::BitWriter<'a, u32, STK_CTLR_SPEC, bool, 4>;
#[doc = "Field `INIT` reader - System Initialization update"]
pub type INIT_R = crate::BitReader<bool>;
#[doc = "Field `INIT` writer - System Initialization update"]
pub type INIT_W<'a> = crate::BitWriter<'a, u32, STK_CTLR_SPEC, bool, 5>;
#[doc = "Field `SWIE` reader - System software triggered interrupts enable"]
pub type SWIE_R = crate::BitReader<bool>;
#[doc = "Field `SWIE` writer - System software triggered interrupts enable"]
pub type SWIE_W<'a> = crate::BitWriter<'a, u32, STK_CTLR_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - System counter enable"]
    #[inline(always)]
    pub fn ste(&self) -> STE_R {
        STE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - System counter interrupt enable"]
    #[inline(always)]
    pub fn stie(&self) -> STIE_R {
        STIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - System selects the clock source"]
    #[inline(always)]
    pub fn stclk(&self) -> STCLK_R {
        STCLK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - System reload register"]
    #[inline(always)]
    pub fn stre(&self) -> STRE_R {
        STRE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - System Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - System Initialization update"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 31 - System software triggered interrupts enable"]
    #[inline(always)]
    pub fn swie(&self) -> SWIE_R {
        SWIE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System counter enable"]
    #[inline(always)]
    pub fn ste(&mut self) -> STE_W {
        STE_W::new(self)
    }
    #[doc = "Bit 1 - System counter interrupt enable"]
    #[inline(always)]
    pub fn stie(&mut self) -> STIE_W {
        STIE_W::new(self)
    }
    #[doc = "Bit 2 - System selects the clock source"]
    #[inline(always)]
    pub fn stclk(&mut self) -> STCLK_W {
        STCLK_W::new(self)
    }
    #[doc = "Bit 3 - System reload register"]
    #[inline(always)]
    pub fn stre(&mut self) -> STRE_W {
        STRE_W::new(self)
    }
    #[doc = "Bit 4 - System Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W::new(self)
    }
    #[doc = "Bit 5 - System Initialization update"]
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W {
        INIT_W::new(self)
    }
    #[doc = "Bit 31 - System software triggered interrupts enable"]
    #[inline(always)]
    pub fn swie(&mut self) -> SWIE_W {
        SWIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System counter control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stk_ctlr](index.html) module"]
pub struct STK_CTLR_SPEC;
impl crate::RegisterSpec for STK_CTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stk_ctlr::R](R) reader structure"]
impl crate::Readable for STK_CTLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stk_ctlr::W](W) writer structure"]
impl crate::Writable for STK_CTLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STK_CTLR to value 0"]
impl crate::Resettable for STK_CTLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}