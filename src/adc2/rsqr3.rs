#[doc = "Register `RSQR3` reader"]
pub struct R(crate::R<RSQR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSQR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSQR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSQR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSQR3` writer"]
pub struct W(crate::W<RSQR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSQR3_SPEC>;
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
impl From<crate::W<RSQR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSQR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SQ6` reader - 6th conversion in regular sequence"]
pub type SQ6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ6` writer - 6th conversion in regular sequence"]
pub type SQ6_W<'a> = crate::FieldWriter<'a, u32, RSQR3_SPEC, u8, u8, 5, 25>;
#[doc = "Field `SQ5` reader - 5th conversion in regular sequence"]
pub type SQ5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ5` writer - 5th conversion in regular sequence"]
pub type SQ5_W<'a> = crate::FieldWriter<'a, u32, RSQR3_SPEC, u8, u8, 5, 20>;
#[doc = "Field `SQ4` reader - 4th conversion in regular sequence"]
pub type SQ4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ4` writer - 4th conversion in regular sequence"]
pub type SQ4_W<'a> = crate::FieldWriter<'a, u32, RSQR3_SPEC, u8, u8, 5, 15>;
#[doc = "Field `SQ3` reader - 3rd conversion in regular sequence"]
pub type SQ3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ3` writer - 3rd conversion in regular sequence"]
pub type SQ3_W<'a> = crate::FieldWriter<'a, u32, RSQR3_SPEC, u8, u8, 5, 10>;
#[doc = "Field `SQ2` reader - 2nd conversion in regular sequence"]
pub type SQ2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ2` writer - 2nd conversion in regular sequence"]
pub type SQ2_W<'a> = crate::FieldWriter<'a, u32, RSQR3_SPEC, u8, u8, 5, 5>;
#[doc = "Field `SQ1` reader - 1st conversion in regular sequence"]
pub type SQ1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ1` writer - 1st conversion in regular sequence"]
pub type SQ1_W<'a> = crate::FieldWriter<'a, u32, RSQR3_SPEC, u8, u8, 5, 0>;
impl R {
    #[doc = "Bits 25:29 - 6th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq6(&self) -> SQ6_R {
        SQ6_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - 5th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq5(&self) -> SQ5_R {
        SQ5_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - 4th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq4(&self) -> SQ4_R {
        SQ4_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 3rd conversion in regular sequence"]
    #[inline(always)]
    pub fn sq3(&self) -> SQ3_R {
        SQ3_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 2nd conversion in regular sequence"]
    #[inline(always)]
    pub fn sq2(&self) -> SQ2_R {
        SQ2_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - 1st conversion in regular sequence"]
    #[inline(always)]
    pub fn sq1(&self) -> SQ1_R {
        SQ1_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 25:29 - 6th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq6(&mut self) -> SQ6_W {
        SQ6_W::new(self)
    }
    #[doc = "Bits 20:24 - 5th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq5(&mut self) -> SQ5_W {
        SQ5_W::new(self)
    }
    #[doc = "Bits 15:19 - 4th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq4(&mut self) -> SQ4_W {
        SQ4_W::new(self)
    }
    #[doc = "Bits 10:14 - 3rd conversion in regular sequence"]
    #[inline(always)]
    pub fn sq3(&mut self) -> SQ3_W {
        SQ3_W::new(self)
    }
    #[doc = "Bits 5:9 - 2nd conversion in regular sequence"]
    #[inline(always)]
    pub fn sq2(&mut self) -> SQ2_W {
        SQ2_W::new(self)
    }
    #[doc = "Bits 0:4 - 1st conversion in regular sequence"]
    #[inline(always)]
    pub fn sq1(&mut self) -> SQ1_W {
        SQ1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "regular sequence register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsqr3](index.html) module"]
pub struct RSQR3_SPEC;
impl crate::RegisterSpec for RSQR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rsqr3::R](R) reader structure"]
impl crate::Readable for RSQR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rsqr3::W](W) writer structure"]
impl crate::Writable for RSQR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSQR3 to value 0"]
impl crate::Resettable for RSQR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}