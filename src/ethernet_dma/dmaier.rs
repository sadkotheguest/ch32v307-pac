#[doc = "Register `DMAIER` reader"]
pub struct R(crate::R<DMAIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAIER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAIER` writer"]
pub struct W(crate::W<DMAIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAIER_SPEC>;
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
impl From<crate::W<DMAIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIE` reader - Transmit interrupt enable"]
pub type TIE_R = crate::BitReader<bool>;
#[doc = "Field `TIE` writer - Transmit interrupt enable"]
pub type TIE_W<'a> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, 0>;
#[doc = "Field `TPSIE` reader - Transmit process stopped interrupt enable"]
pub type TPSIE_R = crate::BitReader<bool>;
#[doc = "Field `TPSIE` writer - Transmit process stopped interrupt enable"]
pub type TPSIE_W<'a> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, 1>;
#[doc = "Field `TBUIE` reader - Transmit buffer unavailable interrupt enable"]
pub type TBUIE_R = crate::BitReader<bool>;
#[doc = "Field `TBUIE` writer - Transmit buffer unavailable interrupt enable"]
pub type TBUIE_W<'a> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, 2>;
#[doc = "Field `TJTIE` reader - Transmit jabber timeout interrupt enable"]
pub type TJTIE_R = crate::BitReader<bool>;
#[doc = "Field `TJTIE` writer - Transmit jabber timeout interrupt enable"]
pub type TJTIE_W<'a> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, 3>;
#[doc = "Field `ROIE` reader - Overflow interrupt enable"]
pub type ROIE_R = crate::BitReader<bool>;
#[doc = "Field `ROIE` writer - Overflow interrupt enable"]
pub type ROIE_W<'a> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, 4>;
#[doc = "Field `TUIE` reader - Underflow interrupt enable"]
pub type TUIE_R = crate::BitReader<bool>;
#[doc = "Field `TUIE` writer - Underflow interrupt enable"]
pub type TUIE_W<'a> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, 5>;
#[doc = "Field `RIE` reader - Receive interrupt enable"]
pub type RIE_R = crate::BitReader<bool>;
#[doc = "Field `RIE` writer - Receive interrupt enable"]
pub type RIE_W<'a> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, 6>;
#[doc = "Field `RBUIE` reader - Receive buffer unavailable interrupt enable"]
pub type RBUIE_R = crate::BitReader<bool>;
#[doc = "Field `RBUIE` writer - Receive buffer unavailable interrupt enable"]
pub type RBUIE_W<'a> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, 7>;
#[doc = "Field `RPSIE` reader - Receive process stopped interrupt enable"]
pub type RPSIE_R = crate::BitReader<bool>;
#[doc = "Field `RPSIE` writer - Receive process stopped interrupt enable"]
pub type RPSIE_W<'a> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, 8>;
#[doc = "Field `RWTIE` reader - receive watchdog timeout interrupt enable"]
pub type RWTIE_R = crate::BitReader<bool>;
#[doc = "Field `RWTIE` writer - receive watchdog timeout interrupt enable"]
pub type RWTIE_W<'a> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, 9>;
#[doc = "Field `ETIE` reader - Early transmit interrupt enable"]
pub type ETIE_R = crate::BitReader<bool>;
#[doc = "Field `ETIE` writer - Early transmit interrupt enable"]
pub type ETIE_W<'a> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, 10>;
#[doc = "Field `FBEIE` reader - Fatal bus error interrupt enable"]
pub type FBEIE_R = crate::BitReader<bool>;
#[doc = "Field `FBEIE` writer - Fatal bus error interrupt enable"]
pub type FBEIE_W<'a> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, 13>;
#[doc = "Field `ERIE` reader - Early receive interrupt enable"]
pub type ERIE_R = crate::BitReader<bool>;
#[doc = "Field `ERIE` writer - Early receive interrupt enable"]
pub type ERIE_W<'a> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, 14>;
#[doc = "Field `AISE` reader - Abnormal interrupt summary enable"]
pub type AISE_R = crate::BitReader<bool>;
#[doc = "Field `AISE` writer - Abnormal interrupt summary enable"]
pub type AISE_W<'a> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, 15>;
#[doc = "Field `NISE` reader - Normal interrupt summary enable"]
pub type NISE_R = crate::BitReader<bool>;
#[doc = "Field `NISE` writer - Normal interrupt summary enable"]
pub type NISE_W<'a> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, 16>;
#[doc = "Field `IPLE` reader - 10M Physical layer connection"]
pub type IPLE_R = crate::BitReader<bool>;
#[doc = "Field `IPLE` writer - 10M Physical layer connection"]
pub type IPLE_W<'a> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - Transmit interrupt enable"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit process stopped interrupt enable"]
    #[inline(always)]
    pub fn tpsie(&self) -> TPSIE_R {
        TPSIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit buffer unavailable interrupt enable"]
    #[inline(always)]
    pub fn tbuie(&self) -> TBUIE_R {
        TBUIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit jabber timeout interrupt enable"]
    #[inline(always)]
    pub fn tjtie(&self) -> TJTIE_R {
        TJTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Overflow interrupt enable"]
    #[inline(always)]
    pub fn roie(&self) -> ROIE_R {
        ROIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Underflow interrupt enable"]
    #[inline(always)]
    pub fn tuie(&self) -> TUIE_R {
        TUIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive interrupt enable"]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive buffer unavailable interrupt enable"]
    #[inline(always)]
    pub fn rbuie(&self) -> RBUIE_R {
        RBUIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive process stopped interrupt enable"]
    #[inline(always)]
    pub fn rpsie(&self) -> RPSIE_R {
        RPSIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - receive watchdog timeout interrupt enable"]
    #[inline(always)]
    pub fn rwtie(&self) -> RWTIE_R {
        RWTIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Early transmit interrupt enable"]
    #[inline(always)]
    pub fn etie(&self) -> ETIE_R {
        ETIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Fatal bus error interrupt enable"]
    #[inline(always)]
    pub fn fbeie(&self) -> FBEIE_R {
        FBEIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Early receive interrupt enable"]
    #[inline(always)]
    pub fn erie(&self) -> ERIE_R {
        ERIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Abnormal interrupt summary enable"]
    #[inline(always)]
    pub fn aise(&self) -> AISE_R {
        AISE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Normal interrupt summary enable"]
    #[inline(always)]
    pub fn nise(&self) -> NISE_R {
        NISE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31 - 10M Physical layer connection"]
    #[inline(always)]
    pub fn iple(&self) -> IPLE_R {
        IPLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit interrupt enable"]
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W {
        TIE_W::new(self)
    }
    #[doc = "Bit 1 - Transmit process stopped interrupt enable"]
    #[inline(always)]
    pub fn tpsie(&mut self) -> TPSIE_W {
        TPSIE_W::new(self)
    }
    #[doc = "Bit 2 - Transmit buffer unavailable interrupt enable"]
    #[inline(always)]
    pub fn tbuie(&mut self) -> TBUIE_W {
        TBUIE_W::new(self)
    }
    #[doc = "Bit 3 - Transmit jabber timeout interrupt enable"]
    #[inline(always)]
    pub fn tjtie(&mut self) -> TJTIE_W {
        TJTIE_W::new(self)
    }
    #[doc = "Bit 4 - Overflow interrupt enable"]
    #[inline(always)]
    pub fn roie(&mut self) -> ROIE_W {
        ROIE_W::new(self)
    }
    #[doc = "Bit 5 - Underflow interrupt enable"]
    #[inline(always)]
    pub fn tuie(&mut self) -> TUIE_W {
        TUIE_W::new(self)
    }
    #[doc = "Bit 6 - Receive interrupt enable"]
    #[inline(always)]
    pub fn rie(&mut self) -> RIE_W {
        RIE_W::new(self)
    }
    #[doc = "Bit 7 - Receive buffer unavailable interrupt enable"]
    #[inline(always)]
    pub fn rbuie(&mut self) -> RBUIE_W {
        RBUIE_W::new(self)
    }
    #[doc = "Bit 8 - Receive process stopped interrupt enable"]
    #[inline(always)]
    pub fn rpsie(&mut self) -> RPSIE_W {
        RPSIE_W::new(self)
    }
    #[doc = "Bit 9 - receive watchdog timeout interrupt enable"]
    #[inline(always)]
    pub fn rwtie(&mut self) -> RWTIE_W {
        RWTIE_W::new(self)
    }
    #[doc = "Bit 10 - Early transmit interrupt enable"]
    #[inline(always)]
    pub fn etie(&mut self) -> ETIE_W {
        ETIE_W::new(self)
    }
    #[doc = "Bit 13 - Fatal bus error interrupt enable"]
    #[inline(always)]
    pub fn fbeie(&mut self) -> FBEIE_W {
        FBEIE_W::new(self)
    }
    #[doc = "Bit 14 - Early receive interrupt enable"]
    #[inline(always)]
    pub fn erie(&mut self) -> ERIE_W {
        ERIE_W::new(self)
    }
    #[doc = "Bit 15 - Abnormal interrupt summary enable"]
    #[inline(always)]
    pub fn aise(&mut self) -> AISE_W {
        AISE_W::new(self)
    }
    #[doc = "Bit 16 - Normal interrupt summary enable"]
    #[inline(always)]
    pub fn nise(&mut self) -> NISE_W {
        NISE_W::new(self)
    }
    #[doc = "Bit 31 - 10M Physical layer connection"]
    #[inline(always)]
    pub fn iple(&mut self) -> IPLE_W {
        IPLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet DMA interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaier](index.html) module"]
pub struct DMAIER_SPEC;
impl crate::RegisterSpec for DMAIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmaier::R](R) reader structure"]
impl crate::Readable for DMAIER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmaier::W](W) writer structure"]
impl crate::Writable for DMAIER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAIER to value 0"]
impl crate::Resettable for DMAIER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}