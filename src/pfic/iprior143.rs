#[doc = "Register `IPRIOR143` reader"]
pub struct R(crate::R<IPRIOR143_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPRIOR143_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPRIOR143_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPRIOR143_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPRIOR143` writer"]
pub struct W(crate::W<IPRIOR143_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPRIOR143_SPEC>;
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
impl From<crate::W<IPRIOR143_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IPRIOR143_SPEC>) -> Self {
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
#[doc = "Interrupt Priority Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iprior143](index.html) module"]
pub struct IPRIOR143_SPEC;
impl crate::RegisterSpec for IPRIOR143_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [iprior143::R](R) reader structure"]
impl crate::Readable for IPRIOR143_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iprior143::W](W) writer structure"]
impl crate::Writable for IPRIOR143_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IPRIOR143 to value 0"]
impl crate::Resettable for IPRIOR143_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}