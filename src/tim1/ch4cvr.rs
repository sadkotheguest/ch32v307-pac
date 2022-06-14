#[doc = "Register `CH4CVR` reader"]
pub struct R(crate::R<CH4CVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH4CVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH4CVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH4CVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH4CVR` writer"]
pub struct W(crate::W<CH4CVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH4CVR_SPEC>;
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
impl From<crate::W<CH4CVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH4CVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH4CVR` reader - Capture/Compare value"]
pub type CH4CVR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CH4CVR` writer - Capture/Compare value"]
pub type CH4CVR_W<'a> = crate::FieldWriter<'a, u32, CH4CVR_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Capture/Compare value"]
    #[inline(always)]
    pub fn ch4cvr(&self) -> CH4CVR_R {
        CH4CVR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/Compare value"]
    #[inline(always)]
    pub fn ch4cvr(&mut self) -> CH4CVR_W {
        CH4CVR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "capture/compare register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4cvr](index.html) module"]
pub struct CH4CVR_SPEC;
impl crate::RegisterSpec for CH4CVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch4cvr::R](R) reader structure"]
impl crate::Readable for CH4CVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch4cvr::W](W) writer structure"]
impl crate::Writable for CH4CVR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH4CVR to value 0"]
impl crate::Resettable for CH4CVR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}