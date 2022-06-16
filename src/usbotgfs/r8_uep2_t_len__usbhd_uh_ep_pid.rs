#[doc = "Register `R8_UEP2_T_LEN__USBHD_UH_EP_PID` reader"]
pub struct R(crate::R<R8_UEP2_T_LEN__USBHD_UH_EP_PID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_UEP2_T_LEN__USBHD_UH_EP_PID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_UEP2_T_LEN__USBHD_UH_EP_PID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_UEP2_T_LEN__USBHD_UH_EP_PID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_UEP2_T_LEN__USBHD_UH_EP_PID` writer"]
pub struct W(crate::W<R8_UEP2_T_LEN__USBHD_UH_EP_PID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_UEP2_T_LEN__USBHD_UH_EP_PID_SPEC>;
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
impl From<crate::W<R8_UEP2_T_LEN__USBHD_UH_EP_PID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_UEP2_T_LEN__USBHD_UH_EP_PID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBHD_UH_ENDP_MASK` reader - bit mask of endpoint number for USB host transfer"]
pub type USBHD_UH_ENDP_MASK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USBHD_UH_ENDP_MASK` writer - bit mask of endpoint number for USB host transfer"]
pub type USBHD_UH_ENDP_MASK_W<'a> =
    crate::FieldWriter<'a, u8, R8_UEP2_T_LEN__USBHD_UH_EP_PID_SPEC, u8, u8, 4, 0>;
#[doc = "Field `USBHD_UH_TOKEN_MASK` reader - bit mask of token PID for USB host transfer"]
pub type USBHD_UH_TOKEN_MASK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USBHD_UH_TOKEN_MASK` writer - bit mask of token PID for USB host transfer"]
pub type USBHD_UH_TOKEN_MASK_W<'a> =
    crate::FieldWriter<'a, u8, R8_UEP2_T_LEN__USBHD_UH_EP_PID_SPEC, u8, u8, 4, 4>;
impl R {
    #[doc = "Bits 0:3 - bit mask of endpoint number for USB host transfer"]
    #[inline(always)]
    pub fn usbhd_uh_endp_mask(&self) -> USBHD_UH_ENDP_MASK_R {
        USBHD_UH_ENDP_MASK_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - bit mask of token PID for USB host transfer"]
    #[inline(always)]
    pub fn usbhd_uh_token_mask(&self) -> USBHD_UH_TOKEN_MASK_R {
        USBHD_UH_TOKEN_MASK_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - bit mask of endpoint number for USB host transfer"]
    #[inline(always)]
    pub fn usbhd_uh_endp_mask(&mut self) -> USBHD_UH_ENDP_MASK_W {
        USBHD_UH_ENDP_MASK_W::new(self)
    }
    #[doc = "Bits 4:7 - bit mask of token PID for USB host transfer"]
    #[inline(always)]
    pub fn usbhd_uh_token_mask(&mut self) -> USBHD_UH_TOKEN_MASK_W {
        USBHD_UH_TOKEN_MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "endpoint 2 transmittal length\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_uep2_t_len__usbhd_uh_ep_pid](index.html) module"]
pub struct R8_UEP2_T_LEN__USBHD_UH_EP_PID_SPEC;
impl crate::RegisterSpec for R8_UEP2_T_LEN__USBHD_UH_EP_PID_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_uep2_t_len__usbhd_uh_ep_pid::R](R) reader structure"]
impl crate::Readable for R8_UEP2_T_LEN__USBHD_UH_EP_PID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_uep2_t_len__usbhd_uh_ep_pid::W](W) writer structure"]
impl crate::Writable for R8_UEP2_T_LEN__USBHD_UH_EP_PID_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_UEP2_T_LEN__USBHD_UH_EP_PID to value 0"]
impl crate::Resettable for R8_UEP2_T_LEN__USBHD_UH_EP_PID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}