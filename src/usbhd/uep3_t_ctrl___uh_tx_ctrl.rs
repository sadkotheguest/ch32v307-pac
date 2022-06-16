#[doc = "Register `UEP3_T_CTRL___UH_TX_CTRL` reader"]
pub struct R(crate::R<UEP3_T_CTRL___UH_TX_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UEP3_T_CTRL___UH_TX_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UEP3_T_CTRL___UH_TX_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UEP3_T_CTRL___UH_TX_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UEP3_T_CTRL___UH_TX_CTRL` writer"]
pub struct W(crate::W<UEP3_T_CTRL___UH_TX_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UEP3_T_CTRL___UH_TX_CTRL_SPEC>;
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
impl From<crate::W<UEP3_T_CTRL___UH_TX_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UEP3_T_CTRL___UH_TX_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASK_UEP_T_RES_____MASK_UH_T_RES` reader - endpoint 3 control of the send response to IN transactions"]
pub type MASK_UEP_T_RES_____MASK_UH_T_RES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MASK_UEP_T_RES_____MASK_UH_T_RES` writer - endpoint 3 control of the send response to IN transactions"]
pub type MASK_UEP_T_RES_____MASK_UH_T_RES_W<'a> =
    crate::FieldWriter<'a, u16, UEP3_T_CTRL___UH_TX_CTRL_SPEC, u8, u8, 2, 0>;
#[doc = "Field `bUH_T_RES_NO` reader - bUH_T_RES_NO"]
pub type BUH_T_RES_NO_R = crate::BitReader<bool>;
#[doc = "Field `bUH_T_RES_NO` writer - bUH_T_RES_NO"]
pub type BUH_T_RES_NO_W<'a> =
    crate::BitWriter<'a, u16, UEP3_T_CTRL___UH_TX_CTRL_SPEC, bool, 2>;
#[doc = "Field `MASK_UEP_T_TOG____MASK_UH_T_TOG` reader - endpoint 3 synchronous trigger bit for the sender to prepare"]
pub type MASK_UEP_T_TOG____MASK_UH_T_TOG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MASK_UEP_T_TOG____MASK_UH_T_TOG` writer - endpoint 3 synchronous trigger bit for the sender to prepare"]
pub type MASK_UEP_T_TOG____MASK_UH_T_TOG_W<'a> =
    crate::FieldWriter<'a, u16, UEP3_T_CTRL___UH_TX_CTRL_SPEC, u8, u8, 2, 3>;
#[doc = "Field `bUEP_T_TOG_AUTO____bUH_T_AUTO_TOG` reader - endpoint 3 synchronous trigger bit automatic filp enables the control bit"]
pub type BUEP_T_TOG_AUTO____BUH_T_AUTO_TOG_R = crate::BitReader<bool>;
#[doc = "Field `bUEP_T_TOG_AUTO____bUH_T_AUTO_TOG` writer - endpoint 3 synchronous trigger bit automatic filp enables the control bit"]
pub type BUEP_T_TOG_AUTO____BUH_T_AUTO_TOG_W<'a> =
    crate::BitWriter<'a, u16, UEP3_T_CTRL___UH_TX_CTRL_SPEC, bool, 5>;
#[doc = "Field `bUH_T_DATA_NO` reader - bUH_T_DATA_NO"]
pub type BUH_T_DATA_NO_R = crate::BitReader<bool>;
#[doc = "Field `bUH_T_DATA_NO` writer - bUH_T_DATA_NO"]
pub type BUH_T_DATA_NO_W<'a> =
    crate::BitWriter<'a, u16, UEP3_T_CTRL___UH_TX_CTRL_SPEC, bool, 6>;
impl R {
    #[doc = "Bits 0:1 - endpoint 3 control of the send response to IN transactions"]
    #[inline(always)]
    pub fn mask_uep_t_res_____mask_uh_t_res(&self) -> MASK_UEP_T_RES_____MASK_UH_T_RES_R {
        MASK_UEP_T_RES_____MASK_UH_T_RES_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - bUH_T_RES_NO"]
    #[inline(always)]
    pub fn b_uh_t_res_no(&self) -> BUH_T_RES_NO_R {
        BUH_T_RES_NO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - endpoint 3 synchronous trigger bit for the sender to prepare"]
    #[inline(always)]
    pub fn mask_uep_t_tog____mask_uh_t_tog(&self) -> MASK_UEP_T_TOG____MASK_UH_T_TOG_R {
        MASK_UEP_T_TOG____MASK_UH_T_TOG_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - endpoint 3 synchronous trigger bit automatic filp enables the control bit"]
    #[inline(always)]
    pub fn b_uep_t_tog_auto____b_uh_t_auto_tog(
        &self,
    ) -> BUEP_T_TOG_AUTO____BUH_T_AUTO_TOG_R {
        BUEP_T_TOG_AUTO____BUH_T_AUTO_TOG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - bUH_T_DATA_NO"]
    #[inline(always)]
    pub fn b_uh_t_data_no(&self) -> BUH_T_DATA_NO_R {
        BUH_T_DATA_NO_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - endpoint 3 control of the send response to IN transactions"]
    #[inline(always)]
    pub fn mask_uep_t_res_____mask_uh_t_res(
        &mut self,
    ) -> MASK_UEP_T_RES_____MASK_UH_T_RES_W {
        MASK_UEP_T_RES_____MASK_UH_T_RES_W::new(self)
    }
    #[doc = "Bit 2 - bUH_T_RES_NO"]
    #[inline(always)]
    pub fn b_uh_t_res_no(&mut self) -> BUH_T_RES_NO_W {
        BUH_T_RES_NO_W::new(self)
    }
    #[doc = "Bits 3:4 - endpoint 3 synchronous trigger bit for the sender to prepare"]
    #[inline(always)]
    pub fn mask_uep_t_tog____mask_uh_t_tog(&mut self) -> MASK_UEP_T_TOG____MASK_UH_T_TOG_W {
        MASK_UEP_T_TOG____MASK_UH_T_TOG_W::new(self)
    }
    #[doc = "Bit 5 - endpoint 3 synchronous trigger bit automatic filp enables the control bit"]
    #[inline(always)]
    pub fn b_uep_t_tog_auto____b_uh_t_auto_tog(
        &mut self,
    ) -> BUEP_T_TOG_AUTO____BUH_T_AUTO_TOG_W {
        BUEP_T_TOG_AUTO____BUH_T_AUTO_TOG_W::new(self)
    }
    #[doc = "Bit 6 - bUH_T_DATA_NO"]
    #[inline(always)]
    pub fn b_uh_t_data_no(&mut self) -> BUH_T_DATA_NO_W {
        BUH_T_DATA_NO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "endpoint 3 send control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uep3_t_ctrl___uh_tx_ctrl](index.html) module"]
pub struct UEP3_T_CTRL___UH_TX_CTRL_SPEC;
impl crate::RegisterSpec for UEP3_T_CTRL___UH_TX_CTRL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [uep3_t_ctrl___uh_tx_ctrl::R](R) reader structure"]
impl crate::Readable for UEP3_T_CTRL___UH_TX_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uep3_t_ctrl___uh_tx_ctrl::W](W) writer structure"]
impl crate::Writable for UEP3_T_CTRL___UH_TX_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UEP3_T_CTRL___UH_TX_CTRL to value 0"]
impl crate::Resettable for UEP3_T_CTRL___UH_TX_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}