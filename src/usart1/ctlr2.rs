#[doc = "Register `CTLR2` reader"]
pub struct R(crate::R<CTLR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTLR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTLR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTLR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTLR2` writer"]
pub struct W(crate::W<CTLR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTLR2_SPEC>;
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
impl From<crate::W<CTLR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTLR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LINEN` reader - LIN mode enable"]
pub type LINEN_R = crate::BitReader<bool>;
#[doc = "Field `LINEN` writer - LIN mode enable"]
pub type LINEN_W<'a> = crate::BitWriter<'a, u32, CTLR2_SPEC, bool, 14>;
#[doc = "Field `STOP` reader - STOP bits"]
pub type STOP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STOP` writer - STOP bits"]
pub type STOP_W<'a> = crate::FieldWriter<'a, u32, CTLR2_SPEC, u8, u8, 2, 12>;
#[doc = "Field `CLKEN` reader - Clock enable"]
pub type CLKEN_R = crate::BitReader<bool>;
#[doc = "Field `CLKEN` writer - Clock enable"]
pub type CLKEN_W<'a> = crate::BitWriter<'a, u32, CTLR2_SPEC, bool, 11>;
#[doc = "Field `CPOL` reader - Clock polarity"]
pub type CPOL_R = crate::BitReader<bool>;
#[doc = "Field `CPOL` writer - Clock polarity"]
pub type CPOL_W<'a> = crate::BitWriter<'a, u32, CTLR2_SPEC, bool, 10>;
#[doc = "Field `CPHA` reader - Clock phase"]
pub type CPHA_R = crate::BitReader<bool>;
#[doc = "Field `CPHA` writer - Clock phase"]
pub type CPHA_W<'a> = crate::BitWriter<'a, u32, CTLR2_SPEC, bool, 9>;
#[doc = "Field `LBCL` reader - Last bit clock pulse"]
pub type LBCL_R = crate::BitReader<bool>;
#[doc = "Field `LBCL` writer - Last bit clock pulse"]
pub type LBCL_W<'a> = crate::BitWriter<'a, u32, CTLR2_SPEC, bool, 8>;
#[doc = "Field `LBDIE` reader - LIN break detection interrupt enable"]
pub type LBDIE_R = crate::BitReader<bool>;
#[doc = "Field `LBDIE` writer - LIN break detection interrupt enable"]
pub type LBDIE_W<'a> = crate::BitWriter<'a, u32, CTLR2_SPEC, bool, 6>;
#[doc = "Field `LBDL` reader - lin break detection length"]
pub type LBDL_R = crate::BitReader<bool>;
#[doc = "Field `LBDL` writer - lin break detection length"]
pub type LBDL_W<'a> = crate::BitWriter<'a, u32, CTLR2_SPEC, bool, 5>;
#[doc = "Field `ADD` reader - Address of the USART node"]
pub type ADD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADD` writer - Address of the USART node"]
pub type ADD_W<'a> = crate::FieldWriter<'a, u32, CTLR2_SPEC, u8, u8, 4, 0>;
impl R {
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline(always)]
    pub fn linen(&self) -> LINEN_R {
        LINEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 12:13 - STOP bits"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 11 - Clock enable"]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Last bit clock pulse"]
    #[inline(always)]
    pub fn lbcl(&self) -> LBCL_R {
        LBCL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable"]
    #[inline(always)]
    pub fn lbdie(&self) -> LBDIE_R {
        LBDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - lin break detection length"]
    #[inline(always)]
    pub fn lbdl(&self) -> LBDL_R {
        LBDL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 0:3 - Address of the USART node"]
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline(always)]
    pub fn linen(&mut self) -> LINEN_W {
        LINEN_W::new(self)
    }
    #[doc = "Bits 12:13 - STOP bits"]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W::new(self)
    }
    #[doc = "Bit 11 - Clock enable"]
    #[inline(always)]
    pub fn clken(&mut self) -> CLKEN_W {
        CLKEN_W::new(self)
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W {
        CPOL_W::new(self)
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W {
        CPHA_W::new(self)
    }
    #[doc = "Bit 8 - Last bit clock pulse"]
    #[inline(always)]
    pub fn lbcl(&mut self) -> LBCL_W {
        LBCL_W::new(self)
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable"]
    #[inline(always)]
    pub fn lbdie(&mut self) -> LBDIE_W {
        LBDIE_W::new(self)
    }
    #[doc = "Bit 5 - lin break detection length"]
    #[inline(always)]
    pub fn lbdl(&mut self) -> LBDL_W {
        LBDL_W::new(self)
    }
    #[doc = "Bits 0:3 - Address of the USART node"]
    #[inline(always)]
    pub fn add(&mut self) -> ADD_W {
        ADD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctlr2](index.html) module"]
pub struct CTLR2_SPEC;
impl crate::RegisterSpec for CTLR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctlr2::R](R) reader structure"]
impl crate::Readable for CTLR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctlr2::W](W) writer structure"]
impl crate::Writable for CTLR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTLR2 to value 0"]
impl crate::Resettable for CTLR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}