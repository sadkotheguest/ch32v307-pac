#[doc = "Register `STK_CMPHR` reader"]
pub struct R(crate::R<STK_CMPHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STK_CMPHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STK_CMPHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STK_CMPHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STK_CMPHR` writer"]
pub struct W(crate::W<STK_CMPHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STK_CMPHR_SPEC>;
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
impl From<crate::W<STK_CMPHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STK_CMPHR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPH` reader - CMPH"]
pub type CMPH_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CMPH` writer - CMPH"]
pub type CMPH_W<'a> = crate::FieldWriter<'a, u32, STK_CMPHR_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - CMPH"]
    #[inline(always)]
    pub fn cmph(&self) -> CMPH_R {
        CMPH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CMPH"]
    #[inline(always)]
    pub fn cmph(&mut self) -> CMPH_W {
        CMPH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System compare high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stk_cmphr](index.html) module"]
pub struct STK_CMPHR_SPEC;
impl crate::RegisterSpec for STK_CMPHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stk_cmphr::R](R) reader structure"]
impl crate::Readable for STK_CMPHR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stk_cmphr::W](W) writer structure"]
impl crate::Writable for STK_CMPHR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STK_CMPHR to value 0"]
impl crate::Resettable for STK_CMPHR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}