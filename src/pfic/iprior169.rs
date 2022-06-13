#[doc = "Register `IPRIOR169` reader"]
pub struct R(crate::R<IPRIOR169_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPRIOR169_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPRIOR169_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPRIOR169_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPRIOR169` writer"]
pub struct W(crate::W<IPRIOR169_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPRIOR169_SPEC>;
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
impl From<crate::W<IPRIOR169_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IPRIOR169_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iprior169](index.html) module"]
pub struct IPRIOR169_SPEC;
impl crate::RegisterSpec for IPRIOR169_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [iprior169::R](R) reader structure"]
impl crate::Readable for IPRIOR169_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iprior169::W](W) writer structure"]
impl crate::Writable for IPRIOR169_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IPRIOR169 to value 0"]
impl crate::Resettable for IPRIOR169_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}