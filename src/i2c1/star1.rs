#[doc = "Register `STAR1` reader"]
pub struct R(crate::R<STAR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STAR1` writer"]
pub struct W(crate::W<STAR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAR1_SPEC>;
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
impl From<crate::W<STAR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMBALERT` reader - SMBus alert"]
pub type SMBALERT_R = crate::BitReader<bool>;
#[doc = "Field `SMBALERT` writer - SMBus alert"]
pub type SMBALERT_W<'a> = crate::BitWriter<'a, u32, STAR1_SPEC, bool, 15>;
#[doc = "Field `TIMEOUT` reader - Timeout or Tlow error"]
pub type TIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `TIMEOUT` writer - Timeout or Tlow error"]
pub type TIMEOUT_W<'a> = crate::BitWriter<'a, u32, STAR1_SPEC, bool, 14>;
#[doc = "Field `PECERR` reader - PEC Error in reception"]
pub type PECERR_R = crate::BitReader<bool>;
#[doc = "Field `PECERR` writer - PEC Error in reception"]
pub type PECERR_W<'a> = crate::BitWriter<'a, u32, STAR1_SPEC, bool, 12>;
#[doc = "Field `OVR` reader - Overrun/Underrun"]
pub type OVR_R = crate::BitReader<bool>;
#[doc = "Field `OVR` writer - Overrun/Underrun"]
pub type OVR_W<'a> = crate::BitWriter<'a, u32, STAR1_SPEC, bool, 11>;
#[doc = "Field `AF` reader - Acknowledge failure"]
pub type AF_R = crate::BitReader<bool>;
#[doc = "Field `AF` writer - Acknowledge failure"]
pub type AF_W<'a> = crate::BitWriter<'a, u32, STAR1_SPEC, bool, 10>;
#[doc = "Field `ARLO` reader - Arbitration lost (master mode)"]
pub type ARLO_R = crate::BitReader<bool>;
#[doc = "Field `ARLO` writer - Arbitration lost (master mode)"]
pub type ARLO_W<'a> = crate::BitWriter<'a, u32, STAR1_SPEC, bool, 9>;
#[doc = "Field `BERR` reader - Bus error"]
pub type BERR_R = crate::BitReader<bool>;
#[doc = "Field `BERR` writer - Bus error"]
pub type BERR_W<'a> = crate::BitWriter<'a, u32, STAR1_SPEC, bool, 8>;
#[doc = "Field `TxE` reader - Data register empty (transmitters)"]
pub type TXE_R = crate::BitReader<bool>;
#[doc = "Field `RxNE` reader - Data register not empty (receivers)"]
pub type RXNE_R = crate::BitReader<bool>;
#[doc = "Field `STOPF` reader - Stop detection (slave mode)"]
pub type STOPF_R = crate::BitReader<bool>;
#[doc = "Field `ADD10` reader - 10-bit header sent (Master mode)"]
pub type ADD10_R = crate::BitReader<bool>;
#[doc = "Field `BTF` reader - Byte transfer finished"]
pub type BTF_R = crate::BitReader<bool>;
#[doc = "Field `ADDR` reader - Address sent (master mode)/matched (slave mode)"]
pub type ADDR_R = crate::BitReader<bool>;
#[doc = "Field `SB` reader - Start bit (Master mode)"]
pub type SB_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 15 - SMBus alert"]
    #[inline(always)]
    pub fn smbalert(&self) -> SMBALERT_R {
        SMBALERT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Timeout or Tlow error"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 12 - PEC Error in reception"]
    #[inline(always)]
    pub fn pecerr(&self) -> PECERR_R {
        PECERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Overrun/Underrun"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Acknowledge failure"]
    #[inline(always)]
    pub fn af(&self) -> AF_R {
        AF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Arbitration lost (master mode)"]
    #[inline(always)]
    pub fn arlo(&self) -> ARLO_R {
        ARLO_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Bus error"]
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Data register empty (transmitters)"]
    #[inline(always)]
    pub fn tx_e(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Data register not empty (receivers)"]
    #[inline(always)]
    pub fn rx_ne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 4 - Stop detection (slave mode)"]
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - 10-bit header sent (Master mode)"]
    #[inline(always)]
    pub fn add10(&self) -> ADD10_R {
        ADD10_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Byte transfer finished"]
    #[inline(always)]
    pub fn btf(&self) -> BTF_R {
        BTF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Address sent (master mode)/matched (slave mode)"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Start bit (Master mode)"]
    #[inline(always)]
    pub fn sb(&self) -> SB_R {
        SB_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - SMBus alert"]
    #[inline(always)]
    pub fn smbalert(&mut self) -> SMBALERT_W {
        SMBALERT_W::new(self)
    }
    #[doc = "Bit 14 - Timeout or Tlow error"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W {
        TIMEOUT_W::new(self)
    }
    #[doc = "Bit 12 - PEC Error in reception"]
    #[inline(always)]
    pub fn pecerr(&mut self) -> PECERR_W {
        PECERR_W::new(self)
    }
    #[doc = "Bit 11 - Overrun/Underrun"]
    #[inline(always)]
    pub fn ovr(&mut self) -> OVR_W {
        OVR_W::new(self)
    }
    #[doc = "Bit 10 - Acknowledge failure"]
    #[inline(always)]
    pub fn af(&mut self) -> AF_W {
        AF_W::new(self)
    }
    #[doc = "Bit 9 - Arbitration lost (master mode)"]
    #[inline(always)]
    pub fn arlo(&mut self) -> ARLO_W {
        ARLO_W::new(self)
    }
    #[doc = "Bit 8 - Bus error"]
    #[inline(always)]
    pub fn berr(&mut self) -> BERR_W {
        BERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [star1](index.html) module"]
pub struct STAR1_SPEC;
impl crate::RegisterSpec for STAR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [star1::R](R) reader structure"]
impl crate::Readable for STAR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [star1::W](W) writer structure"]
impl crate::Writable for STAR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STAR1 to value 0"]
impl crate::Resettable for STAR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}