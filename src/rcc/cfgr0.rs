#[doc = "Register `CFGR0` reader"]
pub struct R(crate::R<CFGR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR0` writer"]
pub struct W(crate::W<CFGR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR0_SPEC>;
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
impl From<crate::W<CFGR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW` reader - System clock Switch"]
pub type SW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SW` writer - System clock Switch"]
pub type SW_W<'a> = crate::FieldWriter<'a, u32, CFGR0_SPEC, u8, u8, 2, 0>;
#[doc = "Field `SWS` reader - System Clock Switch Status"]
pub type SWS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HPRE` reader - AHB prescaler"]
pub type HPRE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HPRE` writer - AHB prescaler"]
pub type HPRE_W<'a> = crate::FieldWriter<'a, u32, CFGR0_SPEC, u8, u8, 4, 4>;
#[doc = "Field `PPRE1` reader - APB Low speed prescaler (APB1)"]
pub type PPRE1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PPRE1` writer - APB Low speed prescaler (APB1)"]
pub type PPRE1_W<'a> = crate::FieldWriter<'a, u32, CFGR0_SPEC, u8, u8, 3, 8>;
#[doc = "Field `PPRE2` reader - APB High speed prescaler (APB2)"]
pub type PPRE2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PPRE2` writer - APB High speed prescaler (APB2)"]
pub type PPRE2_W<'a> = crate::FieldWriter<'a, u32, CFGR0_SPEC, u8, u8, 3, 11>;
#[doc = "Field `ADCPRE` reader - ADC prescaler"]
pub type ADCPRE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADCPRE` writer - ADC prescaler"]
pub type ADCPRE_W<'a> = crate::FieldWriter<'a, u32, CFGR0_SPEC, u8, u8, 2, 14>;
#[doc = "Field `PLLSRC` reader - PLL entry clock source"]
pub type PLLSRC_R = crate::BitReader<bool>;
#[doc = "Field `PLLSRC` writer - PLL entry clock source"]
pub type PLLSRC_W<'a> = crate::BitWriter<'a, u32, CFGR0_SPEC, bool, 16>;
#[doc = "Field `PLLXTPRE` reader - HSE divider for PLL entry"]
pub type PLLXTPRE_R = crate::BitReader<bool>;
#[doc = "Field `PLLXTPRE` writer - HSE divider for PLL entry"]
pub type PLLXTPRE_W<'a> = crate::BitWriter<'a, u32, CFGR0_SPEC, bool, 17>;
#[doc = "Field `PLLMUL` reader - PLL Multiplication Factor"]
pub type PLLMUL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLLMUL` writer - PLL Multiplication Factor"]
pub type PLLMUL_W<'a> = crate::FieldWriter<'a, u32, CFGR0_SPEC, u8, u8, 4, 18>;
#[doc = "Field `USBPRE` reader - USB prescaler"]
pub type USBPRE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USBPRE` writer - USB prescaler"]
pub type USBPRE_W<'a> = crate::FieldWriter<'a, u32, CFGR0_SPEC, u8, u8, 2, 22>;
#[doc = "Field `MCO` reader - Microcontroller clock output"]
pub type MCO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MCO` writer - Microcontroller clock output"]
pub type MCO_W<'a> = crate::FieldWriter<'a, u32, CFGR0_SPEC, u8, u8, 4, 24>;
#[doc = "Field `ADC_CLK_ADJ` reader - ADC clock ADJ"]
pub type ADC_CLK_ADJ_R = crate::BitReader<bool>;
#[doc = "Field `ADC_CLK_ADJ` writer - ADC clock ADJ"]
pub type ADC_CLK_ADJ_W<'a> = crate::BitWriter<'a, u32, CFGR0_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:1 - System clock Switch"]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - System Clock Switch Status"]
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - AHB prescaler"]
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - APB Low speed prescaler (APB1)"]
    #[inline(always)]
    pub fn ppre1(&self) -> PPRE1_R {
        PPRE1_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - APB High speed prescaler (APB2)"]
    #[inline(always)]
    pub fn ppre2(&self) -> PPRE2_R {
        PPRE2_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:15 - ADC prescaler"]
    #[inline(always)]
    pub fn adcpre(&self) -> ADCPRE_R {
        ADCPRE_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - PLL entry clock source"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HSE divider for PLL entry"]
    #[inline(always)]
    pub fn pllxtpre(&self) -> PLLXTPRE_R {
        PLLXTPRE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:21 - PLL Multiplication Factor"]
    #[inline(always)]
    pub fn pllmul(&self) -> PLLMUL_R {
        PLLMUL_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:23 - USB prescaler"]
    #[inline(always)]
    pub fn usbpre(&self) -> USBPRE_R {
        USBPRE_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:27 - Microcontroller clock output"]
    #[inline(always)]
    pub fn mco(&self) -> MCO_R {
        MCO_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - ADC clock ADJ"]
    #[inline(always)]
    pub fn adc_clk_adj(&self) -> ADC_CLK_ADJ_R {
        ADC_CLK_ADJ_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - System clock Switch"]
    #[inline(always)]
    pub fn sw(&mut self) -> SW_W {
        SW_W::new(self)
    }
    #[doc = "Bits 4:7 - AHB prescaler"]
    #[inline(always)]
    pub fn hpre(&mut self) -> HPRE_W {
        HPRE_W::new(self)
    }
    #[doc = "Bits 8:10 - APB Low speed prescaler (APB1)"]
    #[inline(always)]
    pub fn ppre1(&mut self) -> PPRE1_W {
        PPRE1_W::new(self)
    }
    #[doc = "Bits 11:13 - APB High speed prescaler (APB2)"]
    #[inline(always)]
    pub fn ppre2(&mut self) -> PPRE2_W {
        PPRE2_W::new(self)
    }
    #[doc = "Bits 14:15 - ADC prescaler"]
    #[inline(always)]
    pub fn adcpre(&mut self) -> ADCPRE_W {
        ADCPRE_W::new(self)
    }
    #[doc = "Bit 16 - PLL entry clock source"]
    #[inline(always)]
    pub fn pllsrc(&mut self) -> PLLSRC_W {
        PLLSRC_W::new(self)
    }
    #[doc = "Bit 17 - HSE divider for PLL entry"]
    #[inline(always)]
    pub fn pllxtpre(&mut self) -> PLLXTPRE_W {
        PLLXTPRE_W::new(self)
    }
    #[doc = "Bits 18:21 - PLL Multiplication Factor"]
    #[inline(always)]
    pub fn pllmul(&mut self) -> PLLMUL_W {
        PLLMUL_W::new(self)
    }
    #[doc = "Bits 22:23 - USB prescaler"]
    #[inline(always)]
    pub fn usbpre(&mut self) -> USBPRE_W {
        USBPRE_W::new(self)
    }
    #[doc = "Bits 24:27 - Microcontroller clock output"]
    #[inline(always)]
    pub fn mco(&mut self) -> MCO_W {
        MCO_W::new(self)
    }
    #[doc = "Bit 31 - ADC clock ADJ"]
    #[inline(always)]
    pub fn adc_clk_adj(&mut self) -> ADC_CLK_ADJ_W {
        ADC_CLK_ADJ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock configuration register (RCC_CFGR0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr0](index.html) module"]
pub struct CFGR0_SPEC;
impl crate::RegisterSpec for CFGR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr0::R](R) reader structure"]
impl crate::Readable for CFGR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr0::W](W) writer structure"]
impl crate::Writable for CFGR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGR0 to value 0"]
impl crate::Resettable for CFGR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
