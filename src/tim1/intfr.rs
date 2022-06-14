#[doc = "Register `INTFR` reader"]
pub struct R(crate::R<INTFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTFR` writer"]
pub struct W(crate::W<INTFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTFR_SPEC>;
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
impl From<crate::W<INTFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CC4OF` reader - Capture/Compare 4 overcapture flag"]
pub type CC4OF_R = crate::BitReader<bool>;
#[doc = "Field `CC4OF` writer - Capture/Compare 4 overcapture flag"]
pub type CC4OF_W<'a> = crate::BitWriter<'a, u32, INTFR_SPEC, bool, 12>;
#[doc = "Field `CC3OF` reader - Capture/Compare 3 overcapture flag"]
pub type CC3OF_R = crate::BitReader<bool>;
#[doc = "Field `CC3OF` writer - Capture/Compare 3 overcapture flag"]
pub type CC3OF_W<'a> = crate::BitWriter<'a, u32, INTFR_SPEC, bool, 11>;
#[doc = "Field `CC2OF` reader - Capture/compare 2 overcapture flag"]
pub type CC2OF_R = crate::BitReader<bool>;
#[doc = "Field `CC2OF` writer - Capture/compare 2 overcapture flag"]
pub type CC2OF_W<'a> = crate::BitWriter<'a, u32, INTFR_SPEC, bool, 10>;
#[doc = "Field `CC1OF` reader - Capture/Compare 1 overcapture flag"]
pub type CC1OF_R = crate::BitReader<bool>;
#[doc = "Field `CC1OF` writer - Capture/Compare 1 overcapture flag"]
pub type CC1OF_W<'a> = crate::BitWriter<'a, u32, INTFR_SPEC, bool, 9>;
#[doc = "Field `BIF` reader - Break interrupt flag"]
pub type BIF_R = crate::BitReader<bool>;
#[doc = "Field `BIF` writer - Break interrupt flag"]
pub type BIF_W<'a> = crate::BitWriter<'a, u32, INTFR_SPEC, bool, 7>;
#[doc = "Field `TIF` reader - Trigger interrupt flag"]
pub type TIF_R = crate::BitReader<bool>;
#[doc = "Field `TIF` writer - Trigger interrupt flag"]
pub type TIF_W<'a> = crate::BitWriter<'a, u32, INTFR_SPEC, bool, 6>;
#[doc = "Field `COMIF` reader - COM interrupt flag"]
pub type COMIF_R = crate::BitReader<bool>;
#[doc = "Field `COMIF` writer - COM interrupt flag"]
pub type COMIF_W<'a> = crate::BitWriter<'a, u32, INTFR_SPEC, bool, 5>;
#[doc = "Field `CC4IF` reader - Capture/Compare 4 interrupt flag"]
pub type CC4IF_R = crate::BitReader<bool>;
#[doc = "Field `CC4IF` writer - Capture/Compare 4 interrupt flag"]
pub type CC4IF_W<'a> = crate::BitWriter<'a, u32, INTFR_SPEC, bool, 4>;
#[doc = "Field `CC3IF` reader - Capture/Compare 3 interrupt flag"]
pub type CC3IF_R = crate::BitReader<bool>;
#[doc = "Field `CC3IF` writer - Capture/Compare 3 interrupt flag"]
pub type CC3IF_W<'a> = crate::BitWriter<'a, u32, INTFR_SPEC, bool, 3>;
#[doc = "Field `CC2IF` reader - Capture/Compare 2 interrupt flag"]
pub type CC2IF_R = crate::BitReader<bool>;
#[doc = "Field `CC2IF` writer - Capture/Compare 2 interrupt flag"]
pub type CC2IF_W<'a> = crate::BitWriter<'a, u32, INTFR_SPEC, bool, 2>;
#[doc = "Field `CC1IF` reader - Capture/compare 1 interrupt flag"]
pub type CC1IF_R = crate::BitReader<bool>;
#[doc = "Field `CC1IF` writer - Capture/compare 1 interrupt flag"]
pub type CC1IF_W<'a> = crate::BitWriter<'a, u32, INTFR_SPEC, bool, 1>;
#[doc = "Field `UIF` reader - Update interrupt flag"]
pub type UIF_R = crate::BitReader<bool>;
#[doc = "Field `UIF` writer - Update interrupt flag"]
pub type UIF_W<'a> = crate::BitWriter<'a, u32, INTFR_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 12 - Capture/Compare 4 overcapture flag"]
    #[inline(always)]
    pub fn cc4of(&self) -> CC4OF_R {
        CC4OF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture/Compare 3 overcapture flag"]
    #[inline(always)]
    pub fn cc3of(&self) -> CC3OF_R {
        CC3OF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture/compare 2 overcapture flag"]
    #[inline(always)]
    pub fn cc2of(&self) -> CC2OF_R {
        CC2OF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture/Compare 1 overcapture flag"]
    #[inline(always)]
    pub fn cc1of(&self) -> CC1OF_R {
        CC1OF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 7 - Break interrupt flag"]
    #[inline(always)]
    pub fn bif(&self) -> BIF_R {
        BIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger interrupt flag"]
    #[inline(always)]
    pub fn tif(&self) -> TIF_R {
        TIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - COM interrupt flag"]
    #[inline(always)]
    pub fn comif(&self) -> COMIF_R {
        COMIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture/Compare 4 interrupt flag"]
    #[inline(always)]
    pub fn cc4if(&self) -> CC4IF_R {
        CC4IF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/Compare 3 interrupt flag"]
    #[inline(always)]
    pub fn cc3if(&self) -> CC3IF_R {
        CC3IF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture/Compare 2 interrupt flag"]
    #[inline(always)]
    pub fn cc2if(&self) -> CC2IF_R {
        CC2IF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Capture/compare 1 interrupt flag"]
    #[inline(always)]
    pub fn cc1if(&self) -> CC1IF_R {
        CC1IF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    pub fn uif(&self) -> UIF_R {
        UIF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Capture/Compare 4 overcapture flag"]
    #[inline(always)]
    pub fn cc4of(&mut self) -> CC4OF_W {
        CC4OF_W::new(self)
    }
    #[doc = "Bit 11 - Capture/Compare 3 overcapture flag"]
    #[inline(always)]
    pub fn cc3of(&mut self) -> CC3OF_W {
        CC3OF_W::new(self)
    }
    #[doc = "Bit 10 - Capture/compare 2 overcapture flag"]
    #[inline(always)]
    pub fn cc2of(&mut self) -> CC2OF_W {
        CC2OF_W::new(self)
    }
    #[doc = "Bit 9 - Capture/Compare 1 overcapture flag"]
    #[inline(always)]
    pub fn cc1of(&mut self) -> CC1OF_W {
        CC1OF_W::new(self)
    }
    #[doc = "Bit 7 - Break interrupt flag"]
    #[inline(always)]
    pub fn bif(&mut self) -> BIF_W {
        BIF_W::new(self)
    }
    #[doc = "Bit 6 - Trigger interrupt flag"]
    #[inline(always)]
    pub fn tif(&mut self) -> TIF_W {
        TIF_W::new(self)
    }
    #[doc = "Bit 5 - COM interrupt flag"]
    #[inline(always)]
    pub fn comif(&mut self) -> COMIF_W {
        COMIF_W::new(self)
    }
    #[doc = "Bit 4 - Capture/Compare 4 interrupt flag"]
    #[inline(always)]
    pub fn cc4if(&mut self) -> CC4IF_W {
        CC4IF_W::new(self)
    }
    #[doc = "Bit 3 - Capture/Compare 3 interrupt flag"]
    #[inline(always)]
    pub fn cc3if(&mut self) -> CC3IF_W {
        CC3IF_W::new(self)
    }
    #[doc = "Bit 2 - Capture/Compare 2 interrupt flag"]
    #[inline(always)]
    pub fn cc2if(&mut self) -> CC2IF_W {
        CC2IF_W::new(self)
    }
    #[doc = "Bit 1 - Capture/compare 1 interrupt flag"]
    #[inline(always)]
    pub fn cc1if(&mut self) -> CC1IF_W {
        CC1IF_W::new(self)
    }
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    pub fn uif(&mut self) -> UIF_W {
        UIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intfr](index.html) module"]
pub struct INTFR_SPEC;
impl crate::RegisterSpec for INTFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intfr::R](R) reader structure"]
impl crate::Readable for INTFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intfr::W](W) writer structure"]
impl crate::Writable for INTFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTFR to value 0"]
impl crate::Resettable for INTFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}