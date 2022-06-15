#[doc = "Register `IOFR4` reader"]
pub struct R(crate::R<IOFR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOFR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOFR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOFR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOFR4` writer"]
pub struct W(crate::W<IOFR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOFR4_SPEC>;
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
impl From<crate::W<IOFR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOFR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IOFFSET4` reader - Data offset for injected channel x"]
pub type IOFFSET4_R = crate::FieldReader<u16, u16>;
#[doc = "Field `IOFFSET4` writer - Data offset for injected channel x"]
pub type IOFFSET4_W<'a> = crate::FieldWriter<'a, u32, IOFR4_SPEC, u16, u16, 12, 0>;
impl R {
    #[doc = "Bits 0:11 - Data offset for injected channel x"]
    #[inline(always)]
    pub fn ioffset4(&self) -> IOFFSET4_R {
        IOFFSET4_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Data offset for injected channel x"]
    #[inline(always)]
    pub fn ioffset4(&mut self) -> IOFFSET4_W {
        IOFFSET4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "injected channel data offset register x\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iofr4](index.html) module"]
pub struct IOFR4_SPEC;
impl crate::RegisterSpec for IOFR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iofr4::R](R) reader structure"]
impl crate::Readable for IOFR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iofr4::W](W) writer structure"]
impl crate::Writable for IOFR4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IOFR4 to value 0"]
impl crate::Resettable for IOFR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}