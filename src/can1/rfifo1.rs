#[doc = "Register `RFIFO1` reader"]
pub struct R(crate::R<RFIFO1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFIFO1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFIFO1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFIFO1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RFIFO1` writer"]
pub struct W(crate::W<RFIFO1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFIFO1_SPEC>;
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
impl From<crate::W<RFIFO1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFIFO1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFOM1` reader - RFOM1"]
pub type RFOM1_R = crate::BitReader<bool>;
#[doc = "Field `RFOM1` writer - RFOM1"]
pub type RFOM1_W<'a> = crate::BitWriter<'a, u32, RFIFO1_SPEC, bool, 5>;
#[doc = "Field `FOVR1` reader - FOVR1"]
pub type FOVR1_R = crate::BitReader<bool>;
#[doc = "Field `FOVR1` writer - FOVR1"]
pub type FOVR1_W<'a> = crate::BitWriter<'a, u32, RFIFO1_SPEC, bool, 4>;
#[doc = "Field `FULL1` reader - FULL1"]
pub type FULL1_R = crate::BitReader<bool>;
#[doc = "Field `FULL1` writer - FULL1"]
pub type FULL1_W<'a> = crate::BitWriter<'a, u32, RFIFO1_SPEC, bool, 3>;
#[doc = "Field `FMP1` reader - FMP1"]
pub type FMP1_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 5 - RFOM1"]
    #[inline(always)]
    pub fn rfom1(&self) -> RFOM1_R {
        RFOM1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - FOVR1"]
    #[inline(always)]
    pub fn fovr1(&self) -> FOVR1_R {
        FOVR1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - FULL1"]
    #[inline(always)]
    pub fn full1(&self) -> FULL1_R {
        FULL1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 0:1 - FMP1"]
    #[inline(always)]
    pub fn fmp1(&self) -> FMP1_R {
        FMP1_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 5 - RFOM1"]
    #[inline(always)]
    pub fn rfom1(&mut self) -> RFOM1_W {
        RFOM1_W::new(self)
    }
    #[doc = "Bit 4 - FOVR1"]
    #[inline(always)]
    pub fn fovr1(&mut self) -> FOVR1_W {
        FOVR1_W::new(self)
    }
    #[doc = "Bit 3 - FULL1"]
    #[inline(always)]
    pub fn full1(&mut self) -> FULL1_W {
        FULL1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RFIFO1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfifo1](index.html) module"]
pub struct RFIFO1_SPEC;
impl crate::RegisterSpec for RFIFO1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfifo1::R](R) reader structure"]
impl crate::Readable for RFIFO1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfifo1::W](W) writer structure"]
impl crate::Writable for RFIFO1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RFIFO1 to value 0"]
impl crate::Resettable for RFIFO1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}