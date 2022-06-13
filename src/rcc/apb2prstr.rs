#[doc = "Register `APB2PRSTR` reader"]
pub struct R(crate::R<APB2PRSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB2PRSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB2PRSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB2PRSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB2PRSTR` writer"]
pub struct W(crate::W<APB2PRSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB2PRSTR_SPEC>;
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
impl From<crate::W<APB2PRSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB2PRSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AFIORST` reader - Alternate function I/O reset"]
pub type AFIORST_R = crate::BitReader<bool>;
#[doc = "Field `AFIORST` writer - Alternate function I/O reset"]
pub type AFIORST_W<'a> = crate::BitWriter<'a, u32, APB2PRSTR_SPEC, bool, 0>;
#[doc = "Field `IOPARST` reader - IO port A reset"]
pub type IOPARST_R = crate::BitReader<bool>;
#[doc = "Field `IOPARST` writer - IO port A reset"]
pub type IOPARST_W<'a> = crate::BitWriter<'a, u32, APB2PRSTR_SPEC, bool, 2>;
#[doc = "Field `IOPBRST` reader - IO port B reset"]
pub type IOPBRST_R = crate::BitReader<bool>;
#[doc = "Field `IOPBRST` writer - IO port B reset"]
pub type IOPBRST_W<'a> = crate::BitWriter<'a, u32, APB2PRSTR_SPEC, bool, 3>;
#[doc = "Field `IOPCRST` reader - IO port C reset"]
pub type IOPCRST_R = crate::BitReader<bool>;
#[doc = "Field `IOPCRST` writer - IO port C reset"]
pub type IOPCRST_W<'a> = crate::BitWriter<'a, u32, APB2PRSTR_SPEC, bool, 4>;
#[doc = "Field `IOPDRST` reader - IO port D reset"]
pub type IOPDRST_R = crate::BitReader<bool>;
#[doc = "Field `IOPDRST` writer - IO port D reset"]
pub type IOPDRST_W<'a> = crate::BitWriter<'a, u32, APB2PRSTR_SPEC, bool, 5>;
#[doc = "Field `ADCRST` reader - ADC interface reset"]
pub type ADCRST_R = crate::BitReader<bool>;
#[doc = "Field `ADCRST` writer - ADC interface reset"]
pub type ADCRST_W<'a> = crate::BitWriter<'a, u32, APB2PRSTR_SPEC, bool, 9>;
#[doc = "Field `TIM1RST` reader - TIM1 timer reset"]
pub type TIM1RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM1RST` writer - TIM1 timer reset"]
pub type TIM1RST_W<'a> = crate::BitWriter<'a, u32, APB2PRSTR_SPEC, bool, 11>;
#[doc = "Field `SPI1RST` reader - SPI 1 reset"]
pub type SPI1RST_R = crate::BitReader<bool>;
#[doc = "Field `SPI1RST` writer - SPI 1 reset"]
pub type SPI1RST_W<'a> = crate::BitWriter<'a, u32, APB2PRSTR_SPEC, bool, 12>;
#[doc = "Field `USART1RST` reader - USART1 reset"]
pub type USART1RST_R = crate::BitReader<bool>;
#[doc = "Field `USART1RST` writer - USART1 reset"]
pub type USART1RST_W<'a> = crate::BitWriter<'a, u32, APB2PRSTR_SPEC, bool, 14>;
impl R {
    #[doc = "Bit 0 - Alternate function I/O reset"]
    #[inline(always)]
    pub fn afiorst(&self) -> AFIORST_R {
        AFIORST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - IO port A reset"]
    #[inline(always)]
    pub fn ioparst(&self) -> IOPARST_R {
        IOPARST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO port B reset"]
    #[inline(always)]
    pub fn iopbrst(&self) -> IOPBRST_R {
        IOPBRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO port C reset"]
    #[inline(always)]
    pub fn iopcrst(&self) -> IOPCRST_R {
        IOPCRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO port D reset"]
    #[inline(always)]
    pub fn iopdrst(&self) -> IOPDRST_R {
        IOPDRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC interface reset"]
    #[inline(always)]
    pub fn adcrst(&self) -> ADCRST_R {
        ADCRST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer reset"]
    #[inline(always)]
    pub fn tim1rst(&self) -> TIM1RST_R {
        TIM1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI 1 reset"]
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alternate function I/O reset"]
    #[inline(always)]
    pub fn afiorst(&mut self) -> AFIORST_W {
        AFIORST_W::new(self)
    }
    #[doc = "Bit 2 - IO port A reset"]
    #[inline(always)]
    pub fn ioparst(&mut self) -> IOPARST_W {
        IOPARST_W::new(self)
    }
    #[doc = "Bit 3 - IO port B reset"]
    #[inline(always)]
    pub fn iopbrst(&mut self) -> IOPBRST_W {
        IOPBRST_W::new(self)
    }
    #[doc = "Bit 4 - IO port C reset"]
    #[inline(always)]
    pub fn iopcrst(&mut self) -> IOPCRST_W {
        IOPCRST_W::new(self)
    }
    #[doc = "Bit 5 - IO port D reset"]
    #[inline(always)]
    pub fn iopdrst(&mut self) -> IOPDRST_W {
        IOPDRST_W::new(self)
    }
    #[doc = "Bit 9 - ADC interface reset"]
    #[inline(always)]
    pub fn adcrst(&mut self) -> ADCRST_W {
        ADCRST_W::new(self)
    }
    #[doc = "Bit 11 - TIM1 timer reset"]
    #[inline(always)]
    pub fn tim1rst(&mut self) -> TIM1RST_W {
        TIM1RST_W::new(self)
    }
    #[doc = "Bit 12 - SPI 1 reset"]
    #[inline(always)]
    pub fn spi1rst(&mut self) -> SPI1RST_W {
        SPI1RST_W::new(self)
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    pub fn usart1rst(&mut self) -> USART1RST_W {
        USART1RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB2 peripheral reset register (RCC_APB2PRSTR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2prstr](index.html) module"]
pub struct APB2PRSTR_SPEC;
impl crate::RegisterSpec for APB2PRSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb2prstr::R](R) reader structure"]
impl crate::Readable for APB2PRSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb2prstr::W](W) writer structure"]
impl crate::Writable for APB2PRSTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB2PRSTR to value 0"]
impl crate::Resettable for APB2PRSTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}