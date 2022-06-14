#[doc = "Register `ATRLR` reader"]
pub struct R(crate::R<ATRLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ATRLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ATRLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ATRLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ATRLR` writer"]
pub struct W(crate::W<ATRLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ATRLR_SPEC>;
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
impl From<crate::W<ATRLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ATRLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ATRLR` reader - Auto-reload value"]
pub type ATRLR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ATRLR` writer - Auto-reload value"]
pub type ATRLR_W<'a> = crate::FieldWriter<'a, u32, ATRLR_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Auto-reload value"]
    #[inline(always)]
    pub fn atrlr(&self) -> ATRLR_R {
        ATRLR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Auto-reload value"]
    #[inline(always)]
    pub fn atrlr(&mut self) -> ATRLR_W {
        ATRLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "auto-reload register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [atrlr](index.html) module"]
pub struct ATRLR_SPEC;
impl crate::RegisterSpec for ATRLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [atrlr::R](R) reader structure"]
impl crate::Readable for ATRLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [atrlr::W](W) writer structure"]
impl crate::Writable for ATRLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ATRLR to value 0"]
impl crate::Resettable for ATRLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}