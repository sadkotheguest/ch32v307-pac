#[doc = "Register `RSQR3__CHANNEL` reader"]
pub struct R(crate::R<RSQR3__CHANNEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSQR3__CHANNEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSQR3__CHANNEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSQR3__CHANNEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSQR3__CHANNEL` writer"]
pub struct W(crate::W<RSQR3__CHANNEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSQR3__CHANNEL_SPEC>;
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
impl From<crate::W<RSQR3__CHANNEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSQR3__CHANNEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSQ6` reader - 6th conversion in regular sequence"]
pub type RSQ6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSQ6` writer - 6th conversion in regular sequence"]
pub type RSQ6_W<'a> = crate::FieldWriter<'a, u32, RSQR3__CHANNEL_SPEC, u8, u8, 5, 25>;
#[doc = "Field `RSQ5` reader - 5th conversion in regular sequence"]
pub type RSQ5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSQ5` writer - 5th conversion in regular sequence"]
pub type RSQ5_W<'a> = crate::FieldWriter<'a, u32, RSQR3__CHANNEL_SPEC, u8, u8, 5, 20>;
#[doc = "Field `RSQ4` reader - 4th conversion in regular sequence"]
pub type RSQ4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSQ4` writer - 4th conversion in regular sequence"]
pub type RSQ4_W<'a> = crate::FieldWriter<'a, u32, RSQR3__CHANNEL_SPEC, u8, u8, 5, 15>;
#[doc = "Field `RSQ3` reader - 3rd conversion in regular sequence"]
pub type RSQ3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSQ3` writer - 3rd conversion in regular sequence"]
pub type RSQ3_W<'a> = crate::FieldWriter<'a, u32, RSQR3__CHANNEL_SPEC, u8, u8, 5, 10>;
#[doc = "Field `RSQ2` reader - 2nd conversion in regular sequence"]
pub type RSQ2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSQ2` writer - 2nd conversion in regular sequence"]
pub type RSQ2_W<'a> = crate::FieldWriter<'a, u32, RSQR3__CHANNEL_SPEC, u8, u8, 5, 5>;
#[doc = "Field `RSQ1__CHSEL` reader - 1st conversion in regular sequence;TKDY_V channel select"]
pub type RSQ1__CHSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSQ1__CHSEL` writer - 1st conversion in regular sequence;TKDY_V channel select"]
pub type RSQ1__CHSEL_W<'a> = crate::FieldWriter<'a, u32, RSQR3__CHANNEL_SPEC, u8, u8, 5, 0>;
impl R {
    #[doc = "Bits 25:29 - 6th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq6(&self) -> RSQ6_R {
        RSQ6_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - 5th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq5(&self) -> RSQ5_R {
        RSQ5_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - 4th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq4(&self) -> RSQ4_R {
        RSQ4_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 3rd conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq3(&self) -> RSQ3_R {
        RSQ3_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 2nd conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq2(&self) -> RSQ2_R {
        RSQ2_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - 1st conversion in regular sequence;TKDY_V channel select"]
    #[inline(always)]
    pub fn rsq1__chsel(&self) -> RSQ1__CHSEL_R {
        RSQ1__CHSEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 25:29 - 6th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq6(&mut self) -> RSQ6_W {
        RSQ6_W::new(self)
    }
    #[doc = "Bits 20:24 - 5th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq5(&mut self) -> RSQ5_W {
        RSQ5_W::new(self)
    }
    #[doc = "Bits 15:19 - 4th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq4(&mut self) -> RSQ4_W {
        RSQ4_W::new(self)
    }
    #[doc = "Bits 10:14 - 3rd conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq3(&mut self) -> RSQ3_W {
        RSQ3_W::new(self)
    }
    #[doc = "Bits 5:9 - 2nd conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq2(&mut self) -> RSQ2_W {
        RSQ2_W::new(self)
    }
    #[doc = "Bits 0:4 - 1st conversion in regular sequence;TKDY_V channel select"]
    #[inline(always)]
    pub fn rsq1__chsel(&mut self) -> RSQ1__CHSEL_W {
        RSQ1__CHSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "regular sequence register 3;TKEY_V_CHANNEL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsqr3__channel](index.html) module"]
pub struct RSQR3__CHANNEL_SPEC;
impl crate::RegisterSpec for RSQR3__CHANNEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rsqr3__channel::R](R) reader structure"]
impl crate::Readable for RSQR3__CHANNEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rsqr3__channel::W](W) writer structure"]
impl crate::Writable for RSQR3__CHANNEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSQR3__CHANNEL to value 0"]
impl crate::Resettable for RSQR3__CHANNEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}