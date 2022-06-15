#[doc = "Register `CTLR3` reader"]
pub struct R(crate::R<CTLR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTLR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTLR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTLR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTLR3` writer"]
pub struct W(crate::W<CTLR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTLR3_SPEC>;
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
impl From<crate::W<CTLR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTLR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTSIE` reader - CTS interrupt enable"]
pub type CTSIE_R = crate::BitReader<bool>;
#[doc = "Field `CTSIE` writer - CTS interrupt enable"]
pub type CTSIE_W<'a> = crate::BitWriter<'a, u32, CTLR3_SPEC, bool, 10>;
#[doc = "Field `CTSE` reader - CTS enable"]
pub type CTSE_R = crate::BitReader<bool>;
#[doc = "Field `CTSE` writer - CTS enable"]
pub type CTSE_W<'a> = crate::BitWriter<'a, u32, CTLR3_SPEC, bool, 9>;
#[doc = "Field `RTSE` reader - RTS enable"]
pub type RTSE_R = crate::BitReader<bool>;
#[doc = "Field `RTSE` writer - RTS enable"]
pub type RTSE_W<'a> = crate::BitWriter<'a, u32, CTLR3_SPEC, bool, 8>;
#[doc = "Field `DMAT` reader - DMA enable transmitter"]
pub type DMAT_R = crate::BitReader<bool>;
#[doc = "Field `DMAT` writer - DMA enable transmitter"]
pub type DMAT_W<'a> = crate::BitWriter<'a, u32, CTLR3_SPEC, bool, 7>;
#[doc = "Field `DMAR` reader - DMA enable receiver"]
pub type DMAR_R = crate::BitReader<bool>;
#[doc = "Field `DMAR` writer - DMA enable receiver"]
pub type DMAR_W<'a> = crate::BitWriter<'a, u32, CTLR3_SPEC, bool, 6>;
#[doc = "Field `SCEN` reader - Smartcard mode enable"]
pub type SCEN_R = crate::BitReader<bool>;
#[doc = "Field `SCEN` writer - Smartcard mode enable"]
pub type SCEN_W<'a> = crate::BitWriter<'a, u32, CTLR3_SPEC, bool, 5>;
#[doc = "Field `NACK` reader - Smartcard NACK enable"]
pub type NACK_R = crate::BitReader<bool>;
#[doc = "Field `NACK` writer - Smartcard NACK enable"]
pub type NACK_W<'a> = crate::BitWriter<'a, u32, CTLR3_SPEC, bool, 4>;
#[doc = "Field `HDSEL` reader - Half-duplex selection"]
pub type HDSEL_R = crate::BitReader<bool>;
#[doc = "Field `HDSEL` writer - Half-duplex selection"]
pub type HDSEL_W<'a> = crate::BitWriter<'a, u32, CTLR3_SPEC, bool, 3>;
#[doc = "Field `IRLP` reader - IrDA low-power"]
pub type IRLP_R = crate::BitReader<bool>;
#[doc = "Field `IRLP` writer - IrDA low-power"]
pub type IRLP_W<'a> = crate::BitWriter<'a, u32, CTLR3_SPEC, bool, 2>;
#[doc = "Field `IREN` reader - IrDA mode enable"]
pub type IREN_R = crate::BitReader<bool>;
#[doc = "Field `IREN` writer - IrDA mode enable"]
pub type IREN_W<'a> = crate::BitWriter<'a, u32, CTLR3_SPEC, bool, 1>;
#[doc = "Field `EIE` reader - Error interrupt enable"]
pub type EIE_R = crate::BitReader<bool>;
#[doc = "Field `EIE` writer - Error interrupt enable"]
pub type EIE_W<'a> = crate::BitWriter<'a, u32, CTLR3_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 10 - CTS interrupt enable"]
    #[inline(always)]
    pub fn ctsie(&self) -> CTSIE_R {
        CTSIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - CTS enable"]
    #[inline(always)]
    pub fn ctse(&self) -> CTSE_R {
        CTSE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - RTS enable"]
    #[inline(always)]
    pub fn rtse(&self) -> RTSE_R {
        RTSE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA enable transmitter"]
    #[inline(always)]
    pub fn dmat(&self) -> DMAT_R {
        DMAT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA enable receiver"]
    #[inline(always)]
    pub fn dmar(&self) -> DMAR_R {
        DMAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Smartcard mode enable"]
    #[inline(always)]
    pub fn scen(&self) -> SCEN_R {
        SCEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Smartcard NACK enable"]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Half-duplex selection"]
    #[inline(always)]
    pub fn hdsel(&self) -> HDSEL_R {
        HDSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - IrDA low-power"]
    #[inline(always)]
    pub fn irlp(&self) -> IRLP_R {
        IRLP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - IrDA mode enable"]
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - CTS interrupt enable"]
    #[inline(always)]
    pub fn ctsie(&mut self) -> CTSIE_W {
        CTSIE_W::new(self)
    }
    #[doc = "Bit 9 - CTS enable"]
    #[inline(always)]
    pub fn ctse(&mut self) -> CTSE_W {
        CTSE_W::new(self)
    }
    #[doc = "Bit 8 - RTS enable"]
    #[inline(always)]
    pub fn rtse(&mut self) -> RTSE_W {
        RTSE_W::new(self)
    }
    #[doc = "Bit 7 - DMA enable transmitter"]
    #[inline(always)]
    pub fn dmat(&mut self) -> DMAT_W {
        DMAT_W::new(self)
    }
    #[doc = "Bit 6 - DMA enable receiver"]
    #[inline(always)]
    pub fn dmar(&mut self) -> DMAR_W {
        DMAR_W::new(self)
    }
    #[doc = "Bit 5 - Smartcard mode enable"]
    #[inline(always)]
    pub fn scen(&mut self) -> SCEN_W {
        SCEN_W::new(self)
    }
    #[doc = "Bit 4 - Smartcard NACK enable"]
    #[inline(always)]
    pub fn nack(&mut self) -> NACK_W {
        NACK_W::new(self)
    }
    #[doc = "Bit 3 - Half-duplex selection"]
    #[inline(always)]
    pub fn hdsel(&mut self) -> HDSEL_W {
        HDSEL_W::new(self)
    }
    #[doc = "Bit 2 - IrDA low-power"]
    #[inline(always)]
    pub fn irlp(&mut self) -> IRLP_W {
        IRLP_W::new(self)
    }
    #[doc = "Bit 1 - IrDA mode enable"]
    #[inline(always)]
    pub fn iren(&mut self) -> IREN_W {
        IREN_W::new(self)
    }
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline(always)]
    pub fn eie(&mut self) -> EIE_W {
        EIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctlr3](index.html) module"]
pub struct CTLR3_SPEC;
impl crate::RegisterSpec for CTLR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctlr3::R](R) reader structure"]
impl crate::Readable for CTLR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctlr3::W](W) writer structure"]
impl crate::Writable for CTLR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTLR3 to value 0"]
impl crate::Resettable for CTLR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}