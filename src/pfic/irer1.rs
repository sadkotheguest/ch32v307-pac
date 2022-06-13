#[doc = "Register `IRER1` reader"]
pub struct R(crate::R<IRER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRER1` writer"]
pub struct W(crate::W<IRER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRER1_SPEC>;
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
impl From<crate::W<IRER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTRSET` reader - INTRSET"]
pub type INTRSET_R = crate::FieldReader<u32, u32>;
#[doc = "Field `INTRSET` writer - INTRSET"]
pub type INTRSET_W<'a> = crate::FieldWriter<'a, u32, IRER1_SPEC, u32, u32, 21, 12>;
impl R {
    #[doc = "Bits 12:32 - INTRSET"]
    #[inline(always)]
    pub fn intrset(&self) -> INTRSET_R {
        INTRSET_R::new(((self.bits >> 12) & 0x001f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 12:32 - INTRSET"]
    #[inline(always)]
    pub fn intrset(&mut self) -> INTRSET_W {
        INTRSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irer1](index.html) module"]
pub struct IRER1_SPEC;
impl crate::RegisterSpec for IRER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irer1::R](R) reader structure"]
impl crate::Readable for IRER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irer1::W](W) writer structure"]
impl crate::Writable for IRER1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IRER1 to value 0"]
impl crate::Resettable for IRER1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}