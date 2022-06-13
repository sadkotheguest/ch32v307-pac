#[doc = "Register `DATAR` reader"]
pub struct R(crate::R<DATAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATAR` writer"]
pub struct W(crate::W<DATAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATAR_SPEC>;
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
impl From<crate::W<DATAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DR` reader - Data Register"]
pub type DR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DR` writer - Data Register"]
pub type DR_W<'a> = crate::FieldWriter<'a, u32, DATAR_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Data Register"]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data Register"]
    #[inline(always)]
    pub fn dr(&mut self) -> DR_W {
        DR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datar](index.html) module"]
pub struct DATAR_SPEC;
impl crate::RegisterSpec for DATAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [datar::R](R) reader structure"]
impl crate::Readable for DATAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [datar::W](W) writer structure"]
impl crate::Writable for DATAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATAR to value 0xffff_ffff"]
impl crate::Resettable for DATAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}