#[doc = "Register `FMCFGR` reader"]
pub struct R(crate::R<FMCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMCFGR` writer"]
pub struct W(crate::W<FMCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMCFGR_SPEC>;
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
impl From<crate::W<FMCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FBM0` reader - Filter mode"]
pub type FBM0_R = crate::BitReader<bool>;
#[doc = "Field `FBM0` writer - Filter mode"]
pub type FBM0_W<'a> = crate::BitWriter<'a, u32, FMCFGR_SPEC, bool, 0>;
#[doc = "Field `FBM1` reader - Filter mode"]
pub type FBM1_R = crate::BitReader<bool>;
#[doc = "Field `FBM1` writer - Filter mode"]
pub type FBM1_W<'a> = crate::BitWriter<'a, u32, FMCFGR_SPEC, bool, 1>;
#[doc = "Field `FBM2` reader - Filter mode"]
pub type FBM2_R = crate::BitReader<bool>;
#[doc = "Field `FBM2` writer - Filter mode"]
pub type FBM2_W<'a> = crate::BitWriter<'a, u32, FMCFGR_SPEC, bool, 2>;
#[doc = "Field `FBM3` reader - Filter mode"]
pub type FBM3_R = crate::BitReader<bool>;
#[doc = "Field `FBM3` writer - Filter mode"]
pub type FBM3_W<'a> = crate::BitWriter<'a, u32, FMCFGR_SPEC, bool, 3>;
#[doc = "Field `FBM4` reader - Filter mode"]
pub type FBM4_R = crate::BitReader<bool>;
#[doc = "Field `FBM4` writer - Filter mode"]
pub type FBM4_W<'a> = crate::BitWriter<'a, u32, FMCFGR_SPEC, bool, 4>;
#[doc = "Field `FBM5` reader - Filter mode"]
pub type FBM5_R = crate::BitReader<bool>;
#[doc = "Field `FBM5` writer - Filter mode"]
pub type FBM5_W<'a> = crate::BitWriter<'a, u32, FMCFGR_SPEC, bool, 5>;
#[doc = "Field `FBM6` reader - Filter mode"]
pub type FBM6_R = crate::BitReader<bool>;
#[doc = "Field `FBM6` writer - Filter mode"]
pub type FBM6_W<'a> = crate::BitWriter<'a, u32, FMCFGR_SPEC, bool, 6>;
#[doc = "Field `FBM7` reader - Filter mode"]
pub type FBM7_R = crate::BitReader<bool>;
#[doc = "Field `FBM7` writer - Filter mode"]
pub type FBM7_W<'a> = crate::BitWriter<'a, u32, FMCFGR_SPEC, bool, 7>;
#[doc = "Field `FBM8` reader - Filter mode"]
pub type FBM8_R = crate::BitReader<bool>;
#[doc = "Field `FBM8` writer - Filter mode"]
pub type FBM8_W<'a> = crate::BitWriter<'a, u32, FMCFGR_SPEC, bool, 8>;
#[doc = "Field `FBM9` reader - Filter mode"]
pub type FBM9_R = crate::BitReader<bool>;
#[doc = "Field `FBM9` writer - Filter mode"]
pub type FBM9_W<'a> = crate::BitWriter<'a, u32, FMCFGR_SPEC, bool, 9>;
#[doc = "Field `FBM10` reader - Filter mode"]
pub type FBM10_R = crate::BitReader<bool>;
#[doc = "Field `FBM10` writer - Filter mode"]
pub type FBM10_W<'a> = crate::BitWriter<'a, u32, FMCFGR_SPEC, bool, 10>;
#[doc = "Field `FBM11` reader - Filter mode"]
pub type FBM11_R = crate::BitReader<bool>;
#[doc = "Field `FBM11` writer - Filter mode"]
pub type FBM11_W<'a> = crate::BitWriter<'a, u32, FMCFGR_SPEC, bool, 11>;
#[doc = "Field `FBM12` reader - Filter mode"]
pub type FBM12_R = crate::BitReader<bool>;
#[doc = "Field `FBM12` writer - Filter mode"]
pub type FBM12_W<'a> = crate::BitWriter<'a, u32, FMCFGR_SPEC, bool, 12>;
#[doc = "Field `FBM13` reader - Filter mode"]
pub type FBM13_R = crate::BitReader<bool>;
#[doc = "Field `FBM13` writer - Filter mode"]
pub type FBM13_W<'a> = crate::BitWriter<'a, u32, FMCFGR_SPEC, bool, 13>;
impl R {
    #[doc = "Bit 0 - Filter mode"]
    #[inline(always)]
    pub fn fbm0(&self) -> FBM0_R {
        FBM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter mode"]
    #[inline(always)]
    pub fn fbm1(&self) -> FBM1_R {
        FBM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter mode"]
    #[inline(always)]
    pub fn fbm2(&self) -> FBM2_R {
        FBM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter mode"]
    #[inline(always)]
    pub fn fbm3(&self) -> FBM3_R {
        FBM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter mode"]
    #[inline(always)]
    pub fn fbm4(&self) -> FBM4_R {
        FBM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter mode"]
    #[inline(always)]
    pub fn fbm5(&self) -> FBM5_R {
        FBM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter mode"]
    #[inline(always)]
    pub fn fbm6(&self) -> FBM6_R {
        FBM6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter mode"]
    #[inline(always)]
    pub fn fbm7(&self) -> FBM7_R {
        FBM7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter mode"]
    #[inline(always)]
    pub fn fbm8(&self) -> FBM8_R {
        FBM8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter mode"]
    #[inline(always)]
    pub fn fbm9(&self) -> FBM9_R {
        FBM9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Filter mode"]
    #[inline(always)]
    pub fn fbm10(&self) -> FBM10_R {
        FBM10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Filter mode"]
    #[inline(always)]
    pub fn fbm11(&self) -> FBM11_R {
        FBM11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter mode"]
    #[inline(always)]
    pub fn fbm12(&self) -> FBM12_R {
        FBM12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Filter mode"]
    #[inline(always)]
    pub fn fbm13(&self) -> FBM13_R {
        FBM13_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter mode"]
    #[inline(always)]
    pub fn fbm0(&mut self) -> FBM0_W {
        FBM0_W::new(self)
    }
    #[doc = "Bit 1 - Filter mode"]
    #[inline(always)]
    pub fn fbm1(&mut self) -> FBM1_W {
        FBM1_W::new(self)
    }
    #[doc = "Bit 2 - Filter mode"]
    #[inline(always)]
    pub fn fbm2(&mut self) -> FBM2_W {
        FBM2_W::new(self)
    }
    #[doc = "Bit 3 - Filter mode"]
    #[inline(always)]
    pub fn fbm3(&mut self) -> FBM3_W {
        FBM3_W::new(self)
    }
    #[doc = "Bit 4 - Filter mode"]
    #[inline(always)]
    pub fn fbm4(&mut self) -> FBM4_W {
        FBM4_W::new(self)
    }
    #[doc = "Bit 5 - Filter mode"]
    #[inline(always)]
    pub fn fbm5(&mut self) -> FBM5_W {
        FBM5_W::new(self)
    }
    #[doc = "Bit 6 - Filter mode"]
    #[inline(always)]
    pub fn fbm6(&mut self) -> FBM6_W {
        FBM6_W::new(self)
    }
    #[doc = "Bit 7 - Filter mode"]
    #[inline(always)]
    pub fn fbm7(&mut self) -> FBM7_W {
        FBM7_W::new(self)
    }
    #[doc = "Bit 8 - Filter mode"]
    #[inline(always)]
    pub fn fbm8(&mut self) -> FBM8_W {
        FBM8_W::new(self)
    }
    #[doc = "Bit 9 - Filter mode"]
    #[inline(always)]
    pub fn fbm9(&mut self) -> FBM9_W {
        FBM9_W::new(self)
    }
    #[doc = "Bit 10 - Filter mode"]
    #[inline(always)]
    pub fn fbm10(&mut self) -> FBM10_W {
        FBM10_W::new(self)
    }
    #[doc = "Bit 11 - Filter mode"]
    #[inline(always)]
    pub fn fbm11(&mut self) -> FBM11_W {
        FBM11_W::new(self)
    }
    #[doc = "Bit 12 - Filter mode"]
    #[inline(always)]
    pub fn fbm12(&mut self) -> FBM12_W {
        FBM12_W::new(self)
    }
    #[doc = "Bit 13 - Filter mode"]
    #[inline(always)]
    pub fn fbm13(&mut self) -> FBM13_W {
        FBM13_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FMCFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmcfgr](index.html) module"]
pub struct FMCFGR_SPEC;
impl crate::RegisterSpec for FMCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmcfgr::R](R) reader structure"]
impl crate::Readable for FMCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmcfgr::W](W) writer structure"]
impl crate::Writable for FMCFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMCFGR to value 0"]
impl crate::Resettable for FMCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}