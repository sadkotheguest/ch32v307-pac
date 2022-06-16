#[doc = "Register `TPCTLR` reader"]
pub struct R(crate::R<TPCTLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TPCTLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TPCTLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TPCTLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TPCTLR` writer"]
pub struct W(crate::W<TPCTLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TPCTLR_SPEC>;
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
impl From<crate::W<TPCTLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TPCTLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TPE` reader - Tamper pin enable"]
pub type TPE_R = crate::BitReader<bool>;
#[doc = "Field `TPE` writer - Tamper pin enable"]
pub type TPE_W<'a> = crate::BitWriter<'a, u32, TPCTLR_SPEC, bool, 0>;
#[doc = "Field `TPAL` reader - Tamper pin active level"]
pub type TPAL_R = crate::BitReader<bool>;
#[doc = "Field `TPAL` writer - Tamper pin active level"]
pub type TPAL_W<'a> = crate::BitWriter<'a, u32, TPCTLR_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - Tamper pin enable"]
    #[inline(always)]
    pub fn tpe(&self) -> TPE_R {
        TPE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tamper pin active level"]
    #[inline(always)]
    pub fn tpal(&self) -> TPAL_R {
        TPAL_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper pin enable"]
    #[inline(always)]
    pub fn tpe(&mut self) -> TPE_W {
        TPE_W::new(self)
    }
    #[doc = "Bit 1 - Tamper pin active level"]
    #[inline(always)]
    pub fn tpal(&mut self) -> TPAL_W {
        TPAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Backup control register (BKP_TPCTLR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tpctlr](index.html) module"]
pub struct TPCTLR_SPEC;
impl crate::RegisterSpec for TPCTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tpctlr::R](R) reader structure"]
impl crate::Readable for TPCTLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tpctlr::W](W) writer structure"]
impl crate::Writable for TPCTLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TPCTLR to value 0"]
impl crate::Resettable for TPCTLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}