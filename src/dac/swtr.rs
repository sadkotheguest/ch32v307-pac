#[doc = "Register `SWTR` writer"]
pub struct W(crate::W<SWTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWTR_SPEC>;
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
impl From<crate::W<SWTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWTRIG1` writer - DAC channel1 software trigger"]
pub type SWTRIG1_W<'a> = crate::BitWriter<'a, u32, SWTR_SPEC, bool, 0>;
#[doc = "Field `SWTRIG2` writer - DAC channel2 software trigger"]
pub type SWTRIG2_W<'a> = crate::BitWriter<'a, u32, SWTR_SPEC, bool, 1>;
impl W {
    #[doc = "Bit 0 - DAC channel1 software trigger"]
    #[inline(always)]
    pub fn swtrig1(&mut self) -> SWTRIG1_W {
        SWTRIG1_W::new(self)
    }
    #[doc = "Bit 1 - DAC channel2 software trigger"]
    #[inline(always)]
    pub fn swtrig2(&mut self) -> SWTRIG2_W {
        SWTRIG2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC software trigger register (DAC_SWTRIGR)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swtr](index.html) module"]
pub struct SWTR_SPEC;
impl crate::RegisterSpec for SWTR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [swtr::W](W) writer structure"]
impl crate::Writable for SWTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWTR to value 0"]
impl crate::Resettable for SWTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}