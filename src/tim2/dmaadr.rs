#[doc = "Register `DMAADR` reader"]
pub struct R(crate::R<DMAADR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAADR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAADR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAADR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAADR` writer"]
pub struct W(crate::W<DMAADR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAADR_SPEC>;
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
impl From<crate::W<DMAADR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAADR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAADR` reader - DMA register for burst accesses"]
pub type DMAADR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DMAADR` writer - DMA register for burst accesses"]
pub type DMAADR_W<'a> = crate::FieldWriter<'a, u32, DMAADR_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - DMA register for burst accesses"]
    #[inline(always)]
    pub fn dmaadr(&self) -> DMAADR_R {
        DMAADR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DMA register for burst accesses"]
    #[inline(always)]
    pub fn dmaadr(&mut self) -> DMAADR_W {
        DMAADR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA address for full transfer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaadr](index.html) module"]
pub struct DMAADR_SPEC;
impl crate::RegisterSpec for DMAADR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmaadr::R](R) reader structure"]
impl crate::Readable for DMAADR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmaadr::W](W) writer structure"]
impl crate::Writable for DMAADR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAADR to value 0"]
impl crate::Resettable for DMAADR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}