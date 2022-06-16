#[doc = "Register `EXTEND_CTR` reader"]
pub struct R(crate::R<EXTEND_CTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTEND_CTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTEND_CTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTEND_CTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTEND_CTR` writer"]
pub struct W(crate::W<EXTEND_CTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTEND_CTR_SPEC>;
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
impl From<crate::W<EXTEND_CTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTEND_CTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBDLS` reader - USBD Lowspeed Enable"]
pub type USBDLS_R = crate::BitReader<bool>;
#[doc = "Field `USBDLS` writer - USBD Lowspeed Enable"]
pub type USBDLS_W<'a> = crate::BitWriter<'a, u32, EXTEND_CTR_SPEC, bool, 0>;
#[doc = "Field `USBDPU` reader - USBD pullup Enable"]
pub type USBDPU_R = crate::BitReader<bool>;
#[doc = "Field `USBDPU` writer - USBD pullup Enable"]
pub type USBDPU_W<'a> = crate::BitWriter<'a, u32, EXTEND_CTR_SPEC, bool, 1>;
#[doc = "Field `ETH_10M_EN` reader - ETH 10M Enable"]
pub type ETH_10M_EN_R = crate::BitReader<bool>;
#[doc = "Field `ETH_10M_EN` writer - ETH 10M Enable"]
pub type ETH_10M_EN_W<'a> = crate::BitWriter<'a, u32, EXTEND_CTR_SPEC, bool, 2>;
#[doc = "Field `ETH_RGMII_EN` reader - ETH RGMII Enable"]
pub type ETH_RGMII_EN_R = crate::BitReader<bool>;
#[doc = "Field `ETH_RGMII_EN` writer - ETH RGMII Enable"]
pub type ETH_RGMII_EN_W<'a> = crate::BitWriter<'a, u32, EXTEND_CTR_SPEC, bool, 3>;
#[doc = "Field `PLL_HSI_PRE` reader - Whether HSI is divided"]
pub type PLL_HSI_PRE_R = crate::BitReader<bool>;
#[doc = "Field `PLL_HSI_PRE` writer - Whether HSI is divided"]
pub type PLL_HSI_PRE_W<'a> = crate::BitWriter<'a, u32, EXTEND_CTR_SPEC, bool, 4>;
#[doc = "Field `LOCKUP_EN` reader - LOCKUP_Eable"]
pub type LOCKUP_EN_R = crate::BitReader<bool>;
#[doc = "Field `LOCKUP_EN` writer - LOCKUP_Eable"]
pub type LOCKUP_EN_W<'a> = crate::BitWriter<'a, u32, EXTEND_CTR_SPEC, bool, 6>;
#[doc = "Field `LOCKUP_RSTF` reader - LOCKUP RESET"]
pub type LOCKUP_RSTF_R = crate::BitReader<bool>;
#[doc = "Field `LOCKUP_RSTF` writer - LOCKUP RESET"]
pub type LOCKUP_RSTF_W<'a> = crate::BitWriter<'a, u32, EXTEND_CTR_SPEC, bool, 7>;
#[doc = "Field `ULLDO_TRIM` reader - ULLDO_TRIM"]
pub type ULLDO_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ULLDO_TRIM` writer - ULLDO_TRIM"]
pub type ULLDO_TRIM_W<'a> = crate::FieldWriter<'a, u32, EXTEND_CTR_SPEC, u8, u8, 2, 8>;
#[doc = "Field `LDO_TRIM` reader - LDO_TRIM"]
pub type LDO_TRIM_R = crate::BitReader<bool>;
#[doc = "Field `LDO_TRIM` writer - LDO_TRIM"]
pub type LDO_TRIM_W<'a> = crate::BitWriter<'a, u32, EXTEND_CTR_SPEC, bool, 10>;
impl R {
    #[doc = "Bit 0 - USBD Lowspeed Enable"]
    #[inline(always)]
    pub fn usbdls(&self) -> USBDLS_R {
        USBDLS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USBD pullup Enable"]
    #[inline(always)]
    pub fn usbdpu(&self) -> USBDPU_R {
        USBDPU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ETH 10M Enable"]
    #[inline(always)]
    pub fn eth_10m_en(&self) -> ETH_10M_EN_R {
        ETH_10M_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ETH RGMII Enable"]
    #[inline(always)]
    pub fn eth_rgmii_en(&self) -> ETH_RGMII_EN_R {
        ETH_RGMII_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Whether HSI is divided"]
    #[inline(always)]
    pub fn pll_hsi_pre(&self) -> PLL_HSI_PRE_R {
        PLL_HSI_PRE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - LOCKUP_Eable"]
    #[inline(always)]
    pub fn lockup_en(&self) -> LOCKUP_EN_R {
        LOCKUP_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LOCKUP RESET"]
    #[inline(always)]
    pub fn lockup_rstf(&self) -> LOCKUP_RSTF_R {
        LOCKUP_RSTF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - ULLDO_TRIM"]
    #[inline(always)]
    pub fn ulldo_trim(&self) -> ULLDO_TRIM_R {
        ULLDO_TRIM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - LDO_TRIM"]
    #[inline(always)]
    pub fn ldo_trim(&self) -> LDO_TRIM_R {
        LDO_TRIM_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USBD Lowspeed Enable"]
    #[inline(always)]
    pub fn usbdls(&mut self) -> USBDLS_W {
        USBDLS_W::new(self)
    }
    #[doc = "Bit 1 - USBD pullup Enable"]
    #[inline(always)]
    pub fn usbdpu(&mut self) -> USBDPU_W {
        USBDPU_W::new(self)
    }
    #[doc = "Bit 2 - ETH 10M Enable"]
    #[inline(always)]
    pub fn eth_10m_en(&mut self) -> ETH_10M_EN_W {
        ETH_10M_EN_W::new(self)
    }
    #[doc = "Bit 3 - ETH RGMII Enable"]
    #[inline(always)]
    pub fn eth_rgmii_en(&mut self) -> ETH_RGMII_EN_W {
        ETH_RGMII_EN_W::new(self)
    }
    #[doc = "Bit 4 - Whether HSI is divided"]
    #[inline(always)]
    pub fn pll_hsi_pre(&mut self) -> PLL_HSI_PRE_W {
        PLL_HSI_PRE_W::new(self)
    }
    #[doc = "Bit 6 - LOCKUP_Eable"]
    #[inline(always)]
    pub fn lockup_en(&mut self) -> LOCKUP_EN_W {
        LOCKUP_EN_W::new(self)
    }
    #[doc = "Bit 7 - LOCKUP RESET"]
    #[inline(always)]
    pub fn lockup_rstf(&mut self) -> LOCKUP_RSTF_W {
        LOCKUP_RSTF_W::new(self)
    }
    #[doc = "Bits 8:9 - ULLDO_TRIM"]
    #[inline(always)]
    pub fn ulldo_trim(&mut self) -> ULLDO_TRIM_W {
        ULLDO_TRIM_W::new(self)
    }
    #[doc = "Bit 10 - LDO_TRIM"]
    #[inline(always)]
    pub fn ldo_trim(&mut self) -> LDO_TRIM_W {
        LDO_TRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTEND register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extend_ctr](index.html) module"]
pub struct EXTEND_CTR_SPEC;
impl crate::RegisterSpec for EXTEND_CTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extend_ctr::R](R) reader structure"]
impl crate::Readable for EXTEND_CTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extend_ctr::W](W) writer structure"]
impl crate::Writable for EXTEND_CTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTEND_CTR to value 0x20"]
impl crate::Resettable for EXTEND_CTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}