#[doc = "Register `ERRSR` reader"]
pub struct R(crate::R<ERRSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERRSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERRSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERRSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERRSR` writer"]
pub struct W(crate::W<ERRSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERRSR_SPEC>;
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
impl From<crate::W<ERRSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERRSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REC` reader - REC"]
pub type REC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TEC` reader - TEC"]
pub type TEC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LEC` reader - LEC"]
pub type LEC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LEC` writer - LEC"]
pub type LEC_W<'a> = crate::FieldWriter<'a, u32, ERRSR_SPEC, u8, u8, 3, 4>;
#[doc = "Field `BOFF` reader - BOFF"]
pub type BOFF_R = crate::BitReader<bool>;
#[doc = "Field `EPVF` reader - EPVF"]
pub type EPVF_R = crate::BitReader<bool>;
#[doc = "Field `EWGF` reader - EWGF"]
pub type EWGF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 24:31 - REC"]
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - TEC"]
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 4:6 - LEC"]
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 2 - BOFF"]
    #[inline(always)]
    pub fn boff(&self) -> BOFF_R {
        BOFF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - EPVF"]
    #[inline(always)]
    pub fn epvf(&self) -> EPVF_R {
        EPVF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - EWGF"]
    #[inline(always)]
    pub fn ewgf(&self) -> EWGF_R {
        EWGF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:6 - LEC"]
    #[inline(always)]
    pub fn lec(&mut self) -> LEC_W {
        LEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ERRSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [errsr](index.html) module"]
pub struct ERRSR_SPEC;
impl crate::RegisterSpec for ERRSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [errsr::R](R) reader structure"]
impl crate::Readable for ERRSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [errsr::W](W) writer structure"]
impl crate::Writable for ERRSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ERRSR to value 0"]
impl crate::Resettable for ERRSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}