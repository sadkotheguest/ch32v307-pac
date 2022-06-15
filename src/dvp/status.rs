#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
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
impl From<crate::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_DVP_FIFO_RDY` reader - DVP frame start interrupt enable"]
pub type RB_DVP_FIFO_RDY_R = crate::BitReader<bool>;
#[doc = "Field `RB_DVP_FIFO_RDY` writer - DVP frame start interrupt enable"]
pub type RB_DVP_FIFO_RDY_W<'a> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, 0>;
#[doc = "Field `RB_DVP_FIFO_FULL` reader - DVP row received done interrupt enable"]
pub type RB_DVP_FIFO_FULL_R = crate::BitReader<bool>;
#[doc = "Field `RB_DVP_FIFO_FULL` writer - DVP row received done interrupt enable"]
pub type RB_DVP_FIFO_FULL_W<'a> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, 1>;
#[doc = "Field `RB_DVP_FIFO_OV` reader - DVP frame received done interrupt enable"]
pub type RB_DVP_FIFO_OV_R = crate::BitReader<bool>;
#[doc = "Field `RB_DVP_FIFO_OV` writer - DVP frame received done interrupt enable"]
pub type RB_DVP_FIFO_OV_W<'a> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, 2>;
#[doc = "Field `RB_DVP_MSK_FIFO_CNT` reader - DVP receive fifo overflow interrupt enable"]
pub type RB_DVP_MSK_FIFO_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RB_DVP_MSK_FIFO_CNT` writer - DVP receive fifo overflow interrupt enable"]
pub type RB_DVP_MSK_FIFO_CNT_W<'a> = crate::FieldWriter<'a, u32, STATUS_SPEC, u8, u8, 3, 4>;
impl R {
    #[doc = "Bit 0 - DVP frame start interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_fifo_rdy(&self) -> RB_DVP_FIFO_RDY_R {
        RB_DVP_FIFO_RDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DVP row received done interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_fifo_full(&self) -> RB_DVP_FIFO_FULL_R {
        RB_DVP_FIFO_FULL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DVP frame received done interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_fifo_ov(&self) -> RB_DVP_FIFO_OV_R {
        RB_DVP_FIFO_OV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - DVP receive fifo overflow interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_msk_fifo_cnt(&self) -> RB_DVP_MSK_FIFO_CNT_R {
        RB_DVP_MSK_FIFO_CNT_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DVP frame start interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_fifo_rdy(&mut self) -> RB_DVP_FIFO_RDY_W {
        RB_DVP_FIFO_RDY_W::new(self)
    }
    #[doc = "Bit 1 - DVP row received done interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_fifo_full(&mut self) -> RB_DVP_FIFO_FULL_W {
        RB_DVP_FIFO_FULL_W::new(self)
    }
    #[doc = "Bit 2 - DVP frame received done interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_fifo_ov(&mut self) -> RB_DVP_FIFO_OV_W {
        RB_DVP_FIFO_OV_W::new(self)
    }
    #[doc = "Bits 4:6 - DVP receive fifo overflow interrupt enable"]
    #[inline(always)]
    pub fn rb_dvp_msk_fifo_cnt(&mut self) -> RB_DVP_MSK_FIFO_CNT_W {
        RB_DVP_MSK_FIFO_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Digital Video STATUS register (DVP_STATUS)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}