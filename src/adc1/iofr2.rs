#[doc = "Register `IOFR2` reader"]
pub struct R(crate::R<IOFR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOFR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOFR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOFR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOFR2` writer"]
pub struct W(crate::W<IOFR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOFR2_SPEC>;
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
impl From<crate::W<IOFR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOFR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IOFFSET2` reader - Data offset for injected channel x"]
pub type IOFFSET2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `IOFFSET2` writer - Data offset for injected channel x"]
pub type IOFFSET2_W<'a> = crate::FieldWriter<'a, u32, IOFR2_SPEC, u16, u16, 12, 0>;
impl R {
    #[doc = "Bits 0:11 - Data offset for injected channel x"]
    #[inline(always)]
    pub fn ioffset2(&self) -> IOFFSET2_R {
        IOFFSET2_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Data offset for injected channel x"]
    #[inline(always)]
    pub fn ioffset2(&mut self) -> IOFFSET2_W {
        IOFFSET2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "injected channel data offset register x\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iofr2](index.html) module"]
pub struct IOFR2_SPEC;
impl crate::RegisterSpec for IOFR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iofr2::R](R) reader structure"]
impl crate::Readable for IOFR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iofr2::W](W) writer structure"]
impl crate::Writable for IOFR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IOFR2 to value 0"]
impl crate::Resettable for IOFR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}