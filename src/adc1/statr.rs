#[doc = "Register `STATR` reader"]
pub struct R(crate::R<STATR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATR` writer"]
pub struct W(crate::W<STATR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATR_SPEC>;
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
impl From<crate::W<STATR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSTRT` reader - Regular channel start flag"]
pub type RSTRT_R = crate::BitReader<bool>;
#[doc = "Field `RSTRT` writer - Regular channel start flag"]
pub type RSTRT_W<'a> = crate::BitWriter<'a, u32, STATR_SPEC, bool, 4>;
#[doc = "Field `ISTRT` reader - Injected channel start flag"]
pub type ISTRT_R = crate::BitReader<bool>;
#[doc = "Field `ISTRT` writer - Injected channel start flag"]
pub type ISTRT_W<'a> = crate::BitWriter<'a, u32, STATR_SPEC, bool, 3>;
#[doc = "Field `IEOC` reader - Injected channel end of conversion"]
pub type IEOC_R = crate::BitReader<bool>;
#[doc = "Field `IEOC` writer - Injected channel end of conversion"]
pub type IEOC_W<'a> = crate::BitWriter<'a, u32, STATR_SPEC, bool, 2>;
#[doc = "Field `EOC` reader - Regular channel end of conversion"]
pub type EOC_R = crate::BitReader<bool>;
#[doc = "Field `EOC` writer - Regular channel end of conversion"]
pub type EOC_W<'a> = crate::BitWriter<'a, u32, STATR_SPEC, bool, 1>;
#[doc = "Field `AWD` reader - Analog watchdog flag"]
pub type AWD_R = crate::BitReader<bool>;
#[doc = "Field `AWD` writer - Analog watchdog flag"]
pub type AWD_W<'a> = crate::BitWriter<'a, u32, STATR_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 4 - Regular channel start flag"]
    #[inline(always)]
    pub fn rstrt(&self) -> RSTRT_R {
        RSTRT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Injected channel start flag"]
    #[inline(always)]
    pub fn istrt(&self) -> ISTRT_R {
        ISTRT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Injected channel end of conversion"]
    #[inline(always)]
    pub fn ieoc(&self) -> IEOC_R {
        IEOC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Regular channel end of conversion"]
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Analog watchdog flag"]
    #[inline(always)]
    pub fn awd(&self) -> AWD_R {
        AWD_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Regular channel start flag"]
    #[inline(always)]
    pub fn rstrt(&mut self) -> RSTRT_W {
        RSTRT_W::new(self)
    }
    #[doc = "Bit 3 - Injected channel start flag"]
    #[inline(always)]
    pub fn istrt(&mut self) -> ISTRT_W {
        ISTRT_W::new(self)
    }
    #[doc = "Bit 2 - Injected channel end of conversion"]
    #[inline(always)]
    pub fn ieoc(&mut self) -> IEOC_W {
        IEOC_W::new(self)
    }
    #[doc = "Bit 1 - Regular channel end of conversion"]
    #[inline(always)]
    pub fn eoc(&mut self) -> EOC_W {
        EOC_W::new(self)
    }
    #[doc = "Bit 0 - Analog watchdog flag"]
    #[inline(always)]
    pub fn awd(&mut self) -> AWD_W {
        AWD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statr](index.html) module"]
pub struct STATR_SPEC;
impl crate::RegisterSpec for STATR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [statr::R](R) reader structure"]
impl crate::Readable for STATR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [statr::W](W) writer structure"]
impl crate::Writable for STATR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATR to value 0"]
impl crate::Resettable for STATR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}