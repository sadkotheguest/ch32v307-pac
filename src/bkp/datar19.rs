#[doc = "Register `DATAR19` reader"]
pub struct R(crate::R<DATAR19_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATAR19_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATAR19_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATAR19_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATAR19` writer"]
pub struct W(crate::W<DATAR19_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATAR19_SPEC>;
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
impl From<crate::W<DATAR19_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATAR19_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D19` reader - Backup data"]
pub type D19_R = crate::FieldReader<u16, u16>;
#[doc = "Field `D19` writer - Backup data"]
pub type D19_W<'a> = crate::FieldWriter<'a, u32, DATAR19_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d19(&self) -> D19_R {
        D19_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d19(&mut self) -> D19_W {
        D19_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datar19](index.html) module"]
pub struct DATAR19_SPEC;
impl crate::RegisterSpec for DATAR19_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [datar19::R](R) reader structure"]
impl crate::Readable for DATAR19_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [datar19::W](W) writer structure"]
impl crate::Writable for DATAR19_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATAR19 to value 0"]
impl crate::Resettable for DATAR19_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}