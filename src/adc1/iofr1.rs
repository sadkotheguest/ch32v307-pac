#[doc = "Register `IOFR1` reader"]
pub struct R(crate::R<IOFR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOFR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOFR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOFR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOFR1` writer"]
pub struct W(crate::W<IOFR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOFR1_SPEC>;
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
impl From<crate::W<IOFR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOFR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IOFFSET1` reader - Data offset for injected channel x"]
pub type IOFFSET1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `IOFFSET1` writer - Data offset for injected channel x"]
pub type IOFFSET1_W<'a> = crate::FieldWriter<'a, u32, IOFR1_SPEC, u16, u16, 12, 0>;
impl R {
    #[doc = "Bits 0:11 - Data offset for injected channel x"]
    #[inline(always)]
    pub fn ioffset1(&self) -> IOFFSET1_R {
        IOFFSET1_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Data offset for injected channel x"]
    #[inline(always)]
    pub fn ioffset1(&mut self) -> IOFFSET1_W {
        IOFFSET1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "injected channel data offset register x\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iofr1](index.html) module"]
pub struct IOFR1_SPEC;
impl crate::RegisterSpec for IOFR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iofr1::R](R) reader structure"]
impl crate::Readable for IOFR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iofr1::W](W) writer structure"]
impl crate::Writable for IOFR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IOFR1 to value 0"]
impl crate::Resettable for IOFR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}