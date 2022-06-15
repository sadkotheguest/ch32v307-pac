#[doc = "Register `DMASR` reader"]
pub struct R(crate::R<DMASR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMASR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMASR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMASR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMASR` writer"]
pub struct W(crate::W<DMASR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMASR_SPEC>;
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
impl From<crate::W<DMASR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMASR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TS` reader - Transmit status"]
pub type TS_R = crate::BitReader<bool>;
#[doc = "Field `TS` writer - Transmit status"]
pub type TS_W<'a> = crate::BitWriter<'a, u32, DMASR_SPEC, bool, 0>;
#[doc = "Field `TPSS` reader - Transmit process stopped status"]
pub type TPSS_R = crate::BitReader<bool>;
#[doc = "Field `TPSS` writer - Transmit process stopped status"]
pub type TPSS_W<'a> = crate::BitWriter<'a, u32, DMASR_SPEC, bool, 1>;
#[doc = "Field `TBUS` reader - Transmit buffer unavailable status"]
pub type TBUS_R = crate::BitReader<bool>;
#[doc = "Field `TBUS` writer - Transmit buffer unavailable status"]
pub type TBUS_W<'a> = crate::BitWriter<'a, u32, DMASR_SPEC, bool, 2>;
#[doc = "Field `TJTS` reader - Transmit jabber timeout status"]
pub type TJTS_R = crate::BitReader<bool>;
#[doc = "Field `TJTS` writer - Transmit jabber timeout status"]
pub type TJTS_W<'a> = crate::BitWriter<'a, u32, DMASR_SPEC, bool, 3>;
#[doc = "Field `ROS` reader - Receive overflow status"]
pub type ROS_R = crate::BitReader<bool>;
#[doc = "Field `ROS` writer - Receive overflow status"]
pub type ROS_W<'a> = crate::BitWriter<'a, u32, DMASR_SPEC, bool, 4>;
#[doc = "Field `TUS` reader - Transmit underflow status"]
pub type TUS_R = crate::BitReader<bool>;
#[doc = "Field `TUS` writer - Transmit underflow status"]
pub type TUS_W<'a> = crate::BitWriter<'a, u32, DMASR_SPEC, bool, 5>;
#[doc = "Field `RS` reader - Receive status"]
pub type RS_R = crate::BitReader<bool>;
#[doc = "Field `RS` writer - Receive status"]
pub type RS_W<'a> = crate::BitWriter<'a, u32, DMASR_SPEC, bool, 6>;
#[doc = "Field `RBUS` reader - Receive buffer unavailable status"]
pub type RBUS_R = crate::BitReader<bool>;
#[doc = "Field `RBUS` writer - Receive buffer unavailable status"]
pub type RBUS_W<'a> = crate::BitWriter<'a, u32, DMASR_SPEC, bool, 7>;
#[doc = "Field `RPSS` reader - Receive process stopped status"]
pub type RPSS_R = crate::BitReader<bool>;
#[doc = "Field `RPSS` writer - Receive process stopped status"]
pub type RPSS_W<'a> = crate::BitWriter<'a, u32, DMASR_SPEC, bool, 8>;
#[doc = "Field `PWTS` reader - Receive watchdog timeout status"]
pub type PWTS_R = crate::BitReader<bool>;
#[doc = "Field `PWTS` writer - Receive watchdog timeout status"]
pub type PWTS_W<'a> = crate::BitWriter<'a, u32, DMASR_SPEC, bool, 9>;
#[doc = "Field `ETS` reader - Early transmit status"]
pub type ETS_R = crate::BitReader<bool>;
#[doc = "Field `ETS` writer - Early transmit status"]
pub type ETS_W<'a> = crate::BitWriter<'a, u32, DMASR_SPEC, bool, 10>;
#[doc = "Field `FBES` reader - Fatal bus error status"]
pub type FBES_R = crate::BitReader<bool>;
#[doc = "Field `FBES` writer - Fatal bus error status"]
pub type FBES_W<'a> = crate::BitWriter<'a, u32, DMASR_SPEC, bool, 13>;
#[doc = "Field `ERS` reader - Early receive status"]
pub type ERS_R = crate::BitReader<bool>;
#[doc = "Field `ERS` writer - Early receive status"]
pub type ERS_W<'a> = crate::BitWriter<'a, u32, DMASR_SPEC, bool, 14>;
#[doc = "Field `AIS` reader - Abnormal interrupt summary"]
pub type AIS_R = crate::BitReader<bool>;
#[doc = "Field `AIS` writer - Abnormal interrupt summary"]
pub type AIS_W<'a> = crate::BitWriter<'a, u32, DMASR_SPEC, bool, 15>;
#[doc = "Field `NIS` reader - Normal interrupt summary"]
pub type NIS_R = crate::BitReader<bool>;
#[doc = "Field `NIS` writer - Normal interrupt summary"]
pub type NIS_W<'a> = crate::BitWriter<'a, u32, DMASR_SPEC, bool, 16>;
#[doc = "Field `RPS` reader - Receive process state"]
pub type RPS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TPS` reader - Transmit process state"]
pub type TPS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EBS` reader - Error bits status"]
pub type EBS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MMCS` reader - MMC status"]
pub type MMCS_R = crate::BitReader<bool>;
#[doc = "Field `PMTS` reader - PMT status"]
pub type PMTS_R = crate::BitReader<bool>;
#[doc = "Field `TSTS` reader - Time stamp trigger status"]
pub type TSTS_R = crate::BitReader<bool>;
#[doc = "Field `IPLS` reader - 10MPHY Physical layer variation"]
pub type IPLS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Transmit status"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit process stopped status"]
    #[inline(always)]
    pub fn tpss(&self) -> TPSS_R {
        TPSS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit buffer unavailable status"]
    #[inline(always)]
    pub fn tbus(&self) -> TBUS_R {
        TBUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit jabber timeout status"]
    #[inline(always)]
    pub fn tjts(&self) -> TJTS_R {
        TJTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive overflow status"]
    #[inline(always)]
    pub fn ros(&self) -> ROS_R {
        ROS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit underflow status"]
    #[inline(always)]
    pub fn tus(&self) -> TUS_R {
        TUS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive status"]
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive buffer unavailable status"]
    #[inline(always)]
    pub fn rbus(&self) -> RBUS_R {
        RBUS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive process stopped status"]
    #[inline(always)]
    pub fn rpss(&self) -> RPSS_R {
        RPSS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive watchdog timeout status"]
    #[inline(always)]
    pub fn pwts(&self) -> PWTS_R {
        PWTS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Early transmit status"]
    #[inline(always)]
    pub fn ets(&self) -> ETS_R {
        ETS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Fatal bus error status"]
    #[inline(always)]
    pub fn fbes(&self) -> FBES_R {
        FBES_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Early receive status"]
    #[inline(always)]
    pub fn ers(&self) -> ERS_R {
        ERS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Abnormal interrupt summary"]
    #[inline(always)]
    pub fn ais(&self) -> AIS_R {
        AIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Normal interrupt summary"]
    #[inline(always)]
    pub fn nis(&self) -> NIS_R {
        NIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Receive process state"]
    #[inline(always)]
    pub fn rps(&self) -> RPS_R {
        RPS_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Transmit process state"]
    #[inline(always)]
    pub fn tps(&self) -> TPS_R {
        TPS_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:25 - Error bits status"]
    #[inline(always)]
    pub fn ebs(&self) -> EBS_R {
        EBS_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bit 27 - MMC status"]
    #[inline(always)]
    pub fn mmcs(&self) -> MMCS_R {
        MMCS_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PMT status"]
    #[inline(always)]
    pub fn pmts(&self) -> PMTS_R {
        PMTS_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Time stamp trigger status"]
    #[inline(always)]
    pub fn tsts(&self) -> TSTS_R {
        TSTS_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - 10MPHY Physical layer variation"]
    #[inline(always)]
    pub fn ipls(&self) -> IPLS_R {
        IPLS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit status"]
    #[inline(always)]
    pub fn ts(&mut self) -> TS_W {
        TS_W::new(self)
    }
    #[doc = "Bit 1 - Transmit process stopped status"]
    #[inline(always)]
    pub fn tpss(&mut self) -> TPSS_W {
        TPSS_W::new(self)
    }
    #[doc = "Bit 2 - Transmit buffer unavailable status"]
    #[inline(always)]
    pub fn tbus(&mut self) -> TBUS_W {
        TBUS_W::new(self)
    }
    #[doc = "Bit 3 - Transmit jabber timeout status"]
    #[inline(always)]
    pub fn tjts(&mut self) -> TJTS_W {
        TJTS_W::new(self)
    }
    #[doc = "Bit 4 - Receive overflow status"]
    #[inline(always)]
    pub fn ros(&mut self) -> ROS_W {
        ROS_W::new(self)
    }
    #[doc = "Bit 5 - Transmit underflow status"]
    #[inline(always)]
    pub fn tus(&mut self) -> TUS_W {
        TUS_W::new(self)
    }
    #[doc = "Bit 6 - Receive status"]
    #[inline(always)]
    pub fn rs(&mut self) -> RS_W {
        RS_W::new(self)
    }
    #[doc = "Bit 7 - Receive buffer unavailable status"]
    #[inline(always)]
    pub fn rbus(&mut self) -> RBUS_W {
        RBUS_W::new(self)
    }
    #[doc = "Bit 8 - Receive process stopped status"]
    #[inline(always)]
    pub fn rpss(&mut self) -> RPSS_W {
        RPSS_W::new(self)
    }
    #[doc = "Bit 9 - Receive watchdog timeout status"]
    #[inline(always)]
    pub fn pwts(&mut self) -> PWTS_W {
        PWTS_W::new(self)
    }
    #[doc = "Bit 10 - Early transmit status"]
    #[inline(always)]
    pub fn ets(&mut self) -> ETS_W {
        ETS_W::new(self)
    }
    #[doc = "Bit 13 - Fatal bus error status"]
    #[inline(always)]
    pub fn fbes(&mut self) -> FBES_W {
        FBES_W::new(self)
    }
    #[doc = "Bit 14 - Early receive status"]
    #[inline(always)]
    pub fn ers(&mut self) -> ERS_W {
        ERS_W::new(self)
    }
    #[doc = "Bit 15 - Abnormal interrupt summary"]
    #[inline(always)]
    pub fn ais(&mut self) -> AIS_W {
        AIS_W::new(self)
    }
    #[doc = "Bit 16 - Normal interrupt summary"]
    #[inline(always)]
    pub fn nis(&mut self) -> NIS_W {
        NIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet DMA status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmasr](index.html) module"]
pub struct DMASR_SPEC;
impl crate::RegisterSpec for DMASR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmasr::R](R) reader structure"]
impl crate::Readable for DMASR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmasr::W](W) writer structure"]
impl crate::Writable for DMASR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMASR to value 0"]
impl crate::Resettable for DMASR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}