#[doc = "Register `BCR` writer"]
pub struct W(crate::W<BCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCR_SPEC>;
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
impl From<crate::W<BCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BR0` writer - Reset bit 0"]
pub type BR0_W<'a> = crate::BitWriter<'a, u32, BCR_SPEC, bool, 0>;
#[doc = "Field `BR1` writer - Reset bit 1"]
pub type BR1_W<'a> = crate::BitWriter<'a, u32, BCR_SPEC, bool, 1>;
#[doc = "Field `BR2` writer - Reset bit 1"]
pub type BR2_W<'a> = crate::BitWriter<'a, u32, BCR_SPEC, bool, 2>;
#[doc = "Field `BR3` writer - Reset bit 3"]
pub type BR3_W<'a> = crate::BitWriter<'a, u32, BCR_SPEC, bool, 3>;
#[doc = "Field `BR4` writer - Reset bit 4"]
pub type BR4_W<'a> = crate::BitWriter<'a, u32, BCR_SPEC, bool, 4>;
#[doc = "Field `BR5` writer - Reset bit 5"]
pub type BR5_W<'a> = crate::BitWriter<'a, u32, BCR_SPEC, bool, 5>;
#[doc = "Field `BR6` writer - Reset bit 6"]
pub type BR6_W<'a> = crate::BitWriter<'a, u32, BCR_SPEC, bool, 6>;
#[doc = "Field `BR7` writer - Reset bit 7"]
pub type BR7_W<'a> = crate::BitWriter<'a, u32, BCR_SPEC, bool, 7>;
#[doc = "Field `BR8` writer - Reset bit 8"]
pub type BR8_W<'a> = crate::BitWriter<'a, u32, BCR_SPEC, bool, 8>;
#[doc = "Field `BR9` writer - Reset bit 9"]
pub type BR9_W<'a> = crate::BitWriter<'a, u32, BCR_SPEC, bool, 9>;
#[doc = "Field `BR10` writer - Reset bit 10"]
pub type BR10_W<'a> = crate::BitWriter<'a, u32, BCR_SPEC, bool, 10>;
#[doc = "Field `BR11` writer - Reset bit 11"]
pub type BR11_W<'a> = crate::BitWriter<'a, u32, BCR_SPEC, bool, 11>;
#[doc = "Field `BR12` writer - Reset bit 12"]
pub type BR12_W<'a> = crate::BitWriter<'a, u32, BCR_SPEC, bool, 12>;
#[doc = "Field `BR13` writer - Reset bit 13"]
pub type BR13_W<'a> = crate::BitWriter<'a, u32, BCR_SPEC, bool, 13>;
#[doc = "Field `BR14` writer - Reset bit 14"]
pub type BR14_W<'a> = crate::BitWriter<'a, u32, BCR_SPEC, bool, 14>;
#[doc = "Field `BR15` writer - Reset bit 15"]
pub type BR15_W<'a> = crate::BitWriter<'a, u32, BCR_SPEC, bool, 15>;
impl W {
    #[doc = "Bit 0 - Reset bit 0"]
    #[inline(always)]
    pub fn br0(&mut self) -> BR0_W {
        BR0_W::new(self)
    }
    #[doc = "Bit 1 - Reset bit 1"]
    #[inline(always)]
    pub fn br1(&mut self) -> BR1_W {
        BR1_W::new(self)
    }
    #[doc = "Bit 2 - Reset bit 1"]
    #[inline(always)]
    pub fn br2(&mut self) -> BR2_W {
        BR2_W::new(self)
    }
    #[doc = "Bit 3 - Reset bit 3"]
    #[inline(always)]
    pub fn br3(&mut self) -> BR3_W {
        BR3_W::new(self)
    }
    #[doc = "Bit 4 - Reset bit 4"]
    #[inline(always)]
    pub fn br4(&mut self) -> BR4_W {
        BR4_W::new(self)
    }
    #[doc = "Bit 5 - Reset bit 5"]
    #[inline(always)]
    pub fn br5(&mut self) -> BR5_W {
        BR5_W::new(self)
    }
    #[doc = "Bit 6 - Reset bit 6"]
    #[inline(always)]
    pub fn br6(&mut self) -> BR6_W {
        BR6_W::new(self)
    }
    #[doc = "Bit 7 - Reset bit 7"]
    #[inline(always)]
    pub fn br7(&mut self) -> BR7_W {
        BR7_W::new(self)
    }
    #[doc = "Bit 8 - Reset bit 8"]
    #[inline(always)]
    pub fn br8(&mut self) -> BR8_W {
        BR8_W::new(self)
    }
    #[doc = "Bit 9 - Reset bit 9"]
    #[inline(always)]
    pub fn br9(&mut self) -> BR9_W {
        BR9_W::new(self)
    }
    #[doc = "Bit 10 - Reset bit 10"]
    #[inline(always)]
    pub fn br10(&mut self) -> BR10_W {
        BR10_W::new(self)
    }
    #[doc = "Bit 11 - Reset bit 11"]
    #[inline(always)]
    pub fn br11(&mut self) -> BR11_W {
        BR11_W::new(self)
    }
    #[doc = "Bit 12 - Reset bit 12"]
    #[inline(always)]
    pub fn br12(&mut self) -> BR12_W {
        BR12_W::new(self)
    }
    #[doc = "Bit 13 - Reset bit 13"]
    #[inline(always)]
    pub fn br13(&mut self) -> BR13_W {
        BR13_W::new(self)
    }
    #[doc = "Bit 14 - Reset bit 14"]
    #[inline(always)]
    pub fn br14(&mut self) -> BR14_W {
        BR14_W::new(self)
    }
    #[doc = "Bit 15 - Reset bit 15"]
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
#[doc = "Port bit reset register (GPIOn_BCR)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcr](index.html) module"]
pub struct BCR_SPEC;
impl crate::RegisterSpec for BCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [bcr::W](W) writer structure"]
impl crate::Writable for BCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BCR to value 0"]
impl crate::Resettable for BCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}