#[doc = "Register `MACCR` reader"]
pub struct R(crate::R<MACCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACCR` writer"]
pub struct W(crate::W<MACCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACCR_SPEC>;
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
impl From<crate::W<MACCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCES` reader - Send clock selection bit"]
pub type TCES_R = crate::BitReader<bool>;
#[doc = "Field `TCES` writer - Send clock selection bit"]
pub type TCES_W<'a> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, 0>;
#[doc = "Field `TCF` reader - Send clock reversal"]
pub type TCF_R = crate::BitReader<bool>;
#[doc = "Field `TCF` writer - Send clock reversal"]
pub type TCF_W<'a> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, 1>;
#[doc = "Field `RE` reader - Receiver enable"]
pub type RE_R = crate::BitReader<bool>;
#[doc = "Field `RE` writer - Receiver enable"]
pub type RE_W<'a> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, 2>;
#[doc = "Field `TE` reader - Transmitter enable"]
pub type TE_R = crate::BitReader<bool>;
#[doc = "Field `TE` writer - Transmitter enable"]
pub type TE_W<'a> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, 3>;
#[doc = "Field `DC` reader - Deferral check"]
pub type DC_R = crate::BitReader<bool>;
#[doc = "Field `DC` writer - Deferral check"]
pub type DC_W<'a> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, 4>;
#[doc = "Field `BL` reader - Back-off limit"]
pub type BL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BL` writer - Back-off limit"]
pub type BL_W<'a> = crate::FieldWriter<'a, u32, MACCR_SPEC, u8, u8, 2, 5>;
#[doc = "Field `APCS` reader - Automatic pad/CRC stripping"]
pub type APCS_R = crate::BitReader<bool>;
#[doc = "Field `APCS` writer - Automatic pad/CRC stripping"]
pub type APCS_W<'a> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, 7>;
#[doc = "Field `RD` reader - Retry disable"]
pub type RD_R = crate::BitReader<bool>;
#[doc = "Field `RD` writer - Retry disable"]
pub type RD_W<'a> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, 9>;
#[doc = "Field `IPCO` reader - IPv4 checksum offload"]
pub type IPCO_R = crate::BitReader<bool>;
#[doc = "Field `IPCO` writer - IPv4 checksum offload"]
pub type IPCO_W<'a> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, 10>;
#[doc = "Field `DM` reader - Duplex mode"]
pub type DM_R = crate::BitReader<bool>;
#[doc = "Field `DM` writer - Duplex mode"]
pub type DM_W<'a> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, 11>;
#[doc = "Field `LM` reader - Loopback mode"]
pub type LM_R = crate::BitReader<bool>;
#[doc = "Field `LM` writer - Loopback mode"]
pub type LM_W<'a> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, 12>;
#[doc = "Field `ROD` reader - Receive own disable"]
pub type ROD_R = crate::BitReader<bool>;
#[doc = "Field `ROD` writer - Receive own disable"]
pub type ROD_W<'a> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, 13>;
#[doc = "Field `FES` reader - Fast Ethernet speed"]
pub type FES_R = crate::BitReader<bool>;
#[doc = "Field `FES` writer - Fast Ethernet speed"]
pub type FES_W<'a> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, 14>;
#[doc = "Field `CSD` reader - Carrier sense disable"]
pub type CSD_R = crate::BitReader<bool>;
#[doc = "Field `CSD` writer - Carrier sense disable"]
pub type CSD_W<'a> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, 16>;
#[doc = "Field `IFG` reader - Interframe gap"]
pub type IFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IFG` writer - Interframe gap"]
pub type IFG_W<'a> = crate::FieldWriter<'a, u32, MACCR_SPEC, u8, u8, 3, 17>;
#[doc = "Field `IRE` reader - 10MPHY 50Ω set"]
pub type IRE_R = crate::BitReader<bool>;
#[doc = "Field `IRE` writer - 10MPHY 50Ω set"]
pub type IRE_W<'a> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, 20>;
#[doc = "Field `PDI` reader - 10MPHY TX DRIVER bisa current"]
pub type PDI_R = crate::BitReader<bool>;
#[doc = "Field `PDI` writer - 10MPHY TX DRIVER bisa current"]
pub type PDI_W<'a> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, 21>;
#[doc = "Field `JD` reader - Jabber disable"]
pub type JD_R = crate::BitReader<bool>;
#[doc = "Field `JD` writer - Jabber disable"]
pub type JD_W<'a> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, 22>;
#[doc = "Field `WD` reader - Watchdog disable"]
pub type WD_R = crate::BitReader<bool>;
#[doc = "Field `WD` writer - Watchdog disable"]
pub type WD_W<'a> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, 23>;
#[doc = "Field `TCD` reader - SEND clock delay"]
pub type TCD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TCD` writer - SEND clock delay"]
pub type TCD_W<'a> = crate::FieldWriter<'a, u32, MACCR_SPEC, u8, u8, 3, 29>;
impl R {
    #[doc = "Bit 0 - Send clock selection bit"]
    #[inline(always)]
    pub fn tces(&self) -> TCES_R {
        TCES_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Send clock reversal"]
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Deferral check"]
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Back-off limit"]
    #[inline(always)]
    pub fn bl(&self) -> BL_R {
        BL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Automatic pad/CRC stripping"]
    #[inline(always)]
    pub fn apcs(&self) -> APCS_R {
        APCS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Retry disable"]
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IPv4 checksum offload"]
    #[inline(always)]
    pub fn ipco(&self) -> IPCO_R {
        IPCO_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Duplex mode"]
    #[inline(always)]
    pub fn dm(&self) -> DM_R {
        DM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Loopback mode"]
    #[inline(always)]
    pub fn lm(&self) -> LM_R {
        LM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Receive own disable"]
    #[inline(always)]
    pub fn rod(&self) -> ROD_R {
        ROD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Fast Ethernet speed"]
    #[inline(always)]
    pub fn fes(&self) -> FES_R {
        FES_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Carrier sense disable"]
    #[inline(always)]
    pub fn csd(&self) -> CSD_R {
        CSD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Interframe gap"]
    #[inline(always)]
    pub fn ifg(&self) -> IFG_R {
        IFG_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - 10MPHY 50Ω set"]
    #[inline(always)]
    pub fn ire(&self) -> IRE_R {
        IRE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 10MPHY TX DRIVER bisa current"]
    #[inline(always)]
    pub fn pdi(&self) -> PDI_R {
        PDI_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Jabber disable"]
    #[inline(always)]
    pub fn jd(&self) -> JD_R {
        JD_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Watchdog disable"]
    #[inline(always)]
    pub fn wd(&self) -> WD_R {
        WD_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 29:31 - SEND clock delay"]
    #[inline(always)]
    pub fn tcd(&self) -> TCD_R {
        TCD_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Send clock selection bit"]
    #[inline(always)]
    pub fn tces(&mut self) -> TCES_W {
        TCES_W::new(self)
    }
    #[doc = "Bit 1 - Send clock reversal"]
    #[inline(always)]
    pub fn tcf(&mut self) -> TCF_W {
        TCF_W::new(self)
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    pub fn re(&mut self) -> RE_W {
        RE_W::new(self)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    pub fn te(&mut self) -> TE_W {
        TE_W::new(self)
    }
    #[doc = "Bit 4 - Deferral check"]
    #[inline(always)]
    pub fn dc(&mut self) -> DC_W {
        DC_W::new(self)
    }
    #[doc = "Bits 5:6 - Back-off limit"]
    #[inline(always)]
    pub fn bl(&mut self) -> BL_W {
        BL_W::new(self)
    }
    #[doc = "Bit 7 - Automatic pad/CRC stripping"]
    #[inline(always)]
    pub fn apcs(&mut self) -> APCS_W {
        APCS_W::new(self)
    }
    #[doc = "Bit 9 - Retry disable"]
    #[inline(always)]
    pub fn rd(&mut self) -> RD_W {
        RD_W::new(self)
    }
    #[doc = "Bit 10 - IPv4 checksum offload"]
    #[inline(always)]
    pub fn ipco(&mut self) -> IPCO_W {
        IPCO_W::new(self)
    }
    #[doc = "Bit 11 - Duplex mode"]
    #[inline(always)]
    pub fn dm(&mut self) -> DM_W {
        DM_W::new(self)
    }
    #[doc = "Bit 12 - Loopback mode"]
    #[inline(always)]
    pub fn lm(&mut self) -> LM_W {
        LM_W::new(self)
    }
    #[doc = "Bit 13 - Receive own disable"]
    #[inline(always)]
    pub fn rod(&mut self) -> ROD_W {
        ROD_W::new(self)
    }
    #[doc = "Bit 14 - Fast Ethernet speed"]
    #[inline(always)]
    pub fn fes(&mut self) -> FES_W {
        FES_W::new(self)
    }
    #[doc = "Bit 16 - Carrier sense disable"]
    #[inline(always)]
    pub fn csd(&mut self) -> CSD_W {
        CSD_W::new(self)
    }
    #[doc = "Bits 17:19 - Interframe gap"]
    #[inline(always)]
    pub fn ifg(&mut self) -> IFG_W {
        IFG_W::new(self)
    }
    #[doc = "Bit 20 - 10MPHY 50Ω set"]
    #[inline(always)]
    pub fn ire(&mut self) -> IRE_W {
        IRE_W::new(self)
    }
    #[doc = "Bit 21 - 10MPHY TX DRIVER bisa current"]
    #[inline(always)]
    pub fn pdi(&mut self) -> PDI_W {
        PDI_W::new(self)
    }
    #[doc = "Bit 22 - Jabber disable"]
    #[inline(always)]
    pub fn jd(&mut self) -> JD_W {
        JD_W::new(self)
    }
    #[doc = "Bit 23 - Watchdog disable"]
    #[inline(always)]
    pub fn wd(&mut self) -> WD_W {
        WD_W::new(self)
    }
    #[doc = "Bits 29:31 - SEND clock delay"]
    #[inline(always)]
    pub fn tcd(&mut self) -> TCD_W {
        TCD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC configuration register (ETH_MACCR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maccr](index.html) module"]
pub struct MACCR_SPEC;
impl crate::RegisterSpec for MACCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [maccr::R](R) reader structure"]
impl crate::Readable for MACCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [maccr::W](W) writer structure"]
impl crate::Writable for MACCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACCR to value 0x8000"]
impl crate::Resettable for MACCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000
    }
}