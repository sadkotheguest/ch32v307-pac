#[doc = "Register `IPRR1` reader"]
pub struct R(crate::R<IPRR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPRR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPRR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPRR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPRR1` writer"]
pub struct W(crate::W<IPRR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPRR1_SPEC>;
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
impl From<crate::W<IPRR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IPRR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PENDRESET` reader - PENDRESET"]
pub type PENDRESET_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PENDRESET` writer - PENDRESET"]
pub type PENDRESET_W<'a> = crate::FieldWriter<'a, u32, IPRR1_SPEC, u32, u32, 21, 12>;
impl R {
    #[doc = "Bits 12:32 - PENDRESET"]
    #[inline(always)]
    pub fn pendreset(&self) -> PENDRESET_R {
        PENDRESET_R::new(((self.bits >> 12) & 0x001f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 12:32 - PENDRESET"]
    #[inline(always)]
    pub fn pendreset(&mut self) -> PENDRESET_W {
        PENDRESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Pending Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iprr1](index.html) module"]
pub struct IPRR1_SPEC;
impl crate::RegisterSpec for IPRR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iprr1::R](R) reader structure"]
impl crate::Readable for IPRR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iprr1::W](W) writer structure"]
impl crate::Writable for IPRR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IPRR1 to value 0"]
impl crate::Resettable for IPRR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}