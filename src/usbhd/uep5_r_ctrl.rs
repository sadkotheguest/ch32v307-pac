#[doc = "Register `UEP5_R_CTRL` reader"]
pub struct R(crate::R<UEP5_R_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UEP5_R_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UEP5_R_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UEP5_R_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UEP5_R_CTRL` writer"]
pub struct W(crate::W<UEP5_R_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UEP5_R_CTRL_SPEC>;
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
impl From<crate::W<UEP5_R_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UEP5_R_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASK_UEP_R_RES` reader - endpoint 5 control of the accept response to OUT transactions"]
pub type MASK_UEP_R_RES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MASK_UEP_R_RES` writer - endpoint 5 control of the accept response to OUT transactions"]
pub type MASK_UEP_R_RES_W<'a> = crate::FieldWriter<'a, u16, UEP5_R_CTRL_SPEC, u8, u8, 2, 0>;
#[doc = "Field `MASK_UEP_R_TOG` reader - endpoint 5 synchronous trigger bit for the accept to prepare"]
pub type MASK_UEP_R_TOG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MASK_UEP_R_TOG` writer - endpoint 5 synchronous trigger bit for the accept to prepare"]
pub type MASK_UEP_R_TOG_W<'a> = crate::FieldWriter<'a, u16, UEP5_R_CTRL_SPEC, u8, u8, 2, 3>;
#[doc = "Field `bUEP_R_TOG_AUTO` reader - endpoint 5 synchronous trigger bit automatic filp enables the control bit"]
pub type BUEP_R_TOG_AUTO_R = crate::BitReader<bool>;
#[doc = "Field `bUEP_R_TOG_AUTO` writer - endpoint 5 synchronous trigger bit automatic filp enables the control bit"]
pub type BUEP_R_TOG_AUTO_W<'a> = crate::BitWriter<'a, u16, UEP5_R_CTRL_SPEC, bool, 5>;
impl R {
    #[doc = "Bits 0:1 - endpoint 5 control of the accept response to OUT transactions"]
    #[inline(always)]
    pub fn mask_uep_r_res(&self) -> MASK_UEP_R_RES_R {
        MASK_UEP_R_RES_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 3:4 - endpoint 5 synchronous trigger bit for the accept to prepare"]
    #[inline(always)]
    pub fn mask_uep_r_tog(&self) -> MASK_UEP_R_TOG_R {
        MASK_UEP_R_TOG_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - endpoint 5 synchronous trigger bit automatic filp enables the control bit"]
    #[inline(always)]
    pub fn b_uep_r_tog_auto(&self) -> BUEP_R_TOG_AUTO_R {
        BUEP_R_TOG_AUTO_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - endpoint 5 control of the accept response to OUT transactions"]
    #[inline(always)]
    pub fn mask_uep_r_res(&mut self) -> MASK_UEP_R_RES_W {
        MASK_UEP_R_RES_W::new(self)
    }
    #[doc = "Bits 3:4 - endpoint 5 synchronous trigger bit for the accept to prepare"]
    #[inline(always)]
    pub fn mask_uep_r_tog(&mut self) -> MASK_UEP_R_TOG_W {
        MASK_UEP_R_TOG_W::new(self)
    }
    #[doc = "Bit 5 - endpoint 5 synchronous trigger bit automatic filp enables the control bit"]
    #[inline(always)]
    pub fn b_uep_r_tog_auto(&mut self) -> BUEP_R_TOG_AUTO_W {
        BUEP_R_TOG_AUTO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "endpoint 5 send control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uep5_r_ctrl](index.html) module"]
pub struct UEP5_R_CTRL_SPEC;
impl crate::RegisterSpec for UEP5_R_CTRL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [uep5_r_ctrl::R](R) reader structure"]
impl crate::Readable for UEP5_R_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uep5_r_ctrl::W](W) writer structure"]
impl crate::Writable for UEP5_R_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UEP5_R_CTRL to value 0"]
impl crate::Resettable for UEP5_R_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}