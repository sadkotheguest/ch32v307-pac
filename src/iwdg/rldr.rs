#[doc = "Register `RLDR` reader"]
pub struct R(crate::R<RLDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RLDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RLDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RLDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RLDR` writer"]
pub struct W(crate::W<RLDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RLDR_SPEC>;
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
impl From<crate::W<RLDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RLDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RL` reader - Watchdog counter reload value"]
pub type RL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RL` writer - Watchdog counter reload value"]
pub type RL_W<'a> = crate::FieldWriter<'a, u32, RLDR_SPEC, u16, u16, 12, 0>;
impl R {
    #[doc = "Bits 0:11 - Watchdog counter reload value"]
    #[inline(always)]
    pub fn rl(&self) -> RL_R {
        RL_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Watchdog counter reload value"]
    #[inline(always)]
    pub fn rl(&mut self) -> RL_W {
        RL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reload register (IWDG_RLDR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rldr](index.html) module"]
pub struct RLDR_SPEC;
impl crate::RegisterSpec for RLDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rldr::R](R) reader structure"]
impl crate::Readable for RLDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rldr::W](W) writer structure"]
impl crate::Writable for RLDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RLDR to value 0x0fff"]
impl crate::Resettable for RLDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0fff
    }
}