#[doc = "Register `LCKR` reader"]
pub struct R(crate::R<LCKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCKR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCKR` writer"]
pub struct W(crate::W<LCKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCKR_SPEC>;
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
impl From<crate::W<LCKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCKR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCK0` reader - Port A Lock bit 0"]
pub type LCK0_R = crate::BitReader<bool>;
#[doc = "Field `LCK0` writer - Port A Lock bit 0"]
pub type LCK0_W<'a> = crate::BitWriter<'a, u32, LCKR_SPEC, bool, 0>;
#[doc = "Field `LCK1` reader - Port A Lock bit 1"]
pub type LCK1_R = crate::BitReader<bool>;
#[doc = "Field `LCK1` writer - Port A Lock bit 1"]
pub type LCK1_W<'a> = crate::BitWriter<'a, u32, LCKR_SPEC, bool, 1>;
#[doc = "Field `LCK2` reader - Port A Lock bit 2"]
pub type LCK2_R = crate::BitReader<bool>;
#[doc = "Field `LCK2` writer - Port A Lock bit 2"]
pub type LCK2_W<'a> = crate::BitWriter<'a, u32, LCKR_SPEC, bool, 2>;
#[doc = "Field `LCK3` reader - Port A Lock bit 3"]
pub type LCK3_R = crate::BitReader<bool>;
#[doc = "Field `LCK3` writer - Port A Lock bit 3"]
pub type LCK3_W<'a> = crate::BitWriter<'a, u32, LCKR_SPEC, bool, 3>;
#[doc = "Field `LCK4` reader - Port A Lock bit 4"]
pub type LCK4_R = crate::BitReader<bool>;
#[doc = "Field `LCK4` writer - Port A Lock bit 4"]
pub type LCK4_W<'a> = crate::BitWriter<'a, u32, LCKR_SPEC, bool, 4>;
#[doc = "Field `LCK5` reader - Port A Lock bit 5"]
pub type LCK5_R = crate::BitReader<bool>;
#[doc = "Field `LCK5` writer - Port A Lock bit 5"]
pub type LCK5_W<'a> = crate::BitWriter<'a, u32, LCKR_SPEC, bool, 5>;
#[doc = "Field `LCK6` reader - Port A Lock bit 6"]
pub type LCK6_R = crate::BitReader<bool>;
#[doc = "Field `LCK6` writer - Port A Lock bit 6"]
pub type LCK6_W<'a> = crate::BitWriter<'a, u32, LCKR_SPEC, bool, 6>;
#[doc = "Field `LCK7` reader - Port A Lock bit 7"]
pub type LCK7_R = crate::BitReader<bool>;
#[doc = "Field `LCK7` writer - Port A Lock bit 7"]
pub type LCK7_W<'a> = crate::BitWriter<'a, u32, LCKR_SPEC, bool, 7>;
#[doc = "Field `LCK8` reader - Port A Lock bit 8"]
pub type LCK8_R = crate::BitReader<bool>;
#[doc = "Field `LCK8` writer - Port A Lock bit 8"]
pub type LCK8_W<'a> = crate::BitWriter<'a, u32, LCKR_SPEC, bool, 8>;
#[doc = "Field `LCK9` reader - Port A Lock bit 9"]
pub type LCK9_R = crate::BitReader<bool>;
#[doc = "Field `LCK9` writer - Port A Lock bit 9"]
pub type LCK9_W<'a> = crate::BitWriter<'a, u32, LCKR_SPEC, bool, 9>;
#[doc = "Field `LCK10` reader - Port A Lock bit 10"]
pub type LCK10_R = crate::BitReader<bool>;
#[doc = "Field `LCK10` writer - Port A Lock bit 10"]
pub type LCK10_W<'a> = crate::BitWriter<'a, u32, LCKR_SPEC, bool, 10>;
#[doc = "Field `LCK11` reader - Port A Lock bit 11"]
pub type LCK11_R = crate::BitReader<bool>;
#[doc = "Field `LCK11` writer - Port A Lock bit 11"]
pub type LCK11_W<'a> = crate::BitWriter<'a, u32, LCKR_SPEC, bool, 11>;
#[doc = "Field `LCK12` reader - Port A Lock bit 12"]
pub type LCK12_R = crate::BitReader<bool>;
#[doc = "Field `LCK12` writer - Port A Lock bit 12"]
pub type LCK12_W<'a> = crate::BitWriter<'a, u32, LCKR_SPEC, bool, 12>;
#[doc = "Field `LCK13` reader - Port A Lock bit 13"]
pub type LCK13_R = crate::BitReader<bool>;
#[doc = "Field `LCK13` writer - Port A Lock bit 13"]
pub type LCK13_W<'a> = crate::BitWriter<'a, u32, LCKR_SPEC, bool, 13>;
#[doc = "Field `LCK14` reader - Port A Lock bit 14"]
pub type LCK14_R = crate::BitReader<bool>;
#[doc = "Field `LCK14` writer - Port A Lock bit 14"]
pub type LCK14_W<'a> = crate::BitWriter<'a, u32, LCKR_SPEC, bool, 14>;
#[doc = "Field `LCK15` reader - Port A Lock bit 15"]
pub type LCK15_R = crate::BitReader<bool>;
#[doc = "Field `LCK15` writer - Port A Lock bit 15"]
pub type LCK15_W<'a> = crate::BitWriter<'a, u32, LCKR_SPEC, bool, 15>;
#[doc = "Field `LCKK` reader - Lock key"]
pub type LCKK_R = crate::BitReader<bool>;
#[doc = "Field `LCKK` writer - Lock key"]
pub type LCKK_W<'a> = crate::BitWriter<'a, u32, LCKR_SPEC, bool, 16>;
impl R {
    #[doc = "Bit 0 - Port A Lock bit 0"]
    #[inline(always)]
    pub fn lck0(&self) -> LCK0_R {
        LCK0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port A Lock bit 1"]
    #[inline(always)]
    pub fn lck1(&self) -> LCK1_R {
        LCK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port A Lock bit 2"]
    #[inline(always)]
    pub fn lck2(&self) -> LCK2_R {
        LCK2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port A Lock bit 3"]
    #[inline(always)]
    pub fn lck3(&self) -> LCK3_R {
        LCK3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port A Lock bit 4"]
    #[inline(always)]
    pub fn lck4(&self) -> LCK4_R {
        LCK4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port A Lock bit 5"]
    #[inline(always)]
    pub fn lck5(&self) -> LCK5_R {
        LCK5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port A Lock bit 6"]
    #[inline(always)]
    pub fn lck6(&self) -> LCK6_R {
        LCK6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port A Lock bit 7"]
    #[inline(always)]
    pub fn lck7(&self) -> LCK7_R {
        LCK7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port A Lock bit 8"]
    #[inline(always)]
    pub fn lck8(&self) -> LCK8_R {
        LCK8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port A Lock bit 9"]
    #[inline(always)]
    pub fn lck9(&self) -> LCK9_R {
        LCK9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port A Lock bit 10"]
    #[inline(always)]
    pub fn lck10(&self) -> LCK10_R {
        LCK10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port A Lock bit 11"]
    #[inline(always)]
    pub fn lck11(&self) -> LCK11_R {
        LCK11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port A Lock bit 12"]
    #[inline(always)]
    pub fn lck12(&self) -> LCK12_R {
        LCK12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port A Lock bit 13"]
    #[inline(always)]
    pub fn lck13(&self) -> LCK13_R {
        LCK13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port A Lock bit 14"]
    #[inline(always)]
    pub fn lck14(&self) -> LCK14_R {
        LCK14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port A Lock bit 15"]
    #[inline(always)]
    pub fn lck15(&self) -> LCK15_R {
        LCK15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Lock key"]
    #[inline(always)]
    pub fn lckk(&self) -> LCKK_R {
        LCKK_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port A Lock bit 0"]
    #[inline(always)]
    pub fn lck0(&mut self) -> LCK0_W {
        LCK0_W::new(self)
    }
    #[doc = "Bit 1 - Port A Lock bit 1"]
    #[inline(always)]
    pub fn lck1(&mut self) -> LCK1_W {
        LCK1_W::new(self)
    }
    #[doc = "Bit 2 - Port A Lock bit 2"]
    #[inline(always)]
    pub fn lck2(&mut self) -> LCK2_W {
        LCK2_W::new(self)
    }
    #[doc = "Bit 3 - Port A Lock bit 3"]
    #[inline(always)]
    pub fn lck3(&mut self) -> LCK3_W {
        LCK3_W::new(self)
    }
    #[doc = "Bit 4 - Port A Lock bit 4"]
    #[inline(always)]
    pub fn lck4(&mut self) -> LCK4_W {
        LCK4_W::new(self)
    }
    #[doc = "Bit 5 - Port A Lock bit 5"]
    #[inline(always)]
    pub fn lck5(&mut self) -> LCK5_W {
        LCK5_W::new(self)
    }
    #[doc = "Bit 6 - Port A Lock bit 6"]
    #[inline(always)]
    pub fn lck6(&mut self) -> LCK6_W {
        LCK6_W::new(self)
    }
    #[doc = "Bit 7 - Port A Lock bit 7"]
    #[inline(always)]
    pub fn lck7(&mut self) -> LCK7_W {
        LCK7_W::new(self)
    }
    #[doc = "Bit 8 - Port A Lock bit 8"]
    #[inline(always)]
    pub fn lck8(&mut self) -> LCK8_W {
        LCK8_W::new(self)
    }
    #[doc = "Bit 9 - Port A Lock bit 9"]
    #[inline(always)]
    pub fn lck9(&mut self) -> LCK9_W {
        LCK9_W::new(self)
    }
    #[doc = "Bit 10 - Port A Lock bit 10"]
    #[inline(always)]
    pub fn lck10(&mut self) -> LCK10_W {
        LCK10_W::new(self)
    }
    #[doc = "Bit 11 - Port A Lock bit 11"]
    #[inline(always)]
    pub fn lck11(&mut self) -> LCK11_W {
        LCK11_W::new(self)
    }
    #[doc = "Bit 12 - Port A Lock bit 12"]
    #[inline(always)]
    pub fn lck12(&mut self) -> LCK12_W {
        LCK12_W::new(self)
    }
    #[doc = "Bit 13 - Port A Lock bit 13"]
    #[inline(always)]
    pub fn lck13(&mut self) -> LCK13_W {
        LCK13_W::new(self)
    }
    #[doc = "Bit 14 - Port A Lock bit 14"]
    #[inline(always)]
    pub fn lck14(&mut self) -> LCK14_W {
        LCK14_W::new(self)
    }
    #[doc = "Bit 15 - Port A Lock bit 15"]
    #[inline(always)]
    pub fn lck15(&mut self) -> LCK15_W {
        LCK15_W::new(self)
    }
    #[doc = "Bit 16 - Lock key"]
    #[inline(always)]
    pub fn lckk(&mut self) -> LCKK_W {
        LCKK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port configuration lock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lckr](index.html) module"]
pub struct LCKR_SPEC;
impl crate::RegisterSpec for LCKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lckr::R](R) reader structure"]
impl crate::Readable for LCKR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lckr::W](W) writer structure"]
impl crate::Writable for LCKR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCKR to value 0"]
impl crate::Resettable for LCKR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
