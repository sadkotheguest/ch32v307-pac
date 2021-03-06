#[doc = "Register `TXMDHR0` reader"]
pub struct R(crate::R<TXMDHR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXMDHR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXMDHR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXMDHR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXMDHR0` writer"]
pub struct W(crate::W<TXMDHR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXMDHR0_SPEC>;
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
impl From<crate::W<TXMDHR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXMDHR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA7` reader - DATA7"]
pub type DATA7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA7` writer - DATA7"]
pub type DATA7_W<'a> = crate::FieldWriter<'a, u32, TXMDHR0_SPEC, u8, u8, 8, 24>;
#[doc = "Field `DATA6` reader - DATA6"]
pub type DATA6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA6` writer - DATA6"]
pub type DATA6_W<'a> = crate::FieldWriter<'a, u32, TXMDHR0_SPEC, u8, u8, 8, 16>;
#[doc = "Field `DATA5` reader - DATA5"]
pub type DATA5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA5` writer - DATA5"]
pub type DATA5_W<'a> = crate::FieldWriter<'a, u32, TXMDHR0_SPEC, u8, u8, 8, 8>;
#[doc = "Field `DATA4` reader - DATA4"]
pub type DATA4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA4` writer - DATA4"]
pub type DATA4_W<'a> = crate::FieldWriter<'a, u32, TXMDHR0_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 24:31 - DATA7"]
    #[inline(always)]
    pub fn data7(&self) -> DATA7_R {
        DATA7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DATA6"]
    #[inline(always)]
    pub fn data6(&self) -> DATA6_R {
        DATA6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DATA5"]
    #[inline(always)]
    pub fn data5(&self) -> DATA5_R {
        DATA5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - DATA4"]
    #[inline(always)]
    pub fn data4(&self) -> DATA4_R {
        DATA4_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - DATA7"]
    #[inline(always)]
    pub fn data7(&mut self) -> DATA7_W {
        DATA7_W::new(self)
    }
    #[doc = "Bits 16:23 - DATA6"]
    #[inline(always)]
    pub fn data6(&mut self) -> DATA6_W {
        DATA6_W::new(self)
    }
    #[doc = "Bits 8:15 - DATA5"]
    #[inline(always)]
    pub fn data5(&mut self) -> DATA5_W {
        DATA5_W::new(self)
    }
    #[doc = "Bits 0:7 - DATA4"]
    #[inline(always)]
    pub fn data4(&mut self) -> DATA4_W {
        DATA4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TXMDHR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txmdhr0](index.html) module"]
pub struct TXMDHR0_SPEC;
impl crate::RegisterSpec for TXMDHR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txmdhr0::R](R) reader structure"]
impl crate::Readable for TXMDHR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txmdhr0::W](W) writer structure"]
impl crate::Writable for TXMDHR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXMDHR0 to value 0"]
impl crate::Resettable for TXMDHR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}