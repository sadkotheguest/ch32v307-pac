#[doc = "Register `CR0` reader"]
pub struct R(crate::R<CR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR0` writer"]
pub struct W(crate::W<CR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR0_SPEC>;
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
impl From<crate::W<CR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB_DVP_ENABLE` reader - DVP enable"]
pub type RB_DVP_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `RB_DVP_ENABLE` writer - DVP enable"]
pub type RB_DVP_ENABLE_W<'a> = crate::BitWriter<'a, u32, CR0_SPEC, bool, 0>;
#[doc = "Field `RB_DVP_V_POLAR` reader - DVP VSYNC polarity control"]
pub type RB_DVP_V_POLAR_R = crate::BitReader<bool>;
#[doc = "Field `RB_DVP_V_POLAR` writer - DVP VSYNC polarity control"]
pub type RB_DVP_V_POLAR_W<'a> = crate::BitWriter<'a, u32, CR0_SPEC, bool, 1>;
#[doc = "Field `RB_DVP_H_POLAR` reader - DVP HSYNC polarity control"]
pub type RB_DVP_H_POLAR_R = crate::BitReader<bool>;
#[doc = "Field `RB_DVP_H_POLAR` writer - DVP HSYNC polarity control"]
pub type RB_DVP_H_POLAR_W<'a> = crate::BitWriter<'a, u32, CR0_SPEC, bool, 2>;
#[doc = "Field `RB_DVP_P_POLAR` reader - DVP PCLK polarity control"]
pub type RB_DVP_P_POLAR_R = crate::BitReader<bool>;
#[doc = "Field `RB_DVP_P_POLAR` writer - DVP PCLK polarity control"]
pub type RB_DVP_P_POLAR_W<'a> = crate::BitWriter<'a, u32, CR0_SPEC, bool, 3>;
#[doc = "Field `RB_DVP_MSK_DAT_MOD` reader - DVP data mode"]
pub type RB_DVP_MSK_DAT_MOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RB_DVP_MSK_DAT_MOD` writer - DVP data mode"]
pub type RB_DVP_MSK_DAT_MOD_W<'a> = crate::FieldWriter<'a, u32, CR0_SPEC, u8, u8, 2, 4>;
#[doc = "Field `RB_DVP_JPEG` reader - DVP JPEG mode"]
pub type RB_DVP_JPEG_R = crate::BitReader<bool>;
#[doc = "Field `RB_DVP_JPEG` writer - DVP JPEG mode"]
pub type RB_DVP_JPEG_W<'a> = crate::BitWriter<'a, u32, CR0_SPEC, bool, 6>;
impl R {
    #[doc = "Bit 0 - DVP enable"]
    #[inline(always)]
    pub fn rb_dvp_enable(&self) -> RB_DVP_ENABLE_R {
        RB_DVP_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DVP VSYNC polarity control"]
    #[inline(always)]
    pub fn rb_dvp_v_polar(&self) -> RB_DVP_V_POLAR_R {
        RB_DVP_V_POLAR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DVP HSYNC polarity control"]
    #[inline(always)]
    pub fn rb_dvp_h_polar(&self) -> RB_DVP_H_POLAR_R {
        RB_DVP_H_POLAR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DVP PCLK polarity control"]
    #[inline(always)]
    pub fn rb_dvp_p_polar(&self) -> RB_DVP_P_POLAR_R {
        RB_DVP_P_POLAR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - DVP data mode"]
    #[inline(always)]
    pub fn rb_dvp_msk_dat_mod(&self) -> RB_DVP_MSK_DAT_MOD_R {
        RB_DVP_MSK_DAT_MOD_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - DVP JPEG mode"]
    #[inline(always)]
    pub fn rb_dvp_jpeg(&self) -> RB_DVP_JPEG_R {
        RB_DVP_JPEG_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DVP enable"]
    #[inline(always)]
    pub fn rb_dvp_enable(&mut self) -> RB_DVP_ENABLE_W {
        RB_DVP_ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - DVP VSYNC polarity control"]
    #[inline(always)]
    pub fn rb_dvp_v_polar(&mut self) -> RB_DVP_V_POLAR_W {
        RB_DVP_V_POLAR_W::new(self)
    }
    #[doc = "Bit 2 - DVP HSYNC polarity control"]
    #[inline(always)]
    pub fn rb_dvp_h_polar(&mut self) -> RB_DVP_H_POLAR_W {
        RB_DVP_H_POLAR_W::new(self)
    }
    #[doc = "Bit 3 - DVP PCLK polarity control"]
    #[inline(always)]
    pub fn rb_dvp_p_polar(&mut self) -> RB_DVP_P_POLAR_W {
        RB_DVP_P_POLAR_W::new(self)
    }
    #[doc = "Bits 4:5 - DVP data mode"]
    #[inline(always)]
    pub fn rb_dvp_msk_dat_mod(&mut self) -> RB_DVP_MSK_DAT_MOD_W {
        RB_DVP_MSK_DAT_MOD_W::new(self)
    }
    #[doc = "Bit 6 - DVP JPEG mode"]
    #[inline(always)]
    pub fn rb_dvp_jpeg(&mut self) -> RB_DVP_JPEG_W {
        RB_DVP_JPEG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Digital Video control register (DVP_CR0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr0](index.html) module"]
pub struct CR0_SPEC;
impl crate::RegisterSpec for CR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr0::R](R) reader structure"]
impl crate::Readable for CR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr0::W](W) writer structure"]
impl crate::Writable for CR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR0 to value 0"]
impl crate::Resettable for CR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}