#[doc = "Register `FIBADDRR` reader"]
pub struct R(crate::R<FIBADDRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIBADDRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIBADDRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIBADDRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIBADDRR` writer"]
pub struct W(crate::W<FIBADDRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIBADDRR_SPEC>;
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
impl From<crate::W<FIBADDRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIBADDRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BASEADDR` reader - BASEADDR"]
pub type BASEADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BASEADDR` writer - BASEADDR"]
pub type BASEADDR_W<'a> = crate::FieldWriter<'a, u32, FIBADDRR_SPEC, u8, u8, 4, 28>;
impl R {
    #[doc = "Bits 28:31 - BASEADDR"]
    #[inline(always)]
    pub fn baseaddr(&self) -> BASEADDR_R {
        BASEADDR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - BASEADDR"]
    #[inline(always)]
    pub fn baseaddr(&mut self) -> BASEADDR_W {
        BASEADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Fast Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fibaddrr](index.html) module"]
pub struct FIBADDRR_SPEC;
impl crate::RegisterSpec for FIBADDRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fibaddrr::R](R) reader structure"]
impl crate::Readable for FIBADDRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fibaddrr::W](W) writer structure"]
impl crate::Writable for FIBADDRR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIBADDRR to value 0"]
impl crate::Resettable for FIBADDRR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}