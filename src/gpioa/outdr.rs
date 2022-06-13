#[doc = "Register `OUTDR` reader"]
pub struct R(crate::R<OUTDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTDR` writer"]
pub struct W(crate::W<OUTDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTDR_SPEC>;
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
impl From<crate::W<OUTDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ODR0` reader - Port output data"]
pub type ODR0_R = crate::BitReader<bool>;
#[doc = "Field `ODR0` writer - Port output data"]
pub type ODR0_W<'a> = crate::BitWriter<'a, u32, OUTDR_SPEC, bool, 0>;
#[doc = "Field `ODR1` reader - Port output data"]
pub type ODR1_R = crate::BitReader<bool>;
#[doc = "Field `ODR1` writer - Port output data"]
pub type ODR1_W<'a> = crate::BitWriter<'a, u32, OUTDR_SPEC, bool, 1>;
#[doc = "Field `ODR2` reader - Port output data"]
pub type ODR2_R = crate::BitReader<bool>;
#[doc = "Field `ODR2` writer - Port output data"]
pub type ODR2_W<'a> = crate::BitWriter<'a, u32, OUTDR_SPEC, bool, 2>;
#[doc = "Field `ODR3` reader - Port output data"]
pub type ODR3_R = crate::BitReader<bool>;
#[doc = "Field `ODR3` writer - Port output data"]
pub type ODR3_W<'a> = crate::BitWriter<'a, u32, OUTDR_SPEC, bool, 3>;
#[doc = "Field `ODR4` reader - Port output data"]
pub type ODR4_R = crate::BitReader<bool>;
#[doc = "Field `ODR4` writer - Port output data"]
pub type ODR4_W<'a> = crate::BitWriter<'a, u32, OUTDR_SPEC, bool, 4>;
#[doc = "Field `ODR5` reader - Port output data"]
pub type ODR5_R = crate::BitReader<bool>;
#[doc = "Field `ODR5` writer - Port output data"]
pub type ODR5_W<'a> = crate::BitWriter<'a, u32, OUTDR_SPEC, bool, 5>;
#[doc = "Field `ODR6` reader - Port output data"]
pub type ODR6_R = crate::BitReader<bool>;
#[doc = "Field `ODR6` writer - Port output data"]
pub type ODR6_W<'a> = crate::BitWriter<'a, u32, OUTDR_SPEC, bool, 6>;
#[doc = "Field `ODR7` reader - Port output data"]
pub type ODR7_R = crate::BitReader<bool>;
#[doc = "Field `ODR7` writer - Port output data"]
pub type ODR7_W<'a> = crate::BitWriter<'a, u32, OUTDR_SPEC, bool, 7>;
#[doc = "Field `ODR8` reader - Port output data"]
pub type ODR8_R = crate::BitReader<bool>;
#[doc = "Field `ODR8` writer - Port output data"]
pub type ODR8_W<'a> = crate::BitWriter<'a, u32, OUTDR_SPEC, bool, 8>;
#[doc = "Field `ODR9` reader - Port output data"]
pub type ODR9_R = crate::BitReader<bool>;
#[doc = "Field `ODR9` writer - Port output data"]
pub type ODR9_W<'a> = crate::BitWriter<'a, u32, OUTDR_SPEC, bool, 9>;
#[doc = "Field `ODR10` reader - Port output data"]
pub type ODR10_R = crate::BitReader<bool>;
#[doc = "Field `ODR10` writer - Port output data"]
pub type ODR10_W<'a> = crate::BitWriter<'a, u32, OUTDR_SPEC, bool, 10>;
#[doc = "Field `ODR11` reader - Port output data"]
pub type ODR11_R = crate::BitReader<bool>;
#[doc = "Field `ODR11` writer - Port output data"]
pub type ODR11_W<'a> = crate::BitWriter<'a, u32, OUTDR_SPEC, bool, 11>;
#[doc = "Field `ODR12` reader - Port output data"]
pub type ODR12_R = crate::BitReader<bool>;
#[doc = "Field `ODR12` writer - Port output data"]
pub type ODR12_W<'a> = crate::BitWriter<'a, u32, OUTDR_SPEC, bool, 12>;
#[doc = "Field `ODR13` reader - Port output data"]
pub type ODR13_R = crate::BitReader<bool>;
#[doc = "Field `ODR13` writer - Port output data"]
pub type ODR13_W<'a> = crate::BitWriter<'a, u32, OUTDR_SPEC, bool, 13>;
#[doc = "Field `ODR14` reader - Port output data"]
pub type ODR14_R = crate::BitReader<bool>;
#[doc = "Field `ODR14` writer - Port output data"]
pub type ODR14_W<'a> = crate::BitWriter<'a, u32, OUTDR_SPEC, bool, 14>;
#[doc = "Field `ODR15` reader - Port output data"]
pub type ODR15_R = crate::BitReader<bool>;
#[doc = "Field `ODR15` writer - Port output data"]
pub type ODR15_W<'a> = crate::BitWriter<'a, u32, OUTDR_SPEC, bool, 15>;
impl R {
    #[doc = "Bit 0 - Port output data"]
    #[inline(always)]
    pub fn odr0(&self) -> ODR0_R {
        ODR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port output data"]
    #[inline(always)]
    pub fn odr1(&self) -> ODR1_R {
        ODR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port output data"]
    #[inline(always)]
    pub fn odr2(&self) -> ODR2_R {
        ODR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port output data"]
    #[inline(always)]
    pub fn odr3(&self) -> ODR3_R {
        ODR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port output data"]
    #[inline(always)]
    pub fn odr4(&self) -> ODR4_R {
        ODR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port output data"]
    #[inline(always)]
    pub fn odr5(&self) -> ODR5_R {
        ODR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port output data"]
    #[inline(always)]
    pub fn odr6(&self) -> ODR6_R {
        ODR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port output data"]
    #[inline(always)]
    pub fn odr7(&self) -> ODR7_R {
        ODR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port output data"]
    #[inline(always)]
    pub fn odr8(&self) -> ODR8_R {
        ODR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port output data"]
    #[inline(always)]
    pub fn odr9(&self) -> ODR9_R {
        ODR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port output data"]
    #[inline(always)]
    pub fn odr10(&self) -> ODR10_R {
        ODR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port output data"]
    #[inline(always)]
    pub fn odr11(&self) -> ODR11_R {
        ODR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port output data"]
    #[inline(always)]
    pub fn odr12(&self) -> ODR12_R {
        ODR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port output data"]
    #[inline(always)]
    pub fn odr13(&self) -> ODR13_R {
        ODR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port output data"]
    #[inline(always)]
    pub fn odr14(&self) -> ODR14_R {
        ODR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port output data"]
    #[inline(always)]
    pub fn odr15(&self) -> ODR15_R {
        ODR15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port output data"]
    #[inline(always)]
    pub fn odr0(&mut self) -> ODR0_W {
        ODR0_W::new(self)
    }
    #[doc = "Bit 1 - Port output data"]
    #[inline(always)]
    pub fn odr1(&mut self) -> ODR1_W {
        ODR1_W::new(self)
    }
    #[doc = "Bit 2 - Port output data"]
    #[inline(always)]
    pub fn odr2(&mut self) -> ODR2_W {
        ODR2_W::new(self)
    }
    #[doc = "Bit 3 - Port output data"]
    #[inline(always)]
    pub fn odr3(&mut self) -> ODR3_W {
        ODR3_W::new(self)
    }
    #[doc = "Bit 4 - Port output data"]
    #[inline(always)]
    pub fn odr4(&mut self) -> ODR4_W {
        ODR4_W::new(self)
    }
    #[doc = "Bit 5 - Port output data"]
    #[inline(always)]
    pub fn odr5(&mut self) -> ODR5_W {
        ODR5_W::new(self)
    }
    #[doc = "Bit 6 - Port output data"]
    #[inline(always)]
    pub fn odr6(&mut self) -> ODR6_W {
        ODR6_W::new(self)
    }
    #[doc = "Bit 7 - Port output data"]
    #[inline(always)]
    pub fn odr7(&mut self) -> ODR7_W {
        ODR7_W::new(self)
    }
    #[doc = "Bit 8 - Port output data"]
    #[inline(always)]
    pub fn odr8(&mut self) -> ODR8_W {
        ODR8_W::new(self)
    }
    #[doc = "Bit 9 - Port output data"]
    #[inline(always)]
    pub fn odr9(&mut self) -> ODR9_W {
        ODR9_W::new(self)
    }
    #[doc = "Bit 10 - Port output data"]
    #[inline(always)]
    pub fn odr10(&mut self) -> ODR10_W {
        ODR10_W::new(self)
    }
    #[doc = "Bit 11 - Port output data"]
    #[inline(always)]
    pub fn odr11(&mut self) -> ODR11_W {
        ODR11_W::new(self)
    }
    #[doc = "Bit 12 - Port output data"]
    #[inline(always)]
    pub fn odr12(&mut self) -> ODR12_W {
        ODR12_W::new(self)
    }
    #[doc = "Bit 13 - Port output data"]
    #[inline(always)]
    pub fn odr13(&mut self) -> ODR13_W {
        ODR13_W::new(self)
    }
    #[doc = "Bit 14 - Port output data"]
    #[inline(always)]
    pub fn odr14(&mut self) -> ODR14_W {
        ODR14_W::new(self)
    }
    #[doc = "Bit 15 - Port output data"]
    #[inline(always)]
    pub fn odr15(&mut self) -> ODR15_W {
        ODR15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port output data register (GPIOn_OUTDR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outdr](index.html) module"]
pub struct OUTDR_SPEC;
impl crate::RegisterSpec for OUTDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outdr::R](R) reader structure"]
impl crate::Readable for OUTDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [outdr::W](W) writer structure"]
impl crate::Writable for OUTDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUTDR to value 0"]
impl crate::Resettable for OUTDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}