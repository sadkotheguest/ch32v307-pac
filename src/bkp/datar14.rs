#[doc = "Register `DATAR14` reader"]
pub struct R(crate::R<DATAR14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATAR14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATAR14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATAR14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATAR14` writer"]
pub struct W(crate::W<DATAR14_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATAR14_SPEC>;
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
impl From<crate::W<DATAR14_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATAR14_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D14` reader - Backup data"]
pub type D14_R = crate::FieldReader<u16, u16>;
#[doc = "Field `D14` writer - Backup data"]
pub type D14_W<'a> = crate::FieldWriter<'a, u32, DATAR14_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d14(&self) -> D14_R {
        D14_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d14(&mut self) -> D14_W {
        D14_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datar14](index.html) module"]
pub struct DATAR14_SPEC;
impl crate::RegisterSpec for DATAR14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [datar14::R](R) reader structure"]
impl crate::Readable for DATAR14_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [datar14::W](W) writer structure"]
impl crate::Writable for DATAR14_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATAR14 to value 0"]
impl crate::Resettable for DATAR14_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}