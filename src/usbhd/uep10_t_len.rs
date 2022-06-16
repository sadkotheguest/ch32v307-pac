#[doc = "Register `UEP10_T_LEN` reader"]
pub struct R(crate::R<UEP10_T_LEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UEP10_T_LEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UEP10_T_LEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UEP10_T_LEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UEP10_T_LEN` writer"]
pub struct W(crate::W<UEP10_T_LEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UEP10_T_LEN_SPEC>;
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
impl From<crate::W<UEP10_T_LEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UEP10_T_LEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UEP10_T_LEN` reader - endpoint 10 send the length"]
pub type UEP10_T_LEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `UEP10_T_LEN` writer - endpoint 10 send the length"]
pub type UEP10_T_LEN_W<'a> = crate::FieldWriter<'a, u16, UEP10_T_LEN_SPEC, u16, u16, 11, 0>;
impl R {
    #[doc = "Bits 0:10 - endpoint 10 send the length"]
    #[inline(always)]
    pub fn uep10_t_len(&self) -> UEP10_T_LEN_R {
        UEP10_T_LEN_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - endpoint 10 send the length"]
    #[inline(always)]
    pub fn uep10_t_len(&mut self) -> UEP10_T_LEN_W {
        UEP10_T_LEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "endpoint 10 send the length\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uep10_t_len](index.html) module"]
pub struct UEP10_T_LEN_SPEC;
impl crate::RegisterSpec for UEP10_T_LEN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [uep10_t_len::R](R) reader structure"]
impl crate::Readable for UEP10_T_LEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uep10_t_len::W](W) writer structure"]
impl crate::Writable for UEP10_T_LEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UEP10_T_LEN to value 0"]
impl crate::Resettable for UEP10_T_LEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}