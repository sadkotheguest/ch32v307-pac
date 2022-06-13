#[doc = "Register `FAFIFOR` reader"]
pub struct R(crate::R<FAFIFOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FAFIFOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FAFIFOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FAFIFOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FAFIFOR` writer"]
pub struct W(crate::W<FAFIFOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FAFIFOR_SPEC>;
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
impl From<crate::W<FAFIFOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FAFIFOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FFA0` reader - Filter FIFO assignment for filter 0"]
pub type FFA0_R = crate::BitReader<bool>;
#[doc = "Field `FFA0` writer - Filter FIFO assignment for filter 0"]
pub type FFA0_W<'a> = crate::BitWriter<'a, u32, FAFIFOR_SPEC, bool, 0>;
#[doc = "Field `FFA1` reader - Filter FIFO assignment for filter 1"]
pub type FFA1_R = crate::BitReader<bool>;
#[doc = "Field `FFA1` writer - Filter FIFO assignment for filter 1"]
pub type FFA1_W<'a> = crate::BitWriter<'a, u32, FAFIFOR_SPEC, bool, 1>;
#[doc = "Field `FFA2` reader - Filter FIFO assignment for filter 2"]
pub type FFA2_R = crate::BitReader<bool>;
#[doc = "Field `FFA2` writer - Filter FIFO assignment for filter 2"]
pub type FFA2_W<'a> = crate::BitWriter<'a, u32, FAFIFOR_SPEC, bool, 2>;
#[doc = "Field `FFA3` reader - Filter FIFO assignment for filter 3"]
pub type FFA3_R = crate::BitReader<bool>;
#[doc = "Field `FFA3` writer - Filter FIFO assignment for filter 3"]
pub type FFA3_W<'a> = crate::BitWriter<'a, u32, FAFIFOR_SPEC, bool, 3>;
#[doc = "Field `FFA4` reader - Filter FIFO assignment for filter 4"]
pub type FFA4_R = crate::BitReader<bool>;
#[doc = "Field `FFA4` writer - Filter FIFO assignment for filter 4"]
pub type FFA4_W<'a> = crate::BitWriter<'a, u32, FAFIFOR_SPEC, bool, 4>;
#[doc = "Field `FFA5` reader - Filter FIFO assignment for filter 5"]
pub type FFA5_R = crate::BitReader<bool>;
#[doc = "Field `FFA5` writer - Filter FIFO assignment for filter 5"]
pub type FFA5_W<'a> = crate::BitWriter<'a, u32, FAFIFOR_SPEC, bool, 5>;
#[doc = "Field `FFA6` reader - Filter FIFO assignment for filter 6"]
pub type FFA6_R = crate::BitReader<bool>;
#[doc = "Field `FFA6` writer - Filter FIFO assignment for filter 6"]
pub type FFA6_W<'a> = crate::BitWriter<'a, u32, FAFIFOR_SPEC, bool, 6>;
#[doc = "Field `FFA7` reader - Filter FIFO assignment for filter 7"]
pub type FFA7_R = crate::BitReader<bool>;
#[doc = "Field `FFA7` writer - Filter FIFO assignment for filter 7"]
pub type FFA7_W<'a> = crate::BitWriter<'a, u32, FAFIFOR_SPEC, bool, 7>;
#[doc = "Field `FFA8` reader - Filter FIFO assignment for filter 8"]
pub type FFA8_R = crate::BitReader<bool>;
#[doc = "Field `FFA8` writer - Filter FIFO assignment for filter 8"]
pub type FFA8_W<'a> = crate::BitWriter<'a, u32, FAFIFOR_SPEC, bool, 8>;
#[doc = "Field `FFA9` reader - Filter FIFO assignment for filter 9"]
pub type FFA9_R = crate::BitReader<bool>;
#[doc = "Field `FFA9` writer - Filter FIFO assignment for filter 9"]
pub type FFA9_W<'a> = crate::BitWriter<'a, u32, FAFIFOR_SPEC, bool, 9>;
#[doc = "Field `FFA10` reader - Filter FIFO assignment for filter 10"]
pub type FFA10_R = crate::BitReader<bool>;
#[doc = "Field `FFA10` writer - Filter FIFO assignment for filter 10"]
pub type FFA10_W<'a> = crate::BitWriter<'a, u32, FAFIFOR_SPEC, bool, 10>;
#[doc = "Field `FFA11` reader - Filter FIFO assignment for filter 11"]
pub type FFA11_R = crate::BitReader<bool>;
#[doc = "Field `FFA11` writer - Filter FIFO assignment for filter 11"]
pub type FFA11_W<'a> = crate::BitWriter<'a, u32, FAFIFOR_SPEC, bool, 11>;
#[doc = "Field `FFA12` reader - Filter FIFO assignment for filter 12"]
pub type FFA12_R = crate::BitReader<bool>;
#[doc = "Field `FFA12` writer - Filter FIFO assignment for filter 12"]
pub type FFA12_W<'a> = crate::BitWriter<'a, u32, FAFIFOR_SPEC, bool, 12>;
#[doc = "Field `FFA13` reader - Filter FIFO assignment for filter 13"]
pub type FFA13_R = crate::BitReader<bool>;
#[doc = "Field `FFA13` writer - Filter FIFO assignment for filter 13"]
pub type FFA13_W<'a> = crate::BitWriter<'a, u32, FAFIFOR_SPEC, bool, 13>;
impl R {
    #[doc = "Bit 0 - Filter FIFO assignment for filter 0"]
    #[inline(always)]
    pub fn ffa0(&self) -> FFA0_R {
        FFA0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter FIFO assignment for filter 1"]
    #[inline(always)]
    pub fn ffa1(&self) -> FFA1_R {
        FFA1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter FIFO assignment for filter 2"]
    #[inline(always)]
    pub fn ffa2(&self) -> FFA2_R {
        FFA2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter FIFO assignment for filter 3"]
    #[inline(always)]
    pub fn ffa3(&self) -> FFA3_R {
        FFA3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter FIFO assignment for filter 4"]
    #[inline(always)]
    pub fn ffa4(&self) -> FFA4_R {
        FFA4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter FIFO assignment for filter 5"]
    #[inline(always)]
    pub fn ffa5(&self) -> FFA5_R {
        FFA5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter FIFO assignment for filter 6"]
    #[inline(always)]
    pub fn ffa6(&self) -> FFA6_R {
        FFA6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter FIFO assignment for filter 7"]
    #[inline(always)]
    pub fn ffa7(&self) -> FFA7_R {
        FFA7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter FIFO assignment for filter 8"]
    #[inline(always)]
    pub fn ffa8(&self) -> FFA8_R {
        FFA8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter FIFO assignment for filter 9"]
    #[inline(always)]
    pub fn ffa9(&self) -> FFA9_R {
        FFA9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Filter FIFO assignment for filter 10"]
    #[inline(always)]
    pub fn ffa10(&self) -> FFA10_R {
        FFA10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Filter FIFO assignment for filter 11"]
    #[inline(always)]
    pub fn ffa11(&self) -> FFA11_R {
        FFA11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter FIFO assignment for filter 12"]
    #[inline(always)]
    pub fn ffa12(&self) -> FFA12_R {
        FFA12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Filter FIFO assignment for filter 13"]
    #[inline(always)]
    pub fn ffa13(&self) -> FFA13_R {
        FFA13_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter FIFO assignment for filter 0"]
    #[inline(always)]
    pub fn ffa0(&mut self) -> FFA0_W {
        FFA0_W::new(self)
    }
    #[doc = "Bit 1 - Filter FIFO assignment for filter 1"]
    #[inline(always)]
    pub fn ffa1(&mut self) -> FFA1_W {
        FFA1_W::new(self)
    }
    #[doc = "Bit 2 - Filter FIFO assignment for filter 2"]
    #[inline(always)]
    pub fn ffa2(&mut self) -> FFA2_W {
        FFA2_W::new(self)
    }
    #[doc = "Bit 3 - Filter FIFO assignment for filter 3"]
    #[inline(always)]
    pub fn ffa3(&mut self) -> FFA3_W {
        FFA3_W::new(self)
    }
    #[doc = "Bit 4 - Filter FIFO assignment for filter 4"]
    #[inline(always)]
    pub fn ffa4(&mut self) -> FFA4_W {
        FFA4_W::new(self)
    }
    #[doc = "Bit 5 - Filter FIFO assignment for filter 5"]
    #[inline(always)]
    pub fn ffa5(&mut self) -> FFA5_W {
        FFA5_W::new(self)
    }
    #[doc = "Bit 6 - Filter FIFO assignment for filter 6"]
    #[inline(always)]
    pub fn ffa6(&mut self) -> FFA6_W {
        FFA6_W::new(self)
    }
    #[doc = "Bit 7 - Filter FIFO assignment for filter 7"]
    #[inline(always)]
    pub fn ffa7(&mut self) -> FFA7_W {
        FFA7_W::new(self)
    }
    #[doc = "Bit 8 - Filter FIFO assignment for filter 8"]
    #[inline(always)]
    pub fn ffa8(&mut self) -> FFA8_W {
        FFA8_W::new(self)
    }
    #[doc = "Bit 9 - Filter FIFO assignment for filter 9"]
    #[inline(always)]
    pub fn ffa9(&mut self) -> FFA9_W {
        FFA9_W::new(self)
    }
    #[doc = "Bit 10 - Filter FIFO assignment for filter 10"]
    #[inline(always)]
    pub fn ffa10(&mut self) -> FFA10_W {
        FFA10_W::new(self)
    }
    #[doc = "Bit 11 - Filter FIFO assignment for filter 11"]
    #[inline(always)]
    pub fn ffa11(&mut self) -> FFA11_W {
        FFA11_W::new(self)
    }
    #[doc = "Bit 12 - Filter FIFO assignment for filter 12"]
    #[inline(always)]
    pub fn ffa12(&mut self) -> FFA12_W {
        FFA12_W::new(self)
    }
    #[doc = "Bit 13 - Filter FIFO assignment for filter 13"]
    #[inline(always)]
    pub fn ffa13(&mut self) -> FFA13_W {
        FFA13_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FAFIFOR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fafifor](index.html) module"]
pub struct FAFIFOR_SPEC;
impl crate::RegisterSpec for FAFIFOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fafifor::R](R) reader structure"]
impl crate::Readable for FAFIFOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fafifor::W](W) writer structure"]
impl crate::Writable for FAFIFOR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FAFIFOR to value 0"]
impl crate::Resettable for FAFIFOR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}