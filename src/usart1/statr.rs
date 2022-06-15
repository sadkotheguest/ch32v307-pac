#[doc = "Register `STATR` reader"]
pub struct R(crate::R<STATR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATR` writer"]
pub struct W(crate::W<STATR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATR_SPEC>;
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
impl From<crate::W<STATR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTS` reader - CTS flag"]
pub type CTS_R = crate::BitReader<bool>;
#[doc = "Field `CTS` writer - CTS flag"]
pub type CTS_W<'a> = crate::BitWriter<'a, u32, STATR_SPEC, bool, 9>;
#[doc = "Field `LBD` reader - LIN break detection flag"]
pub type LBD_R = crate::BitReader<bool>;
#[doc = "Field `LBD` writer - LIN break detection flag"]
pub type LBD_W<'a> = crate::BitWriter<'a, u32, STATR_SPEC, bool, 8>;
#[doc = "Field `TXE` reader - Transmit data register empty"]
pub type TXE_R = crate::BitReader<bool>;
#[doc = "Field `TC` reader - Transmission complete"]
pub type TC_R = crate::BitReader<bool>;
#[doc = "Field `TC` writer - Transmission complete"]
pub type TC_W<'a> = crate::BitWriter<'a, u32, STATR_SPEC, bool, 6>;
#[doc = "Field `RXNE` reader - Read data register not empty"]
pub type RXNE_R = crate::BitReader<bool>;
#[doc = "Field `RXNE` writer - Read data register not empty"]
pub type RXNE_W<'a> = crate::BitWriter<'a, u32, STATR_SPEC, bool, 5>;
#[doc = "Field `IDLE` reader - IDLE line detected"]
pub type IDLE_R = crate::BitReader<bool>;
#[doc = "Field `ORE` reader - Overrun error"]
pub type ORE_R = crate::BitReader<bool>;
#[doc = "Field `NE` reader - Noise error flag"]
pub type NE_R = crate::BitReader<bool>;
#[doc = "Field `FE` reader - Framing error"]
pub type FE_R = crate::BitReader<bool>;
#[doc = "Field `PE` reader - Parity error"]
pub type PE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 9 - CTS flag"]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - LIN break detection flag"]
    #[inline(always)]
    pub fn lbd(&self) -> LBD_R {
        LBD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit data register empty"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission complete"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Read data register not empty"]
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - IDLE line detected"]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun error"]
    #[inline(always)]
    pub fn ore(&self) -> ORE_R {
        ORE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Noise error flag"]
    #[inline(always)]
    pub fn ne(&self) -> NE_R {
        NE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Framing error"]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Parity error"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - CTS flag"]
    #[inline(always)]
    pub fn cts(&mut self) -> CTS_W {
        CTS_W::new(self)
    }
    #[doc = "Bit 8 - LIN break detection flag"]
    #[inline(always)]
    pub fn lbd(&mut self) -> LBD_W {
        LBD_W::new(self)
    }
    #[doc = "Bit 6 - Transmission complete"]
    #[inline(always)]
    pub fn tc(&mut self) -> TC_W {
        TC_W::new(self)
    }
    #[doc = "Bit 5 - Read data register not empty"]
    #[inline(always)]
    pub fn rxne(&mut self) -> RXNE_W {
        RXNE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statr](index.html) module"]
pub struct STATR_SPEC;
impl crate::RegisterSpec for STATR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [statr::R](R) reader structure"]
impl crate::Readable for STATR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [statr::W](W) writer structure"]
impl crate::Writable for STATR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATR to value 0xc0"]
impl crate::Resettable for STATR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc0
    }
}