#[doc = "Register `DMACFGR` reader"]
pub struct R(crate::R<DMACFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACFGR` writer"]
pub struct W(crate::W<DMACFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACFGR_SPEC>;
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
impl From<crate::W<DMACFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBL` reader - DMA burst length"]
pub type DBL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBL` writer - DMA burst length"]
pub type DBL_W<'a> = crate::FieldWriter<'a, u32, DMACFGR_SPEC, u8, u8, 5, 8>;
#[doc = "Field `DBA` reader - DMA base address"]
pub type DBA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBA` writer - DMA base address"]
pub type DBA_W<'a> = crate::FieldWriter<'a, u32, DMACFGR_SPEC, u8, u8, 5, 0>;
impl R {
    #[doc = "Bits 8:12 - DMA burst length"]
    #[inline(always)]
    pub fn dbl(&self) -> DBL_R {
        DBL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - DMA base address"]
    #[inline(always)]
    pub fn dba(&self) -> DBA_R {
        DBA_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:12 - DMA burst length"]
    #[inline(always)]
    pub fn dbl(&mut self) -> DBL_W {
        DBL_W::new(self)
    }
    #[doc = "Bits 0:4 - DMA base address"]
    #[inline(always)]
    pub fn dba(&mut self) -> DBA_W {
        DBA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacfgr](index.html) module"]
pub struct DMACFGR_SPEC;
impl crate::RegisterSpec for DMACFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmacfgr::R](R) reader structure"]
impl crate::Readable for DMACFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmacfgr::W](W) writer structure"]
impl crate::Writable for DMACFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMACFGR to value 0"]
impl crate::Resettable for DMACFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}