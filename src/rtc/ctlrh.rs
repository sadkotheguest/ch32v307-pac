#[doc = "Register `CTLRH` reader"]
pub struct R(crate::R<CTLRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTLRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTLRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTLRH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTLRH` writer"]
pub struct W(crate::W<CTLRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTLRH_SPEC>;
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
impl From<crate::W<CTLRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTLRH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SECIE` reader - Second interrupt Enable"]
pub type SECIE_R = crate::BitReader<bool>;
#[doc = "Field `SECIE` writer - Second interrupt Enable"]
pub type SECIE_W<'a> = crate::BitWriter<'a, u32, CTLRH_SPEC, bool, 0>;
#[doc = "Field `ALRIE` reader - Alarm interrupt Enable"]
pub type ALRIE_R = crate::BitReader<bool>;
#[doc = "Field `ALRIE` writer - Alarm interrupt Enable"]
pub type ALRIE_W<'a> = crate::BitWriter<'a, u32, CTLRH_SPEC, bool, 1>;
#[doc = "Field `OWIE` reader - Overflow interrupt Enable"]
pub type OWIE_R = crate::BitReader<bool>;
#[doc = "Field `OWIE` writer - Overflow interrupt Enable"]
pub type OWIE_W<'a> = crate::BitWriter<'a, u32, CTLRH_SPEC, bool, 2>;
impl R {
    #[doc = "Bit 0 - Second interrupt Enable"]
    #[inline(always)]
    pub fn secie(&self) -> SECIE_R {
        SECIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm interrupt Enable"]
    #[inline(always)]
    pub fn alrie(&self) -> ALRIE_R {
        ALRIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Overflow interrupt Enable"]
    #[inline(always)]
    pub fn owie(&self) -> OWIE_R {
        OWIE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Second interrupt Enable"]
    #[inline(always)]
    pub fn secie(&mut self) -> SECIE_W {
        SECIE_W::new(self)
    }
    #[doc = "Bit 1 - Alarm interrupt Enable"]
    #[inline(always)]
    pub fn alrie(&mut self) -> ALRIE_W {
        ALRIE_W::new(self)
    }
    #[doc = "Bit 2 - Overflow interrupt Enable"]
    #[inline(always)]
    pub fn owie(&mut self) -> OWIE_W {
        OWIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Control Register High\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctlrh](index.html) module"]
pub struct CTLRH_SPEC;
impl crate::RegisterSpec for CTLRH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctlrh::R](R) reader structure"]
impl crate::Readable for CTLRH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctlrh::W](W) writer structure"]
impl crate::Writable for CTLRH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTLRH to value 0"]
impl crate::Resettable for CTLRH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}