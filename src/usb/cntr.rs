#[doc = "Register `CNTR` reader"]
pub struct R(crate::R<CNTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNTR` writer"]
pub struct W(crate::W<CNTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNTR_SPEC>;
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
impl From<crate::W<CNTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRES` reader - Force USB Reset"]
pub type FRES_R = crate::BitReader<bool>;
#[doc = "Field `FRES` writer - Force USB Reset"]
pub type FRES_W<'a> = crate::BitWriter<'a, u32, CNTR_SPEC, bool, 0>;
#[doc = "Field `PDWN` reader - Power down"]
pub type PDWN_R = crate::BitReader<bool>;
#[doc = "Field `PDWN` writer - Power down"]
pub type PDWN_W<'a> = crate::BitWriter<'a, u32, CNTR_SPEC, bool, 1>;
#[doc = "Field `LPMODE` reader - Low-power mode"]
pub type LPMODE_R = crate::BitReader<bool>;
#[doc = "Field `LPMODE` writer - Low-power mode"]
pub type LPMODE_W<'a> = crate::BitWriter<'a, u32, CNTR_SPEC, bool, 2>;
#[doc = "Field `FSUSP` reader - Force suspend"]
pub type FSUSP_R = crate::BitReader<bool>;
#[doc = "Field `FSUSP` writer - Force suspend"]
pub type FSUSP_W<'a> = crate::BitWriter<'a, u32, CNTR_SPEC, bool, 3>;
#[doc = "Field `RESUME` reader - Resume request"]
pub type RESUME_R = crate::BitReader<bool>;
#[doc = "Field `RESUME` writer - Resume request"]
pub type RESUME_W<'a> = crate::BitWriter<'a, u32, CNTR_SPEC, bool, 4>;
#[doc = "Field `ESOFM` reader - Expected start of frame interrupt mask"]
pub type ESOFM_R = crate::BitReader<bool>;
#[doc = "Field `ESOFM` writer - Expected start of frame interrupt mask"]
pub type ESOFM_W<'a> = crate::BitWriter<'a, u32, CNTR_SPEC, bool, 8>;
#[doc = "Field `SOFM` reader - Start of frame interrupt mask"]
pub type SOFM_R = crate::BitReader<bool>;
#[doc = "Field `SOFM` writer - Start of frame interrupt mask"]
pub type SOFM_W<'a> = crate::BitWriter<'a, u32, CNTR_SPEC, bool, 9>;
#[doc = "Field `RESETM` reader - USB reset interrupt mask"]
pub type RESETM_R = crate::BitReader<bool>;
#[doc = "Field `RESETM` writer - USB reset interrupt mask"]
pub type RESETM_W<'a> = crate::BitWriter<'a, u32, CNTR_SPEC, bool, 10>;
#[doc = "Field `SUSPM` reader - Suspend mode interrupt mask"]
pub type SUSPM_R = crate::BitReader<bool>;
#[doc = "Field `SUSPM` writer - Suspend mode interrupt mask"]
pub type SUSPM_W<'a> = crate::BitWriter<'a, u32, CNTR_SPEC, bool, 11>;
#[doc = "Field `WKUPM` reader - Wakeup interrupt mask"]
pub type WKUPM_R = crate::BitReader<bool>;
#[doc = "Field `WKUPM` writer - Wakeup interrupt mask"]
pub type WKUPM_W<'a> = crate::BitWriter<'a, u32, CNTR_SPEC, bool, 12>;
#[doc = "Field `ERRM` reader - Error interrupt mask"]
pub type ERRM_R = crate::BitReader<bool>;
#[doc = "Field `ERRM` writer - Error interrupt mask"]
pub type ERRM_W<'a> = crate::BitWriter<'a, u32, CNTR_SPEC, bool, 13>;
#[doc = "Field `PMAOVRM` reader - Packet memory area over / underrun interrupt mask"]
pub type PMAOVRM_R = crate::BitReader<bool>;
#[doc = "Field `PMAOVRM` writer - Packet memory area over / underrun interrupt mask"]
pub type PMAOVRM_W<'a> = crate::BitWriter<'a, u32, CNTR_SPEC, bool, 14>;
#[doc = "Field `CTRM` reader - Correct transfer interrupt mask"]
pub type CTRM_R = crate::BitReader<bool>;
#[doc = "Field `CTRM` writer - Correct transfer interrupt mask"]
pub type CTRM_W<'a> = crate::BitWriter<'a, u32, CNTR_SPEC, bool, 15>;
impl R {
    #[doc = "Bit 0 - Force USB Reset"]
    #[inline(always)]
    pub fn fres(&self) -> FRES_R {
        FRES_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power down"]
    #[inline(always)]
    pub fn pdwn(&self) -> PDWN_R {
        PDWN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Low-power mode"]
    #[inline(always)]
    pub fn lpmode(&self) -> LPMODE_R {
        LPMODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Force suspend"]
    #[inline(always)]
    pub fn fsusp(&self) -> FSUSP_R {
        FSUSP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Resume request"]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Expected start of frame interrupt mask"]
    #[inline(always)]
    pub fn esofm(&self) -> ESOFM_R {
        ESOFM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start of frame interrupt mask"]
    #[inline(always)]
    pub fn sofm(&self) -> SOFM_R {
        SOFM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - USB reset interrupt mask"]
    #[inline(always)]
    pub fn resetm(&self) -> RESETM_R {
        RESETM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Suspend mode interrupt mask"]
    #[inline(always)]
    pub fn suspm(&self) -> SUSPM_R {
        SUSPM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Wakeup interrupt mask"]
    #[inline(always)]
    pub fn wkupm(&self) -> WKUPM_R {
        WKUPM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Error interrupt mask"]
    #[inline(always)]
    pub fn errm(&self) -> ERRM_R {
        ERRM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Packet memory area over / underrun interrupt mask"]
    #[inline(always)]
    pub fn pmaovrm(&self) -> PMAOVRM_R {
        PMAOVRM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Correct transfer interrupt mask"]
    #[inline(always)]
    pub fn ctrm(&self) -> CTRM_R {
        CTRM_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Force USB Reset"]
    #[inline(always)]
    pub fn fres(&mut self) -> FRES_W {
        FRES_W::new(self)
    }
    #[doc = "Bit 1 - Power down"]
    #[inline(always)]
    pub fn pdwn(&mut self) -> PDWN_W {
        PDWN_W::new(self)
    }
    #[doc = "Bit 2 - Low-power mode"]
    #[inline(always)]
    pub fn lpmode(&mut self) -> LPMODE_W {
        LPMODE_W::new(self)
    }
    #[doc = "Bit 3 - Force suspend"]
    #[inline(always)]
    pub fn fsusp(&mut self) -> FSUSP_W {
        FSUSP_W::new(self)
    }
    #[doc = "Bit 4 - Resume request"]
    #[inline(always)]
    pub fn resume(&mut self) -> RESUME_W {
        RESUME_W::new(self)
    }
    #[doc = "Bit 8 - Expected start of frame interrupt mask"]
    #[inline(always)]
    pub fn esofm(&mut self) -> ESOFM_W {
        ESOFM_W::new(self)
    }
    #[doc = "Bit 9 - Start of frame interrupt mask"]
    #[inline(always)]
    pub fn sofm(&mut self) -> SOFM_W {
        SOFM_W::new(self)
    }
    #[doc = "Bit 10 - USB reset interrupt mask"]
    #[inline(always)]
    pub fn resetm(&mut self) -> RESETM_W {
        RESETM_W::new(self)
    }
    #[doc = "Bit 11 - Suspend mode interrupt mask"]
    #[inline(always)]
    pub fn suspm(&mut self) -> SUSPM_W {
        SUSPM_W::new(self)
    }
    #[doc = "Bit 12 - Wakeup interrupt mask"]
    #[inline(always)]
    pub fn wkupm(&mut self) -> WKUPM_W {
        WKUPM_W::new(self)
    }
    #[doc = "Bit 13 - Error interrupt mask"]
    #[inline(always)]
    pub fn errm(&mut self) -> ERRM_W {
        ERRM_W::new(self)
    }
    #[doc = "Bit 14 - Packet memory area over / underrun interrupt mask"]
    #[inline(always)]
    pub fn pmaovrm(&mut self) -> PMAOVRM_W {
        PMAOVRM_W::new(self)
    }
    #[doc = "Bit 15 - Correct transfer interrupt mask"]
    #[inline(always)]
    pub fn ctrm(&mut self) -> CTRM_W {
        CTRM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntr](index.html) module"]
pub struct CNTR_SPEC;
impl crate::RegisterSpec for CNTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cntr::R](R) reader structure"]
impl crate::Readable for CNTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cntr::W](W) writer structure"]
impl crate::Writable for CNTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNTR to value 0x03"]
impl crate::Resettable for CNTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}