#[doc = "Register `STK_SR` reader"]
pub struct R(crate::R<STK_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STK_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STK_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STK_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STK_SR` writer"]
pub struct W(crate::W<STK_SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STK_SR_SPEC>;
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
impl From<crate::W<STK_SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STK_SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNTIF` reader - CNTIF"]
pub type CNTIF_R = crate::BitReader<bool>;
#[doc = "Field `CNTIF` writer - CNTIF"]
pub type CNTIF_W<'a> = crate::BitWriter<'a, u32, STK_SR_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - CNTIF"]
    #[inline(always)]
    pub fn cntif(&self) -> CNTIF_R {
        CNTIF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CNTIF"]
    #[inline(always)]
    pub fn cntif(&mut self) -> CNTIF_W {
        CNTIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System START\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stk_sr](index.html) module"]
pub struct STK_SR_SPEC;
impl crate::RegisterSpec for STK_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stk_sr::R](R) reader structure"]
impl crate::Readable for STK_SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stk_sr::W](W) writer structure"]
impl crate::Writable for STK_SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STK_SR to value 0"]
impl crate::Resettable for STK_SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}