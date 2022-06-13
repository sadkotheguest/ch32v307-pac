#[doc = "Register `IPRIOR45` reader"]
pub struct R(crate::R<IPRIOR45_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPRIOR45_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPRIOR45_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPRIOR45_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPRIOR45` writer"]
pub struct W(crate::W<IPRIOR45_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPRIOR45_SPEC>;
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
impl From<crate::W<IPRIOR45_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IPRIOR45_SPEC>) -> Self {
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
#[doc = "Interrupt Priority Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iprior45](index.html) module"]
pub struct IPRIOR45_SPEC;
impl crate::RegisterSpec for IPRIOR45_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [iprior45::R](R) reader structure"]
impl crate::Readable for IPRIOR45_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iprior45::W](W) writer structure"]
impl crate::Writable for IPRIOR45_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IPRIOR45 to value 0"]
impl crate::Resettable for IPRIOR45_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}