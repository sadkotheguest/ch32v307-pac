#[doc = "Register `INTR` reader"]
pub struct R(crate::R<INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR` writer"]
pub struct W(crate::W<INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_SPEC>;
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
impl From<crate::W<INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSIRDYF` reader - LSI Ready Interrupt flag"]
pub type LSIRDYF_R = crate::BitReader<bool>;
#[doc = "Field `LSERDYF` reader - LSE Ready Interrupt flag"]
pub type LSERDYF_R = crate::BitReader<bool>;
#[doc = "Field `HSIRDYF` reader - HSI Ready Interrupt flag"]
pub type HSIRDYF_R = crate::BitReader<bool>;
#[doc = "Field `HSERDYF` reader - HSE Ready Interrupt flag"]
pub type HSERDYF_R = crate::BitReader<bool>;
#[doc = "Field `PLLRDYF` reader - PLL Ready Interrupt flag"]
pub type PLLRDYF_R = crate::BitReader<bool>;
#[doc = "Field `CSSF` reader - Clock Security System Interrupt flag"]
pub type CSSF_R = crate::BitReader<bool>;
#[doc = "Field `LSIRDYIE` reader - LSI Ready Interrupt Enable"]
pub type LSIRDYIE_R = crate::BitReader<bool>;
#[doc = "Field `LSIRDYIE` writer - LSI Ready Interrupt Enable"]
pub type LSIRDYIE_W<'a> = crate::BitWriter<'a, u32, INTR_SPEC, bool, 8>;
#[doc = "Field `LSERDYIE` reader - LSE Ready Interrupt Enable"]
pub type LSERDYIE_R = crate::BitReader<bool>;
#[doc = "Field `LSERDYIE` writer - LSE Ready Interrupt Enable"]
pub type LSERDYIE_W<'a> = crate::BitWriter<'a, u32, INTR_SPEC, bool, 9>;
#[doc = "Field `HSIRDYIE` reader - HSI Ready Interrupt Enable"]
pub type HSIRDYIE_R = crate::BitReader<bool>;
#[doc = "Field `HSIRDYIE` writer - HSI Ready Interrupt Enable"]
pub type HSIRDYIE_W<'a> = crate::BitWriter<'a, u32, INTR_SPEC, bool, 10>;
#[doc = "Field `HSERDYIE` reader - HSE Ready Interrupt Enable"]
pub type HSERDYIE_R = crate::BitReader<bool>;
#[doc = "Field `HSERDYIE` writer - HSE Ready Interrupt Enable"]
pub type HSERDYIE_W<'a> = crate::BitWriter<'a, u32, INTR_SPEC, bool, 11>;
#[doc = "Field `PLLRDYIE` reader - PLL Ready Interrupt Enable"]
pub type PLLRDYIE_R = crate::BitReader<bool>;
#[doc = "Field `PLLRDYIE` writer - PLL Ready Interrupt Enable"]
pub type PLLRDYIE_W<'a> = crate::BitWriter<'a, u32, INTR_SPEC, bool, 12>;
#[doc = "Field `LSIRDYC` writer - LSI Ready Interrupt Clear"]
pub type LSIRDYC_W<'a> = crate::BitWriter<'a, u32, INTR_SPEC, bool, 16>;
#[doc = "Field `LSERDYC` writer - LSE Ready Interrupt Clear"]
pub type LSERDYC_W<'a> = crate::BitWriter<'a, u32, INTR_SPEC, bool, 17>;
#[doc = "Field `HSIRDYC` writer - HSI Ready Interrupt Clear"]
pub type HSIRDYC_W<'a> = crate::BitWriter<'a, u32, INTR_SPEC, bool, 18>;
#[doc = "Field `HSERDYC` writer - HSE Ready Interrupt Clear"]
pub type HSERDYC_W<'a> = crate::BitWriter<'a, u32, INTR_SPEC, bool, 19>;
#[doc = "Field `PLLRDYC` writer - PLL Ready Interrupt Clear"]
pub type PLLRDYC_W<'a> = crate::BitWriter<'a, u32, INTR_SPEC, bool, 20>;
#[doc = "Field `CSSC` writer - Clock security system interrupt clear"]
pub type CSSC_W<'a> = crate::BitWriter<'a, u32, INTR_SPEC, bool, 23>;
impl R {
    #[doc = "Bit 0 - LSI Ready Interrupt flag"]
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE Ready Interrupt flag"]
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSI Ready Interrupt flag"]
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSE Ready Interrupt flag"]
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PLL Ready Interrupt flag"]
    #[inline(always)]
    pub fn pllrdyf(&self) -> PLLRDYF_R {
        PLLRDYF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Clock Security System Interrupt flag"]
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LSI Ready Interrupt Enable"]
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LSE Ready Interrupt Enable"]
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSI Ready Interrupt Enable"]
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HSE Ready Interrupt Enable"]
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PLL Ready Interrupt Enable"]
    #[inline(always)]
    pub fn pllrdyie(&self) -> PLLRDYIE_R {
        PLLRDYIE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - LSI Ready Interrupt Enable"]
    #[inline(always)]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W {
        LSIRDYIE_W::new(self)
    }
    #[doc = "Bit 9 - LSE Ready Interrupt Enable"]
    #[inline(always)]
    pub fn lserdyie(&mut self) -> LSERDYIE_W {
        LSERDYIE_W::new(self)
    }
    #[doc = "Bit 10 - HSI Ready Interrupt Enable"]
    #[inline(always)]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W {
        HSIRDYIE_W::new(self)
    }
    #[doc = "Bit 11 - HSE Ready Interrupt Enable"]
    #[inline(always)]
    pub fn hserdyie(&mut self) -> HSERDYIE_W {
        HSERDYIE_W::new(self)
    }
    #[doc = "Bit 12 - PLL Ready Interrupt Enable"]
    #[inline(always)]
    pub fn pllrdyie(&mut self) -> PLLRDYIE_W {
        PLLRDYIE_W::new(self)
    }
    #[doc = "Bit 16 - LSI Ready Interrupt Clear"]
    #[inline(always)]
    pub fn lsirdyc(&mut self) -> LSIRDYC_W {
        LSIRDYC_W::new(self)
    }
    #[doc = "Bit 17 - LSE Ready Interrupt Clear"]
    #[inline(always)]
    pub fn lserdyc(&mut self) -> LSERDYC_W {
        LSERDYC_W::new(self)
    }
    #[doc = "Bit 18 - HSI Ready Interrupt Clear"]
    #[inline(always)]
    pub fn hsirdyc(&mut self) -> HSIRDYC_W {
        HSIRDYC_W::new(self)
    }
    #[doc = "Bit 19 - HSE Ready Interrupt Clear"]
    #[inline(always)]
    pub fn hserdyc(&mut self) -> HSERDYC_W {
        HSERDYC_W::new(self)
    }
    #[doc = "Bit 20 - PLL Ready Interrupt Clear"]
    #[inline(always)]
    pub fn pllrdyc(&mut self) -> PLLRDYC_W {
        PLLRDYC_W::new(self)
    }
    #[doc = "Bit 23 - Clock security system interrupt clear"]
    #[inline(always)]
    pub fn cssc(&mut self) -> CSSC_W {
        CSSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock interrupt register (RCC_INTR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](index.html) module"]
pub struct INTR_SPEC;
impl crate::RegisterSpec for INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr::R](R) reader structure"]
impl crate::Readable for INTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr::W](W) writer structure"]
impl crate::Writable for INTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for INTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
