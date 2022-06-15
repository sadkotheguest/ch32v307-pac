#[doc = "Register `RSQR1` reader"]
pub struct R(crate::R<RSQR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSQR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSQR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSQR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSQR1` writer"]
pub struct W(crate::W<RSQR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSQR1_SPEC>;
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
impl From<crate::W<RSQR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSQR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RLEN` reader - Regular channel sequence length"]
pub type RLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RLEN` writer - Regular channel sequence length"]
pub type RLEN_W<'a> = crate::FieldWriter<'a, u32, RSQR1_SPEC, u8, u8, 4, 20>;
#[doc = "Field `RSQ16` reader - 16th conversion in regular sequence"]
pub type RSQ16_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSQ16` writer - 16th conversion in regular sequence"]
pub type RSQ16_W<'a> = crate::FieldWriter<'a, u32, RSQR1_SPEC, u8, u8, 5, 15>;
#[doc = "Field `RSQ15` reader - 15th conversion in regular sequence"]
pub type RSQ15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSQ15` writer - 15th conversion in regular sequence"]
pub type RSQ15_W<'a> = crate::FieldWriter<'a, u32, RSQR1_SPEC, u8, u8, 5, 10>;
#[doc = "Field `RSQ14` reader - 14th conversion in regular sequence"]
pub type RSQ14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSQ14` writer - 14th conversion in regular sequence"]
pub type RSQ14_W<'a> = crate::FieldWriter<'a, u32, RSQR1_SPEC, u8, u8, 5, 5>;
#[doc = "Field `RSQ13` reader - 13th conversion in regular sequence"]
pub type RSQ13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSQ13` writer - 13th conversion in regular sequence"]
pub type RSQ13_W<'a> = crate::FieldWriter<'a, u32, RSQR1_SPEC, u8, u8, 5, 0>;
impl R {
    #[doc = "Bits 20:23 - Regular channel sequence length"]
    #[inline(always)]
    pub fn rlen(&self) -> RLEN_R {
        RLEN_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 15:19 - 16th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq16(&self) -> RSQ16_R {
        RSQ16_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 15th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq15(&self) -> RSQ15_R {
        RSQ15_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 14th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq14(&self) -> RSQ14_R {
        RSQ14_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - 13th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq13(&self) -> RSQ13_R {
        RSQ13_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 20:23 - Regular channel sequence length"]
    #[inline(always)]
    pub fn rlen(&mut self) -> RLEN_W {
        RLEN_W::new(self)
    }
    #[doc = "Bits 15:19 - 16th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq16(&mut self) -> RSQ16_W {
        RSQ16_W::new(self)
    }
    #[doc = "Bits 10:14 - 15th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq15(&mut self) -> RSQ15_W {
        RSQ15_W::new(self)
    }
    #[doc = "Bits 5:9 - 14th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq14(&mut self) -> RSQ14_W {
        RSQ14_W::new(self)
    }
    #[doc = "Bits 0:4 - 13th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq13(&mut self) -> RSQ13_W {
        RSQ13_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "regular sequence register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsqr1](index.html) module"]
pub struct RSQR1_SPEC;
impl crate::RegisterSpec for RSQR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rsqr1::R](R) reader structure"]
impl crate::Readable for RSQR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rsqr1::W](W) writer structure"]
impl crate::Writable for RSQR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSQR1 to value 0"]
impl crate::Resettable for RSQR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}