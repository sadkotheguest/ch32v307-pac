#[doc = "Register `SMCFGR` reader"]
pub struct R(crate::R<SMCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMCFGR` writer"]
pub struct W(crate::W<SMCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMCFGR_SPEC>;
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
impl From<crate::W<SMCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ETP` reader - External trigger polarity"]
pub type ETP_R = crate::BitReader<bool>;
#[doc = "Field `ETP` writer - External trigger polarity"]
pub type ETP_W<'a> = crate::BitWriter<'a, u32, SMCFGR_SPEC, bool, 15>;
#[doc = "Field `ECE` reader - External clock enable"]
pub type ECE_R = crate::BitReader<bool>;
#[doc = "Field `ECE` writer - External clock enable"]
pub type ECE_W<'a> = crate::BitWriter<'a, u32, SMCFGR_SPEC, bool, 14>;
#[doc = "Field `ETPS` reader - External trigger prescaler"]
pub type ETPS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ETPS` writer - External trigger prescaler"]
pub type ETPS_W<'a> = crate::FieldWriter<'a, u32, SMCFGR_SPEC, u8, u8, 2, 12>;
#[doc = "Field `ETF` reader - External trigger filter"]
pub type ETF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ETF` writer - External trigger filter"]
pub type ETF_W<'a> = crate::FieldWriter<'a, u32, SMCFGR_SPEC, u8, u8, 4, 8>;
#[doc = "Field `MSM` reader - Master/Slave mode"]
pub type MSM_R = crate::BitReader<bool>;
#[doc = "Field `MSM` writer - Master/Slave mode"]
pub type MSM_W<'a> = crate::BitWriter<'a, u32, SMCFGR_SPEC, bool, 7>;
#[doc = "Field `TS` reader - Trigger selection"]
pub type TS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TS` writer - Trigger selection"]
pub type TS_W<'a> = crate::FieldWriter<'a, u32, SMCFGR_SPEC, u8, u8, 3, 4>;
#[doc = "Field `SMS` reader - Slave mode selection"]
pub type SMS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMS` writer - Slave mode selection"]
pub type SMS_W<'a> = crate::FieldWriter<'a, u32, SMCFGR_SPEC, u8, u8, 3, 0>;
impl R {
    #[doc = "Bit 15 - External trigger polarity"]
    #[inline(always)]
    pub fn etp(&self) -> ETP_R {
        ETP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - External clock enable"]
    #[inline(always)]
    pub fn ece(&self) -> ECE_R {
        ECE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 12:13 - External trigger prescaler"]
    #[inline(always)]
    pub fn etps(&self) -> ETPS_R {
        ETPS_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 8:11 - External trigger filter"]
    #[inline(always)]
    pub fn etf(&self) -> ETF_R {
        ETF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Master/Slave mode"]
    #[inline(always)]
    pub fn msm(&self) -> MSM_R {
        MSM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Trigger selection"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 0:2 - Slave mode selection"]
    #[inline(always)]
    pub fn sms(&self) -> SMS_R {
        SMS_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 15 - External trigger polarity"]
    #[inline(always)]
    pub fn etp(&mut self) -> ETP_W {
        ETP_W::new(self)
    }
    #[doc = "Bit 14 - External clock enable"]
    #[inline(always)]
    pub fn ece(&mut self) -> ECE_W {
        ECE_W::new(self)
    }
    #[doc = "Bits 12:13 - External trigger prescaler"]
    #[inline(always)]
    pub fn etps(&mut self) -> ETPS_W {
        ETPS_W::new(self)
    }
    #[doc = "Bits 8:11 - External trigger filter"]
    #[inline(always)]
    pub fn etf(&mut self) -> ETF_W {
        ETF_W::new(self)
    }
    #[doc = "Bit 7 - Master/Slave mode"]
    #[inline(always)]
    pub fn msm(&mut self) -> MSM_W {
        MSM_W::new(self)
    }
    #[doc = "Bits 4:6 - Trigger selection"]
    #[inline(always)]
    pub fn ts(&mut self) -> TS_W {
        TS_W::new(self)
    }
    #[doc = "Bits 0:2 - Slave mode selection"]
    #[inline(always)]
    pub fn sms(&mut self) -> SMS_W {
        SMS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "slave mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcfgr](index.html) module"]
pub struct SMCFGR_SPEC;
impl crate::RegisterSpec for SMCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smcfgr::R](R) reader structure"]
impl crate::Readable for SMCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smcfgr::W](W) writer structure"]
impl crate::Writable for SMCFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMCFGR to value 0"]
impl crate::Resettable for SMCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}