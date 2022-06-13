#[doc = "Register `IPRIOR199` reader"]
pub struct R(crate::R<IPRIOR199_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPRIOR199_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPRIOR199_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPRIOR199_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPRIOR199` writer"]
pub struct W(crate::W<IPRIOR199_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPRIOR199_SPEC>;
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
impl From<crate::W<IPRIOR199_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IPRIOR199_SPEC>) -> Self {
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
#[doc = "Interrupt Priority Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iprior199](index.html) module"]
pub struct IPRIOR199_SPEC;
impl crate::RegisterSpec for IPRIOR199_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [iprior199::R](R) reader structure"]
impl crate::Readable for IPRIOR199_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iprior199::W](W) writer structure"]
impl crate::Writable for IPRIOR199_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IPRIOR199 to value 0"]
impl crate::Resettable for IPRIOR199_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}