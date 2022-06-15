#[doc = "Register `CH1CVR` reader"]
pub struct R(crate::R<CH1CVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH1CVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH1CVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH1CVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH1CVR` writer"]
pub struct W(crate::W<CH1CVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH1CVR_SPEC>;
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
impl From<crate::W<CH1CVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH1CVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH1CVR` reader - Capture/Compare 1 value"]
pub type CH1CVR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CH1CVR` writer - Capture/Compare 1 value"]
pub type CH1CVR_W<'a> = crate::FieldWriter<'a, u32, CH1CVR_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Capture/Compare 1 value"]
    #[inline(always)]
    pub fn ch1cvr(&self) -> CH1CVR_R {
        CH1CVR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/Compare 1 value"]
    #[inline(always)]
    pub fn ch1cvr(&mut self) -> CH1CVR_W {
        CH1CVR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "capture/compare register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1cvr](index.html) module"]
pub struct CH1CVR_SPEC;
impl crate::RegisterSpec for CH1CVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch1cvr::R](R) reader structure"]
impl crate::Readable for CH1CVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch1cvr::W](W) writer structure"]
impl crate::Writable for CH1CVR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH1CVR to value 0"]
impl crate::Resettable for CH1CVR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}