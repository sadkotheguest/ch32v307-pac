#[doc = "Register `IER` reader"]
pub struct R(crate::R<IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER` writer"]
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_DVP_IE_STR_FRM` reader - DVP frame start interrupt enable"]
pub type RB_DVP_IE_STR_FRM_R = crate::BitReader<bool>;
#[doc = "Field `RB_DVP_IE_STR_FRM` writer - DVP frame start interrupt enable"]
pub type RB_DVP_IE_STR_FRM_W<'a> = crate::BitWriter<'a, u32, IER_SPEC, bool, 0>;
#[doc = "Field `RB_DVP_IE_ROW_DONE` reader - DVP row received done interrupt enable"]
pub type RB_DVP_IE_ROW_DONE_R = crate::BitReader<bool>;
#[doc = "Field `RB_DVP_IE_ROW_DONE` writer - DVP row received done interrupt enable"]
pub type RB_DVP_IE_ROW_DONE_W<'a> = crate::BitWriter<'a, u32, IER_SPEC, bool, 1>;
#[doc = "Field `RB_DVP_IE_FRM_DONE` reader - DVP frame received done interrupt enable"]
pub type RB_DVP_IE_FRM_DONE_R = crate::BitReader<bool>;
#[doc = "Field `RB_DVP_IE_FRM_DONE` writer - DVP frame received done interrupt enable"]
pub type RB_DVP_IE_FRM_DONE_W<'a> = crate::BitWriter<'a, u32, IER_SPEC, bool, 2>;
#[doc = "Field `RB_DVP_IE_FIFO_OV` reader - DVP receive fifo overflow interrupt enable"]
pub type RB_DVP_IE_FIFO_OV_R = crate::BitReader<bool>;
#[doc = "Field `RB_DVP_IE_FIFO_OV` writer - DVP receive fifo overflow interrupt enable"]
pub type RB_DVP_IE_FIFO_OV_W<'a> = crate::BitWriter<'a, u32, IER_SPEC, bool, 3>;
#[doc = "Field `RB_DVP_IE_STP_FRM` reader - DVP frame stop interrupt enable"]
pub type RB_DVP_IE_STP_FRM_R = crate::BitReader<bool>;
#[doc = "Field `RB_DVP_IE_STP_FRM` writer - DVP frame stop interrupt enable"]
pub type RB_DVP_IE_STP_FRM_W<'a> = crate::BitWriter<'a, u32, IER_SPEC, bool, 4>;
impl R {
    #[doc = "Bit 0 - DVP frame start interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_ie_str_frm(&self) -> RB_DVP_IE_STR_FRM_R {
        RB_DVP_IE_STR_FRM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DVP row received done interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_ie_row_done(&self) -> RB_DVP_IE_ROW_DONE_R {
        RB_DVP_IE_ROW_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DVP frame received done interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_ie_frm_done(&self) -> RB_DVP_IE_FRM_DONE_R {
        RB_DVP_IE_FRM_DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DVP receive fifo overflow interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_ie_fifo_ov(&self) -> RB_DVP_IE_FIFO_OV_R {
        RB_DVP_IE_FIFO_OV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DVP frame stop interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_ie_stp_frm(&self) -> RB_DVP_IE_STP_FRM_R {
        RB_DVP_IE_STP_FRM_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DVP frame start interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_ie_str_frm(&mut self) -> RB_DVP_IE_STR_FRM_W {
        RB_DVP_IE_STR_FRM_W::new(self)
    }
    #[doc = "Bit 1 - DVP row received done interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_ie_row_done(&mut self) -> RB_DVP_IE_ROW_DONE_W {
        RB_DVP_IE_ROW_DONE_W::new(self)
    }
    #[doc = "Bit 2 - DVP frame received done interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_ie_frm_done(&mut self) -> RB_DVP_IE_FRM_DONE_W {
        RB_DVP_IE_FRM_DONE_W::new(self)
    }
    #[doc = "Bit 3 - DVP receive fifo overflow interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_ie_fifo_ov(&mut self) -> RB_DVP_IE_FIFO_OV_W {
        RB_DVP_IE_FIFO_OV_W::new(self)
    }
    #[doc = "Bit 4 - DVP frame stop interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_ie_stp_frm(&mut self) -> RB_DVP_IE_STP_FRM_W {
        RB_DVP_IE_STP_FRM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Digital Video Interrupt register (DVP_IER)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ier::R](R) reader structure"]
impl crate::Readable for IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier::W](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}