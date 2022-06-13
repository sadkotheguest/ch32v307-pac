#[doc = "Register `IPSR1` reader"]
pub struct R(crate::R<IPSR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPSR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPSR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPSR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPSR1` writer"]
pub struct W(crate::W<IPSR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPSR1_SPEC>;
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
impl From<crate::W<IPSR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IPSR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PENDSET` reader - PENDSET"]
pub type PENDSET_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PENDSET` writer - PENDSET"]
pub type PENDSET_W<'a> = crate::FieldWriter<'a, u32, IPSR1_SPEC, u32, u32, 21, 12>;
impl R {
    #[doc = "Bits 12:32 - PENDSET"]
    #[inline(always)]
    pub fn pendset(&self) -> PENDSET_R {
        PENDSET_R::new(((self.bits >> 12) & 0x001f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 12:32 - PENDSET"]
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
#[doc = "Interrupt Pending Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipsr1](index.html) module"]
pub struct IPSR1_SPEC;
impl crate::RegisterSpec for IPSR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipsr1::R](R) reader structure"]
impl crate::Readable for IPSR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ipsr1::W](W) writer structure"]
impl crate::Writable for IPSR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IPSR1 to value 0"]
impl crate::Resettable for IPSR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}