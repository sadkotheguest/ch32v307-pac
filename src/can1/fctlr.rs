#[doc = "Register `FCTLR` reader"]
pub struct R(crate::R<FCTLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCTLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCTLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCTLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCTLR` writer"]
pub struct W(crate::W<FCTLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCTLR_SPEC>;
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
impl From<crate::W<FCTLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCTLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FINIT` reader - FINIT"]
pub type FINIT_R = crate::BitReader<bool>;
#[doc = "Field `FINIT` writer - FINIT"]
pub type FINIT_W<'a> = crate::BitWriter<'a, u32, FCTLR_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - FINIT"]
    #[inline(always)]
    pub fn finit(&self) -> FINIT_R {
        FINIT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FINIT"]
    #[inline(always)]
    pub fn finit(&mut self) -> FINIT_W {
        FINIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FCTLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fctlr](index.html) module"]
pub struct FCTLR_SPEC;
impl crate::RegisterSpec for FCTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fctlr::R](R) reader structure"]
impl crate::Readable for FCTLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fctlr::W](W) writer structure"]
impl crate::Writable for FCTLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCTLR to value 0"]
impl crate::Resettable for FCTLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}