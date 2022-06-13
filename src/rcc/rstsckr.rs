#[doc = "Register `RSTSCKR` reader"]
pub struct R(crate::R<RSTSCKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTSCKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTSCKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTSCKR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSTSCKR` writer"]
pub struct W(crate::W<RSTSCKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTSCKR_SPEC>;
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
impl From<crate::W<RSTSCKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTSCKR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSION` reader - Internal low speed oscillator enable"]
pub type LSION_R = crate::BitReader<bool>;
#[doc = "Field `LSION` writer - Internal low speed oscillator enable"]
pub type LSION_W<'a> = crate::BitWriter<'a, u32, RSTSCKR_SPEC, bool, 0>;
#[doc = "Field `LSIRDY` reader - Internal low speed oscillator ready"]
pub type LSIRDY_R = crate::BitReader<bool>;
#[doc = "Field `RMVF` reader - Remove reset flag"]
pub type RMVF_R = crate::BitReader<bool>;
#[doc = "Field `RMVF` writer - Remove reset flag"]
pub type RMVF_W<'a> = crate::BitWriter<'a, u32, RSTSCKR_SPEC, bool, 24>;
#[doc = "Field `PINRSTF` reader - PIN reset flag"]
pub type PINRSTF_R = crate::BitReader<bool>;
#[doc = "Field `PORRSTF` reader - POR/PDR reset flag"]
pub type PORRSTF_R = crate::BitReader<bool>;
#[doc = "Field `SFTRSTF` reader - Software reset flag"]
pub type SFTRSTF_R = crate::BitReader<bool>;
#[doc = "Field `IWDGRSTF` reader - Independent watchdog reset flag"]
pub type IWDGRSTF_R = crate::BitReader<bool>;
#[doc = "Field `WWDGRSTF` reader - Window watchdog reset flag"]
pub type WWDGRSTF_R = crate::BitReader<bool>;
#[doc = "Field `LPWRRSTF` reader - Low-power reset flag"]
pub type LPWRRSTF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Internal low speed oscillator enable"]
    #[inline(always)]
    pub fn lsion(&self) -> LSION_R {
        LSION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal low speed oscillator ready"]
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 24 - Remove reset flag"]
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - PIN reset flag"]
    #[inline(always)]
    pub fn pinrstf(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - POR/PDR reset flag"]
    #[inline(always)]
    pub fn porrstf(&self) -> PORRSTF_R {
        PORRSTF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Software reset flag"]
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Independent watchdog reset flag"]
    #[inline(always)]
    pub fn iwdgrstf(&self) -> IWDGRSTF_R {
        IWDGRSTF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Window watchdog reset flag"]
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WWDGRSTF_R {
        WWDGRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low-power reset flag"]
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LPWRRSTF_R {
        LPWRRSTF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal low speed oscillator enable"]
    #[inline(always)]
    pub fn lsion(&mut self) -> LSION_W {
        LSION_W::new(self)
    }
    #[doc = "Bit 24 - Remove reset flag"]
    #[inline(always)]
    pub fn rmvf(&mut self) -> RMVF_W {
        RMVF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control/status register (RCC_RSTSCKR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstsckr](index.html) module"]
pub struct RSTSCKR_SPEC;
impl crate::RegisterSpec for RSTSCKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstsckr::R](R) reader structure"]
impl crate::Readable for RSTSCKR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rstsckr::W](W) writer structure"]
impl crate::Writable for RSTSCKR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSTSCKR to value 0x0c00_0000"]
impl crate::Resettable for RSTSCKR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0c00_0000
    }
}
