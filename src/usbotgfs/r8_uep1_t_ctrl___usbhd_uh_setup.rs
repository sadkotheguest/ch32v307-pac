#[doc = "Register `R8_UEP1_T_CTRL___USBHD_UH_SETUP` reader"]
pub struct R(crate::R<R8_UEP1_T_CTRL___USBHD_UH_SETUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R8_UEP1_T_CTRL___USBHD_UH_SETUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R8_UEP1_T_CTRL___USBHD_UH_SETUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R8_UEP1_T_CTRL___USBHD_UH_SETUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R8_UEP1_T_CTRL___USBHD_UH_SETUP` writer"]
pub struct W(crate::W<R8_UEP1_T_CTRL___USBHD_UH_SETUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R8_UEP1_T_CTRL___USBHD_UH_SETUP_SPEC>;
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
impl From<crate::W<R8_UEP1_T_CTRL___USBHD_UH_SETUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R8_UEP1_T_CTRL___USBHD_UH_SETUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASK_UEP_T_RES` reader - bit mask of handshake response type for USB endpoint X transmittal (IN)"]
pub type MASK_UEP_T_RES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MASK_UEP_T_RES` writer - bit mask of handshake response type for USB endpoint X transmittal (IN)"]
pub type MASK_UEP_T_RES_W<'a> =
    crate::FieldWriter<'a, u8, R8_UEP1_T_CTRL___USBHD_UH_SETUP_SPEC, u8, u8, 2, 0>;
#[doc = "Field `USBHD_UEP_T_TOG_` reader - prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1"]
pub type USBHD_UEP_T_TOG__R = crate::BitReader<bool>;
#[doc = "Field `USBHD_UEP_T_TOG_` writer - prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1"]
pub type USBHD_UEP_T_TOG__W<'a> =
    crate::BitWriter<'a, u8, R8_UEP1_T_CTRL___USBHD_UH_SETUP_SPEC, bool, 2>;
#[doc = "Field `USBHD_UEP_AUTO_TOG` reader - enable automatic toggle after successful transfer completion on endpoint 1/2/3: 0=manual toggle, 1=automatic toggle"]
pub type USBHD_UEP_AUTO_TOG_R = crate::BitReader<bool>;
#[doc = "Field `USBHD_UEP_AUTO_TOG` writer - enable automatic toggle after successful transfer completion on endpoint 1/2/3: 0=manual toggle, 1=automatic toggle"]
pub type USBHD_UEP_AUTO_TOG_W<'a> =
    crate::BitWriter<'a, u8, R8_UEP1_T_CTRL___USBHD_UH_SETUP_SPEC, bool, 3>;
#[doc = "Field `USBHD_UH_SOF_EN` reader - USB host automatic SOF enable"]
pub type USBHD_UH_SOF_EN_R = crate::BitReader<bool>;
#[doc = "Field `USBHD_UH_SOF_EN` writer - USB host automatic SOF enable"]
pub type USBHD_UH_SOF_EN_W<'a> =
    crate::BitWriter<'a, u8, R8_UEP1_T_CTRL___USBHD_UH_SETUP_SPEC, bool, 6>;
#[doc = "Field `USBHD_UH_PRE_PID_EN` reader - USB host PRE PID enable for low speed device via hub"]
pub type USBHD_UH_PRE_PID_EN_R = crate::BitReader<bool>;
#[doc = "Field `USBHD_UH_PRE_PID_EN` writer - USB host PRE PID enable for low speed device via hub"]
pub type USBHD_UH_PRE_PID_EN_W<'a> =
    crate::BitWriter<'a, u8, R8_UEP1_T_CTRL___USBHD_UH_SETUP_SPEC, bool, 7>;
impl R {
    #[doc = "Bits 0:1 - bit mask of handshake response type for USB endpoint X transmittal (IN)"]
    #[inline(always)]
    pub fn mask_uep_t_res(&self) -> MASK_UEP_T_RES_R {
        MASK_UEP_T_RES_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1"]
    #[inline(always)]
    pub fn usbhd_uep_t_tog_(&self) -> USBHD_UEP_T_TOG__R {
        USBHD_UEP_T_TOG__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable automatic toggle after successful transfer completion on endpoint 1/2/3: 0=manual toggle, 1=automatic toggle"]
    #[inline(always)]
    pub fn usbhd_uep_auto_tog(&self) -> USBHD_UEP_AUTO_TOG_R {
        USBHD_UEP_AUTO_TOG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - USB host automatic SOF enable"]
    #[inline(always)]
    pub fn usbhd_uh_sof_en(&self) -> USBHD_UH_SOF_EN_R {
        USBHD_UH_SOF_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USB host PRE PID enable for low speed device via hub"]
    #[inline(always)]
    pub fn usbhd_uh_pre_pid_en(&self) -> USBHD_UH_PRE_PID_EN_R {
        USBHD_UH_PRE_PID_EN_R::new(((self.bits >> 7) & 1) != 0)
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
    pub fn usbhd_uep_t_tog_(&mut self) -> USBHD_UEP_T_TOG__W {
        USBHD_UEP_T_TOG__W::new(self)
    }
    #[doc = "Bit 3 - enable automatic toggle after successful transfer completion on endpoint 1/2/3: 0=manual toggle, 1=automatic toggle"]
    #[inline(always)]
    pub fn usbhd_uep_auto_tog(&mut self) -> USBHD_UEP_AUTO_TOG_W {
        USBHD_UEP_AUTO_TOG_W::new(self)
    }
    #[doc = "Bit 6 - USB host automatic SOF enable"]
    #[inline(always)]
    pub fn usbhd_uh_sof_en(&mut self) -> USBHD_UH_SOF_EN_W {
        USBHD_UH_SOF_EN_W::new(self)
    }
    #[doc = "Bit 7 - USB host PRE PID enable for low speed device via hub"]
    #[inline(always)]
    pub fn usbhd_uh_pre_pid_en(&mut self) -> USBHD_UH_PRE_PID_EN_W {
        USBHD_UH_PRE_PID_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "endpoint 1 control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r8_uep1_t_ctrl___usbhd_uh_setup](index.html) module"]
pub struct R8_UEP1_T_CTRL___USBHD_UH_SETUP_SPEC;
impl crate::RegisterSpec for R8_UEP1_T_CTRL___USBHD_UH_SETUP_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r8_uep1_t_ctrl___usbhd_uh_setup::R](R) reader structure"]
impl crate::Readable for R8_UEP1_T_CTRL___USBHD_UH_SETUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r8_uep1_t_ctrl___usbhd_uh_setup::W](W) writer structure"]
impl crate::Writable for R8_UEP1_T_CTRL___USBHD_UH_SETUP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R8_UEP1_T_CTRL___USBHD_UH_SETUP to value 0"]
impl crate::Resettable for R8_UEP1_T_CTRL___USBHD_UH_SETUP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}