#[doc = "Register `F7R1` reader"]
pub struct R(crate::R<F7R1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<F7R1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<F7R1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<F7R1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `F7R1` writer"]
pub struct W(crate::W<F7R1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<F7R1_SPEC>;
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
impl From<crate::W<F7R1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<F7R1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FB0` reader - Filter bits"]
pub type FB0_R = crate::BitReader<bool>;
#[doc = "Field `FB0` writer - Filter bits"]
pub type FB0_W<'a> = crate::BitWriter<'a, u32, F7R1_SPEC, bool, 0>;
#[doc = "Field `FB1` reader - Filter bits"]
pub type FB1_R = crate::BitReader<bool>;
#[doc = "Field `FB1` writer - Filter bits"]
pub type FB1_W<'a> = crate::BitWriter<'a, u32, F7R1_SPEC, bool, 1>;
#[doc = "Field `FB2` reader - Filter bits"]
pub type FB2_R = crate::BitReader<bool>;
#[doc = "Field `FB2` writer - Filter bits"]
pub type FB2_W<'a> = crate::BitWriter<'a, u32, F7R1_SPEC, bool, 2>;
#[doc = "Field `FB3` reader - Filter bits"]
pub type FB3_R = crate::BitReader<bool>;
#[doc = "Field `FB3` writer - Filter bits"]
pub type FB3_W<'a> = crate::BitWriter<'a, u32, F7R1_SPEC, bool, 3>;
#[doc = "Field `FB4` reader - Filter bits"]
pub type FB4_R = crate::BitReader<bool>;
#[doc = "Field `FB4` writer - Filter bits"]
pub type FB4_W<'a> = crate::BitWriter<'a, u32, F7R1_SPEC, bool, 4>;
#[doc = "Field `FB5` reader - Filter bits"]
pub type FB5_R = crate::BitReader<bool>;
#[doc = "Field `FB5` writer - Filter bits"]
pub type FB5_W<'a> = crate::BitWriter<'a, u32, F7R1_SPEC, bool, 5>;
#[doc = "Field `FB6` reader - Filter bits"]
pub type FB6_R = crate::BitReader<bool>;
#[doc = "Field `FB6` writer - Filter bits"]
pub type FB6_W<'a> = crate::BitWriter<'a, u32, F7R1_SPEC, bool, 6>;
#[doc = "Field `FB7` reader - Filter bits"]
pub type FB7_R = crate::BitReader<bool>;
#[doc = "Field `FB7` writer - Filter bits"]
pub type FB7_W<'a> = crate::BitWriter<'a, u32, F7R1_SPEC, bool, 7>;
#[doc = "Field `FB8` reader - Filter bits"]
pub type FB8_R = crate::BitReader<bool>;
#[doc = "Field `FB8` writer - Filter bits"]
pub type FB8_W<'a> = crate::BitWriter<'a, u32, F7R1_SPEC, bool, 8>;
#[doc = "Field `FB9` reader - Filter bits"]
pub type FB9_R = crate::BitReader<bool>;
#[doc = "Field `FB9` writer - Filter bits"]
pub type FB9_W<'a> = crate::BitWriter<'a, u32, F7R1_SPEC, bool, 9>;
#[doc = "Field `FB10` reader - Filter bits"]
pub type FB10_R = crate::BitReader<bool>;
#[doc = "Field `FB10` writer - Filter bits"]
pub type FB10_W<'a> = crate::BitWriter<'a, u32, F7R1_SPEC, bool, 10>;
#[doc = "Field `FB11` reader - Filter bits"]
pub type FB11_R = crate::BitReader<bool>;
#[doc = "Field `FB11` writer - Filter bits"]
pub type FB11_W<'a> = crate::BitWriter<'a, u32, F7R1_SPEC, bool, 11>;
#[doc = "Field `FB12` reader - Filter bits"]
pub type FB12_R = crate::BitReader<bool>;
#[doc = "Field `FB12` writer - Filter bits"]
pub type FB12_W<'a> = crate::BitWriter<'a, u32, F7R1_SPEC, bool, 12>;
#[doc = "Field `FB13` reader - Filter bits"]
pub type FB13_R = crate::BitReader<bool>;
#[doc = "Field `FB13` writer - Filter bits"]
pub type FB13_W<'a> = crate::BitWriter<'a, u32, F7R1_SPEC, bool, 13>;
#[doc = "Field `FB14` reader - Filter bits"]
pub type FB14_R = crate::BitReader<bool>;
#[doc = "Field `FB14` writer - Filter bits"]
pub type FB14_W<'a> = crate::BitWriter<'a, u32, F7R1_SPEC, bool, 14>;
#[doc = "Field `FB15` reader - Filter bits"]
pub type FB15_R = crate::BitReader<bool>;
#[doc = "Field `FB15` writer - Filter bits"]
pub type FB15_W<'a> = crate::BitWriter<'a, u32, F7R1_SPEC, bool, 15>;
#[doc = "Field `FB16` reader - Filter bits"]
pub type FB16_R = crate::BitReader<bool>;
#[doc = "Field `FB16` writer - Filter bits"]
pub type FB16_W<'a> = crate::BitWriter<'a, u32, F7R1_SPEC, bool, 16>;
#[doc = "Field `FB17` reader - Filter bits"]
pub type FB17_R = crate::BitReader<bool>;
#[doc = "Field `FB17` writer - Filter bits"]
pub type FB17_W<'a> = crate::BitWriter<'a, u32, F7R1_SPEC, bool, 17>;
#[doc = "Field `FB18` reader - Filter bits"]
pub type FB18_R = crate::BitReader<bool>;
#[doc = "Field `FB18` writer - Filter bits"]
pub type FB18_W<'a> = crate::BitWriter<'a, u32, F7R1_SPEC, bool, 18>;
#[doc = "Field `FB19` reader - Filter bits"]
pub type FB19_R = crate::BitReader<bool>;
#[doc = "Field `FB19` writer - Filter bits"]
pub type FB19_W<'a> = crate::BitWriter<'a, u32, F7R1_SPEC, bool, 19>;
#[doc = "Field `FB20` reader - Filter bits"]
pub type FB20_R = crate::BitReader<bool>;
#[doc = "Field `FB20` writer - Filter bits"]
pub type FB20_W<'a> = crate::BitWriter<'a, u32, F7R1_SPEC, bool, 20>;
#[doc = "Field `FB21` reader - Filter bits"]
pub type FB21_R = crate::BitReader<bool>;
#[doc = "Field `FB21` writer - Filter bits"]
pub type FB21_W<'a> = crate::BitWriter<'a, u32, F7R1_SPEC, bool, 21>;
#[doc = "Field `FB22` reader - Filter bits"]
pub type FB22_R = crate::BitReader<bool>;
#[doc = "Field `FB22` writer - Filter bits"]
pub type FB22_W<'a> = crate::BitWriter<'a, u32, F7R1_SPEC, bool, 22>;
#[doc = "Field `FB23` reader - Filter bits"]
pub type FB23_R = crate::BitReader<bool>;
#[doc = "Field `FB23` writer - Filter bits"]
pub type FB23_W<'a> = crate::BitWriter<'a, u32, F7R1_SPEC, bool, 23>;
#[doc = "Field `FB24` reader - Filter bits"]
pub type FB24_R = crate::BitReader<bool>;
#[doc = "Field `FB24` writer - Filter bits"]
pub type FB24_W<'a> = crate::BitWriter<'a, u32, F7R1_SPEC, bool, 24>;
#[doc = "Field `FB25` reader - Filter bits"]
pub type FB25_R = crate::BitReader<bool>;
#[doc = "Field `FB25` writer - Filter bits"]
pub type FB25_W<'a> = crate::BitWriter<'a, u32, F7R1_SPEC, bool, 25>;
#[doc = "Field `FB26` reader - Filter bits"]
pub type FB26_R = crate::BitReader<bool>;
#[doc = "Field `FB26` writer - Filter bits"]
pub type FB26_W<'a> = crate::BitWriter<'a, u32, F7R1_SPEC, bool, 26>;
#[doc = "Field `FB27` reader - Filter bits"]
pub type FB27_R = crate::BitReader<bool>;
#[doc = "Field `FB27` writer - Filter bits"]
pub type FB27_W<'a> = crate::BitWriter<'a, u32, F7R1_SPEC, bool, 27>;
#[doc = "Field `FB28` reader - Filter bits"]
pub type FB28_R = crate::BitReader<bool>;
#[doc = "Field `FB28` writer - Filter bits"]
pub type FB28_W<'a> = crate::BitWriter<'a, u32, F7R1_SPEC, bool, 28>;
#[doc = "Field `FB29` reader - Filter bits"]
pub type FB29_R = crate::BitReader<bool>;
#[doc = "Field `FB29` writer - Filter bits"]
pub type FB29_W<'a> = crate::BitWriter<'a, u32, F7R1_SPEC, bool, 29>;
#[doc = "Field `FB30` reader - Filter bits"]
pub type FB30_R = crate::BitReader<bool>;
#[doc = "Field `FB30` writer - Filter bits"]
pub type FB30_W<'a> = crate::BitWriter<'a, u32, F7R1_SPEC, bool, 30>;
#[doc = "Field `FB31` reader - Filter bits"]
pub type FB31_R = crate::BitReader<bool>;
#[doc = "Field `FB31` writer - Filter bits"]
pub type FB31_W<'a> = crate::BitWriter<'a, u32, F7R1_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - Filter bits"]
    #[inline(always)]
    pub fn fb0(&self) -> FB0_R {
        FB0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter bits"]
    #[inline(always)]
    pub fn fb1(&self) -> FB1_R {
        FB1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter bits"]
    #[inline(always)]
    pub fn fb2(&self) -> FB2_R {
        FB2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter bits"]
    #[inline(always)]
    pub fn fb3(&self) -> FB3_R {
        FB3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter bits"]
    #[inline(always)]
    pub fn fb4(&self) -> FB4_R {
        FB4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter bits"]
    #[inline(always)]
    pub fn fb5(&self) -> FB5_R {
        FB5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter bits"]
    #[inline(always)]
    pub fn fb6(&self) -> FB6_R {
        FB6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter bits"]
    #[inline(always)]
    pub fn fb7(&self) -> FB7_R {
        FB7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter bits"]
    #[inline(always)]
    pub fn fb8(&self) -> FB8_R {
        FB8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter bits"]
    #[inline(always)]
    pub fn fb9(&self) -> FB9_R {
        FB9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Filter bits"]
    #[inline(always)]
    pub fn fb10(&self) -> FB10_R {
        FB10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Filter bits"]
    #[inline(always)]
    pub fn fb11(&self) -> FB11_R {
        FB11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter bits"]
    #[inline(always)]
    pub fn fb12(&self) -> FB12_R {
        FB12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Filter bits"]
    #[inline(always)]
    pub fn fb13(&self) -> FB13_R {
        FB13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Filter bits"]
    #[inline(always)]
    pub fn fb14(&self) -> FB14_R {
        FB14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Filter bits"]
    #[inline(always)]
    pub fn fb15(&self) -> FB15_R {
        FB15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Filter bits"]
    #[inline(always)]
    pub fn fb16(&self) -> FB16_R {
        FB16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Filter bits"]
    #[inline(always)]
    pub fn fb17(&self) -> FB17_R {
        FB17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Filter bits"]
    #[inline(always)]
    pub fn fb18(&self) -> FB18_R {
        FB18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Filter bits"]
    #[inline(always)]
    pub fn fb19(&self) -> FB19_R {
        FB19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Filter bits"]
    #[inline(always)]
    pub fn fb20(&self) -> FB20_R {
        FB20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Filter bits"]
    #[inline(always)]
    pub fn fb21(&self) -> FB21_R {
        FB21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Filter bits"]
    #[inline(always)]
    pub fn fb22(&self) -> FB22_R {
        FB22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Filter bits"]
    #[inline(always)]
    pub fn fb23(&self) -> FB23_R {
        FB23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Filter bits"]
    #[inline(always)]
    pub fn fb24(&self) -> FB24_R {
        FB24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Filter bits"]
    #[inline(always)]
    pub fn fb25(&self) -> FB25_R {
        FB25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Filter bits"]
    #[inline(always)]
    pub fn fb26(&self) -> FB26_R {
        FB26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Filter bits"]
    #[inline(always)]
    pub fn fb27(&self) -> FB27_R {
        FB27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Filter bits"]
    #[inline(always)]
    pub fn fb28(&self) -> FB28_R {
        FB28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Filter bits"]
    #[inline(always)]
    pub fn fb29(&self) -> FB29_R {
        FB29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Filter bits"]
    #[inline(always)]
    pub fn fb30(&self) -> FB30_R {
        FB30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Filter bits"]
    #[inline(always)]
    pub fn fb31(&self) -> FB31_R {
        FB31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter bits"]
    #[inline(always)]
    pub fn fb0(&mut self) -> FB0_W {
        FB0_W::new(self)
    }
    #[doc = "Bit 1 - Filter bits"]
    #[inline(always)]
    pub fn fb1(&mut self) -> FB1_W {
        FB1_W::new(self)
    }
    #[doc = "Bit 2 - Filter bits"]
    #[inline(always)]
    pub fn fb2(&mut self) -> FB2_W {
        FB2_W::new(self)
    }
    #[doc = "Bit 3 - Filter bits"]
    #[inline(always)]
    pub fn fb3(&mut self) -> FB3_W {
        FB3_W::new(self)
    }
    #[doc = "Bit 4 - Filter bits"]
    #[inline(always)]
    pub fn fb4(&mut self) -> FB4_W {
        FB4_W::new(self)
    }
    #[doc = "Bit 5 - Filter bits"]
    #[inline(always)]
    pub fn fb5(&mut self) -> FB5_W {
        FB5_W::new(self)
    }
    #[doc = "Bit 6 - Filter bits"]
    #[inline(always)]
    pub fn fb6(&mut self) -> FB6_W {
        FB6_W::new(self)
    }
    #[doc = "Bit 7 - Filter bits"]
    #[inline(always)]
    pub fn fb7(&mut self) -> FB7_W {
        FB7_W::new(self)
    }
    #[doc = "Bit 8 - Filter bits"]
    #[inline(always)]
    pub fn fb8(&mut self) -> FB8_W {
        FB8_W::new(self)
    }
    #[doc = "Bit 9 - Filter bits"]
    #[inline(always)]
    pub fn fb9(&mut self) -> FB9_W {
        FB9_W::new(self)
    }
    #[doc = "Bit 10 - Filter bits"]
    #[inline(always)]
    pub fn fb10(&mut self) -> FB10_W {
        FB10_W::new(self)
    }
    #[doc = "Bit 11 - Filter bits"]
    #[inline(always)]
    pub fn fb11(&mut self) -> FB11_W {
        FB11_W::new(self)
    }
    #[doc = "Bit 12 - Filter bits"]
    #[inline(always)]
    pub fn fb12(&mut self) -> FB12_W {
        FB12_W::new(self)
    }
    #[doc = "Bit 13 - Filter bits"]
    #[inline(always)]
    pub fn fb13(&mut self) -> FB13_W {
        FB13_W::new(self)
    }
    #[doc = "Bit 14 - Filter bits"]
    #[inline(always)]
    pub fn fb14(&mut self) -> FB14_W {
        FB14_W::new(self)
    }
    #[doc = "Bit 15 - Filter bits"]
    #[inline(always)]
    pub fn fb15(&mut self) -> FB15_W {
        FB15_W::new(self)
    }
    #[doc = "Bit 16 - Filter bits"]
    #[inline(always)]
    pub fn fb16(&mut self) -> FB16_W {
        FB16_W::new(self)
    }
    #[doc = "Bit 17 - Filter bits"]
    #[inline(always)]
    pub fn fb17(&mut self) -> FB17_W {
        FB17_W::new(self)
    }
    #[doc = "Bit 18 - Filter bits"]
    #[inline(always)]
    pub fn fb18(&mut self) -> FB18_W {
        FB18_W::new(self)
    }
    #[doc = "Bit 19 - Filter bits"]
    #[inline(always)]
    pub fn fb19(&mut self) -> FB19_W {
        FB19_W::new(self)
    }
    #[doc = "Bit 20 - Filter bits"]
    #[inline(always)]
    pub fn fb20(&mut self) -> FB20_W {
        FB20_W::new(self)
    }
    #[doc = "Bit 21 - Filter bits"]
    #[inline(always)]
    pub fn fb21(&mut self) -> FB21_W {
        FB21_W::new(self)
    }
    #[doc = "Bit 22 - Filter bits"]
    #[inline(always)]
    pub fn fb22(&mut self) -> FB22_W {
        FB22_W::new(self)
    }
    #[doc = "Bit 23 - Filter bits"]
    #[inline(always)]
    pub fn fb23(&mut self) -> FB23_W {
        FB23_W::new(self)
    }
    #[doc = "Bit 24 - Filter bits"]
    #[inline(always)]
    pub fn fb24(&mut self) -> FB24_W {
        FB24_W::new(self)
    }
    #[doc = "Bit 25 - Filter bits"]
    #[inline(always)]
    pub fn fb25(&mut self) -> FB25_W {
        FB25_W::new(self)
    }
    #[doc = "Bit 26 - Filter bits"]
    #[inline(always)]
    pub fn fb26(&mut self) -> FB26_W {
        FB26_W::new(self)
    }
    #[doc = "Bit 27 - Filter bits"]
    #[inline(always)]
    pub fn fb27(&mut self) -> FB27_W {
        FB27_W::new(self)
    }
    #[doc = "Bit 28 - Filter bits"]
    #[inline(always)]
    pub fn fb28(&mut self) -> FB28_W {
        FB28_W::new(self)
    }
    #[doc = "Bit 29 - Filter bits"]
    #[inline(always)]
    pub fn fb29(&mut self) -> FB29_W {
        FB29_W::new(self)
    }
    #[doc = "Bit 30 - Filter bits"]
    #[inline(always)]
    pub fn fb30(&mut self) -> FB30_W {
        FB30_W::new(self)
    }
    #[doc = "Bit 31 - Filter bits"]
    #[inline(always)]
    pub fn fb31(&mut self) -> FB31_W {
        FB31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Filter bank 7 register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f7r1](index.html) module"]
pub struct F7R1_SPEC;
impl crate::RegisterSpec for F7R1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [f7r1::R](R) reader structure"]
impl crate::Readable for F7R1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [f7r1::W](W) writer structure"]
impl crate::Writable for F7R1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets F7R1 to value 0"]
impl crate::Resettable for F7R1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}