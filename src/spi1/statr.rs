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
#[doc = "Field `BSY` reader - Busy flag"]
pub type BSY_R = crate::BitReader<bool>;
#[doc = "Field `OVR` reader - Overrun flag"]
pub type OVR_R = crate::BitReader<bool>;
#[doc = "Field `MODF` reader - Mode fault"]
pub type MODF_R = crate::BitReader<bool>;
#[doc = "Field `CRCERR` reader - CRC error flag"]
pub type CRCERR_R = crate::BitReader<bool>;
#[doc = "Field `CRCERR` writer - CRC error flag"]
pub type CRCERR_W<'a> = crate::BitWriter<'a, u32, STATR_SPEC, bool, 4>;
#[doc = "Field `TXE` reader - Transmit buffer empty"]
pub type TXE_R = crate::BitReader<bool>;
#[doc = "Field `RXNE` reader - Receive buffer not empty"]
pub type RXNE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 7 - Busy flag"]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Overrun flag"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Mode fault"]
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - CRC error flag"]
    #[inline(always)]
    pub fn crcerr(&self) -> CRCERR_R {
        CRCERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit buffer empty"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Receive buffer not empty"]
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - CRC error flag"]
    #[inline(always)]
    pub fn crcerr(&mut self) -> CRCERR_W {
        CRCERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statr](index.html) module"]
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
#[doc = "`reset()` method sets STATR to value 0x02"]
impl crate::Resettable for STATR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}