#[doc = "Register `IENR1` reader"]
pub struct R(crate::R<IENR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IENR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IENR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IENR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IENR1` writer"]
pub struct W(crate::W<IENR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IENR1_SPEC>;
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
impl From<crate::W<IENR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IENR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTEN` reader - INTEN"]
pub type INTEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INTEN` writer - INTEN"]
pub type INTEN_W<'a> = crate::FieldWriter<'a, u32, IENR1_SPEC, u8, u8, 3, 0>;
impl R {
    #[doc = "Bits 0:2 - INTEN"]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - INTEN"]
    #[inline(always)]
    pub fn inten(&mut self) -> INTEN_W {
        INTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ienr1](index.html) module"]
pub struct IENR1_SPEC;
impl crate::RegisterSpec for IENR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ienr1::R](R) reader structure"]
impl crate::Readable for IENR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ienr1::W](W) writer structure"]
impl crate::Writable for IENR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IENR1 to value 0"]
impl crate::Resettable for IENR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}