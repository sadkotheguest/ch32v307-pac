#[doc = "Register `DATAR27` reader"]
pub struct R(crate::R<DATAR27_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATAR27_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATAR27_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATAR27_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATAR27` writer"]
pub struct W(crate::W<DATAR27_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATAR27_SPEC>;
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
impl From<crate::W<DATAR27_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATAR27_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D27` reader - Backup data"]
pub type D27_R = crate::FieldReader<u16, u16>;
#[doc = "Field `D27` writer - Backup data"]
pub type D27_W<'a> = crate::FieldWriter<'a, u32, DATAR27_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d27(&self) -> D27_R {
        D27_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d27(&mut self) -> D27_W {
        D27_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datar27](index.html) module"]
pub struct DATAR27_SPEC;
impl crate::RegisterSpec for DATAR27_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [datar27::R](R) reader structure"]
impl crate::Readable for DATAR27_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [datar27::W](W) writer structure"]
impl crate::Writable for DATAR27_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATAR27 to value 0"]
impl crate::Resettable for DATAR27_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}