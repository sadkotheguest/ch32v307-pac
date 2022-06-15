#[doc = "Register `GPR` reader"]
pub struct R(crate::R<GPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPR` writer"]
pub struct W(crate::W<GPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPR_SPEC>;
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
impl From<crate::W<GPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GT` reader - Guard time value"]
pub type GT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GT` writer - Guard time value"]
pub type GT_W<'a> = crate::FieldWriter<'a, u32, GPR_SPEC, u8, u8, 8, 8>;
#[doc = "Field `PSC` reader - Prescaler value"]
pub type PSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PSC` writer - Prescaler value"]
pub type PSC_W<'a> = crate::FieldWriter<'a, u32, GPR_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 8:15 - Guard time value"]
    #[inline(always)]
    pub fn gt(&self) -> GT_R {
        GT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Prescaler value"]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Guard time value"]
    #[inline(always)]
    pub fn gt(&mut self) -> GT_W {
        GT_W::new(self)
    }
    #[doc = "Bits 0:7 - Prescaler value"]
    #[inline(always)]
    pub fn psc(&mut self) -> PSC_W {
        PSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Guard time and prescaler register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr](index.html) module"]
pub struct GPR_SPEC;
impl crate::RegisterSpec for GPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpr::R](R) reader structure"]
impl crate::Readable for GPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpr::W](W) writer structure"]
impl crate::Writable for GPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPR to value 0"]
impl crate::Resettable for GPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}