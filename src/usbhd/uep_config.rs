#[doc = "Register `UEP_CONFIG` reader"]
pub struct R(crate::R<UEP_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UEP_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UEP_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UEP_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UEP_CONFIG` writer"]
pub struct W(crate::W<UEP_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UEP_CONFIG_SPEC>;
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
impl From<crate::W<UEP_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UEP_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `bUEP_R_EN__UH_EP_MOD` reader - endpoint RX enable/bUH_TX_EN"]
pub type BUEP_R_EN__UH_EP_MOD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `bUEP_R_EN__UH_EP_MOD` writer - endpoint RX enable/bUH_TX_EN"]
pub type BUEP_R_EN__UH_EP_MOD_W<'a> =
    crate::FieldWriter<'a, u32, UEP_CONFIG_SPEC, u16, u16, 16, 16>;
#[doc = "Field `bUEP_T_EN/bUH_TX_EN` reader - endpoint TX enable/bUH_TX_EN"]
pub type BUEP_T_ENBUH_TX_EN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `bUEP_T_EN/bUH_TX_EN` writer - endpoint TX enable/bUH_TX_EN"]
pub type BUEP_T_ENBUH_TX_EN_W<'a> =
    crate::FieldWriter<'a, u32, UEP_CONFIG_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 16:31 - endpoint RX enable/bUH_TX_EN"]
    #[inline(always)]
    pub fn b_uep_r_en__uh_ep_mod(&self) -> BUEP_R_EN__UH_EP_MOD_R {
        BUEP_R_EN__UH_EP_MOD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - endpoint TX enable/bUH_TX_EN"]
    #[inline(always)]
    pub fn b_uep_t_enb_uh_tx_en(&self) -> BUEP_T_ENBUH_TX_EN_R {
        BUEP_T_ENBUH_TX_EN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - endpoint RX enable/bUH_TX_EN"]
    #[inline(always)]
    pub fn b_uep_r_en__uh_ep_mod(&mut self) -> BUEP_R_EN__UH_EP_MOD_W {
        BUEP_R_EN__UH_EP_MOD_W::new(self)
    }
    #[doc = "Bits 0:15 - endpoint TX enable/bUH_TX_EN"]
    #[inline(always)]
    pub fn b_uep_t_enb_uh_tx_en(&mut self) -> BUEP_T_ENBUH_TX_EN_W {
        BUEP_T_ENBUH_TX_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB endpoint configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uep_config](index.html) module"]
pub struct UEP_CONFIG_SPEC;
impl crate::RegisterSpec for UEP_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uep_config::R](R) reader structure"]
impl crate::Readable for UEP_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uep_config::W](W) writer structure"]
impl crate::Writable for UEP_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UEP_CONFIG to value 0"]
impl crate::Resettable for UEP_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}