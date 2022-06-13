#[doc = "Register `PCFR2` reader"]
pub struct R(crate::R<PCFR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCFR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCFR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCFR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCFR2` writer"]
pub struct W(crate::W<PCFR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCFR2_SPEC>;
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
impl From<crate::W<PCFR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCFR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM8_REMAP` reader - TIM8 remapping"]
pub type TIM8_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `TIM8_REMAP` writer - TIM8 remapping"]
pub type TIM8_REMAP_W<'a> = crate::BitWriter<'a, u32, PCFR2_SPEC, bool, 2>;
#[doc = "Field `TIM9_REMAP` reader - TIM9 remapping"]
pub type TIM9_REMAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIM9_REMAP` writer - TIM9 remapping"]
pub type TIM9_REMAP_W<'a> = crate::FieldWriter<'a, u32, PCFR2_SPEC, u8, u8, 2, 3>;
#[doc = "Field `TIM10_REMAP` reader - TIM10 remapping"]
pub type TIM10_REMAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIM10_REMAP` writer - TIM10 remapping"]
pub type TIM10_REMAP_W<'a> = crate::FieldWriter<'a, u32, PCFR2_SPEC, u8, u8, 2, 5>;
#[doc = "Field `FSMC_NADV` reader - FSMC_NADV"]
pub type FSMC_NADV_R = crate::BitReader<bool>;
#[doc = "Field `FSMC_NADV` writer - FSMC_NADV"]
pub type FSMC_NADV_W<'a> = crate::BitWriter<'a, u32, PCFR2_SPEC, bool, 10>;
#[doc = "Field `UART4_REMAP` reader - UART4 remapping"]
pub type UART4_REMAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UART4_REMAP` writer - UART4 remapping"]
pub type UART4_REMAP_W<'a> = crate::FieldWriter<'a, u32, PCFR2_SPEC, u8, u8, 2, 16>;
#[doc = "Field `UART5_REMAP` reader - UART5 remapping"]
pub type UART5_REMAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UART5_REMAP` writer - UART5 remapping"]
pub type UART5_REMAP_W<'a> = crate::FieldWriter<'a, u32, PCFR2_SPEC, u8, u8, 2, 18>;
#[doc = "Field `UART6_REMAP` reader - UART6 remapping"]
pub type UART6_REMAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UART6_REMAP` writer - UART6 remapping"]
pub type UART6_REMAP_W<'a> = crate::FieldWriter<'a, u32, PCFR2_SPEC, u8, u8, 2, 20>;
#[doc = "Field `UART7_REMAP` reader - UART7 remapping"]
pub type UART7_REMAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UART7_REMAP` writer - UART7 remapping"]
pub type UART7_REMAP_W<'a> = crate::FieldWriter<'a, u32, PCFR2_SPEC, u8, u8, 2, 22>;
#[doc = "Field `UART8_REMAP` reader - UART8 remapping"]
pub type UART8_REMAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UART8_REMAP` writer - UART8 remapping"]
pub type UART8_REMAP_W<'a> = crate::FieldWriter<'a, u32, PCFR2_SPEC, u8, u8, 2, 24>;
#[doc = "Field `UART1_REMAP2` reader - UART1 remapping"]
pub type UART1_REMAP2_R = crate::BitReader<bool>;
#[doc = "Field `UART1_REMAP2` writer - UART1 remapping"]
pub type UART1_REMAP2_W<'a> = crate::BitWriter<'a, u32, PCFR2_SPEC, bool, 26>;
impl R {
    #[doc = "Bit 2 - TIM8 remapping"]
    #[inline(always)]
    pub fn tim8_remap(&self) -> TIM8_REMAP_R {
        TIM8_REMAP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - TIM9 remapping"]
    #[inline(always)]
    pub fn tim9_remap(&self) -> TIM9_REMAP_R {
        TIM9_REMAP_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - TIM10 remapping"]
    #[inline(always)]
    pub fn tim10_remap(&self) -> TIM10_REMAP_R {
        TIM10_REMAP_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 10 - FSMC_NADV"]
    #[inline(always)]
    pub fn fsmc_nadv(&self) -> FSMC_NADV_R {
        FSMC_NADV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 16:17 - UART4 remapping"]
    #[inline(always)]
    pub fn uart4_remap(&self) -> UART4_REMAP_R {
        UART4_REMAP_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - UART5 remapping"]
    #[inline(always)]
    pub fn uart5_remap(&self) -> UART5_REMAP_R {
        UART5_REMAP_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - UART6 remapping"]
    #[inline(always)]
    pub fn uart6_remap(&self) -> UART6_REMAP_R {
        UART6_REMAP_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - UART7 remapping"]
    #[inline(always)]
    pub fn uart7_remap(&self) -> UART7_REMAP_R {
        UART7_REMAP_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - UART8 remapping"]
    #[inline(always)]
    pub fn uart8_remap(&self) -> UART8_REMAP_R {
        UART8_REMAP_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - UART1 remapping"]
    #[inline(always)]
    pub fn uart1_remap2(&self) -> UART1_REMAP2_R {
        UART1_REMAP2_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - TIM8 remapping"]
    #[inline(always)]
    pub fn tim8_remap(&mut self) -> TIM8_REMAP_W {
        TIM8_REMAP_W::new(self)
    }
    #[doc = "Bits 3:4 - TIM9 remapping"]
    #[inline(always)]
    pub fn tim9_remap(&mut self) -> TIM9_REMAP_W {
        TIM9_REMAP_W::new(self)
    }
    #[doc = "Bits 5:6 - TIM10 remapping"]
    #[inline(always)]
    pub fn tim10_remap(&mut self) -> TIM10_REMAP_W {
        TIM10_REMAP_W::new(self)
    }
    #[doc = "Bit 10 - FSMC_NADV"]
    #[inline(always)]
    pub fn fsmc_nadv(&mut self) -> FSMC_NADV_W {
        FSMC_NADV_W::new(self)
    }
    #[doc = "Bits 16:17 - UART4 remapping"]
    #[inline(always)]
    pub fn uart4_remap(&mut self) -> UART4_REMAP_W {
        UART4_REMAP_W::new(self)
    }
    #[doc = "Bits 18:19 - UART5 remapping"]
    #[inline(always)]
    pub fn uart5_remap(&mut self) -> UART5_REMAP_W {
        UART5_REMAP_W::new(self)
    }
    #[doc = "Bits 20:21 - UART6 remapping"]
    #[inline(always)]
    pub fn uart6_remap(&mut self) -> UART6_REMAP_W {
        UART6_REMAP_W::new(self)
    }
    #[doc = "Bits 22:23 - UART7 remapping"]
    #[inline(always)]
    pub fn uart7_remap(&mut self) -> UART7_REMAP_W {
        UART7_REMAP_W::new(self)
    }
    #[doc = "Bits 24:25 - UART8 remapping"]
    #[inline(always)]
    pub fn uart8_remap(&mut self) -> UART8_REMAP_W {
        UART8_REMAP_W::new(self)
    }
    #[doc = "Bit 26 - UART1 remapping"]
    #[inline(always)]
    pub fn uart1_remap2(&mut self) -> UART1_REMAP2_W {
        UART1_REMAP2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AF remap and debug I/O configuration register (AFIO_PCFR2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcfr2](index.html) module"]
pub struct PCFR2_SPEC;
impl crate::RegisterSpec for PCFR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcfr2::R](R) reader structure"]
impl crate::Readable for PCFR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcfr2::W](W) writer structure"]
impl crate::Writable for PCFR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCFR2 to value 0"]
impl crate::Resettable for PCFR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}