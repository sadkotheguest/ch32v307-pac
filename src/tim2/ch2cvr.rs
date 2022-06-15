#[doc = "Register `CH2CVR` reader"]
pub struct R(crate::R<CH2CVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH2CVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH2CVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH2CVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH2CVR` writer"]
pub struct W(crate::W<CH2CVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH2CVR_SPEC>;
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
impl From<crate::W<CH2CVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH2CVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH2CVR` reader - Capture/Compare 2 value"]
pub type CH2CVR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CH2CVR` writer - Capture/Compare 2 value"]
pub type CH2CVR_W<'a> = crate::FieldWriter<'a, u32, CH2CVR_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Capture/Compare 2 value"]
    #[inline(always)]
    pub fn ch2cvr(&self) -> CH2CVR_R {
        CH2CVR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/Compare 2 value"]
    #[inline(always)]
    pub fn ch2cvr(&mut self) -> CH2CVR_W {
        CH2CVR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "capture/compare register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2cvr](index.html) module"]
pub struct CH2CVR_SPEC;
impl crate::RegisterSpec for CH2CVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch2cvr::R](R) reader structure"]
impl crate::Readable for CH2CVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch2cvr::W](W) writer structure"]
impl crate::Writable for CH2CVR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH2CVR to value 0"]
impl crate::Resettable for CH2CVR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}