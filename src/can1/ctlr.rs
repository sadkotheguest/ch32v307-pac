#[doc = "Register `CTLR` reader"]
pub struct R(crate::R<CTLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTLR` writer"]
pub struct W(crate::W<CTLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTLR_SPEC>;
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
impl From<crate::W<CTLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBF` reader - DBF"]
pub type DBF_R = crate::BitReader<bool>;
#[doc = "Field `DBF` writer - DBF"]
pub type DBF_W<'a> = crate::BitWriter<'a, u32, CTLR_SPEC, bool, 16>;
#[doc = "Field `RESET` reader - RESET"]
pub type RESET_R = crate::BitReader<bool>;
#[doc = "Field `RESET` writer - RESET"]
pub type RESET_W<'a> = crate::BitWriter<'a, u32, CTLR_SPEC, bool, 15>;
#[doc = "Field `TTCM` reader - TTCM"]
pub type TTCM_R = crate::BitReader<bool>;
#[doc = "Field `TTCM` writer - TTCM"]
pub type TTCM_W<'a> = crate::BitWriter<'a, u32, CTLR_SPEC, bool, 7>;
#[doc = "Field `ABOM` reader - ABOM"]
pub type ABOM_R = crate::BitReader<bool>;
#[doc = "Field `ABOM` writer - ABOM"]
pub type ABOM_W<'a> = crate::BitWriter<'a, u32, CTLR_SPEC, bool, 6>;
#[doc = "Field `AWUM` reader - AWUM"]
pub type AWUM_R = crate::BitReader<bool>;
#[doc = "Field `AWUM` writer - AWUM"]
pub type AWUM_W<'a> = crate::BitWriter<'a, u32, CTLR_SPEC, bool, 5>;
#[doc = "Field `NART` reader - NART"]
pub type NART_R = crate::BitReader<bool>;
#[doc = "Field `NART` writer - NART"]
pub type NART_W<'a> = crate::BitWriter<'a, u32, CTLR_SPEC, bool, 4>;
#[doc = "Field `RFLM` reader - RFLM"]
pub type RFLM_R = crate::BitReader<bool>;
#[doc = "Field `RFLM` writer - RFLM"]
pub type RFLM_W<'a> = crate::BitWriter<'a, u32, CTLR_SPEC, bool, 3>;
#[doc = "Field `TXFP` reader - TXFP"]
pub type TXFP_R = crate::BitReader<bool>;
#[doc = "Field `TXFP` writer - TXFP"]
pub type TXFP_W<'a> = crate::BitWriter<'a, u32, CTLR_SPEC, bool, 2>;
#[doc = "Field `SLEEP` reader - SLEEP"]
pub type SLEEP_R = crate::BitReader<bool>;
#[doc = "Field `SLEEP` writer - SLEEP"]
pub type SLEEP_W<'a> = crate::BitWriter<'a, u32, CTLR_SPEC, bool, 1>;
#[doc = "Field `INRQ` reader - INRQ"]
pub type INRQ_R = crate::BitReader<bool>;
#[doc = "Field `INRQ` writer - INRQ"]
pub type INRQ_W<'a> = crate::BitWriter<'a, u32, CTLR_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 16 - DBF"]
    #[inline(always)]
    pub fn dbf(&self) -> DBF_R {
        DBF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 15 - RESET"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 7 - TTCM"]
    #[inline(always)]
    pub fn ttcm(&self) -> TTCM_R {
        TTCM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - ABOM"]
    #[inline(always)]
    pub fn abom(&self) -> ABOM_R {
        ABOM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - AWUM"]
    #[inline(always)]
    pub fn awum(&self) -> AWUM_R {
        AWUM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - NART"]
    #[inline(always)]
    pub fn nart(&self) -> NART_R {
        NART_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - RFLM"]
    #[inline(always)]
    pub fn rflm(&self) -> RFLM_R {
        RFLM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - TXFP"]
    #[inline(always)]
    pub fn txfp(&self) -> TXFP_R {
        TXFP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - SLEEP"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - INRQ"]
    #[inline(always)]
    pub fn inrq(&self) -> INRQ_R {
        INRQ_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - DBF"]
    #[inline(always)]
    pub fn dbf(&mut self) -> DBF_W {
        DBF_W::new(self)
    }
    #[doc = "Bit 15 - RESET"]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W::new(self)
    }
    #[doc = "Bit 7 - TTCM"]
    #[inline(always)]
    pub fn ttcm(&mut self) -> TTCM_W {
        TTCM_W::new(self)
    }
    #[doc = "Bit 6 - ABOM"]
    #[inline(always)]
    pub fn abom(&mut self) -> ABOM_W {
        ABOM_W::new(self)
    }
    #[doc = "Bit 5 - AWUM"]
    #[inline(always)]
    pub fn awum(&mut self) -> AWUM_W {
        AWUM_W::new(self)
    }
    #[doc = "Bit 4 - NART"]
    #[inline(always)]
    pub fn nart(&mut self) -> NART_W {
        NART_W::new(self)
    }
    #[doc = "Bit 3 - RFLM"]
    #[inline(always)]
    pub fn rflm(&mut self) -> RFLM_W {
        RFLM_W::new(self)
    }
    #[doc = "Bit 2 - TXFP"]
    #[inline(always)]
    pub fn txfp(&mut self) -> TXFP_W {
        TXFP_W::new(self)
    }
    #[doc = "Bit 1 - SLEEP"]
    #[inline(always)]
    pub fn sleep(&mut self) -> SLEEP_W {
        SLEEP_W::new(self)
    }
    #[doc = "Bit 0 - INRQ"]
    #[inline(always)]
    pub fn inrq(&mut self) -> INRQ_W {
        INRQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctlr](index.html) module"]
pub struct CTLR_SPEC;
impl crate::RegisterSpec for CTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctlr::R](R) reader structure"]
impl crate::Readable for CTLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctlr::W](W) writer structure"]
impl crate::Writable for CTLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTLR to value 0"]
impl crate::Resettable for CTLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}