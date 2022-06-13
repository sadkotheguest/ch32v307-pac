#[doc = "Register `RFIFO0` reader"]
pub struct R(crate::R<RFIFO0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFIFO0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFIFO0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFIFO0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RFIFO0` writer"]
pub struct W(crate::W<RFIFO0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFIFO0_SPEC>;
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
impl From<crate::W<RFIFO0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFIFO0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFOM0` reader - RFOM0"]
pub type RFOM0_R = crate::BitReader<bool>;
#[doc = "Field `RFOM0` writer - RFOM0"]
pub type RFOM0_W<'a> = crate::BitWriter<'a, u32, RFIFO0_SPEC, bool, 5>;
#[doc = "Field `FOVR0` reader - FOVR0"]
pub type FOVR0_R = crate::BitReader<bool>;
#[doc = "Field `FOVR0` writer - FOVR0"]
pub type FOVR0_W<'a> = crate::BitWriter<'a, u32, RFIFO0_SPEC, bool, 4>;
#[doc = "Field `FULL0` reader - FULL0"]
pub type FULL0_R = crate::BitReader<bool>;
#[doc = "Field `FULL0` writer - FULL0"]
pub type FULL0_W<'a> = crate::BitWriter<'a, u32, RFIFO0_SPEC, bool, 3>;
#[doc = "Field `FMP0` reader - FMP0"]
pub type FMP0_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 5 - RFOM0"]
    #[inline(always)]
    pub fn rfom0(&self) -> RFOM0_R {
        RFOM0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - FOVR0"]
    #[inline(always)]
    pub fn fovr0(&self) -> FOVR0_R {
        FOVR0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - FULL0"]
    #[inline(always)]
    pub fn full0(&self) -> FULL0_R {
        FULL0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 0:1 - FMP0"]
    #[inline(always)]
    pub fn fmp0(&self) -> FMP0_R {
        FMP0_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 5 - RFOM0"]
    #[inline(always)]
    pub fn rfom0(&mut self) -> RFOM0_W {
        RFOM0_W::new(self)
    }
    #[doc = "Bit 4 - FOVR0"]
    #[inline(always)]
    pub fn fovr0(&mut self) -> FOVR0_W {
        FOVR0_W::new(self)
    }
    #[doc = "Bit 3 - FULL0"]
    #[inline(always)]
    pub fn full0(&mut self) -> FULL0_W {
        FULL0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RFIFO0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfifo0](index.html) module"]
pub struct RFIFO0_SPEC;
impl crate::RegisterSpec for RFIFO0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfifo0::R](R) reader structure"]
impl crate::Readable for RFIFO0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfifo0::W](W) writer structure"]
impl crate::Writable for RFIFO0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RFIFO0 to value 0"]
impl crate::Resettable for RFIFO0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}