#[doc = "Register `ISQR` reader"]
pub struct R(crate::R<ISQR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISQR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISQR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISQR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISQR` writer"]
pub struct W(crate::W<ISQR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISQR_SPEC>;
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
impl From<crate::W<ISQR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISQR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `JL` reader - Injected sequence length"]
pub type JL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `JL` writer - Injected sequence length"]
pub type JL_W<'a> = crate::FieldWriter<'a, u32, ISQR_SPEC, u8, u8, 2, 20>;
#[doc = "Field `JSQ4` reader - 4th conversion in injected sequence"]
pub type JSQ4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `JSQ4` writer - 4th conversion in injected sequence"]
pub type JSQ4_W<'a> = crate::FieldWriter<'a, u32, ISQR_SPEC, u8, u8, 5, 15>;
#[doc = "Field `JSQ3` reader - 3rd conversion in injected sequence"]
pub type JSQ3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `JSQ3` writer - 3rd conversion in injected sequence"]
pub type JSQ3_W<'a> = crate::FieldWriter<'a, u32, ISQR_SPEC, u8, u8, 5, 10>;
#[doc = "Field `JSQ2` reader - 2nd conversion in injected sequence"]
pub type JSQ2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `JSQ2` writer - 2nd conversion in injected sequence"]
pub type JSQ2_W<'a> = crate::FieldWriter<'a, u32, ISQR_SPEC, u8, u8, 5, 5>;
#[doc = "Field `JSQ1` reader - 1st conversion in injected sequence"]
pub type JSQ1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `JSQ1` writer - 1st conversion in injected sequence"]
pub type JSQ1_W<'a> = crate::FieldWriter<'a, u32, ISQR_SPEC, u8, u8, 5, 0>;
impl R {
    #[doc = "Bits 20:21 - Injected sequence length"]
    #[inline(always)]
    pub fn jl(&self) -> JL_R {
        JL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 15:19 - 4th conversion in injected sequence"]
    #[inline(always)]
    pub fn jsq4(&self) -> JSQ4_R {
        JSQ4_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 3rd conversion in injected sequence"]
    #[inline(always)]
    pub fn jsq3(&self) -> JSQ3_R {
        JSQ3_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 2nd conversion in injected sequence"]
    #[inline(always)]
    pub fn jsq2(&self) -> JSQ2_R {
        JSQ2_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - 1st conversion in injected sequence"]
    #[inline(always)]
    pub fn jsq1(&self) -> JSQ1_R {
        JSQ1_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 20:21 - Injected sequence length"]
    #[inline(always)]
    pub fn jl(&mut self) -> JL_W {
        JL_W::new(self)
    }
    #[doc = "Bits 15:19 - 4th conversion in injected sequence"]
    #[inline(always)]
    pub fn jsq4(&mut self) -> JSQ4_W {
        JSQ4_W::new(self)
    }
    #[doc = "Bits 10:14 - 3rd conversion in injected sequence"]
    #[inline(always)]
    pub fn jsq3(&mut self) -> JSQ3_W {
        JSQ3_W::new(self)
    }
    #[doc = "Bits 5:9 - 2nd conversion in injected sequence"]
    #[inline(always)]
    pub fn jsq2(&mut self) -> JSQ2_W {
        JSQ2_W::new(self)
    }
    #[doc = "Bits 0:4 - 1st conversion in injected sequence"]
    #[inline(always)]
    pub fn jsq1(&mut self) -> JSQ1_W {
        JSQ1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "injected sequence register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isqr](index.html) module"]
pub struct ISQR_SPEC;
impl crate::RegisterSpec for ISQR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isqr::R](R) reader structure"]
impl crate::Readable for ISQR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isqr::W](W) writer structure"]
impl crate::Writable for ISQR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ISQR to value 0"]
impl crate::Resettable for ISQR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}