#[doc = "Register `UEP15_MAX_LEN` reader"]
pub struct R(crate::R<UEP15_MAX_LEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UEP15_MAX_LEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UEP15_MAX_LEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UEP15_MAX_LEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UEP15_MAX_LEN` writer"]
pub struct W(crate::W<UEP15_MAX_LEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UEP15_MAX_LEN_SPEC>;
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
impl From<crate::W<UEP15_MAX_LEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UEP15_MAX_LEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UEP15_MAX_LEN` reader - endpoint 15 max acceptable length"]
pub type UEP15_MAX_LEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `UEP15_MAX_LEN` writer - endpoint 15 max acceptable length"]
pub type UEP15_MAX_LEN_W<'a> =
    crate::FieldWriter<'a, u16, UEP15_MAX_LEN_SPEC, u16, u16, 11, 0>;
impl R {
    #[doc = "Bits 0:10 - endpoint 15 max acceptable length"]
    #[inline(always)]
    pub fn uep15_max_len(&self) -> UEP15_MAX_LEN_R {
        UEP15_MAX_LEN_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - endpoint 15 max acceptable length"]
    #[inline(always)]
    pub fn uep15_max_len(&mut self) -> UEP15_MAX_LEN_W {
        UEP15_MAX_LEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "endpoint 15 max acceptable length\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uep15_max_len](index.html) module"]
pub struct UEP15_MAX_LEN_SPEC;
impl crate::RegisterSpec for UEP15_MAX_LEN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [uep15_max_len::R](R) reader structure"]
impl crate::Readable for UEP15_MAX_LEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uep15_max_len::W](W) writer structure"]
impl crate::Writable for UEP15_MAX_LEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UEP15_MAX_LEN to value 0"]
impl crate::Resettable for UEP15_MAX_LEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}