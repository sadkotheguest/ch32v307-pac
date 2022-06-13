#[doc = "Register `FIFOADDRR0` reader"]
pub struct R(crate::R<FIFOADDRR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOADDRR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOADDRR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOADDRR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOADDRR0` writer"]
pub struct W(crate::W<FIFOADDRR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOADDRR0_SPEC>;
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
impl From<crate::W<FIFOADDRR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOADDRR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FI0EN` reader - FI0EN"]
pub type FI0EN_R = crate::BitReader<bool>;
#[doc = "Field `FI0EN` writer - FI0EN"]
pub type FI0EN_W<'a> = crate::BitWriter<'a, u32, FIFOADDRR0_SPEC, bool, 0>;
#[doc = "Field ` ADDR0` reader - ADDR0"]
pub type ADDR0_R = crate::FieldReader<u32, u32>;
#[doc = "Field ` ADDR0` writer - ADDR0"]
pub type ADDR0_W<'a> = crate::FieldWriter<'a, u32, FIFOADDRR0_SPEC, u32, u32, 31, 1>;
impl R {
    #[doc = "Bit 0 - FI0EN"]
    #[inline(always)]
    pub fn fi0en(&self) -> FI0EN_R {
        FI0EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - ADDR0"]
    #[inline(always)]
    pub fn addr0(&self) -> ADDR0_R {
        ADDR0_R::new(((self.bits >> 1) & 0x7fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 0 - FI0EN"]
    #[inline(always)]
    pub fn fi0en(&mut self) -> FI0EN_W {
        FI0EN_W::new(self)
    }
    #[doc = "Bits 1:31 - ADDR0"]
    #[inline(always)]
    pub fn addr0(&mut self) -> ADDR0_W {
        ADDR0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt 0 address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifoaddrr0](index.html) module"]
pub struct FIFOADDRR0_SPEC;
impl crate::RegisterSpec for FIFOADDRR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifoaddrr0::R](R) reader structure"]
impl crate::Readable for FIFOADDRR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifoaddrr0::W](W) writer structure"]
impl crate::Writable for FIFOADDRR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFOADDRR0 to value 0"]
impl crate::Resettable for FIFOADDRR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}