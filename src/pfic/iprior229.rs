#[doc = "Register `IPRIOR229` reader"]
pub struct R(crate::R<IPRIOR229_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPRIOR229_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPRIOR229_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPRIOR229_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPRIOR229` writer"]
pub struct W(crate::W<IPRIOR229_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPRIOR229_SPEC>;
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
impl From<crate::W<IPRIOR229_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IPRIOR229_SPEC>) -> Self {
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
#[doc = "Interrupt Priority Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iprior229](index.html) module"]
pub struct IPRIOR229_SPEC;
impl crate::RegisterSpec for IPRIOR229_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [iprior229::R](R) reader structure"]
impl crate::Readable for IPRIOR229_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iprior229::W](W) writer structure"]
impl crate::Writable for IPRIOR229_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IPRIOR229 to value 0"]
impl crate::Resettable for IPRIOR229_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}