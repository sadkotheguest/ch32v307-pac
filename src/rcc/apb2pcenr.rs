#[doc = "Register `APB2PCENR` reader"]
pub struct R(crate::R<APB2PCENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB2PCENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB2PCENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB2PCENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB2PCENR` writer"]
pub struct W(crate::W<APB2PCENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB2PCENR_SPEC>;
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
impl From<crate::W<APB2PCENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB2PCENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AFIOEN` reader - Alternate function I/O clock enable"]
pub type AFIOEN_R = crate::BitReader<bool>;
#[doc = "Field `AFIOEN` writer - Alternate function I/O clock enable"]
pub type AFIOEN_W<'a> = crate::BitWriter<'a, u32, APB2PCENR_SPEC, bool, 0>;
#[doc = "Field `IOPAEN` reader - I/O port A clock enable"]
pub type IOPAEN_R = crate::BitReader<bool>;
#[doc = "Field `IOPAEN` writer - I/O port A clock enable"]
pub type IOPAEN_W<'a> = crate::BitWriter<'a, u32, APB2PCENR_SPEC, bool, 2>;
#[doc = "Field `IOPBEN` reader - I/O port B clock enable"]
pub type IOPBEN_R = crate::BitReader<bool>;
#[doc = "Field `IOPBEN` writer - I/O port B clock enable"]
pub type IOPBEN_W<'a> = crate::BitWriter<'a, u32, APB2PCENR_SPEC, bool, 3>;
#[doc = "Field `IOPCEN` reader - I/O port C clock enable"]
pub type IOPCEN_R = crate::BitReader<bool>;
#[doc = "Field `IOPCEN` writer - I/O port C clock enable"]
pub type IOPCEN_W<'a> = crate::BitWriter<'a, u32, APB2PCENR_SPEC, bool, 4>;
#[doc = "Field `IOPDEN` reader - I/O port D clock enable"]
pub type IOPDEN_R = crate::BitReader<bool>;
#[doc = "Field `IOPDEN` writer - I/O port D clock enable"]
pub type IOPDEN_W<'a> = crate::BitWriter<'a, u32, APB2PCENR_SPEC, bool, 5>;
#[doc = "Field `ADCEN` reader - ADC interface clock enable"]
pub type ADCEN_R = crate::BitReader<bool>;
#[doc = "Field `ADCEN` writer - ADC interface clock enable"]
pub type ADCEN_W<'a> = crate::BitWriter<'a, u32, APB2PCENR_SPEC, bool, 9>;
#[doc = "Field `ADC2EN` reader - ADC 2 interface clock enable"]
pub type ADC2EN_R = crate::BitReader<bool>;
#[doc = "Field `ADC2EN` writer - ADC 2 interface clock enable"]
pub type ADC2EN_W<'a> = crate::BitWriter<'a, u32, APB2PCENR_SPEC, bool, 10>;
#[doc = "Field `TIM1EN` reader - TIM1 Timer clock enable"]
pub type TIM1EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM1EN` writer - TIM1 Timer clock enable"]
pub type TIM1EN_W<'a> = crate::BitWriter<'a, u32, APB2PCENR_SPEC, bool, 11>;
#[doc = "Field `SPI1EN` reader - SPI 1 clock enable"]
pub type SPI1EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI1EN` writer - SPI 1 clock enable"]
pub type SPI1EN_W<'a> = crate::BitWriter<'a, u32, APB2PCENR_SPEC, bool, 12>;
#[doc = "Field `USART1EN` reader - USART1 clock enable"]
pub type USART1EN_R = crate::BitReader<bool>;
#[doc = "Field `USART1EN` writer - USART1 clock enable"]
pub type USART1EN_W<'a> = crate::BitWriter<'a, u32, APB2PCENR_SPEC, bool, 14>;
#[doc = "Field `TIM9_EN` reader - TIM9 Timer clock enable"]
pub type TIM9_EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM9_EN` writer - TIM9 Timer clock enable"]
pub type TIM9_EN_W<'a> = crate::BitWriter<'a, u32, APB2PCENR_SPEC, bool, 19>;
#[doc = "Field `TIM10_EN` reader - TIM10 Timer clock enable"]
pub type TIM10_EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM10_EN` writer - TIM10 Timer clock enable"]
pub type TIM10_EN_W<'a> = crate::BitWriter<'a, u32, APB2PCENR_SPEC, bool, 20>;
impl R {
    #[doc = "Bit 0 - Alternate function I/O clock enable"]
    #[inline(always)]
    pub fn afioen(&self) -> AFIOEN_R {
        AFIOEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - I/O port A clock enable"]
    #[inline(always)]
    pub fn iopaen(&self) -> IOPAEN_R {
        IOPAEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I/O port B clock enable"]
    #[inline(always)]
    pub fn iopben(&self) -> IOPBEN_R {
        IOPBEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I/O port C clock enable"]
    #[inline(always)]
    pub fn iopcen(&self) -> IOPCEN_R {
        IOPCEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I/O port D clock enable"]
    #[inline(always)]
    pub fn iopden(&self) -> IOPDEN_R {
        IOPDEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC interface clock enable"]
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC 2 interface clock enable"]
    #[inline(always)]
    pub fn adc2en(&self) -> ADC2EN_R {
        ADC2EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 Timer clock enable"]
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI 1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 19 - TIM9 Timer clock enable"]
    #[inline(always)]
    pub fn tim9_en(&self) -> TIM9_EN_R {
        TIM9_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TIM10 Timer clock enable"]
    #[inline(always)]
    pub fn tim10_en(&self) -> TIM10_EN_R {
        TIM10_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alternate function I/O clock enable"]
    #[inline(always)]
    pub fn afioen(&mut self) -> AFIOEN_W {
        AFIOEN_W::new(self)
    }
    #[doc = "Bit 2 - I/O port A clock enable"]
    #[inline(always)]
    pub fn iopaen(&mut self) -> IOPAEN_W {
        IOPAEN_W::new(self)
    }
    #[doc = "Bit 3 - I/O port B clock enable"]
    #[inline(always)]
    pub fn iopben(&mut self) -> IOPBEN_W {
        IOPBEN_W::new(self)
    }
    #[doc = "Bit 4 - I/O port C clock enable"]
    #[inline(always)]
    pub fn iopcen(&mut self) -> IOPCEN_W {
        IOPCEN_W::new(self)
    }
    #[doc = "Bit 5 - I/O port D clock enable"]
    #[inline(always)]
    pub fn iopden(&mut self) -> IOPDEN_W {
        IOPDEN_W::new(self)
    }
    #[doc = "Bit 9 - ADC interface clock enable"]
    #[inline(always)]
    pub fn adcen(&mut self) -> ADCEN_W {
        ADCEN_W::new(self)
    }
    #[doc = "Bit 10 - ADC 2 interface clock enable"]
    #[inline(always)]
    pub fn adc2en(&mut self) -> ADC2EN_W {
        ADC2EN_W::new(self)
    }
    #[doc = "Bit 11 - TIM1 Timer clock enable"]
    #[inline(always)]
    pub fn tim1en(&mut self) -> TIM1EN_W {
        TIM1EN_W::new(self)
    }
    #[doc = "Bit 12 - SPI 1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&mut self) -> SPI1EN_W {
        SPI1EN_W::new(self)
    }
    #[doc = "Bit 14 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1en(&mut self) -> USART1EN_W {
        USART1EN_W::new(self)
    }
    #[doc = "Bit 19 - TIM9 Timer clock enable"]
    #[inline(always)]
    pub fn tim9_en(&mut self) -> TIM9_EN_W {
        TIM9_EN_W::new(self)
    }
    #[doc = "Bit 20 - TIM10 Timer clock enable"]
    #[inline(always)]
    pub fn tim10_en(&mut self) -> TIM10_EN_W {
        TIM10_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB2 peripheral clock enable register (RCC_APB2PCENR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2pcenr](index.html) module"]
pub struct APB2PCENR_SPEC;
impl crate::RegisterSpec for APB2PCENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb2pcenr::R](R) reader structure"]
impl crate::Readable for APB2PCENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb2pcenr::W](W) writer structure"]
impl crate::Writable for APB2PCENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB2PCENR to value 0"]
impl crate::Resettable for APB2PCENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}