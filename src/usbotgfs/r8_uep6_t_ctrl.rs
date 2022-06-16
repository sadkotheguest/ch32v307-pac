#[doc = "Register `R8_UEP6_T_CTRL` reader"]
pub struct R(crate::R<R8_UEP6_T_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_UEP6_T_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_UEP6_T_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_UEP6_T_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_UEP6_T_CTRL` writer"]
pub struct W(crate::W<R8_UEP6_T_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_UEP6_T_CTRL_SPEC>;
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
impl From<crate::W<R8_UEP6_T_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_UEP6_T_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASK_UEP_T_RES` reader - bit mask of handshake response type for USB endpoint X transmittal (IN)"]
pub type MASK_UEP_T_RES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MASK_UEP_T_RES` writer - bit mask of handshake response type for USB endpoint X transmittal (IN)"]
pub type MASK_UEP_T_RES_W<'a> =
    crate::FieldWriter<'a, u8, R8_UEP6_T_CTRL_SPEC, u8, u8, 2, 0>;
#[doc = "Field `USBHD_UEP_T_TOG___USBHD_UH_T_TOG` reader - prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1"]
pub type USBHD_UEP_T_TOG___USBHD_UH_T_TOG_R = crate::BitReader<bool>;
#[doc = "Field `USBHD_UEP_T_TOG___USBHD_UH_T_TOG` writer - prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1"]
pub type USBHD_UEP_T_TOG___USBHD_UH_T_TOG_W<'a> =
    crate::BitWriter<'a, u8, R8_UEP6_T_CTRL_SPEC, bool, 2>;
#[doc = "Field `USBHD_UEP_AUTO_TOG__USBHD_UH_T_AUTO_TOG` reader - enable automatic toggle after successful transfer completion on endpoint 1/2/3: 0=manual toggle, 1=automatic toggle"]
pub type USBHD_UEP_AUTO_TOG__USBHD_UH_T_AUTO_TOG_R = crate::BitReader<bool>;
#[doc = "Field `USBHD_UEP_AUTO_TOG__USBHD_UH_T_AUTO_TOG` writer - enable automatic toggle after successful transfer completion on endpoint 1/2/3: 0=manual toggle, 1=automatic toggle"]
pub type USBHD_UEP_AUTO_TOG__USBHD_UH_T_AUTO_TOG_W<'a> =
    crate::BitWriter<'a, u8, R8_UEP6_T_CTRL_SPEC, bool, 3>;
impl R {
    #[doc = "Bits 0:1 - bit mask of handshake response type for USB endpoint X transmittal (IN)"]
    #[inline(always)]
    pub fn mask_uep_t_res(&self) -> MASK_UEP_T_RES_R {
        MASK_UEP_T_RES_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1"]
    #[inline(always)]
    pub fn usbhd_uep_t_tog___usbhd_uh_t_tog(&self) -> USBHD_UEP_T_TOG___USBHD_UH_T_TOG_R {
        USBHD_UEP_T_TOG___USBHD_UH_T_TOG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable automatic toggle after successful transfer completion on endpoint 1/2/3: 0=manual toggle, 1=automatic toggle"]
    #[inline(always)]
    pub fn usbhd_uep_auto_tog__usbhd_uh_t_auto_tog(
        &self,
    ) -> USBHD_UEP_AUTO_TOG__USBHD_UH_T_AUTO_TOG_R {
        USBHD_UEP_AUTO_TOG__USBHD_UH_T_AUTO_TOG_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - bit mask of handshake response type for USB endpoint X transmittal (IN)"]
    #[inline(always)]
    pub fn mask_uep_t_res(&mut self) -> MASK_UEP_T_RES_W {
        MASK_UEP_T_RES_W::new(self)
    }
    #[doc = "Bit 2 - prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1"]
    #[inline(always)]
    pub fn usbhd_uep_t_tog___usbhd_uh_t_tog(
        &mut self,
    ) -> USBHD_UEP_T_TOG___USBHD_UH_T_TOG_W {
        USBHD_UEP_T_TOG___USBHD_UH_T_TOG_W::new(self)
    }
    #[doc = "Bit 3 - enable automatic toggle after successful transfer completion on endpoint 1/2/3: 0=manual toggle, 1=automatic toggle"]
    #[inline(always)]
    pub fn usbhd_uep_auto_tog__usbhd_uh_t_auto_tog(
        &mut self,
    ) -> USBHD_UEP_AUTO_TOG__USBHD_UH_T_AUTO_TOG_W {
        USBHD_UEP_AUTO_TOG__USBHD_UH_T_AUTO_TOG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "endpoint 6 control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_uep6_t_ctrl](index.html) module"]
pub struct R8_UEP6_T_CTRL_SPEC;
impl crate::RegisterSpec for R8_UEP6_T_CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_uep6_t_ctrl::R](R) reader structure"]
impl crate::Readable for R8_UEP6_T_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_uep6_t_ctrl::W](W) writer structure"]
impl crate::Writable for R8_UEP6_T_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_UEP6_T_CTRL to value 0"]
impl crate::Resettable for R8_UEP6_T_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}