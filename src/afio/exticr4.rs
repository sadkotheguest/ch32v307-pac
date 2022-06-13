#[doc = "Register `EXTICR4` reader"]
pub struct R(crate::R<EXTICR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTICR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTICR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTICR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTICR4` writer"]
pub struct W(crate::W<EXTICR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTICR4_SPEC>;
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
impl From<crate::W<EXTICR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTICR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTI12` reader - EXTI12 configuration"]
pub type EXTI12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI12` writer - EXTI12 configuration"]
pub type EXTI12_W<'a> = crate::FieldWriter<'a, u32, EXTICR4_SPEC, u8, u8, 4, 0>;
#[doc = "Field `EXTI13` reader - EXTI13 configuration"]
pub type EXTI13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI13` writer - EXTI13 configuration"]
pub type EXTI13_W<'a> = crate::FieldWriter<'a, u32, EXTICR4_SPEC, u8, u8, 4, 4>;
#[doc = "Field `EXTI14` reader - EXTI14 configuration"]
pub type EXTI14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI14` writer - EXTI14 configuration"]
pub type EXTI14_W<'a> = crate::FieldWriter<'a, u32, EXTICR4_SPEC, u8, u8, 4, 8>;
#[doc = "Field `EXTI15` reader - EXTI15 configuration"]
pub type EXTI15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI15` writer - EXTI15 configuration"]
pub type EXTI15_W<'a> = crate::FieldWriter<'a, u32, EXTICR4_SPEC, u8, u8, 4, 12>;
impl R {
    #[doc = "Bits 0:3 - EXTI12 configuration"]
    #[inline(always)]
    pub fn exti12(&self) -> EXTI12_R {
        EXTI12_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI13 configuration"]
    #[inline(always)]
    pub fn exti13(&self) -> EXTI13_R {
        EXTI13_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI14 configuration"]
    #[inline(always)]
    pub fn exti14(&self) -> EXTI14_R {
        EXTI14_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTI15 configuration"]
    #[inline(always)]
    pub fn exti15(&self) -> EXTI15_R {
        EXTI15_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTI12 configuration"]
    #[inline(always)]
    pub fn exti12(&mut self) -> EXTI12_W {
        EXTI12_W::new(self)
    }
    #[doc = "Bits 4:7 - EXTI13 configuration"]
    #[inline(always)]
    pub fn exti13(&mut self) -> EXTI13_W {
        EXTI13_W::new(self)
    }
    #[doc = "Bits 8:11 - EXTI14 configuration"]
    #[inline(always)]
    pub fn exti14(&mut self) -> EXTI14_W {
        EXTI14_W::new(self)
    }
    #[doc = "Bits 12:15 - EXTI15 configuration"]
    #[inline(always)]
    pub fn exti15(&mut self) -> EXTI15_W {
        EXTI15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External interrupt configuration register 4 (AFIO_EXTICR4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr4](index.html) module"]
pub struct EXTICR4_SPEC;
impl crate::RegisterSpec for EXTICR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exticr4::R](R) reader structure"]
impl crate::Readable for EXTICR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exticr4::W](W) writer structure"]
impl crate::Writable for EXTICR4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTICR4 to value 0"]
impl crate::Resettable for EXTICR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}