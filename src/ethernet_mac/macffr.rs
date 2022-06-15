#[doc = "Register `MACFFR` reader"]
pub struct R(crate::R<MACFFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACFFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACFFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACFFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACFFR` writer"]
pub struct W(crate::W<MACFFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACFFR_SPEC>;
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
impl From<crate::W<MACFFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACFFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PM` reader - Promiscuous mode"]
pub type PM_R = crate::BitReader<bool>;
#[doc = "Field `PM` writer - Promiscuous mode"]
pub type PM_W<'a> = crate::BitWriter<'a, u32, MACFFR_SPEC, bool, 0>;
#[doc = "Field `HU` reader - Hash unicast"]
pub type HU_R = crate::BitReader<bool>;
#[doc = "Field `HU` writer - Hash unicast"]
pub type HU_W<'a> = crate::BitWriter<'a, u32, MACFFR_SPEC, bool, 1>;
#[doc = "Field `HM` reader - Hash multicast"]
pub type HM_R = crate::BitReader<bool>;
#[doc = "Field `HM` writer - Hash multicast"]
pub type HM_W<'a> = crate::BitWriter<'a, u32, MACFFR_SPEC, bool, 2>;
#[doc = "Field `DAIF` reader - Destination address inverse filtering"]
pub type DAIF_R = crate::BitReader<bool>;
#[doc = "Field `DAIF` writer - Destination address inverse filtering"]
pub type DAIF_W<'a> = crate::BitWriter<'a, u32, MACFFR_SPEC, bool, 3>;
#[doc = "Field `PAM` reader - Pass all multicast"]
pub type PAM_R = crate::BitReader<bool>;
#[doc = "Field `PAM` writer - Pass all multicast"]
pub type PAM_W<'a> = crate::BitWriter<'a, u32, MACFFR_SPEC, bool, 4>;
#[doc = "Field `BFD` reader - Broadcast frames disable"]
pub type BFD_R = crate::BitReader<bool>;
#[doc = "Field `BFD` writer - Broadcast frames disable"]
pub type BFD_W<'a> = crate::BitWriter<'a, u32, MACFFR_SPEC, bool, 5>;
#[doc = "Field `PCF` reader - Pass control frames"]
pub type PCF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCF` writer - Pass control frames"]
pub type PCF_W<'a> = crate::FieldWriter<'a, u32, MACFFR_SPEC, u8, u8, 2, 6>;
#[doc = "Field `SAIF` reader - Source address inverse filtering"]
pub type SAIF_R = crate::BitReader<bool>;
#[doc = "Field `SAIF` writer - Source address inverse filtering"]
pub type SAIF_W<'a> = crate::BitWriter<'a, u32, MACFFR_SPEC, bool, 8>;
#[doc = "Field `SAF` reader - Source address filter"]
pub type SAF_R = crate::BitReader<bool>;
#[doc = "Field `SAF` writer - Source address filter"]
pub type SAF_W<'a> = crate::BitWriter<'a, u32, MACFFR_SPEC, bool, 9>;
#[doc = "Field `HPF` reader - Hash or perfect filter"]
pub type HPF_R = crate::BitReader<bool>;
#[doc = "Field `HPF` writer - Hash or perfect filter"]
pub type HPF_W<'a> = crate::BitWriter<'a, u32, MACFFR_SPEC, bool, 10>;
#[doc = "Field `RA` reader - Receive all"]
pub type RA_R = crate::BitReader<bool>;
#[doc = "Field `RA` writer - Receive all"]
pub type RA_W<'a> = crate::BitWriter<'a, u32, MACFFR_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - Promiscuous mode"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Hash unicast"]
    #[inline(always)]
    pub fn hu(&self) -> HU_R {
        HU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Hash multicast"]
    #[inline(always)]
    pub fn hm(&self) -> HM_R {
        HM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Destination address inverse filtering"]
    #[inline(always)]
    pub fn daif(&self) -> DAIF_R {
        DAIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pass all multicast"]
    #[inline(always)]
    pub fn pam(&self) -> PAM_R {
        PAM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Broadcast frames disable"]
    #[inline(always)]
    pub fn bfd(&self) -> BFD_R {
        BFD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Pass control frames"]
    #[inline(always)]
    pub fn pcf(&self) -> PCF_R {
        PCF_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Source address inverse filtering"]
    #[inline(always)]
    pub fn saif(&self) -> SAIF_R {
        SAIF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Source address filter"]
    #[inline(always)]
    pub fn saf(&self) -> SAF_R {
        SAF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Hash or perfect filter"]
    #[inline(always)]
    pub fn hpf(&self) -> HPF_R {
        HPF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 31 - Receive all"]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Promiscuous mode"]
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W {
        PM_W::new(self)
    }
    #[doc = "Bit 1 - Hash unicast"]
    #[inline(always)]
    pub fn hu(&mut self) -> HU_W {
        HU_W::new(self)
    }
    #[doc = "Bit 2 - Hash multicast"]
    #[inline(always)]
    pub fn hm(&mut self) -> HM_W {
        HM_W::new(self)
    }
    #[doc = "Bit 3 - Destination address inverse filtering"]
    #[inline(always)]
    pub fn daif(&mut self) -> DAIF_W {
        DAIF_W::new(self)
    }
    #[doc = "Bit 4 - Pass all multicast"]
    #[inline(always)]
    pub fn pam(&mut self) -> PAM_W {
        PAM_W::new(self)
    }
    #[doc = "Bit 5 - Broadcast frames disable"]
    #[inline(always)]
    pub fn bfd(&mut self) -> BFD_W {
        BFD_W::new(self)
    }
    #[doc = "Bits 6:7 - Pass control frames"]
    #[inline(always)]
    pub fn pcf(&mut self) -> PCF_W {
        PCF_W::new(self)
    }
    #[doc = "Bit 8 - Source address inverse filtering"]
    #[inline(always)]
    pub fn saif(&mut self) -> SAIF_W {
        SAIF_W::new(self)
    }
    #[doc = "Bit 9 - Source address filter"]
    #[inline(always)]
    pub fn saf(&mut self) -> SAF_W {
        SAF_W::new(self)
    }
    #[doc = "Bit 10 - Hash or perfect filter"]
    #[inline(always)]
    pub fn hpf(&mut self) -> HPF_W {
        HPF_W::new(self)
    }
    #[doc = "Bit 31 - Receive all"]
    #[inline(always)]
    pub fn ra(&mut self) -> RA_W {
        RA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC frame filter register (ETH_MACCFFR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macffr](index.html) module"]
pub struct MACFFR_SPEC;
impl crate::RegisterSpec for MACFFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macffr::R](R) reader structure"]
impl crate::Readable for MACFFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macffr::W](W) writer structure"]
impl crate::Writable for MACFFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACFFR to value 0"]
impl crate::Resettable for MACFFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}