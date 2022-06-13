#[doc = "Register `IDATAR` reader"]
pub struct R(crate::R<IDATAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDATAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDATAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDATAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IDATAR` writer"]
pub struct W(crate::W<IDATAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDATAR_SPEC>;
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
impl From<crate::W<IDATAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDATAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDR` reader - Independent Data register"]
pub type IDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IDR` writer - Independent Data register"]
pub type IDR_W<'a> = crate::FieldWriter<'a, u32, IDATAR_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - Independent Data register"]
    #[inline(always)]
    pub fn idr(&self) -> IDR_R {
        IDR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Independent Data register"]
    #[inline(always)]
    pub fn idr(&mut self) -> IDR_W {
        IDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Independent Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idatar](index.html) module"]
pub struct IDATAR_SPEC;
impl crate::RegisterSpec for IDATAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idatar::R](R) reader structure"]
impl crate::Readable for IDATAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [idatar::W](W) writer structure"]
impl crate::Writable for IDATAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IDATAR to value 0"]
impl crate::Resettable for IDATAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}