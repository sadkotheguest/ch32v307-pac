#[doc = "Register `DATAR5` reader"]
pub struct R(crate::R<DATAR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATAR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATAR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATAR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATAR5` writer"]
pub struct W(crate::W<DATAR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATAR5_SPEC>;
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
impl From<crate::W<DATAR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATAR5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D5` reader - Backup data"]
pub type D5_R = crate::FieldReader<u16, u16>;
#[doc = "Field `D5` writer - Backup data"]
pub type D5_W<'a> = crate::FieldWriter<'a, u32, DATAR5_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d5(&self) -> D5_R {
        D5_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d5(&mut self) -> D5_W {
        D5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datar5](index.html) module"]
pub struct DATAR5_SPEC;
impl crate::RegisterSpec for DATAR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [datar5::R](R) reader structure"]
impl crate::Readable for DATAR5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [datar5::W](W) writer structure"]
impl crate::Writable for DATAR5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATAR5 to value 0"]
impl crate::Resettable for DATAR5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}