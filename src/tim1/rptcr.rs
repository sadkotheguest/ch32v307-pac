#[doc = "Register `RPTCR` reader"]
pub struct R(crate::R<RPTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RPTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RPTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RPTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RPTCR` writer"]
pub struct W(crate::W<RPTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RPTCR_SPEC>;
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
impl From<crate::W<RPTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RPTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RPTCR` reader - Repetition counter value"]
pub type RPTCR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RPTCR` writer - Repetition counter value"]
pub type RPTCR_W<'a> = crate::FieldWriter<'a, u32, RPTCR_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - Repetition counter value"]
    #[inline(always)]
    pub fn rptcr(&self) -> RPTCR_R {
        RPTCR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Repetition counter value"]
    #[inline(always)]
    pub fn rptcr(&mut self) -> RPTCR_W {
        RPTCR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "repetition counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rptcr](index.html) module"]
pub struct RPTCR_SPEC;
impl crate::RegisterSpec for RPTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rptcr::R](R) reader structure"]
impl crate::Readable for RPTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rptcr::W](W) writer structure"]
impl crate::Writable for RPTCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RPTCR to value 0"]
impl crate::Resettable for RPTCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}