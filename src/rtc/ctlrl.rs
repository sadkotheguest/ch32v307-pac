#[doc = "Register `CTLRL` reader"]
pub struct R(crate::R<CTLRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTLRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTLRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTLRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTLRL` writer"]
pub struct W(crate::W<CTLRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTLRL_SPEC>;
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
impl From<crate::W<CTLRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTLRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SECF` reader - Second Flag"]
pub type SECF_R = crate::BitReader<bool>;
#[doc = "Field `SECF` writer - Second Flag"]
pub type SECF_W<'a> = crate::BitWriter<'a, u32, CTLRL_SPEC, bool, 0>;
#[doc = "Field `ALRF` reader - Alarm Flag"]
pub type ALRF_R = crate::BitReader<bool>;
#[doc = "Field `ALRF` writer - Alarm Flag"]
pub type ALRF_W<'a> = crate::BitWriter<'a, u32, CTLRL_SPEC, bool, 1>;
#[doc = "Field `OWF` reader - Overflow Flag"]
pub type OWF_R = crate::BitReader<bool>;
#[doc = "Field `OWF` writer - Overflow Flag"]
pub type OWF_W<'a> = crate::BitWriter<'a, u32, CTLRL_SPEC, bool, 2>;
#[doc = "Field `RSF` reader - Registers Synchronized Flag"]
pub type RSF_R = crate::BitReader<bool>;
#[doc = "Field `RSF` writer - Registers Synchronized Flag"]
pub type RSF_W<'a> = crate::BitWriter<'a, u32, CTLRL_SPEC, bool, 3>;
#[doc = "Field `CNF` reader - Configuration Flag"]
pub type CNF_R = crate::BitReader<bool>;
#[doc = "Field `CNF` writer - Configuration Flag"]
pub type CNF_W<'a> = crate::BitWriter<'a, u32, CTLRL_SPEC, bool, 4>;
#[doc = "Field `RTOFF` reader - RTC operation OFF"]
pub type RTOFF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Second Flag"]
    #[inline(always)]
    pub fn secf(&self) -> SECF_R {
        SECF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm Flag"]
    #[inline(always)]
    pub fn alrf(&self) -> ALRF_R {
        ALRF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Overflow Flag"]
    #[inline(always)]
    pub fn owf(&self) -> OWF_R {
        OWF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Registers Synchronized Flag"]
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configuration Flag"]
    #[inline(always)]
    pub fn cnf(&self) -> CNF_R {
        CNF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTC operation OFF"]
    #[inline(always)]
    pub fn rtoff(&self) -> RTOFF_R {
        RTOFF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Second Flag"]
    #[inline(always)]
    pub fn secf(&mut self) -> SECF_W {
        SECF_W::new(self)
    }
    #[doc = "Bit 1 - Alarm Flag"]
    #[inline(always)]
    pub fn alrf(&mut self) -> ALRF_W {
        ALRF_W::new(self)
    }
    #[doc = "Bit 2 - Overflow Flag"]
    #[inline(always)]
    pub fn owf(&mut self) -> OWF_W {
        OWF_W::new(self)
    }
    #[doc = "Bit 3 - Registers Synchronized Flag"]
    #[inline(always)]
    pub fn rsf(&mut self) -> RSF_W {
        RSF_W::new(self)
    }
    #[doc = "Bit 4 - Configuration Flag"]
    #[inline(always)]
    pub fn cnf(&mut self) -> CNF_W {
        CNF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Control Register Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctlrl](index.html) module"]
pub struct CTLRL_SPEC;
impl crate::RegisterSpec for CTLRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctlrl::R](R) reader structure"]
impl crate::Readable for CTLRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctlrl::W](W) writer structure"]
impl crate::Writable for CTLRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTLRL to value 0x20"]
impl crate::Resettable for CTLRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}