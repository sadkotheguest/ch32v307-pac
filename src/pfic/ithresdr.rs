#[doc = "Register `ITHRESDR` reader"]
pub struct R(crate::R<ITHRESDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITHRESDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITHRESDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITHRESDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ITHRESDR` writer"]
pub struct W(crate::W<ITHRESDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ITHRESDR_SPEC>;
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
impl From<crate::W<ITHRESDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ITHRESDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `THRESHOLD` reader - THRESHOLD"]
pub type THRESHOLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `THRESHOLD` writer - THRESHOLD"]
pub type THRESHOLD_W<'a> = crate::FieldWriter<'a, u32, ITHRESDR_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - THRESHOLD"]
    #[inline(always)]
    pub fn threshold(&self) -> THRESHOLD_R {
        THRESHOLD_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - THRESHOLD"]
    #[inline(always)]
    pub fn threshold(&mut self) -> THRESHOLD_W {
        THRESHOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ithresdr](index.html) module"]
pub struct ITHRESDR_SPEC;
impl crate::RegisterSpec for ITHRESDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ithresdr::R](R) reader structure"]
impl crate::Readable for ITHRESDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ithresdr::W](W) writer structure"]
impl crate::Writable for ITHRESDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ITHRESDR to value 0"]
impl crate::Resettable for ITHRESDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}