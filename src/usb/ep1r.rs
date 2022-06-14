#[doc = "Register `EP1R` reader"]
pub struct R(crate::R<EP1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EP1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EP1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EP1R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EP1R` writer"]
pub struct W(crate::W<EP1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EP1R_SPEC>;
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
impl From<crate::W<EP1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EP1R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EA` reader - Endpoint address"]
pub type EA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EA` writer - Endpoint address"]
pub type EA_W<'a> = crate::FieldWriter<'a, u32, EP1R_SPEC, u8, u8, 4, 0>;
#[doc = "Field `STAT_TX` reader - Status bits, for transmission transfers"]
pub type STAT_TX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STAT_TX` writer - Status bits, for transmission transfers"]
pub type STAT_TX_W<'a> = crate::FieldWriter<'a, u32, EP1R_SPEC, u8, u8, 2, 4>;
#[doc = "Field `DTOG_TX` reader - Data Toggle, for transmission transfers"]
pub type DTOG_TX_R = crate::BitReader<bool>;
#[doc = "Field `DTOG_TX` writer - Data Toggle, for transmission transfers"]
pub type DTOG_TX_W<'a> = crate::BitWriter<'a, u32, EP1R_SPEC, bool, 6>;
#[doc = "Field `CTR_TX` reader - Correct Transfer for transmission"]
pub type CTR_TX_R = crate::BitReader<bool>;
#[doc = "Field `CTR_TX` writer - Correct Transfer for transmission"]
pub type CTR_TX_W<'a> = crate::BitWriter<'a, u32, EP1R_SPEC, bool, 7>;
#[doc = "Field `EP_KIND` reader - Endpoint kind"]
pub type EP_KIND_R = crate::BitReader<bool>;
#[doc = "Field `EP_KIND` writer - Endpoint kind"]
pub type EP_KIND_W<'a> = crate::BitWriter<'a, u32, EP1R_SPEC, bool, 8>;
#[doc = "Field `EP_TYPE` reader - Endpoint type"]
pub type EP_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EP_TYPE` writer - Endpoint type"]
pub type EP_TYPE_W<'a> = crate::FieldWriter<'a, u32, EP1R_SPEC, u8, u8, 2, 9>;
#[doc = "Field `SETUP` reader - Setup transaction completed"]
pub type SETUP_R = crate::BitReader<bool>;
#[doc = "Field `SETUP` writer - Setup transaction completed"]
pub type SETUP_W<'a> = crate::BitWriter<'a, u32, EP1R_SPEC, bool, 11>;
#[doc = "Field `STAT_RX` reader - Status bits, for reception transfers"]
pub type STAT_RX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STAT_RX` writer - Status bits, for reception transfers"]
pub type STAT_RX_W<'a> = crate::FieldWriter<'a, u32, EP1R_SPEC, u8, u8, 2, 12>;
#[doc = "Field `DTOG_RX` reader - Data Toggle, for reception transfers"]
pub type DTOG_RX_R = crate::BitReader<bool>;
#[doc = "Field `DTOG_RX` writer - Data Toggle, for reception transfers"]
pub type DTOG_RX_W<'a> = crate::BitWriter<'a, u32, EP1R_SPEC, bool, 14>;
#[doc = "Field `CTR_RX` reader - Correct transfer for reception"]
pub type CTR_RX_R = crate::BitReader<bool>;
#[doc = "Field `CTR_RX` writer - Correct transfer for reception"]
pub type CTR_RX_W<'a> = crate::BitWriter<'a, u32, EP1R_SPEC, bool, 15>;
impl R {
    #[doc = "Bits 0:3 - Endpoint address"]
    #[inline(always)]
    pub fn ea(&self) -> EA_R {
        EA_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Status bits, for transmission transfers"]
    #[inline(always)]
    pub fn stat_tx(&self) -> STAT_TX_R {
        STAT_TX_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Data Toggle, for transmission transfers"]
    #[inline(always)]
    pub fn dtog_tx(&self) -> DTOG_TX_R {
        DTOG_TX_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Correct Transfer for transmission"]
    #[inline(always)]
    pub fn ctr_tx(&self) -> CTR_TX_R {
        CTR_TX_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Endpoint kind"]
    #[inline(always)]
    pub fn ep_kind(&self) -> EP_KIND_R {
        EP_KIND_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Endpoint type"]
    #[inline(always)]
    pub fn ep_type(&self) -> EP_TYPE_R {
        EP_TYPE_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Setup transaction completed"]
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Status bits, for reception transfers"]
    #[inline(always)]
    pub fn stat_rx(&self) -> STAT_RX_R {
        STAT_RX_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Data Toggle, for reception transfers"]
    #[inline(always)]
    pub fn dtog_rx(&self) -> DTOG_RX_R {
        DTOG_RX_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Correct transfer for reception"]
    #[inline(always)]
    pub fn ctr_rx(&self) -> CTR_RX_R {
        CTR_RX_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Endpoint address"]
    #[inline(always)]
    pub fn ea(&mut self) -> EA_W {
        EA_W::new(self)
    }
    #[doc = "Bits 4:5 - Status bits, for transmission transfers"]
    #[inline(always)]
    pub fn stat_tx(&mut self) -> STAT_TX_W {
        STAT_TX_W::new(self)
    }
    #[doc = "Bit 6 - Data Toggle, for transmission transfers"]
    #[inline(always)]
    pub fn dtog_tx(&mut self) -> DTOG_TX_W {
        DTOG_TX_W::new(self)
    }
    #[doc = "Bit 7 - Correct Transfer for transmission"]
    #[inline(always)]
    pub fn ctr_tx(&mut self) -> CTR_TX_W {
        CTR_TX_W::new(self)
    }
    #[doc = "Bit 8 - Endpoint kind"]
    #[inline(always)]
    pub fn ep_kind(&mut self) -> EP_KIND_W {
        EP_KIND_W::new(self)
    }
    #[doc = "Bits 9:10 - Endpoint type"]
    #[inline(always)]
    pub fn ep_type(&mut self) -> EP_TYPE_W {
        EP_TYPE_W::new(self)
    }
    #[doc = "Bit 11 - Setup transaction completed"]
    #[inline(always)]
    pub fn setup(&mut self) -> SETUP_W {
        SETUP_W::new(self)
    }
    #[doc = "Bits 12:13 - Status bits, for reception transfers"]
    #[inline(always)]
    pub fn stat_rx(&mut self) -> STAT_RX_W {
        STAT_RX_W::new(self)
    }
    #[doc = "Bit 14 - Data Toggle, for reception transfers"]
    #[inline(always)]
    pub fn dtog_rx(&mut self) -> DTOG_RX_W {
        DTOG_RX_W::new(self)
    }
    #[doc = "Bit 15 - Correct transfer for reception"]
    #[inline(always)]
    pub fn ctr_rx(&mut self) -> CTR_RX_W {
        CTR_RX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "endpoint 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep1r](index.html) module"]
pub struct EP1R_SPEC;
impl crate::RegisterSpec for EP1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ep1r::R](R) reader structure"]
impl crate::Readable for EP1R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ep1r::W](W) writer structure"]
impl crate::Writable for EP1R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EP1R to value 0"]
impl crate::Resettable for EP1R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}