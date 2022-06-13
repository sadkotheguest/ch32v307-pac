#[doc = "Register `INTFR` reader"]
pub struct R(crate::R<INTFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTFR` writer"]
pub struct W(crate::W<INTFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTFR_SPEC>;
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
impl From<crate::W<INTFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PR0` reader - Pending bit 0"]
pub type PR0_R = crate::BitReader<bool>;
#[doc = "Field `PR0` writer - Pending bit 0"]
pub type PR0_W<'a> = crate::BitWriter<'a, u32, INTFR_SPEC, bool, 0>;
#[doc = "Field `PR1` reader - Pending bit 1"]
pub type PR1_R = crate::BitReader<bool>;
#[doc = "Field `PR1` writer - Pending bit 1"]
pub type PR1_W<'a> = crate::BitWriter<'a, u32, INTFR_SPEC, bool, 1>;
#[doc = "Field `PR2` reader - Pending bit 2"]
pub type PR2_R = crate::BitReader<bool>;
#[doc = "Field `PR2` writer - Pending bit 2"]
pub type PR2_W<'a> = crate::BitWriter<'a, u32, INTFR_SPEC, bool, 2>;
#[doc = "Field `PR3` reader - Pending bit 3"]
pub type PR3_R = crate::BitReader<bool>;
#[doc = "Field `PR3` writer - Pending bit 3"]
pub type PR3_W<'a> = crate::BitWriter<'a, u32, INTFR_SPEC, bool, 3>;
#[doc = "Field `PR4` reader - Pending bit 4"]
pub type PR4_R = crate::BitReader<bool>;
#[doc = "Field `PR4` writer - Pending bit 4"]
pub type PR4_W<'a> = crate::BitWriter<'a, u32, INTFR_SPEC, bool, 4>;
#[doc = "Field `PR5` reader - Pending bit 5"]
pub type PR5_R = crate::BitReader<bool>;
#[doc = "Field `PR5` writer - Pending bit 5"]
pub type PR5_W<'a> = crate::BitWriter<'a, u32, INTFR_SPEC, bool, 5>;
#[doc = "Field `PR6` reader - Pending bit 6"]
pub type PR6_R = crate::BitReader<bool>;
#[doc = "Field `PR6` writer - Pending bit 6"]
pub type PR6_W<'a> = crate::BitWriter<'a, u32, INTFR_SPEC, bool, 6>;
#[doc = "Field `PR7` reader - Pending bit 7"]
pub type PR7_R = crate::BitReader<bool>;
#[doc = "Field `PR7` writer - Pending bit 7"]
pub type PR7_W<'a> = crate::BitWriter<'a, u32, INTFR_SPEC, bool, 7>;
#[doc = "Field `PR8` reader - Pending bit 8"]
pub type PR8_R = crate::BitReader<bool>;
#[doc = "Field `PR8` writer - Pending bit 8"]
pub type PR8_W<'a> = crate::BitWriter<'a, u32, INTFR_SPEC, bool, 8>;
#[doc = "Field `PR9` reader - Pending bit 9"]
pub type PR9_R = crate::BitReader<bool>;
#[doc = "Field `PR9` writer - Pending bit 9"]
pub type PR9_W<'a> = crate::BitWriter<'a, u32, INTFR_SPEC, bool, 9>;
#[doc = "Field `PR10` reader - Pending bit 10"]
pub type PR10_R = crate::BitReader<bool>;
#[doc = "Field `PR10` writer - Pending bit 10"]
pub type PR10_W<'a> = crate::BitWriter<'a, u32, INTFR_SPEC, bool, 10>;
#[doc = "Field `PR11` reader - Pending bit 11"]
pub type PR11_R = crate::BitReader<bool>;
#[doc = "Field `PR11` writer - Pending bit 11"]
pub type PR11_W<'a> = crate::BitWriter<'a, u32, INTFR_SPEC, bool, 11>;
#[doc = "Field `PR12` reader - Pending bit 12"]
pub type PR12_R = crate::BitReader<bool>;
#[doc = "Field `PR12` writer - Pending bit 12"]
pub type PR12_W<'a> = crate::BitWriter<'a, u32, INTFR_SPEC, bool, 12>;
#[doc = "Field `PR13` reader - Pending bit 13"]
pub type PR13_R = crate::BitReader<bool>;
#[doc = "Field `PR13` writer - Pending bit 13"]
pub type PR13_W<'a> = crate::BitWriter<'a, u32, INTFR_SPEC, bool, 13>;
#[doc = "Field `PR14` reader - Pending bit 14"]
pub type PR14_R = crate::BitReader<bool>;
#[doc = "Field `PR14` writer - Pending bit 14"]
pub type PR14_W<'a> = crate::BitWriter<'a, u32, INTFR_SPEC, bool, 14>;
#[doc = "Field `PR15` reader - Pending bit 15"]
pub type PR15_R = crate::BitReader<bool>;
#[doc = "Field `PR15` writer - Pending bit 15"]
pub type PR15_W<'a> = crate::BitWriter<'a, u32, INTFR_SPEC, bool, 15>;
#[doc = "Field `PR16` reader - Pending bit 16"]
pub type PR16_R = crate::BitReader<bool>;
#[doc = "Field `PR16` writer - Pending bit 16"]
pub type PR16_W<'a> = crate::BitWriter<'a, u32, INTFR_SPEC, bool, 16>;
#[doc = "Field `PR17` reader - Pending bit 17"]
pub type PR17_R = crate::BitReader<bool>;
#[doc = "Field `PR17` writer - Pending bit 17"]
pub type PR17_W<'a> = crate::BitWriter<'a, u32, INTFR_SPEC, bool, 17>;
#[doc = "Field `PR18` reader - Pending bit 18"]
pub type PR18_R = crate::BitReader<bool>;
#[doc = "Field `PR18` writer - Pending bit 18"]
pub type PR18_W<'a> = crate::BitWriter<'a, u32, INTFR_SPEC, bool, 18>;
impl R {
    #[doc = "Bit 0 - Pending bit 0"]
    #[inline(always)]
    pub fn pr0(&self) -> PR0_R {
        PR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pending bit 1"]
    #[inline(always)]
    pub fn pr1(&self) -> PR1_R {
        PR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pending bit 2"]
    #[inline(always)]
    pub fn pr2(&self) -> PR2_R {
        PR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pending bit 3"]
    #[inline(always)]
    pub fn pr3(&self) -> PR3_R {
        PR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pending bit 4"]
    #[inline(always)]
    pub fn pr4(&self) -> PR4_R {
        PR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pending bit 5"]
    #[inline(always)]
    pub fn pr5(&self) -> PR5_R {
        PR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pending bit 6"]
    #[inline(always)]
    pub fn pr6(&self) -> PR6_R {
        PR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pending bit 7"]
    #[inline(always)]
    pub fn pr7(&self) -> PR7_R {
        PR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pending bit 8"]
    #[inline(always)]
    pub fn pr8(&self) -> PR8_R {
        PR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pending bit 9"]
    #[inline(always)]
    pub fn pr9(&self) -> PR9_R {
        PR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pending bit 10"]
    #[inline(always)]
    pub fn pr10(&self) -> PR10_R {
        PR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pending bit 11"]
    #[inline(always)]
    pub fn pr11(&self) -> PR11_R {
        PR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pending bit 12"]
    #[inline(always)]
    pub fn pr12(&self) -> PR12_R {
        PR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pending bit 13"]
    #[inline(always)]
    pub fn pr13(&self) -> PR13_R {
        PR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pending bit 14"]
    #[inline(always)]
    pub fn pr14(&self) -> PR14_R {
        PR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pending bit 15"]
    #[inline(always)]
    pub fn pr15(&self) -> PR15_R {
        PR15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Pending bit 16"]
    #[inline(always)]
    pub fn pr16(&self) -> PR16_R {
        PR16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Pending bit 17"]
    #[inline(always)]
    pub fn pr17(&self) -> PR17_R {
        PR17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Pending bit 18"]
    #[inline(always)]
    pub fn pr18(&self) -> PR18_R {
        PR18_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pending bit 0"]
    #[inline(always)]
    pub fn pr0(&mut self) -> PR0_W {
        PR0_W::new(self)
    }
    #[doc = "Bit 1 - Pending bit 1"]
    #[inline(always)]
    pub fn pr1(&mut self) -> PR1_W {
        PR1_W::new(self)
    }
    #[doc = "Bit 2 - Pending bit 2"]
    #[inline(always)]
    pub fn pr2(&mut self) -> PR2_W {
        PR2_W::new(self)
    }
    #[doc = "Bit 3 - Pending bit 3"]
    #[inline(always)]
    pub fn pr3(&mut self) -> PR3_W {
        PR3_W::new(self)
    }
    #[doc = "Bit 4 - Pending bit 4"]
    #[inline(always)]
    pub fn pr4(&mut self) -> PR4_W {
        PR4_W::new(self)
    }
    #[doc = "Bit 5 - Pending bit 5"]
    #[inline(always)]
    pub fn pr5(&mut self) -> PR5_W {
        PR5_W::new(self)
    }
    #[doc = "Bit 6 - Pending bit 6"]
    #[inline(always)]
    pub fn pr6(&mut self) -> PR6_W {
        PR6_W::new(self)
    }
    #[doc = "Bit 7 - Pending bit 7"]
    #[inline(always)]
    pub fn pr7(&mut self) -> PR7_W {
        PR7_W::new(self)
    }
    #[doc = "Bit 8 - Pending bit 8"]
    #[inline(always)]
    pub fn pr8(&mut self) -> PR8_W {
        PR8_W::new(self)
    }
    #[doc = "Bit 9 - Pending bit 9"]
    #[inline(always)]
    pub fn pr9(&mut self) -> PR9_W {
        PR9_W::new(self)
    }
    #[doc = "Bit 10 - Pending bit 10"]
    #[inline(always)]
    pub fn pr10(&mut self) -> PR10_W {
        PR10_W::new(self)
    }
    #[doc = "Bit 11 - Pending bit 11"]
    #[inline(always)]
    pub fn pr11(&mut self) -> PR11_W {
        PR11_W::new(self)
    }
    #[doc = "Bit 12 - Pending bit 12"]
    #[inline(always)]
    pub fn pr12(&mut self) -> PR12_W {
        PR12_W::new(self)
    }
    #[doc = "Bit 13 - Pending bit 13"]
    #[inline(always)]
    pub fn pr13(&mut self) -> PR13_W {
        PR13_W::new(self)
    }
    #[doc = "Bit 14 - Pending bit 14"]
    #[inline(always)]
    pub fn pr14(&mut self) -> PR14_W {
        PR14_W::new(self)
    }
    #[doc = "Bit 15 - Pending bit 15"]
    #[inline(always)]
    pub fn pr15(&mut self) -> PR15_W {
        PR15_W::new(self)
    }
    #[doc = "Bit 16 - Pending bit 16"]
    #[inline(always)]
    pub fn pr16(&mut self) -> PR16_W {
        PR16_W::new(self)
    }
    #[doc = "Bit 17 - Pending bit 17"]
    #[inline(always)]
    pub fn pr17(&mut self) -> PR17_W {
        PR17_W::new(self)
    }
    #[doc = "Bit 18 - Pending bit 18"]
    #[inline(always)]
    pub fn pr18(&mut self) -> PR18_W {
        PR18_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pending register (EXTI_INTFR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intfr](index.html) module"]
pub struct INTFR_SPEC;
impl crate::RegisterSpec for INTFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intfr::R](R) reader structure"]
impl crate::Readable for INTFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intfr::W](W) writer structure"]
impl crate::Writable for INTFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTFR to value 0"]
impl crate::Resettable for INTFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}