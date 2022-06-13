#[doc = "Register `PCFR` reader"]
pub struct R(crate::R<PCFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCFR` writer"]
pub struct W(crate::W<PCFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCFR_SPEC>;
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
impl From<crate::W<PCFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI1RM` reader - SPI1 remapping"]
pub type SPI1RM_R = crate::BitReader<bool>;
#[doc = "Field `SPI1RM` writer - SPI1 remapping"]
pub type SPI1RM_W<'a> = crate::BitWriter<'a, u32, PCFR_SPEC, bool, 0>;
#[doc = "Field `I2C1RM` reader - I2C1 remapping"]
pub type I2C1RM_R = crate::BitReader<bool>;
#[doc = "Field `I2C1RM` writer - I2C1 remapping"]
pub type I2C1RM_W<'a> = crate::BitWriter<'a, u32, PCFR_SPEC, bool, 1>;
#[doc = "Field `USART1RM` reader - USART1 remapping"]
pub type USART1RM_R = crate::BitReader<bool>;
#[doc = "Field `USART1RM` writer - USART1 remapping"]
pub type USART1RM_W<'a> = crate::BitWriter<'a, u32, PCFR_SPEC, bool, 2>;
#[doc = "Field `USART2RM` reader - USART2 remapping"]
pub type USART2RM_R = crate::BitReader<bool>;
#[doc = "Field `USART2RM` writer - USART2 remapping"]
pub type USART2RM_W<'a> = crate::BitWriter<'a, u32, PCFR_SPEC, bool, 3>;
#[doc = "Field `USART3RM` reader - USART3 remapping"]
pub type USART3RM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USART3RM` writer - USART3 remapping"]
pub type USART3RM_W<'a> = crate::FieldWriter<'a, u32, PCFR_SPEC, u8, u8, 2, 4>;
#[doc = "Field `TIM1RM` reader - TIM1 remapping"]
pub type TIM1RM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIM1RM` writer - TIM1 remapping"]
pub type TIM1RM_W<'a> = crate::FieldWriter<'a, u32, PCFR_SPEC, u8, u8, 2, 6>;
#[doc = "Field `TIM2RM` reader - TIM2 remapping"]
pub type TIM2RM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIM2RM` writer - TIM2 remapping"]
pub type TIM2RM_W<'a> = crate::FieldWriter<'a, u32, PCFR_SPEC, u8, u8, 2, 8>;
#[doc = "Field `TIM3RM` reader - TIM3 remapping"]
pub type TIM3RM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIM3RM` writer - TIM3 remapping"]
pub type TIM3RM_W<'a> = crate::FieldWriter<'a, u32, PCFR_SPEC, u8, u8, 2, 10>;
#[doc = "Field `CANRM` reader - CAN1 remapping"]
pub type CANRM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CANRM` writer - CAN1 remapping"]
pub type CANRM_W<'a> = crate::FieldWriter<'a, u32, PCFR_SPEC, u8, u8, 2, 13>;
#[doc = "Field `PD01RM` reader - Port D0/Port D1 mapping on OSCIN/OSCOUT"]
pub type PD01RM_R = crate::BitReader<bool>;
#[doc = "Field `PD01RM` writer - Port D0/Port D1 mapping on OSCIN/OSCOUT"]
pub type PD01RM_W<'a> = crate::BitWriter<'a, u32, PCFR_SPEC, bool, 15>;
#[doc = "Field `SWCFG` writer - Serial wire JTAG configuration"]
pub type SWCFG_W<'a> = crate::FieldWriter<'a, u32, PCFR_SPEC, u8, u8, 3, 24>;
impl R {
    #[doc = "Bit 0 - SPI1 remapping"]
    #[inline(always)]
    pub fn spi1rm(&self) -> SPI1RM_R {
        SPI1RM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C1 remapping"]
    #[inline(always)]
    pub fn i2c1rm(&self) -> I2C1RM_R {
        I2C1RM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USART1 remapping"]
    #[inline(always)]
    pub fn usart1rm(&self) -> USART1RM_R {
        USART1RM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USART2 remapping"]
    #[inline(always)]
    pub fn usart2rm(&self) -> USART2RM_R {
        USART2RM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - USART3 remapping"]
    #[inline(always)]
    pub fn usart3rm(&self) -> USART3RM_R {
        USART3RM_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - TIM1 remapping"]
    #[inline(always)]
    pub fn tim1rm(&self) -> TIM1RM_R {
        TIM1RM_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - TIM2 remapping"]
    #[inline(always)]
    pub fn tim2rm(&self) -> TIM2RM_R {
        TIM2RM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - TIM3 remapping"]
    #[inline(always)]
    pub fn tim3rm(&self) -> TIM3RM_R {
        TIM3RM_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 13:14 - CAN1 remapping"]
    #[inline(always)]
    pub fn canrm(&self) -> CANRM_R {
        CANRM_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Port D0/Port D1 mapping on OSCIN/OSCOUT"]
    #[inline(always)]
    pub fn pd01rm(&self) -> PD01RM_R {
        PD01RM_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI1 remapping"]
    #[inline(always)]
    pub fn spi1rm(&mut self) -> SPI1RM_W {
        SPI1RM_W::new(self)
    }
    #[doc = "Bit 1 - I2C1 remapping"]
    #[inline(always)]
    pub fn i2c1rm(&mut self) -> I2C1RM_W {
        I2C1RM_W::new(self)
    }
    #[doc = "Bit 2 - USART1 remapping"]
    #[inline(always)]
    pub fn usart1rm(&mut self) -> USART1RM_W {
        USART1RM_W::new(self)
    }
    #[doc = "Bit 3 - USART2 remapping"]
    #[inline(always)]
    pub fn usart2rm(&mut self) -> USART2RM_W {
        USART2RM_W::new(self)
    }
    #[doc = "Bits 4:5 - USART3 remapping"]
    #[inline(always)]
    pub fn usart3rm(&mut self) -> USART3RM_W {
        USART3RM_W::new(self)
    }
    #[doc = "Bits 6:7 - TIM1 remapping"]
    #[inline(always)]
    pub fn tim1rm(&mut self) -> TIM1RM_W {
        TIM1RM_W::new(self)
    }
    #[doc = "Bits 8:9 - TIM2 remapping"]
    #[inline(always)]
    pub fn tim2rm(&mut self) -> TIM2RM_W {
        TIM2RM_W::new(self)
    }
    #[doc = "Bits 10:11 - TIM3 remapping"]
    #[inline(always)]
    pub fn tim3rm(&mut self) -> TIM3RM_W {
        TIM3RM_W::new(self)
    }
    #[doc = "Bits 13:14 - CAN1 remapping"]
    #[inline(always)]
    pub fn canrm(&mut self) -> CANRM_W {
        CANRM_W::new(self)
    }
    #[doc = "Bit 15 - Port D0/Port D1 mapping on OSCIN/OSCOUT"]
    #[inline(always)]
    pub fn pd01rm(&mut self) -> PD01RM_W {
        PD01RM_W::new(self)
    }
    #[doc = "Bits 24:26 - Serial wire JTAG configuration"]
    #[inline(always)]
    pub fn swcfg(&mut self) -> SWCFG_W {
        SWCFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AF remap and debug I/O configuration register (AFIO_PCFR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcfr](index.html) module"]
pub struct PCFR_SPEC;
impl crate::RegisterSpec for PCFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcfr::R](R) reader structure"]
impl crate::Readable for PCFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcfr::W](W) writer structure"]
impl crate::Writable for PCFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCFR to value 0"]
impl crate::Resettable for PCFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}