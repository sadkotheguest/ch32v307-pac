#[doc = "Register `IPSR2` reader"]
pub struct R(crate::R<IPSR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPSR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPSR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPSR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPSR2` writer"]
pub struct W(crate::W<IPSR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPSR2_SPEC>;
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
impl From<crate::W<IPSR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IPSR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PENDSET` reader - PENDSET"]
pub type PENDSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PENDSET` writer - PENDSET"]
pub type PENDSET_W<'a> = crate::FieldWriter<'a, u32, IPSR2_SPEC, u8, u8, 3, 0>;
impl R {
    #[doc = "Bits 0:2 - PENDSET"]
    #[inline(always)]
    pub fn pendset(&self) -> PENDSET_R {
        PENDSET_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - PENDSET"]
    #[inline(always)]
    pub fn pendset(&mut self) -> PENDSET_W {
        PENDSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Pending Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipsr2](index.html) module"]
pub struct IPSR2_SPEC;
impl crate::RegisterSpec for IPSR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipsr2::R](R) reader structure"]
impl crate::Readable for IPSR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ipsr2::W](W) writer structure"]
impl crate::Writable for IPSR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IPSR2 to value 0"]
impl crate::Resettable for IPSR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}