#[doc = "Register `UEP2_R_CTRL__UH_RX_CTRL` reader"]
pub struct R(crate::R<UEP2_R_CTRL__UH_RX_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UEP2_R_CTRL__UH_RX_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UEP2_R_CTRL__UH_RX_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UEP2_R_CTRL__UH_RX_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UEP2_R_CTRL__UH_RX_CTRL` writer"]
pub struct W(crate::W<UEP2_R_CTRL__UH_RX_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UEP2_R_CTRL__UH_RX_CTRL_SPEC>;
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
impl From<crate::W<UEP2_R_CTRL__UH_RX_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UEP2_R_CTRL__UH_RX_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASK_UEP_R_RES__MASK_UH_R_RES` reader - endpoint 2 control of the accept response to OUT transactions"]
pub type MASK_UEP_R_RES__MASK_UH_R_RES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MASK_UEP_R_RES__MASK_UH_R_RES` writer - endpoint 2 control of the accept response to OUT transactions"]
pub type MASK_UEP_R_RES__MASK_UH_R_RES_W<'a> =
    crate::FieldWriter<'a, u16, UEP2_R_CTRL__UH_RX_CTRL_SPEC, u8, u8, 2, 0>;
#[doc = "Field `bUH_R_RES_NO` reader - bUH_R_RES_NO"]
pub type BUH_R_RES_NO_R = crate::BitReader<bool>;
#[doc = "Field `bUH_R_RES_NO` writer - bUH_R_RES_NO"]
pub type BUH_R_RES_NO_W<'a> =
    crate::BitWriter<'a, u16, UEP2_R_CTRL__UH_RX_CTRL_SPEC, bool, 2>;
#[doc = "Field `MASK_UEP_R_TOG__MASK_UH_R_TOG` reader - endpoint 2 synchronous trigger bit for the accept to prepare"]
pub type MASK_UEP_R_TOG__MASK_UH_R_TOG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MASK_UEP_R_TOG__MASK_UH_R_TOG` writer - endpoint 2 synchronous trigger bit for the accept to prepare"]
pub type MASK_UEP_R_TOG__MASK_UH_R_TOG_W<'a> =
    crate::FieldWriter<'a, u16, UEP2_R_CTRL__UH_RX_CTRL_SPEC, u8, u8, 2, 3>;
#[doc = "Field `bUEP_R_TOG_AUTO__bUH_R_AUTO_TOG` reader - endpoint 2 synchronous trigger bit automatic filp enables the control bit"]
pub type BUEP_R_TOG_AUTO__BUH_R_AUTO_TOG_R = crate::BitReader<bool>;
#[doc = "Field `bUEP_R_TOG_AUTO__bUH_R_AUTO_TOG` writer - endpoint 2 synchronous trigger bit automatic filp enables the control bit"]
pub type BUEP_R_TOG_AUTO__BUH_R_AUTO_TOG_W<'a> =
    crate::BitWriter<'a, u16, UEP2_R_CTRL__UH_RX_CTRL_SPEC, bool, 5>;
#[doc = "Field `bUH_R_DATA_NO` reader - bUH_R_DATA_NO"]
pub type BUH_R_DATA_NO_R = crate::BitReader<bool>;
#[doc = "Field `bUH_R_DATA_NO` writer - bUH_R_DATA_NO"]
pub type BUH_R_DATA_NO_W<'a> =
    crate::BitWriter<'a, u16, UEP2_R_CTRL__UH_RX_CTRL_SPEC, bool, 6>;
impl R {
    #[doc = "Bits 0:1 - endpoint 2 control of the accept response to OUT transactions"]
    #[inline(always)]
    pub fn mask_uep_r_res__mask_uh_r_res(&self) -> MASK_UEP_R_RES__MASK_UH_R_RES_R {
        MASK_UEP_R_RES__MASK_UH_R_RES_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - bUH_R_RES_NO"]
    #[inline(always)]
    pub fn b_uh_r_res_no(&self) -> BUH_R_RES_NO_R {
        BUH_R_RES_NO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - endpoint 2 synchronous trigger bit for the accept to prepare"]
    #[inline(always)]
    pub fn mask_uep_r_tog__mask_uh_r_tog(&self) -> MASK_UEP_R_TOG__MASK_UH_R_TOG_R {
        MASK_UEP_R_TOG__MASK_UH_R_TOG_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - endpoint 2 synchronous trigger bit automatic filp enables the control bit"]
    #[inline(always)]
    pub fn b_uep_r_tog_auto__b_uh_r_auto_tog(&self) -> BUEP_R_TOG_AUTO__BUH_R_AUTO_TOG_R {
        BUEP_R_TOG_AUTO__BUH_R_AUTO_TOG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - bUH_R_DATA_NO"]
    #[inline(always)]
    pub fn b_uh_r_data_no(&self) -> BUH_R_DATA_NO_R {
        BUH_R_DATA_NO_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - endpoint 2 control of the accept response to OUT transactions"]
    #[inline(always)]
    pub fn mask_uep_r_res__mask_uh_r_res(&mut self) -> MASK_UEP_R_RES__MASK_UH_R_RES_W {
        MASK_UEP_R_RES__MASK_UH_R_RES_W::new(self)
    }
    #[doc = "Bit 2 - bUH_R_RES_NO"]
    #[inline(always)]
    pub fn b_uh_r_res_no(&mut self) -> BUH_R_RES_NO_W {
        BUH_R_RES_NO_W::new(self)
    }
    #[doc = "Bits 3:4 - endpoint 2 synchronous trigger bit for the accept to prepare"]
    #[inline(always)]
    pub fn mask_uep_r_tog__mask_uh_r_tog(&mut self) -> MASK_UEP_R_TOG__MASK_UH_R_TOG_W {
        MASK_UEP_R_TOG__MASK_UH_R_TOG_W::new(self)
    }
    #[doc = "Bit 5 - endpoint 2 synchronous trigger bit automatic filp enables the control bit"]
    #[inline(always)]
    pub fn b_uep_r_tog_auto__b_uh_r_auto_tog(
        &mut self,
    ) -> BUEP_R_TOG_AUTO__BUH_R_AUTO_TOG_W {
        BUEP_R_TOG_AUTO__BUH_R_AUTO_TOG_W::new(self)
    }
    #[doc = "Bit 6 - bUH_R_DATA_NO"]
    #[inline(always)]
    pub fn b_uh_r_data_no(&mut self) -> BUH_R_DATA_NO_W {
        BUH_R_DATA_NO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "endpoint 2 send control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uep2_r_ctrl__uh_rx_ctrl](index.html) module"]
pub struct UEP2_R_CTRL__UH_RX_CTRL_SPEC;
impl crate::RegisterSpec for UEP2_R_CTRL__UH_RX_CTRL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [uep2_r_ctrl__uh_rx_ctrl::R](R) reader structure"]
impl crate::Readable for UEP2_R_CTRL__UH_RX_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uep2_r_ctrl__uh_rx_ctrl::W](W) writer structure"]
impl crate::Writable for UEP2_R_CTRL__UH_RX_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UEP2_R_CTRL__UH_RX_CTRL to value 0"]
impl crate::Resettable for UEP2_R_CTRL__UH_RX_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}