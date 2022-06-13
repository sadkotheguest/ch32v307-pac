#[doc = "Register `SCTLR` reader"]
pub struct R(crate::R<SCTLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCTLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCTLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCTLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCTLR` writer"]
pub struct W(crate::W<SCTLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCTLR_SPEC>;
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
impl From<crate::W<SCTLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCTLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLEEPONEXIT` reader - SLEEPONEXIT"]
pub type SLEEPONEXIT_R = crate::BitReader<bool>;
#[doc = "Field `SLEEPONEXIT` writer - SLEEPONEXIT"]
pub type SLEEPONEXIT_W<'a> = crate::BitWriter<'a, u32, SCTLR_SPEC, bool, 1>;
#[doc = "Field `SLEEPDEEP` reader - SLEEPDEEP"]
pub type SLEEPDEEP_R = crate::BitReader<bool>;
#[doc = "Field `SLEEPDEEP` writer - SLEEPDEEP"]
pub type SLEEPDEEP_W<'a> = crate::BitWriter<'a, u32, SCTLR_SPEC, bool, 2>;
#[doc = "Field `WFITOWFE` reader - WFITOWFE"]
pub type WFITOWFE_R = crate::BitReader<bool>;
#[doc = "Field `WFITOWFE` writer - WFITOWFE"]
pub type WFITOWFE_W<'a> = crate::BitWriter<'a, u32, SCTLR_SPEC, bool, 3>;
#[doc = "Field `SEVONPEND` reader - SEVONPEND"]
pub type SEVONPEND_R = crate::BitReader<bool>;
#[doc = "Field `SEVONPEND` writer - SEVONPEND"]
pub type SEVONPEND_W<'a> = crate::BitWriter<'a, u32, SCTLR_SPEC, bool, 4>;
#[doc = "Field `SETEVENT` reader - SETEVENT"]
pub type SETEVENT_R = crate::BitReader<bool>;
#[doc = "Field `SETEVENT` writer - SETEVENT"]
pub type SETEVENT_W<'a> = crate::BitWriter<'a, u32, SCTLR_SPEC, bool, 5>;
#[doc = "Field `SYSRESET` reader - SYSRESET"]
pub type SYSRESET_R = crate::BitReader<bool>;
#[doc = "Field `SYSRESET` writer - SYSRESET"]
pub type SYSRESET_W<'a> = crate::BitWriter<'a, u32, SCTLR_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 1 - SLEEPONEXIT"]
    #[inline(always)]
    pub fn sleeponexit(&self) -> SLEEPONEXIT_R {
        SLEEPONEXIT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SLEEPDEEP"]
    #[inline(always)]
    pub fn sleepdeep(&self) -> SLEEPDEEP_R {
        SLEEPDEEP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WFITOWFE"]
    #[inline(always)]
    pub fn wfitowfe(&self) -> WFITOWFE_R {
        WFITOWFE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SEVONPEND"]
    #[inline(always)]
    pub fn sevonpend(&self) -> SEVONPEND_R {
        SEVONPEND_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SETEVENT"]
    #[inline(always)]
    pub fn setevent(&self) -> SETEVENT_R {
        SETEVENT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 31 - SYSRESET"]
    #[inline(always)]
    pub fn sysreset(&self) -> SYSRESET_R {
        SYSRESET_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SLEEPONEXIT"]
    #[inline(always)]
    pub fn sleeponexit(&mut self) -> SLEEPONEXIT_W {
        SLEEPONEXIT_W::new(self)
    }
    #[doc = "Bit 2 - SLEEPDEEP"]
    #[inline(always)]
    pub fn sleepdeep(&mut self) -> SLEEPDEEP_W {
        SLEEPDEEP_W::new(self)
    }
    #[doc = "Bit 3 - WFITOWFE"]
    #[inline(always)]
    pub fn wfitowfe(&mut self) -> WFITOWFE_W {
        WFITOWFE_W::new(self)
    }
    #[doc = "Bit 4 - SEVONPEND"]
    #[inline(always)]
    pub fn sevonpend(&mut self) -> SEVONPEND_W {
        SEVONPEND_W::new(self)
    }
    #[doc = "Bit 5 - SETEVENT"]
    #[inline(always)]
    pub fn setevent(&mut self) -> SETEVENT_W {
        SETEVENT_W::new(self)
    }
    #[doc = "Bit 31 - SYSRESET"]
    #[inline(always)]
    pub fn sysreset(&mut self) -> SYSRESET_W {
        SYSRESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctlr](index.html) module"]
pub struct SCTLR_SPEC;
impl crate::RegisterSpec for SCTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sctlr::R](R) reader structure"]
impl crate::Readable for SCTLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sctlr::W](W) writer structure"]
impl crate::Writable for SCTLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCTLR to value 0"]
impl crate::Resettable for SCTLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}