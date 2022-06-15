#[doc = "Register `CTLR1` reader"]
pub struct R(crate::R<CTLR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTLR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTLR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTLR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTLR1` writer"]
pub struct W(crate::W<CTLR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTLR1_SPEC>;
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
impl From<crate::W<CTLR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTLR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UE` reader - USART enable"]
pub type UE_R = crate::BitReader<bool>;
#[doc = "Field `UE` writer - USART enable"]
pub type UE_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 13>;
#[doc = "Field `M` reader - Word length"]
pub type M_R = crate::BitReader<bool>;
#[doc = "Field `M` writer - Word length"]
pub type M_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 12>;
#[doc = "Field `WAKE` reader - Wakeup method"]
pub type WAKE_R = crate::BitReader<bool>;
#[doc = "Field `WAKE` writer - Wakeup method"]
pub type WAKE_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 11>;
#[doc = "Field `PCE` reader - Parity control enable"]
pub type PCE_R = crate::BitReader<bool>;
#[doc = "Field `PCE` writer - Parity control enable"]
pub type PCE_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 10>;
#[doc = "Field `PS` reader - Parity selection"]
pub type PS_R = crate::BitReader<bool>;
#[doc = "Field `PS` writer - Parity selection"]
pub type PS_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 9>;
#[doc = "Field `PEIE` reader - PE interrupt enable"]
pub type PEIE_R = crate::BitReader<bool>;
#[doc = "Field `PEIE` writer - PE interrupt enable"]
pub type PEIE_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 8>;
#[doc = "Field `TXEIE` reader - TXE interrupt enable"]
pub type TXEIE_R = crate::BitReader<bool>;
#[doc = "Field `TXEIE` writer - TXE interrupt enable"]
pub type TXEIE_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 7>;
#[doc = "Field `TCIE` reader - Transmission complete interrupt enable"]
pub type TCIE_R = crate::BitReader<bool>;
#[doc = "Field `TCIE` writer - Transmission complete interrupt enable"]
pub type TCIE_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 6>;
#[doc = "Field `RXNEIE` reader - RXNE interrupt enable"]
pub type RXNEIE_R = crate::BitReader<bool>;
#[doc = "Field `RXNEIE` writer - RXNE interrupt enable"]
pub type RXNEIE_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 5>;
#[doc = "Field `IDLEIE` reader - IDLE interrupt enable"]
pub type IDLEIE_R = crate::BitReader<bool>;
#[doc = "Field `IDLEIE` writer - IDLE interrupt enable"]
pub type IDLEIE_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 4>;
#[doc = "Field `TE` reader - Transmitter enable"]
pub type TE_R = crate::BitReader<bool>;
#[doc = "Field `TE` writer - Transmitter enable"]
pub type TE_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 3>;
#[doc = "Field `RE` reader - Receiver enable"]
pub type RE_R = crate::BitReader<bool>;
#[doc = "Field `RE` writer - Receiver enable"]
pub type RE_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 2>;
#[doc = "Field `RWU` reader - Receiver wakeup"]
pub type RWU_R = crate::BitReader<bool>;
#[doc = "Field `RWU` writer - Receiver wakeup"]
pub type RWU_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 1>;
#[doc = "Field `SBK` reader - Send break"]
pub type SBK_R = crate::BitReader<bool>;
#[doc = "Field `SBK` writer - Send break"]
pub type SBK_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 13 - USART enable"]
    #[inline(always)]
    pub fn ue(&self) -> UE_R {
        UE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Word length"]
    #[inline(always)]
    pub fn m(&self) -> M_R {
        M_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Wakeup method"]
    #[inline(always)]
    pub fn wake(&self) -> WAKE_R {
        WAKE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Parity control enable"]
    #[inline(always)]
    pub fn pce(&self) -> PCE_R {
        PCE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Parity selection"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - PE interrupt enable"]
    #[inline(always)]
    pub fn peie(&self) -> PEIE_R {
        PEIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - TXE interrupt enable"]
    #[inline(always)]
    pub fn txeie(&self) -> TXEIE_R {
        TXEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - RXNE interrupt enable"]
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - IDLE interrupt enable"]
    #[inline(always)]
    pub fn idleie(&self) -> IDLEIE_R {
        IDLEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Receiver wakeup"]
    #[inline(always)]
    pub fn rwu(&self) -> RWU_R {
        RWU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Send break"]
    #[inline(always)]
    pub fn sbk(&self) -> SBK_R {
        SBK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - USART enable"]
    #[inline(always)]
    pub fn ue(&mut self) -> UE_W {
        UE_W::new(self)
    }
    #[doc = "Bit 12 - Word length"]
    #[inline(always)]
    pub fn m(&mut self) -> M_W {
        M_W::new(self)
    }
    #[doc = "Bit 11 - Wakeup method"]
    #[inline(always)]
    pub fn wake(&mut self) -> WAKE_W {
        WAKE_W::new(self)
    }
    #[doc = "Bit 10 - Parity control enable"]
    #[inline(always)]
    pub fn pce(&mut self) -> PCE_W {
        PCE_W::new(self)
    }
    #[doc = "Bit 9 - Parity selection"]
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W {
        PS_W::new(self)
    }
    #[doc = "Bit 8 - PE interrupt enable"]
    #[inline(always)]
    pub fn peie(&mut self) -> PEIE_W {
        PEIE_W::new(self)
    }
    #[doc = "Bit 7 - TXE interrupt enable"]
    #[inline(always)]
    pub fn txeie(&mut self) -> TXEIE_W {
        TXEIE_W::new(self)
    }
    #[doc = "Bit 6 - Transmission complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W {
        TCIE_W::new(self)
    }
    #[doc = "Bit 5 - RXNE interrupt enable"]
    #[inline(always)]
    pub fn rxneie(&mut self) -> RXNEIE_W {
        RXNEIE_W::new(self)
    }
    #[doc = "Bit 4 - IDLE interrupt enable"]
    #[inline(always)]
    pub fn idleie(&mut self) -> IDLEIE_W {
        IDLEIE_W::new(self)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    pub fn te(&mut self) -> TE_W {
        TE_W::new(self)
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    pub fn re(&mut self) -> RE_W {
        RE_W::new(self)
    }
    #[doc = "Bit 1 - Receiver wakeup"]
    #[inline(always)]
    pub fn rwu(&mut self) -> RWU_W {
        RWU_W::new(self)
    }
    #[doc = "Bit 0 - Send break"]
    #[inline(always)]
    pub fn sbk(&mut self) -> SBK_W {
        SBK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctlr1](index.html) module"]
pub struct CTLR1_SPEC;
impl crate::RegisterSpec for CTLR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctlr1::R](R) reader structure"]
impl crate::Readable for CTLR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctlr1::W](W) writer structure"]
impl crate::Writable for CTLR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTLR1 to value 0"]
impl crate::Resettable for CTLR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}