#[doc = "Register `FSCFGR` reader"]
pub struct R(crate::R<FSCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSCFGR` writer"]
pub struct W(crate::W<FSCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSCFGR_SPEC>;
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
impl From<crate::W<FSCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FSC0` reader - Filter scale configuration"]
pub type FSC0_R = crate::BitReader<bool>;
#[doc = "Field `FSC0` writer - Filter scale configuration"]
pub type FSC0_W<'a> = crate::BitWriter<'a, u32, FSCFGR_SPEC, bool, 0>;
#[doc = "Field `FSC1` reader - Filter scale configuration"]
pub type FSC1_R = crate::BitReader<bool>;
#[doc = "Field `FSC1` writer - Filter scale configuration"]
pub type FSC1_W<'a> = crate::BitWriter<'a, u32, FSCFGR_SPEC, bool, 1>;
#[doc = "Field `FSC2` reader - Filter scale configuration"]
pub type FSC2_R = crate::BitReader<bool>;
#[doc = "Field `FSC2` writer - Filter scale configuration"]
pub type FSC2_W<'a> = crate::BitWriter<'a, u32, FSCFGR_SPEC, bool, 2>;
#[doc = "Field `FSC3` reader - Filter scale configuration"]
pub type FSC3_R = crate::BitReader<bool>;
#[doc = "Field `FSC3` writer - Filter scale configuration"]
pub type FSC3_W<'a> = crate::BitWriter<'a, u32, FSCFGR_SPEC, bool, 3>;
#[doc = "Field `FSC4` reader - Filter scale configuration"]
pub type FSC4_R = crate::BitReader<bool>;
#[doc = "Field `FSC4` writer - Filter scale configuration"]
pub type FSC4_W<'a> = crate::BitWriter<'a, u32, FSCFGR_SPEC, bool, 4>;
#[doc = "Field `FSC5` reader - Filter scale configuration"]
pub type FSC5_R = crate::BitReader<bool>;
#[doc = "Field `FSC5` writer - Filter scale configuration"]
pub type FSC5_W<'a> = crate::BitWriter<'a, u32, FSCFGR_SPEC, bool, 5>;
#[doc = "Field `FSC6` reader - Filter scale configuration"]
pub type FSC6_R = crate::BitReader<bool>;
#[doc = "Field `FSC6` writer - Filter scale configuration"]
pub type FSC6_W<'a> = crate::BitWriter<'a, u32, FSCFGR_SPEC, bool, 6>;
#[doc = "Field `FSC7` reader - Filter scale configuration"]
pub type FSC7_R = crate::BitReader<bool>;
#[doc = "Field `FSC7` writer - Filter scale configuration"]
pub type FSC7_W<'a> = crate::BitWriter<'a, u32, FSCFGR_SPEC, bool, 7>;
#[doc = "Field `FSC8` reader - Filter scale configuration"]
pub type FSC8_R = crate::BitReader<bool>;
#[doc = "Field `FSC8` writer - Filter scale configuration"]
pub type FSC8_W<'a> = crate::BitWriter<'a, u32, FSCFGR_SPEC, bool, 8>;
#[doc = "Field `FSC9` reader - Filter scale configuration"]
pub type FSC9_R = crate::BitReader<bool>;
#[doc = "Field `FSC9` writer - Filter scale configuration"]
pub type FSC9_W<'a> = crate::BitWriter<'a, u32, FSCFGR_SPEC, bool, 9>;
#[doc = "Field `FSC10` reader - Filter scale configuration"]
pub type FSC10_R = crate::BitReader<bool>;
#[doc = "Field `FSC10` writer - Filter scale configuration"]
pub type FSC10_W<'a> = crate::BitWriter<'a, u32, FSCFGR_SPEC, bool, 10>;
#[doc = "Field `FSC11` reader - Filter scale configuration"]
pub type FSC11_R = crate::BitReader<bool>;
#[doc = "Field `FSC11` writer - Filter scale configuration"]
pub type FSC11_W<'a> = crate::BitWriter<'a, u32, FSCFGR_SPEC, bool, 11>;
#[doc = "Field `FSC12` reader - Filter scale configuration"]
pub type FSC12_R = crate::BitReader<bool>;
#[doc = "Field `FSC12` writer - Filter scale configuration"]
pub type FSC12_W<'a> = crate::BitWriter<'a, u32, FSCFGR_SPEC, bool, 12>;
#[doc = "Field `FSC13` reader - Filter scale configuration"]
pub type FSC13_R = crate::BitReader<bool>;
#[doc = "Field `FSC13` writer - Filter scale configuration"]
pub type FSC13_W<'a> = crate::BitWriter<'a, u32, FSCFGR_SPEC, bool, 13>;
impl R {
    #[doc = "Bit 0 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc0(&self) -> FSC0_R {
        FSC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc1(&self) -> FSC1_R {
        FSC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc2(&self) -> FSC2_R {
        FSC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc3(&self) -> FSC3_R {
        FSC3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc4(&self) -> FSC4_R {
        FSC4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc5(&self) -> FSC5_R {
        FSC5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc6(&self) -> FSC6_R {
        FSC6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc7(&self) -> FSC7_R {
        FSC7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc8(&self) -> FSC8_R {
        FSC8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc9(&self) -> FSC9_R {
        FSC9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc10(&self) -> FSC10_R {
        FSC10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc11(&self) -> FSC11_R {
        FSC11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc12(&self) -> FSC12_R {
        FSC12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc13(&self) -> FSC13_R {
        FSC13_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc0(&mut self) -> FSC0_W {
        FSC0_W::new(self)
    }
    #[doc = "Bit 1 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc1(&mut self) -> FSC1_W {
        FSC1_W::new(self)
    }
    #[doc = "Bit 2 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc2(&mut self) -> FSC2_W {
        FSC2_W::new(self)
    }
    #[doc = "Bit 3 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc3(&mut self) -> FSC3_W {
        FSC3_W::new(self)
    }
    #[doc = "Bit 4 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc4(&mut self) -> FSC4_W {
        FSC4_W::new(self)
    }
    #[doc = "Bit 5 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc5(&mut self) -> FSC5_W {
        FSC5_W::new(self)
    }
    #[doc = "Bit 6 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc6(&mut self) -> FSC6_W {
        FSC6_W::new(self)
    }
    #[doc = "Bit 7 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc7(&mut self) -> FSC7_W {
        FSC7_W::new(self)
    }
    #[doc = "Bit 8 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc8(&mut self) -> FSC8_W {
        FSC8_W::new(self)
    }
    #[doc = "Bit 9 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc9(&mut self) -> FSC9_W {
        FSC9_W::new(self)
    }
    #[doc = "Bit 10 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc10(&mut self) -> FSC10_W {
        FSC10_W::new(self)
    }
    #[doc = "Bit 11 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc11(&mut self) -> FSC11_W {
        FSC11_W::new(self)
    }
    #[doc = "Bit 12 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc12(&mut self) -> FSC12_W {
        FSC12_W::new(self)
    }
    #[doc = "Bit 13 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc13(&mut self) -> FSC13_W {
        FSC13_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FSCFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fscfgr](index.html) module"]
pub struct FSCFGR_SPEC;
impl crate::RegisterSpec for FSCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fscfgr::R](R) reader structure"]
impl crate::Readable for FSCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fscfgr::W](W) writer structure"]
impl crate::Writable for FSCFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FSCFGR to value 0"]
impl crate::Resettable for FSCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}