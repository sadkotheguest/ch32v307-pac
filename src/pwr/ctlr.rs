#[doc = "Register `CTLR` reader"]
pub struct R(crate::R<CTLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTLR` writer"]
pub struct W(crate::W<CTLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTLR_SPEC>;
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
impl From<crate::W<CTLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPDS` reader - Low Power Deep Sleep"]
pub type LPDS_R = crate::BitReader<bool>;
#[doc = "Field `LPDS` writer - Low Power Deep Sleep"]
pub type LPDS_W<'a> = crate::BitWriter<'a, u32, CTLR_SPEC, bool, 0>;
#[doc = "Field `PDDS` reader - Power Down Deep Sleep"]
pub type PDDS_R = crate::BitReader<bool>;
#[doc = "Field `PDDS` writer - Power Down Deep Sleep"]
pub type PDDS_W<'a> = crate::BitWriter<'a, u32, CTLR_SPEC, bool, 1>;
#[doc = "Field `CWUF` reader - Clear Wake-up Flag"]
pub type CWUF_R = crate::BitReader<bool>;
#[doc = "Field `CWUF` writer - Clear Wake-up Flag"]
pub type CWUF_W<'a> = crate::BitWriter<'a, u32, CTLR_SPEC, bool, 2>;
#[doc = "Field `CSBF` reader - Clear STANDBY Flag"]
pub type CSBF_R = crate::BitReader<bool>;
#[doc = "Field `CSBF` writer - Clear STANDBY Flag"]
pub type CSBF_W<'a> = crate::BitWriter<'a, u32, CTLR_SPEC, bool, 3>;
#[doc = "Field `PVDE` reader - Power Voltage Detector Enable"]
pub type PVDE_R = crate::BitReader<bool>;
#[doc = "Field `PVDE` writer - Power Voltage Detector Enable"]
pub type PVDE_W<'a> = crate::BitWriter<'a, u32, CTLR_SPEC, bool, 4>;
#[doc = "Field `PLS` reader - PVD Level Selection"]
pub type PLS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLS` writer - PVD Level Selection"]
pub type PLS_W<'a> = crate::FieldWriter<'a, u32, CTLR_SPEC, u8, u8, 3, 5>;
#[doc = "Field `DBP` reader - Disable Backup Domain write protection"]
pub type DBP_R = crate::BitReader<bool>;
#[doc = "Field `DBP` writer - Disable Backup Domain write protection"]
pub type DBP_W<'a> = crate::BitWriter<'a, u32, CTLR_SPEC, bool, 8>;
#[doc = "Field `R2K_STY_EN` reader - standby 2k ram enable"]
pub type R2K_STY_EN_R = crate::BitReader<bool>;
#[doc = "Field `R2K_STY_EN` writer - standby 2k ram enable"]
pub type R2K_STY_EN_W<'a> = crate::BitWriter<'a, u32, CTLR_SPEC, bool, 16>;
#[doc = "Field `R30K_STY_EN` reader - standby 30k ram enable"]
pub type R30K_STY_EN_R = crate::BitReader<bool>;
#[doc = "Field `R30K_STY_EN` writer - standby 30k ram enable"]
pub type R30K_STY_EN_W<'a> = crate::BitWriter<'a, u32, CTLR_SPEC, bool, 17>;
#[doc = "Field `R2K_VBAT_EN` reader - VBAT 30k ram enable"]
pub type R2K_VBAT_EN_R = crate::BitReader<bool>;
#[doc = "Field `R2K_VBAT_EN` writer - VBAT 30k ram enable"]
pub type R2K_VBAT_EN_W<'a> = crate::BitWriter<'a, u32, CTLR_SPEC, bool, 18>;
#[doc = "Field `R30K_VBAT_EN` reader - VBAT 30k ram enable"]
pub type R30K_VBAT_EN_R = crate::BitReader<bool>;
#[doc = "Field `R30K_VBAT_EN` writer - VBAT 30k ram enable"]
pub type R30K_VBAT_EN_W<'a> = crate::BitWriter<'a, u32, CTLR_SPEC, bool, 19>;
#[doc = "Field `RAM_LV_EN` reader - Ram LV Enable"]
pub type RAM_LV_EN_R = crate::BitReader<bool>;
#[doc = "Field `RAM_LV_EN` writer - Ram LV Enable"]
pub type RAM_LV_EN_W<'a> = crate::BitWriter<'a, u32, CTLR_SPEC, bool, 20>;
impl R {
    #[doc = "Bit 0 - Low Power Deep Sleep"]
    #[inline(always)]
    pub fn lpds(&self) -> LPDS_R {
        LPDS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power Down Deep Sleep"]
    #[inline(always)]
    pub fn pdds(&self) -> PDDS_R {
        PDDS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear Wake-up Flag"]
    #[inline(always)]
    pub fn cwuf(&self) -> CWUF_R {
        CWUF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear STANDBY Flag"]
    #[inline(always)]
    pub fn csbf(&self) -> CSBF_R {
        CSBF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Power Voltage Detector Enable"]
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - PVD Level Selection"]
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Disable Backup Domain write protection"]
    #[inline(always)]
    pub fn dbp(&self) -> DBP_R {
        DBP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - standby 2k ram enable"]
    #[inline(always)]
    pub fn r2k_sty_en(&self) -> R2K_STY_EN_R {
        R2K_STY_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - standby 30k ram enable"]
    #[inline(always)]
    pub fn r30k_sty_en(&self) -> R30K_STY_EN_R {
        R30K_STY_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - VBAT 30k ram enable"]
    #[inline(always)]
    pub fn r2k_vbat_en(&self) -> R2K_VBAT_EN_R {
        R2K_VBAT_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - VBAT 30k ram enable"]
    #[inline(always)]
    pub fn r30k_vbat_en(&self) -> R30K_VBAT_EN_R {
        R30K_VBAT_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Ram LV Enable"]
    #[inline(always)]
    pub fn ram_lv_en(&self) -> RAM_LV_EN_R {
        RAM_LV_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low Power Deep Sleep"]
    #[inline(always)]
    pub fn lpds(&mut self) -> LPDS_W {
        LPDS_W::new(self)
    }
    #[doc = "Bit 1 - Power Down Deep Sleep"]
    #[inline(always)]
    pub fn pdds(&mut self) -> PDDS_W {
        PDDS_W::new(self)
    }
    #[doc = "Bit 2 - Clear Wake-up Flag"]
    #[inline(always)]
    pub fn cwuf(&mut self) -> CWUF_W {
        CWUF_W::new(self)
    }
    #[doc = "Bit 3 - Clear STANDBY Flag"]
    #[inline(always)]
    pub fn csbf(&mut self) -> CSBF_W {
        CSBF_W::new(self)
    }
    #[doc = "Bit 4 - Power Voltage Detector Enable"]
    #[inline(always)]
    pub fn pvde(&mut self) -> PVDE_W {
        PVDE_W::new(self)
    }
    #[doc = "Bits 5:7 - PVD Level Selection"]
    #[inline(always)]
    pub fn pls(&mut self) -> PLS_W {
        PLS_W::new(self)
    }
    #[doc = "Bit 8 - Disable Backup Domain write protection"]
    #[inline(always)]
    pub fn dbp(&mut self) -> DBP_W {
        DBP_W::new(self)
    }
    #[doc = "Bit 16 - standby 2k ram enable"]
    #[inline(always)]
    pub fn r2k_sty_en(&mut self) -> R2K_STY_EN_W {
        R2K_STY_EN_W::new(self)
    }
    #[doc = "Bit 17 - standby 30k ram enable"]
    #[inline(always)]
    pub fn r30k_sty_en(&mut self) -> R30K_STY_EN_W {
        R30K_STY_EN_W::new(self)
    }
    #[doc = "Bit 18 - VBAT 30k ram enable"]
    #[inline(always)]
    pub fn r2k_vbat_en(&mut self) -> R2K_VBAT_EN_W {
        R2K_VBAT_EN_W::new(self)
    }
    #[doc = "Bit 19 - VBAT 30k ram enable"]
    #[inline(always)]
    pub fn r30k_vbat_en(&mut self) -> R30K_VBAT_EN_W {
        R30K_VBAT_EN_W::new(self)
    }
    #[doc = "Bit 20 - Ram LV Enable"]
    #[inline(always)]
    pub fn ram_lv_en(&mut self) -> RAM_LV_EN_W {
        RAM_LV_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power control register (PWR_CTRL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctlr](index.html) module"]
pub struct CTLR_SPEC;
impl crate::RegisterSpec for CTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctlr::R](R) reader structure"]
impl crate::Readable for CTLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctlr::W](W) writer structure"]
impl crate::Writable for CTLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTLR to value 0"]
impl crate::Resettable for CTLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
