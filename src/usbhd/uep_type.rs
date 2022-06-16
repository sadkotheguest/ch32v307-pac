#[doc = "Register `UEP_TYPE` reader"]
pub struct R(crate::R<UEP_TYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UEP_TYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UEP_TYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UEP_TYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UEP_TYPE` writer"]
pub struct W(crate::W<UEP_TYPE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UEP_TYPE_SPEC>;
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
impl From<crate::W<UEP_TYPE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UEP_TYPE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `bUEP_R_TYPE` reader - endpoint RX type"]
pub type BUEP_R_TYPE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `bUEP_R_TYPE` writer - endpoint RX type"]
pub type BUEP_R_TYPE_W<'a> = crate::FieldWriter<'a, u32, UEP_TYPE_SPEC, u16, u16, 16, 16>;
#[doc = "Field `bUEP_T_TYPE` reader - endpoint TX type"]
pub type BUEP_T_TYPE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `bUEP_T_TYPE` writer - endpoint TX type"]
pub type BUEP_T_TYPE_W<'a> = crate::FieldWriter<'a, u32, UEP_TYPE_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 16:31 - endpoint RX type"]
    #[inline(always)]
    pub fn b_uep_r_type(&self) -> BUEP_R_TYPE_R {
        BUEP_R_TYPE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - endpoint TX type"]
    #[inline(always)]
    pub fn b_uep_t_type(&self) -> BUEP_T_TYPE_R {
        BUEP_T_TYPE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - endpoint RX type"]
    #[inline(always)]
    pub fn b_uep_r_type(&mut self) -> BUEP_R_TYPE_W {
        BUEP_R_TYPE_W::new(self)
    }
    #[doc = "Bits 0:15 - endpoint TX type"]
    #[inline(always)]
    pub fn b_uep_t_type(&mut self) -> BUEP_T_TYPE_W {
        BUEP_T_TYPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB endpoint type\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uep_type](index.html) module"]
pub struct UEP_TYPE_SPEC;
impl crate::RegisterSpec for UEP_TYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uep_type::R](R) reader structure"]
impl crate::Readable for UEP_TYPE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uep_type::W](W) writer structure"]
impl crate::Writable for UEP_TYPE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UEP_TYPE to value 0"]
impl crate::Resettable for UEP_TYPE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}