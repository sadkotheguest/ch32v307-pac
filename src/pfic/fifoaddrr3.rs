#[doc = "Register `FIFOADDRR3` reader"]
pub struct R(crate::R<FIFOADDRR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOADDRR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOADDRR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOADDRR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOADDRR3` writer"]
pub struct W(crate::W<FIFOADDRR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOADDRR3_SPEC>;
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
impl From<crate::W<FIFOADDRR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOADDRR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FI3EN` reader - FI3EN"]
pub type FI3EN_R = crate::BitReader<bool>;
#[doc = "Field `FI3EN` writer - FI3EN"]
pub type FI3EN_W<'a> = crate::BitWriter<'a, u32, FIFOADDRR3_SPEC, bool, 0>;
#[doc = "Field ` ADDR3` reader - ADDR3"]
pub type ADDR3_R = crate::FieldReader<u32, u32>;
#[doc = "Field ` ADDR3` writer - ADDR3"]
pub type ADDR3_W<'a> = crate::FieldWriter<'a, u32, FIFOADDRR3_SPEC, u32, u32, 31, 1>;
impl R {
    #[doc = "Bit 0 - FI3EN"]
    #[inline(always)]
    pub fn fi3en(&self) -> FI3EN_R {
        FI3EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - ADDR3"]
    #[inline(always)]
    pub fn addr3(&self) -> ADDR3_R {
        ADDR3_R::new(((self.bits >> 1) & 0x7fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 0 - FI3EN"]
    #[inline(always)]
    pub fn fi3en(&mut self) -> FI3EN_W {
        FI3EN_W::new(self)
    }
    #[doc = "Bits 1:31 - ADDR3"]
    #[inline(always)]
    pub fn addr3(&mut self) -> ADDR3_W {
        ADDR3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt 3 address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifoaddrr3](index.html) module"]
pub struct FIFOADDRR3_SPEC;
impl crate::RegisterSpec for FIFOADDRR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifoaddrr3::R](R) reader structure"]
impl crate::Readable for FIFOADDRR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifoaddrr3::W](W) writer structure"]
impl crate::Writable for FIFOADDRR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFOADDRR3 to value 0"]
impl crate::Resettable for FIFOADDRR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}