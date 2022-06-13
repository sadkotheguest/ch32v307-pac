#[doc = "Register `FIFOADDRR1` reader"]
pub struct R(crate::R<FIFOADDRR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOADDRR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOADDRR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOADDRR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOADDRR1` writer"]
pub struct W(crate::W<FIFOADDRR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOADDRR1_SPEC>;
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
impl From<crate::W<FIFOADDRR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOADDRR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FI1EN` reader - FI1EN"]
pub type FI1EN_R = crate::BitReader<bool>;
#[doc = "Field `FI1EN` writer - FI1EN"]
pub type FI1EN_W<'a> = crate::BitWriter<'a, u32, FIFOADDRR1_SPEC, bool, 0>;
#[doc = "Field ` ADDR1` reader - ADDR1"]
pub type ADDR1_R = crate::FieldReader<u32, u32>;
#[doc = "Field ` ADDR1` writer - ADDR1"]
pub type ADDR1_W<'a> = crate::FieldWriter<'a, u32, FIFOADDRR1_SPEC, u32, u32, 31, 1>;
impl R {
    #[doc = "Bit 0 - FI1EN"]
    #[inline(always)]
    pub fn fi1en(&self) -> FI1EN_R {
        FI1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - ADDR1"]
    #[inline(always)]
    pub fn addr1(&self) -> ADDR1_R {
        ADDR1_R::new(((self.bits >> 1) & 0x7fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 0 - FI1EN"]
    #[inline(always)]
    pub fn fi1en(&mut self) -> FI1EN_W {
        FI1EN_W::new(self)
    }
    #[doc = "Bits 1:31 - ADDR1"]
    #[inline(always)]
    pub fn addr1(&mut self) -> ADDR1_W {
        ADDR1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt 1 address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifoaddrr1](index.html) module"]
pub struct FIFOADDRR1_SPEC;
impl crate::RegisterSpec for FIFOADDRR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifoaddrr1::R](R) reader structure"]
impl crate::Readable for FIFOADDRR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifoaddrr1::W](W) writer structure"]
impl crate::Writable for FIFOADDRR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFOADDRR1 to value 0"]
impl crate::Resettable for FIFOADDRR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}