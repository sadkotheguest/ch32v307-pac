#[doc = "Register `RSQR2` reader"]
pub struct R(crate::R<RSQR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSQR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSQR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSQR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSQR2` writer"]
pub struct W(crate::W<RSQR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSQR2_SPEC>;
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
impl From<crate::W<RSQR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSQR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SQ12` reader - 12th conversion in regular sequence"]
pub type SQ12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ12` writer - 12th conversion in regular sequence"]
pub type SQ12_W<'a> = crate::FieldWriter<'a, u32, RSQR2_SPEC, u8, u8, 5, 25>;
#[doc = "Field `SQ11` reader - 11th conversion in regular sequence"]
pub type SQ11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ11` writer - 11th conversion in regular sequence"]
pub type SQ11_W<'a> = crate::FieldWriter<'a, u32, RSQR2_SPEC, u8, u8, 5, 20>;
#[doc = "Field `SQ10` reader - 10th conversion in regular sequence"]
pub type SQ10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ10` writer - 10th conversion in regular sequence"]
pub type SQ10_W<'a> = crate::FieldWriter<'a, u32, RSQR2_SPEC, u8, u8, 5, 15>;
#[doc = "Field `SQ9` reader - 9th conversion in regular sequence"]
pub type SQ9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ9` writer - 9th conversion in regular sequence"]
pub type SQ9_W<'a> = crate::FieldWriter<'a, u32, RSQR2_SPEC, u8, u8, 5, 10>;
#[doc = "Field `SQ8` reader - 8th conversion in regular sequence"]
pub type SQ8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ8` writer - 8th conversion in regular sequence"]
pub type SQ8_W<'a> = crate::FieldWriter<'a, u32, RSQR2_SPEC, u8, u8, 5, 5>;
#[doc = "Field `SQ7` reader - 7th conversion in regular sequence"]
pub type SQ7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ7` writer - 7th conversion in regular sequence"]
pub type SQ7_W<'a> = crate::FieldWriter<'a, u32, RSQR2_SPEC, u8, u8, 5, 0>;
impl R {
    #[doc = "Bits 25:29 - 12th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq12(&self) -> SQ12_R {
        SQ12_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - 11th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq11(&self) -> SQ11_R {
        SQ11_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - 10th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq10(&self) -> SQ10_R {
        SQ10_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 9th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq9(&self) -> SQ9_R {
        SQ9_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 8th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq8(&self) -> SQ8_R {
        SQ8_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - 7th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq7(&self) -> SQ7_R {
        SQ7_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 25:29 - 12th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq12(&mut self) -> SQ12_W {
        SQ12_W::new(self)
    }
    #[doc = "Bits 20:24 - 11th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq11(&mut self) -> SQ11_W {
        SQ11_W::new(self)
    }
    #[doc = "Bits 15:19 - 10th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq10(&mut self) -> SQ10_W {
        SQ10_W::new(self)
    }
    #[doc = "Bits 10:14 - 9th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq9(&mut self) -> SQ9_W {
        SQ9_W::new(self)
    }
    #[doc = "Bits 5:9 - 8th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq8(&mut self) -> SQ8_W {
        SQ8_W::new(self)
    }
    #[doc = "Bits 0:4 - 7th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq7(&mut self) -> SQ7_W {
        SQ7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "regular sequence register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsqr2](index.html) module"]
pub struct RSQR2_SPEC;
impl crate::RegisterSpec for RSQR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rsqr2::R](R) reader structure"]
impl crate::Readable for RSQR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rsqr2::W](W) writer structure"]
impl crate::Writable for RSQR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSQR2 to value 0"]
impl crate::Resettable for RSQR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}