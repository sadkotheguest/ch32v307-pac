#[doc = "Register `CNTR11` reader"]
pub struct R(crate::R<CNTR11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNTR11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNTR11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNTR11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNTR11` writer"]
pub struct W(crate::W<CNTR11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNTR11_SPEC>;
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
impl From<crate::W<CNTR11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNTR11_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NDT` reader - Number of data to transfer"]
pub type NDT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NDT` writer - Number of data to transfer"]
pub type NDT_W<'a> = crate::FieldWriter<'a, u32, CNTR11_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Number of data to transfer"]
    #[inline(always)]
    pub fn ndt(&self) -> NDT_R {
        NDT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of data to transfer"]
    #[inline(always)]
    pub fn ndt(&mut self) -> NDT_W {
        NDT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA channel 11 number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntr11](index.html) module"]
pub struct CNTR11_SPEC;
impl crate::RegisterSpec for CNTR11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cntr11::R](R) reader structure"]
impl crate::Readable for CNTR11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cntr11::W](W) writer structure"]
impl crate::Writable for CNTR11_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNTR11 to value 0"]
impl crate::Resettable for CNTR11_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}