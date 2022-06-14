#[doc = "Register `SWEVGR` writer"]
pub struct W(crate::W<SWEVGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWEVGR_SPEC>;
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
impl From<crate::W<SWEVGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWEVGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BG` writer - Break generation"]
pub type BG_W<'a> = crate::BitWriter<'a, u32, SWEVGR_SPEC, bool, 7>;
#[doc = "Field `TG` writer - Trigger generation"]
pub type TG_W<'a> = crate::BitWriter<'a, u32, SWEVGR_SPEC, bool, 6>;
#[doc = "Field `COMG` writer - Capture/Compare control update generation"]
pub type COMG_W<'a> = crate::BitWriter<'a, u32, SWEVGR_SPEC, bool, 5>;
#[doc = "Field `CC4G` writer - Capture/compare 4 generation"]
pub type CC4G_W<'a> = crate::BitWriter<'a, u32, SWEVGR_SPEC, bool, 4>;
#[doc = "Field `CC3G` writer - Capture/compare 3 generation"]
pub type CC3G_W<'a> = crate::BitWriter<'a, u32, SWEVGR_SPEC, bool, 3>;
#[doc = "Field `CC2G` writer - Capture/compare 2 generation"]
pub type CC2G_W<'a> = crate::BitWriter<'a, u32, SWEVGR_SPEC, bool, 2>;
#[doc = "Field `CC1G` writer - Capture/compare 1 generation"]
pub type CC1G_W<'a> = crate::BitWriter<'a, u32, SWEVGR_SPEC, bool, 1>;
#[doc = "Field `UG` writer - Update generation"]
pub type UG_W<'a> = crate::BitWriter<'a, u32, SWEVGR_SPEC, bool, 0>;
impl W {
    #[doc = "Bit 7 - Break generation"]
    #[inline(always)]
    pub fn bg(&mut self) -> BG_W {
        BG_W::new(self)
    }
    #[doc = "Bit 6 - Trigger generation"]
    #[inline(always)]
    pub fn tg(&mut self) -> TG_W {
        TG_W::new(self)
    }
    #[doc = "Bit 5 - Capture/Compare control update generation"]
    #[inline(always)]
    pub fn comg(&mut self) -> COMG_W {
        COMG_W::new(self)
    }
    #[doc = "Bit 4 - Capture/compare 4 generation"]
    #[inline(always)]
    pub fn cc4g(&mut self) -> CC4G_W {
        CC4G_W::new(self)
    }
    #[doc = "Bit 3 - Capture/compare 3 generation"]
    #[inline(always)]
    pub fn cc3g(&mut self) -> CC3G_W {
        CC3G_W::new(self)
    }
    #[doc = "Bit 2 - Capture/compare 2 generation"]
    #[inline(always)]
    pub fn cc2g(&mut self) -> CC2G_W {
        CC2G_W::new(self)
    }
    #[doc = "Bit 1 - Capture/compare 1 generation"]
    #[inline(always)]
    pub fn cc1g(&mut self) -> CC1G_W {
        CC1G_W::new(self)
    }
    #[doc = "Bit 0 - Update generation"]
    #[inline(always)]
    pub fn ug(&mut self) -> UG_W {
        UG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "event generation register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swevgr](index.html) module"]
pub struct SWEVGR_SPEC;
impl crate::RegisterSpec for SWEVGR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [swevgr::W](W) writer structure"]
impl crate::Writable for SWEVGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWEVGR to value 0"]
impl crate::Resettable for SWEVGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}