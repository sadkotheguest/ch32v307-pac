#[doc = "Register `FIFOADDRR2` reader"]
pub struct R(crate::R<FIFOADDRR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOADDRR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOADDRR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOADDRR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOADDRR2` writer"]
pub struct W(crate::W<FIFOADDRR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOADDRR2_SPEC>;
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
impl From<crate::W<FIFOADDRR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOADDRR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FI2EN` reader - FI2EN"]
pub type FI2EN_R = crate::BitReader<bool>;
#[doc = "Field `FI2EN` writer - FI2EN"]
pub type FI2EN_W<'a> = crate::BitWriter<'a, u32, FIFOADDRR2_SPEC, bool, 0>;
#[doc = "Field ` ADDR2` reader - ADDR2"]
pub type ADDR2_R = crate::FieldReader<u32, u32>;
#[doc = "Field ` ADDR2` writer - ADDR2"]
pub type ADDR2_W<'a> = crate::FieldWriter<'a, u32, FIFOADDRR2_SPEC, u32, u32, 31, 1>;
impl R {
    #[doc = "Bit 0 - FI2EN"]
    #[inline(always)]
    pub fn fi2en(&self) -> FI2EN_R {
        FI2EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - ADDR2"]
    #[inline(always)]
    pub fn addr2(&self) -> ADDR2_R {
        ADDR2_R::new(((self.bits >> 1) & 0x7fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 0 - FI2EN"]
    #[inline(always)]
    pub fn fi2en(&mut self) -> FI2EN_W {
        FI2EN_W::new(self)
    }
    #[doc = "Bits 1:31 - ADDR2"]
    #[inline(always)]
    pub fn addr2(&mut self) -> ADDR2_W {
        ADDR2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt 2 address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifoaddrr2](index.html) module"]
pub struct FIFOADDRR2_SPEC;
impl crate::RegisterSpec for FIFOADDRR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifoaddrr2::R](R) reader structure"]
impl crate::Readable for FIFOADDRR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifoaddrr2::W](W) writer structure"]
impl crate::Writable for FIFOADDRR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFOADDRR2 to value 0"]
impl crate::Resettable for FIFOADDRR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}