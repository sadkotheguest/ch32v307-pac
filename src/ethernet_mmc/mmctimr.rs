#[doc = "Register `MMCTIMR` reader"]
pub struct R(crate::R<MMCTIMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMCTIMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMCTIMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMCTIMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMCTIMR` writer"]
pub struct W(crate::W<MMCTIMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMCTIMR_SPEC>;
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
impl From<crate::W<MMCTIMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMCTIMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TGFSCM` reader - Transmitted good frames single collision mask"]
pub type TGFSCM_R = crate::BitReader<bool>;
#[doc = "Field `TGFSCM` writer - Transmitted good frames single collision mask"]
pub type TGFSCM_W<'a> = crate::BitWriter<'a, u32, MMCTIMR_SPEC, bool, 14>;
#[doc = "Field `TGFMSCM` reader - Transmitted good frames more single collision mask"]
pub type TGFMSCM_R = crate::BitReader<bool>;
#[doc = "Field `TGFMSCM` writer - Transmitted good frames more single collision mask"]
pub type TGFMSCM_W<'a> = crate::BitWriter<'a, u32, MMCTIMR_SPEC, bool, 15>;
#[doc = "Field `TGFM` reader - Transmitted good frames mask"]
pub type TGFM_R = crate::BitReader<bool>;
#[doc = "Field `TGFM` writer - Transmitted good frames mask"]
pub type TGFM_W<'a> = crate::BitWriter<'a, u32, MMCTIMR_SPEC, bool, 21>;
impl R {
    #[doc = "Bit 14 - Transmitted good frames single collision mask"]
    #[inline(always)]
    pub fn tgfscm(&self) -> TGFSCM_R {
        TGFSCM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Transmitted good frames more single collision mask"]
    #[inline(always)]
    pub fn tgfmscm(&self) -> TGFMSCM_R {
        TGFMSCM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmitted good frames mask"]
    #[inline(always)]
    pub fn tgfm(&self) -> TGFM_R {
        TGFM_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - Transmitted good frames single collision mask"]
    #[inline(always)]
    pub fn tgfscm(&mut self) -> TGFSCM_W {
        TGFSCM_W::new(self)
    }
    #[doc = "Bit 15 - Transmitted good frames more single collision mask"]
    #[inline(always)]
    pub fn tgfmscm(&mut self) -> TGFMSCM_W {
        TGFMSCM_W::new(self)
    }
    #[doc = "Bit 21 - Transmitted good frames mask"]
    #[inline(always)]
    pub fn tgfm(&mut self) -> TGFM_W {
        TGFM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MMC transmit interrupt mask register (ETH_MMCTIMR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmctimr](index.html) module"]
pub struct MMCTIMR_SPEC;
impl crate::RegisterSpec for MMCTIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmctimr::R](R) reader structure"]
impl crate::Readable for MMCTIMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmctimr::W](W) writer structure"]
impl crate::Writable for MMCTIMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MMCTIMR to value 0"]
impl crate::Resettable for MMCTIMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}