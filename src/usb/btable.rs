#[doc = "Register `BTABLE` reader"]
pub struct R(crate::R<BTABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BTABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BTABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BTABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BTABLE` writer"]
pub struct W(crate::W<BTABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BTABLE_SPEC>;
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
impl From<crate::W<BTABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BTABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BTABLE` reader - Buffer table"]
pub type BTABLE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BTABLE` writer - Buffer table"]
pub type BTABLE_W<'a> = crate::FieldWriter<'a, u32, BTABLE_SPEC, u16, u16, 13, 3>;
impl R {
    #[doc = "Bits 3:15 - Buffer table"]
    #[inline(always)]
    pub fn btable(&self) -> BTABLE_R {
        BTABLE_R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 3:15 - Buffer table"]
    #[inline(always)]
    pub fn btable(&mut self) -> BTABLE_W {
        BTABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Buffer table address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [btable](index.html) module"]
pub struct BTABLE_SPEC;
impl crate::RegisterSpec for BTABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [btable::R](R) reader structure"]
impl crate::Readable for BTABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [btable::W](W) writer structure"]
impl crate::Writable for BTABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BTABLE to value 0"]
impl crate::Resettable for BTABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}