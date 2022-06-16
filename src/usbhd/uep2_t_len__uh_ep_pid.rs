#[doc = "Register `UEP2_T_LEN__UH_EP_PID` reader"]
pub struct R(crate::R<UEP2_T_LEN__UH_EP_PID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UEP2_T_LEN__UH_EP_PID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UEP2_T_LEN__UH_EP_PID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UEP2_T_LEN__UH_EP_PID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UEP2_T_LEN__UH_EP_PID` writer"]
pub struct W(crate::W<UEP2_T_LEN__UH_EP_PID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UEP2_T_LEN__UH_EP_PID_SPEC>;
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
impl From<crate::W<UEP2_T_LEN__UH_EP_PID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UEP2_T_LEN__UH_EP_PID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UEP2_T_LEN__MASK_UH_ENDP__MASK_UH_TOKEN` reader - endpoint 2 send the length"]
pub type UEP2_T_LEN__MASK_UH_ENDP__MASK_UH_TOKEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `UEP2_T_LEN__MASK_UH_ENDP__MASK_UH_TOKEN` writer - endpoint 2 send the length"]
pub type UEP2_T_LEN__MASK_UH_ENDP__MASK_UH_TOKEN_W<'a> =
    crate::FieldWriter<'a, u16, UEP2_T_LEN__UH_EP_PID_SPEC, u16, u16, 11, 0>;
impl R {
    #[doc = "Bits 0:10 - endpoint 2 send the length"]
    #[inline(always)]
    pub fn uep2_t_len__mask_uh_endp__mask_uh_token(
        &self,
    ) -> UEP2_T_LEN__MASK_UH_ENDP__MASK_UH_TOKEN_R {
        UEP2_T_LEN__MASK_UH_ENDP__MASK_UH_TOKEN_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - endpoint 2 send the length"]
    #[inline(always)]
    pub fn uep2_t_len__mask_uh_endp__mask_uh_token(
        &mut self,
    ) -> UEP2_T_LEN__MASK_UH_ENDP__MASK_UH_TOKEN_W {
        UEP2_T_LEN__MASK_UH_ENDP__MASK_UH_TOKEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "endpoint 2 send the length\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uep2_t_len__uh_ep_pid](index.html) module"]
pub struct UEP2_T_LEN__UH_EP_PID_SPEC;
impl crate::RegisterSpec for UEP2_T_LEN__UH_EP_PID_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [uep2_t_len__uh_ep_pid::R](R) reader structure"]
impl crate::Readable for UEP2_T_LEN__UH_EP_PID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uep2_t_len__uh_ep_pid::W](W) writer structure"]
impl crate::Writable for UEP2_T_LEN__UH_EP_PID_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UEP2_T_LEN__UH_EP_PID to value 0"]
impl crate::Resettable for UEP2_T_LEN__UH_EP_PID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}