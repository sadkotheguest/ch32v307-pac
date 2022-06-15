#[doc = "Register `MACPMTCSR` reader"]
pub struct R(crate::R<MACPMTCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACPMTCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACPMTCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACPMTCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACPMTCSR` writer"]
pub struct W(crate::W<MACPMTCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACPMTCSR_SPEC>;
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
impl From<crate::W<MACPMTCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACPMTCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PD` reader - Power down"]
pub type PD_R = crate::BitReader<bool>;
#[doc = "Field `PD` writer - Power down"]
pub type PD_W<'a> = crate::BitWriter<'a, u32, MACPMTCSR_SPEC, bool, 0>;
#[doc = "Field `MPE` reader - Magic Packet enable"]
pub type MPE_R = crate::BitReader<bool>;
#[doc = "Field `MPE` writer - Magic Packet enable"]
pub type MPE_W<'a> = crate::BitWriter<'a, u32, MACPMTCSR_SPEC, bool, 1>;
#[doc = "Field `WFE` reader - Wakeup frame enable"]
pub type WFE_R = crate::BitReader<bool>;
#[doc = "Field `WFE` writer - Wakeup frame enable"]
pub type WFE_W<'a> = crate::BitWriter<'a, u32, MACPMTCSR_SPEC, bool, 2>;
#[doc = "Field `MPR` reader - Magic packet received"]
pub type MPR_R = crate::BitReader<bool>;
#[doc = "Field `MPR` writer - Magic packet received"]
pub type MPR_W<'a> = crate::BitWriter<'a, u32, MACPMTCSR_SPEC, bool, 5>;
#[doc = "Field `WFR` reader - Wakeup frame received"]
pub type WFR_R = crate::BitReader<bool>;
#[doc = "Field `WFR` writer - Wakeup frame received"]
pub type WFR_W<'a> = crate::BitWriter<'a, u32, MACPMTCSR_SPEC, bool, 6>;
#[doc = "Field `GU` reader - Global unicast"]
pub type GU_R = crate::BitReader<bool>;
#[doc = "Field `GU` writer - Global unicast"]
pub type GU_W<'a> = crate::BitWriter<'a, u32, MACPMTCSR_SPEC, bool, 9>;
#[doc = "Field `WFFRPR` reader - Wakeup frame filter register pointer reset"]
pub type WFFRPR_R = crate::BitReader<bool>;
#[doc = "Field `WFFRPR` writer - Wakeup frame filter register pointer reset"]
pub type WFFRPR_W<'a> = crate::BitWriter<'a, u32, MACPMTCSR_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - Power down"]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Magic Packet enable"]
    #[inline(always)]
    pub fn mpe(&self) -> MPE_R {
        MPE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup frame enable"]
    #[inline(always)]
    pub fn wfe(&self) -> WFE_R {
        WFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Magic packet received"]
    #[inline(always)]
    pub fn mpr(&self) -> MPR_R {
        MPR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Wakeup frame received"]
    #[inline(always)]
    pub fn wfr(&self) -> WFR_R {
        WFR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Global unicast"]
    #[inline(always)]
    pub fn gu(&self) -> GU_R {
        GU_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 31 - Wakeup frame filter register pointer reset"]
    #[inline(always)]
    pub fn wffrpr(&self) -> WFFRPR_R {
        WFFRPR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power down"]
    #[inline(always)]
    pub fn pd(&mut self) -> PD_W {
        PD_W::new(self)
    }
    #[doc = "Bit 1 - Magic Packet enable"]
    #[inline(always)]
    pub fn mpe(&mut self) -> MPE_W {
        MPE_W::new(self)
    }
    #[doc = "Bit 2 - Wakeup frame enable"]
    #[inline(always)]
    pub fn wfe(&mut self) -> WFE_W {
        WFE_W::new(self)
    }
    #[doc = "Bit 5 - Magic packet received"]
    #[inline(always)]
    pub fn mpr(&mut self) -> MPR_W {
        MPR_W::new(self)
    }
    #[doc = "Bit 6 - Wakeup frame received"]
    #[inline(always)]
    pub fn wfr(&mut self) -> WFR_W {
        WFR_W::new(self)
    }
    #[doc = "Bit 9 - Global unicast"]
    #[inline(always)]
    pub fn gu(&mut self) -> GU_W {
        GU_W::new(self)
    }
    #[doc = "Bit 31 - Wakeup frame filter register pointer reset"]
    #[inline(always)]
    pub fn wffrpr(&mut self) -> WFFRPR_W {
        WFFRPR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC PMT control and status register (ETH_MACPMTCSR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macpmtcsr](index.html) module"]
pub struct MACPMTCSR_SPEC;
impl crate::RegisterSpec for MACPMTCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macpmtcsr::R](R) reader structure"]
impl crate::Readable for MACPMTCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macpmtcsr::W](W) writer structure"]
impl crate::Writable for MACPMTCSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACPMTCSR to value 0"]
impl crate::Resettable for MACPMTCSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}