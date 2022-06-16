#[doc = "Register `OCTLR` reader"]
pub struct R(crate::R<OCTLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCTLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCTLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCTLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OCTLR` writer"]
pub struct W(crate::W<OCTLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCTLR_SPEC>;
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
impl From<crate::W<OCTLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OCTLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAL` reader - Calibration value"]
pub type CAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAL` writer - Calibration value"]
pub type CAL_W<'a> = crate::FieldWriter<'a, u32, OCTLR_SPEC, u8, u8, 7, 0>;
#[doc = "Field `CCO` reader - Calibration Clock Output"]
pub type CCO_R = crate::BitReader<bool>;
#[doc = "Field `CCO` writer - Calibration Clock Output"]
pub type CCO_W<'a> = crate::BitWriter<'a, u32, OCTLR_SPEC, bool, 7>;
#[doc = "Field `ASOE` reader - Alarm or second output enable"]
pub type ASOE_R = crate::BitReader<bool>;
#[doc = "Field `ASOE` writer - Alarm or second output enable"]
pub type ASOE_W<'a> = crate::BitWriter<'a, u32, OCTLR_SPEC, bool, 8>;
#[doc = "Field `ASOS` reader - Alarm or second output selection"]
pub type ASOS_R = crate::BitReader<bool>;
#[doc = "Field `ASOS` writer - Alarm or second output selection"]
pub type ASOS_W<'a> = crate::BitWriter<'a, u32, OCTLR_SPEC, bool, 9>;
impl R {
    #[doc = "Bits 0:6 - Calibration value"]
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Calibration Clock Output"]
    #[inline(always)]
    pub fn cco(&self) -> CCO_R {
        CCO_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm or second output enable"]
    #[inline(always)]
    pub fn asoe(&self) -> ASOE_R {
        ASOE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Alarm or second output selection"]
    #[inline(always)]
    pub fn asos(&self) -> ASOS_R {
        ASOS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Calibration value"]
    #[inline(always)]
    pub fn cal(&mut self) -> CAL_W {
        CAL_W::new(self)
    }
    #[doc = "Bit 7 - Calibration Clock Output"]
    #[inline(always)]
    pub fn cco(&mut self) -> CCO_W {
        CCO_W::new(self)
    }
    #[doc = "Bit 8 - Alarm or second output enable"]
    #[inline(always)]
    pub fn asoe(&mut self) -> ASOE_W {
        ASOE_W::new(self)
    }
    #[doc = "Bit 9 - Alarm or second output selection"]
    #[inline(always)]
    pub fn asos(&mut self) -> ASOS_W {
        ASOS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC clock calibration register (BKP_OCTLR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [octlr](index.html) module"]
pub struct OCTLR_SPEC;
impl crate::RegisterSpec for OCTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [octlr::R](R) reader structure"]
impl crate::Readable for OCTLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [octlr::W](W) writer structure"]
impl crate::Writable for OCTLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OCTLR to value 0"]
impl crate::Resettable for OCTLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}