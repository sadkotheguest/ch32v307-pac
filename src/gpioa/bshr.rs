#[doc = "Register `BSHR` writer"]
pub struct W(crate::W<BSHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BSHR_SPEC>;
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
impl From<crate::W<BSHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BSHR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BS0` writer - Set bit 0"]
pub type BS0_W<'a> = crate::BitWriter<'a, u32, BSHR_SPEC, bool, 0>;
#[doc = "Field `BS1` writer - Set bit 1"]
pub type BS1_W<'a> = crate::BitWriter<'a, u32, BSHR_SPEC, bool, 1>;
#[doc = "Field `BS2` writer - Set bit 1"]
pub type BS2_W<'a> = crate::BitWriter<'a, u32, BSHR_SPEC, bool, 2>;
#[doc = "Field `BS3` writer - Set bit 3"]
pub type BS3_W<'a> = crate::BitWriter<'a, u32, BSHR_SPEC, bool, 3>;
#[doc = "Field `BS4` writer - Set bit 4"]
pub type BS4_W<'a> = crate::BitWriter<'a, u32, BSHR_SPEC, bool, 4>;
#[doc = "Field `BS5` writer - Set bit 5"]
pub type BS5_W<'a> = crate::BitWriter<'a, u32, BSHR_SPEC, bool, 5>;
#[doc = "Field `BS6` writer - Set bit 6"]
pub type BS6_W<'a> = crate::BitWriter<'a, u32, BSHR_SPEC, bool, 6>;
#[doc = "Field `BS7` writer - Set bit 7"]
pub type BS7_W<'a> = crate::BitWriter<'a, u32, BSHR_SPEC, bool, 7>;
#[doc = "Field `BS8` writer - Set bit 8"]
pub type BS8_W<'a> = crate::BitWriter<'a, u32, BSHR_SPEC, bool, 8>;
#[doc = "Field `BS9` writer - Set bit 9"]
pub type BS9_W<'a> = crate::BitWriter<'a, u32, BSHR_SPEC, bool, 9>;
#[doc = "Field `BS10` writer - Set bit 10"]
pub type BS10_W<'a> = crate::BitWriter<'a, u32, BSHR_SPEC, bool, 10>;
#[doc = "Field `BS11` writer - Set bit 11"]
pub type BS11_W<'a> = crate::BitWriter<'a, u32, BSHR_SPEC, bool, 11>;
#[doc = "Field `BS12` writer - Set bit 12"]
pub type BS12_W<'a> = crate::BitWriter<'a, u32, BSHR_SPEC, bool, 12>;
#[doc = "Field `BS13` writer - Set bit 13"]
pub type BS13_W<'a> = crate::BitWriter<'a, u32, BSHR_SPEC, bool, 13>;
#[doc = "Field `BS14` writer - Set bit 14"]
pub type BS14_W<'a> = crate::BitWriter<'a, u32, BSHR_SPEC, bool, 14>;
#[doc = "Field `BS15` writer - Set bit 15"]
pub type BS15_W<'a> = crate::BitWriter<'a, u32, BSHR_SPEC, bool, 15>;
#[doc = "Field `BR0` writer - Reset bit 0"]
pub type BR0_W<'a> = crate::BitWriter<'a, u32, BSHR_SPEC, bool, 16>;
#[doc = "Field `BR1` writer - Reset bit 1"]
pub type BR1_W<'a> = crate::BitWriter<'a, u32, BSHR_SPEC, bool, 17>;
#[doc = "Field `BR2` writer - Reset bit 2"]
pub type BR2_W<'a> = crate::BitWriter<'a, u32, BSHR_SPEC, bool, 18>;
#[doc = "Field `BR3` writer - Reset bit 3"]
pub type BR3_W<'a> = crate::BitWriter<'a, u32, BSHR_SPEC, bool, 19>;
#[doc = "Field `BR4` writer - Reset bit 4"]
pub type BR4_W<'a> = crate::BitWriter<'a, u32, BSHR_SPEC, bool, 20>;
#[doc = "Field `BR5` writer - Reset bit 5"]
pub type BR5_W<'a> = crate::BitWriter<'a, u32, BSHR_SPEC, bool, 21>;
#[doc = "Field `BR6` writer - Reset bit 6"]
pub type BR6_W<'a> = crate::BitWriter<'a, u32, BSHR_SPEC, bool, 22>;
#[doc = "Field `BR7` writer - Reset bit 7"]
pub type BR7_W<'a> = crate::BitWriter<'a, u32, BSHR_SPEC, bool, 23>;
#[doc = "Field `BR8` writer - Reset bit 8"]
pub type BR8_W<'a> = crate::BitWriter<'a, u32, BSHR_SPEC, bool, 24>;
#[doc = "Field `BR9` writer - Reset bit 9"]
pub type BR9_W<'a> = crate::BitWriter<'a, u32, BSHR_SPEC, bool, 25>;
#[doc = "Field `BR10` writer - Reset bit 10"]
pub type BR10_W<'a> = crate::BitWriter<'a, u32, BSHR_SPEC, bool, 26>;
#[doc = "Field `BR11` writer - Reset bit 11"]
pub type BR11_W<'a> = crate::BitWriter<'a, u32, BSHR_SPEC, bool, 27>;
#[doc = "Field `BR12` writer - Reset bit 12"]
pub type BR12_W<'a> = crate::BitWriter<'a, u32, BSHR_SPEC, bool, 28>;
#[doc = "Field `BR13` writer - Reset bit 13"]
pub type BR13_W<'a> = crate::BitWriter<'a, u32, BSHR_SPEC, bool, 29>;
#[doc = "Field `BR14` writer - Reset bit 14"]
pub type BR14_W<'a> = crate::BitWriter<'a, u32, BSHR_SPEC, bool, 30>;
#[doc = "Field `BR15` writer - Reset bit 15"]
pub type BR15_W<'a> = crate::BitWriter<'a, u32, BSHR_SPEC, bool, 31>;
impl W {
    #[doc = "Bit 0 - Set bit 0"]
    #[inline(always)]
    pub fn bs0(&mut self) -> BS0_W {
        BS0_W::new(self)
    }
    #[doc = "Bit 1 - Set bit 1"]
    #[inline(always)]
    pub fn bs1(&mut self) -> BS1_W {
        BS1_W::new(self)
    }
    #[doc = "Bit 2 - Set bit 1"]
    #[inline(always)]
    pub fn bs2(&mut self) -> BS2_W {
        BS2_W::new(self)
    }
    #[doc = "Bit 3 - Set bit 3"]
    #[inline(always)]
    pub fn bs3(&mut self) -> BS3_W {
        BS3_W::new(self)
    }
    #[doc = "Bit 4 - Set bit 4"]
    #[inline(always)]
    pub fn bs4(&mut self) -> BS4_W {
        BS4_W::new(self)
    }
    #[doc = "Bit 5 - Set bit 5"]
    #[inline(always)]
    pub fn bs5(&mut self) -> BS5_W {
        BS5_W::new(self)
    }
    #[doc = "Bit 6 - Set bit 6"]
    #[inline(always)]
    pub fn bs6(&mut self) -> BS6_W {
        BS6_W::new(self)
    }
    #[doc = "Bit 7 - Set bit 7"]
    #[inline(always)]
    pub fn bs7(&mut self) -> BS7_W {
        BS7_W::new(self)
    }
    #[doc = "Bit 8 - Set bit 8"]
    #[inline(always)]
    pub fn bs8(&mut self) -> BS8_W {
        BS8_W::new(self)
    }
    #[doc = "Bit 9 - Set bit 9"]
    #[inline(always)]
    pub fn bs9(&mut self) -> BS9_W {
        BS9_W::new(self)
    }
    #[doc = "Bit 10 - Set bit 10"]
    #[inline(always)]
    pub fn bs10(&mut self) -> BS10_W {
        BS10_W::new(self)
    }
    #[doc = "Bit 11 - Set bit 11"]
    #[inline(always)]
    pub fn bs11(&mut self) -> BS11_W {
        BS11_W::new(self)
    }
    #[doc = "Bit 12 - Set bit 12"]
    #[inline(always)]
    pub fn bs12(&mut self) -> BS12_W {
        BS12_W::new(self)
    }
    #[doc = "Bit 13 - Set bit 13"]
    #[inline(always)]
    pub fn bs13(&mut self) -> BS13_W {
        BS13_W::new(self)
    }
    #[doc = "Bit 14 - Set bit 14"]
    #[inline(always)]
    pub fn bs14(&mut self) -> BS14_W {
        BS14_W::new(self)
    }
    #[doc = "Bit 15 - Set bit 15"]
    #[inline(always)]
    pub fn bs15(&mut self) -> BS15_W {
        BS15_W::new(self)
    }
    #[doc = "Bit 16 - Reset bit 0"]
    #[inline(always)]
    pub fn br0(&mut self) -> BR0_W {
        BR0_W::new(self)
    }
    #[doc = "Bit 17 - Reset bit 1"]
    #[inline(always)]
    pub fn br1(&mut self) -> BR1_W {
        BR1_W::new(self)
    }
    #[doc = "Bit 18 - Reset bit 2"]
    #[inline(always)]
    pub fn br2(&mut self) -> BR2_W {
        BR2_W::new(self)
    }
    #[doc = "Bit 19 - Reset bit 3"]
    #[inline(always)]
    pub fn br3(&mut self) -> BR3_W {
        BR3_W::new(self)
    }
    #[doc = "Bit 20 - Reset bit 4"]
    #[inline(always)]
    pub fn br4(&mut self) -> BR4_W {
        BR4_W::new(self)
    }
    #[doc = "Bit 21 - Reset bit 5"]
    #[inline(always)]
    pub fn br5(&mut self) -> BR5_W {
        BR5_W::new(self)
    }
    #[doc = "Bit 22 - Reset bit 6"]
    #[inline(always)]
    pub fn br6(&mut self) -> BR6_W {
        BR6_W::new(self)
    }
    #[doc = "Bit 23 - Reset bit 7"]
    #[inline(always)]
    pub fn br7(&mut self) -> BR7_W {
        BR7_W::new(self)
    }
    #[doc = "Bit 24 - Reset bit 8"]
    #[inline(always)]
    pub fn br8(&mut self) -> BR8_W {
        BR8_W::new(self)
    }
    #[doc = "Bit 25 - Reset bit 9"]
    #[inline(always)]
    pub fn br9(&mut self) -> BR9_W {
        BR9_W::new(self)
    }
    #[doc = "Bit 26 - Reset bit 10"]
    #[inline(always)]
    pub fn br10(&mut self) -> BR10_W {
        BR10_W::new(self)
    }
    #[doc = "Bit 27 - Reset bit 11"]
    #[inline(always)]
    pub fn br11(&mut self) -> BR11_W {
        BR11_W::new(self)
    }
    #[doc = "Bit 28 - Reset bit 12"]
    #[inline(always)]
    pub fn br12(&mut self) -> BR12_W {
        BR12_W::new(self)
    }
    #[doc = "Bit 29 - Reset bit 13"]
    #[inline(always)]
    pub fn br13(&mut self) -> BR13_W {
        BR13_W::new(self)
    }
    #[doc = "Bit 30 - Reset bit 14"]
    #[inline(always)]
    pub fn br14(&mut self) -> BR14_W {
        BR14_W::new(self)
    }
    #[doc = "Bit 31 - Reset bit 15"]
    #[inline(always)]
    pub fn br15(&mut self) -> BR15_W {
        BR15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port bit set/reset register (GPIOn_BSHR)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bshr](index.html) module"]
pub struct BSHR_SPEC;
impl crate::RegisterSpec for BSHR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [bshr::W](W) writer structure"]
impl crate::Writable for BSHR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BSHR to value 0"]
impl crate::Resettable for BSHR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}