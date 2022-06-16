#[doc = "Register `UEP_BUF_MOD` reader"]
pub struct R(crate::R<UEP_BUF_MOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UEP_BUF_MOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UEP_BUF_MOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UEP_BUF_MOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UEP_BUF_MOD` writer"]
pub struct W(crate::W<UEP_BUF_MOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UEP_BUF_MOD_SPEC>;
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
impl From<crate::W<UEP_BUF_MOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UEP_BUF_MOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `bUEP_ISO_BUF_MOD` reader - buffer mode of USB endpoint"]
pub type BUEP_ISO_BUF_MOD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `bUEP_ISO_BUF_MOD` writer - buffer mode of USB endpoint"]
pub type BUEP_ISO_BUF_MOD_W<'a> =
    crate::FieldWriter<'a, u32, UEP_BUF_MOD_SPEC, u16, u16, 16, 16>;
#[doc = "Field `bUEP_BUF_MOD` reader - buffer mode of USB endpoint"]
pub type BUEP_BUF_MOD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `bUEP_BUF_MOD` writer - buffer mode of USB endpoint"]
pub type BUEP_BUF_MOD_W<'a> =
    crate::FieldWriter<'a, u32, UEP_BUF_MOD_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 16:31 - buffer mode of USB endpoint"]
    #[inline(always)]
    pub fn b_uep_iso_buf_mod(&self) -> BUEP_ISO_BUF_MOD_R {
        BUEP_ISO_BUF_MOD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - buffer mode of USB endpoint"]
    #[inline(always)]
    pub fn b_uep_buf_mod(&self) -> BUEP_BUF_MOD_R {
        BUEP_BUF_MOD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - buffer mode of USB endpoint"]
    #[inline(always)]
    pub fn b_uep_iso_buf_mod(&mut self) -> BUEP_ISO_BUF_MOD_W {
        BUEP_ISO_BUF_MOD_W::new(self)
    }
    #[doc = "Bits 0:15 - buffer mode of USB endpoint"]
    #[inline(always)]
    pub fn b_uep_buf_mod(&mut self) -> BUEP_BUF_MOD_W {
        BUEP_BUF_MOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB endpoint buffer mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uep_buf_mod](index.html) module"]
pub struct UEP_BUF_MOD_SPEC;
impl crate::RegisterSpec for UEP_BUF_MOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uep_buf_mod::R](R) reader structure"]
impl crate::Readable for UEP_BUF_MOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uep_buf_mod::W](W) writer structure"]
impl crate::Writable for UEP_BUF_MOD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UEP_BUF_MOD to value 0"]
impl crate::Resettable for UEP_BUF_MOD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}