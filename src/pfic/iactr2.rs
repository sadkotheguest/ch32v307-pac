#[doc = "Register `IACTR2` reader"]
pub struct R(crate::R<IACTR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IACTR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IACTR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IACTR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IACTR2` writer"]
pub struct W(crate::W<IACTR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IACTR2_SPEC>;
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
impl From<crate::W<IACTR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IACTR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IACTS` reader - IACTS"]
pub type IACTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IACTS` writer - IACTS"]
pub type IACTS_W<'a> = crate::FieldWriter<'a, u32, IACTR2_SPEC, u8, u8, 3, 0>;
impl R {
    #[doc = "Bits 0:2 - IACTS"]
    #[inline(always)]
    pub fn iacts(&self) -> IACTS_R {
        IACTS_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - IACTS"]
    #[inline(always)]
    pub fn iacts(&mut self) -> IACTS_W {
        IACTS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt ACTIVE Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iactr2](index.html) module"]
pub struct IACTR2_SPEC;
impl crate::RegisterSpec for IACTR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iactr2::R](R) reader structure"]
impl crate::Readable for IACTR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iactr2::W](W) writer structure"]
impl crate::Writable for IACTR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IACTR2 to value 0"]
impl crate::Resettable for IACTR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}