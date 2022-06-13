#[doc = "Register `BDCTLR` reader"]
pub struct R(crate::R<BDCTLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BDCTLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BDCTLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BDCTLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BDCTLR` writer"]
pub struct W(crate::W<BDCTLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BDCTLR_SPEC>;
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
impl From<crate::W<BDCTLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BDCTLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSEON` reader - External Low Speed oscillator enable"]
pub type LSEON_R = crate::BitReader<bool>;
#[doc = "Field `LSEON` writer - External Low Speed oscillator enable"]
pub type LSEON_W<'a> = crate::BitWriter<'a, u32, BDCTLR_SPEC, bool, 0>;
#[doc = "Field `LSERDY` reader - External Low Speed oscillator ready"]
pub type LSERDY_R = crate::BitReader<bool>;
#[doc = "Field `LSEBYP` reader - External Low Speed oscillator bypass"]
pub type LSEBYP_R = crate::BitReader<bool>;
#[doc = "Field `LSEBYP` writer - External Low Speed oscillator bypass"]
pub type LSEBYP_W<'a> = crate::BitWriter<'a, u32, BDCTLR_SPEC, bool, 2>;
#[doc = "Field `RTCSEL` reader - RTC clock source selection"]
pub type RTCSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTCSEL` writer - RTC clock source selection"]
pub type RTCSEL_W<'a> = crate::FieldWriter<'a, u32, BDCTLR_SPEC, u8, u8, 2, 8>;
#[doc = "Field `RTCEN` reader - RTC clock enable"]
pub type RTCEN_R = crate::BitReader<bool>;
#[doc = "Field `RTCEN` writer - RTC clock enable"]
pub type RTCEN_W<'a> = crate::BitWriter<'a, u32, BDCTLR_SPEC, bool, 15>;
#[doc = "Field `BDRST` reader - Backup domain software reset"]
pub type BDRST_R = crate::BitReader<bool>;
#[doc = "Field `BDRST` writer - Backup domain software reset"]
pub type BDRST_W<'a> = crate::BitWriter<'a, u32, BDCTLR_SPEC, bool, 16>;
impl R {
    #[doc = "Bit 0 - External Low Speed oscillator enable"]
    #[inline(always)]
    pub fn lseon(&self) -> LSEON_R {
        LSEON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Low Speed oscillator ready"]
    #[inline(always)]
    pub fn lserdy(&self) -> LSERDY_R {
        LSERDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External Low Speed oscillator bypass"]
    #[inline(always)]
    pub fn lsebyp(&self) -> LSEBYP_R {
        LSEBYP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:9 - RTC clock source selection"]
    #[inline(always)]
    pub fn rtcsel(&self) -> RTCSEL_R {
        RTCSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 15 - RTC clock enable"]
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Backup domain software reset"]
    #[inline(always)]
    pub fn bdrst(&self) -> BDRST_R {
        BDRST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Low Speed oscillator enable"]
    #[inline(always)]
    pub fn lseon(&mut self) -> LSEON_W {
        LSEON_W::new(self)
    }
    #[doc = "Bit 2 - External Low Speed oscillator bypass"]
    #[inline(always)]
    pub fn lsebyp(&mut self) -> LSEBYP_W {
        LSEBYP_W::new(self)
    }
    #[doc = "Bits 8:9 - RTC clock source selection"]
    #[inline(always)]
    pub fn rtcsel(&mut self) -> RTCSEL_W {
        RTCSEL_W::new(self)
    }
    #[doc = "Bit 15 - RTC clock enable"]
    #[inline(always)]
    pub fn rtcen(&mut self) -> RTCEN_W {
        RTCEN_W::new(self)
    }
    #[doc = "Bit 16 - Backup domain software reset"]
    #[inline(always)]
    pub fn bdrst(&mut self) -> BDRST_W {
        BDRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Backup domain control register (RCC_BDCTLR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdctlr](index.html) module"]
pub struct BDCTLR_SPEC;
impl crate::RegisterSpec for BDCTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bdctlr::R](R) reader structure"]
impl crate::Readable for BDCTLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bdctlr::W](W) writer structure"]
impl crate::Writable for BDCTLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BDCTLR to value 0"]
impl crate::Resettable for BDCTLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}