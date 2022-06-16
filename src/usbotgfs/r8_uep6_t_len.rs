#[doc = "Register `R8_UEP6_T_LEN` reader"]
pub struct R(crate::R<R8_UEP6_T_LEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_UEP6_T_LEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_UEP6_T_LEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_UEP6_T_LEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_UEP6_T_LEN` writer"]
pub struct W(crate::W<R8_UEP6_T_LEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_UEP6_T_LEN_SPEC>;
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
impl From<crate::W<R8_UEP6_T_LEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_UEP6_T_LEN_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "endpoint 6 transmittal length\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_uep6_t_len](index.html) module"]
pub struct R8_UEP6_T_LEN_SPEC;
impl crate::RegisterSpec for R8_UEP6_T_LEN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_uep6_t_len::R](R) reader structure"]
impl crate::Readable for R8_UEP6_T_LEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_uep6_t_len::W](W) writer structure"]
impl crate::Writable for R8_UEP6_T_LEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_UEP6_T_LEN to value 0"]
impl crate::Resettable for R8_UEP6_T_LEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}