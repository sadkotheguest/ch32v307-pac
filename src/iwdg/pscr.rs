#[doc = "Register `PSCR` reader"]
pub struct R(crate::R<PSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSCR` writer"]
pub struct W(crate::W<PSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSCR_SPEC>;
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
impl From<crate::W<PSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PR` reader - Prescaler divider"]
pub type PR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PR` writer - Prescaler divider"]
pub type PR_W<'a> = crate::FieldWriter<'a, u32, PSCR_SPEC, u8, u8, 3, 0>;
impl R {
    #[doc = "Bits 0:2 - Prescaler divider"]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Prescaler divider"]
    #[inline(always)]
    pub fn pr(&mut self) -> PR_W {
        PR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Prescaler register (IWDG_PSCR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pscr](index.html) module"]
pub struct PSCR_SPEC;
impl crate::RegisterSpec for PSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pscr::R](R) reader structure"]
impl crate::Readable for PSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pscr::W](W) writer structure"]
impl crate::Writable for PSCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PSCR to value 0"]
impl crate::Resettable for PSCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}