#[doc = "Register `CRCR` reader"]
pub struct R(crate::R<CRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRCR` writer"]
pub struct W(crate::W<CRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRCR_SPEC>;
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
impl From<crate::W<CRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRCPOLY` reader - CRC polynomial register"]
pub type CRCPOLY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CRCPOLY` writer - CRC polynomial register"]
pub type CRCPOLY_W<'a> = crate::FieldWriter<'a, u32, CRCR_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - CRC polynomial register"]
    #[inline(always)]
    pub fn crcpoly(&self) -> CRCPOLY_R {
        CRCPOLY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC polynomial register"]
    #[inline(always)]
    pub fn crcpoly(&mut self) -> CRCPOLY_W {
        CRCPOLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRCR polynomial register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcr](index.html) module"]
pub struct CRCR_SPEC;
impl crate::RegisterSpec for CRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crcr::R](R) reader structure"]
impl crate::Readable for CRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crcr::W](W) writer structure"]
impl crate::Writable for CRCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRCR to value 0x07"]
impl crate::Resettable for CRCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07
    }
}