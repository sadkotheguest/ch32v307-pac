#[doc = "Register `DATAR25` reader"]
pub struct R(crate::R<DATAR25_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATAR25_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATAR25_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATAR25_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATAR25` writer"]
pub struct W(crate::W<DATAR25_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATAR25_SPEC>;
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
impl From<crate::W<DATAR25_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATAR25_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D25` reader - Backup data"]
pub type D25_R = crate::FieldReader<u16, u16>;
#[doc = "Field `D25` writer - Backup data"]
pub type D25_W<'a> = crate::FieldWriter<'a, u32, DATAR25_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d25(&self) -> D25_R {
        D25_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d25(&mut self) -> D25_W {
        D25_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datar25](index.html) module"]
pub struct DATAR25_SPEC;
impl crate::RegisterSpec for DATAR25_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [datar25::R](R) reader structure"]
impl crate::Readable for DATAR25_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [datar25::W](W) writer structure"]
impl crate::Writable for DATAR25_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATAR25 to value 0"]
impl crate::Resettable for DATAR25_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}