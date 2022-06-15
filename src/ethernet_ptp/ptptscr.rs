#[doc = "Register `PTPTSCR` reader"]
pub struct R(crate::R<PTPTSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTPTSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTPTSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTPTSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTPTSCR` writer"]
pub struct W(crate::W<PTPTSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTPTSCR_SPEC>;
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
impl From<crate::W<PTPTSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTPTSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSE` reader - Time stamp enable"]
pub type TSE_R = crate::BitReader<bool>;
#[doc = "Field `TSE` writer - Time stamp enable"]
pub type TSE_W<'a> = crate::BitWriter<'a, u32, PTPTSCR_SPEC, bool, 0>;
#[doc = "Field `TSFCU` reader - Time stamp fine or coarse update"]
pub type TSFCU_R = crate::BitReader<bool>;
#[doc = "Field `TSFCU` writer - Time stamp fine or coarse update"]
pub type TSFCU_W<'a> = crate::BitWriter<'a, u32, PTPTSCR_SPEC, bool, 1>;
#[doc = "Field `TSSTI` reader - Time stamp system time initialize"]
pub type TSSTI_R = crate::BitReader<bool>;
#[doc = "Field `TSSTI` writer - Time stamp system time initialize"]
pub type TSSTI_W<'a> = crate::BitWriter<'a, u32, PTPTSCR_SPEC, bool, 2>;
#[doc = "Field `TSSTU` reader - Time stamp system time update"]
pub type TSSTU_R = crate::BitReader<bool>;
#[doc = "Field `TSSTU` writer - Time stamp system time update"]
pub type TSSTU_W<'a> = crate::BitWriter<'a, u32, PTPTSCR_SPEC, bool, 3>;
#[doc = "Field `TSITE` reader - Time stamp interrupt trigger enable"]
pub type TSITE_R = crate::BitReader<bool>;
#[doc = "Field `TSITE` writer - Time stamp interrupt trigger enable"]
pub type TSITE_W<'a> = crate::BitWriter<'a, u32, PTPTSCR_SPEC, bool, 4>;
#[doc = "Field `TSARU` reader - Time stamp addend register update"]
pub type TSARU_R = crate::BitReader<bool>;
#[doc = "Field `TSARU` writer - Time stamp addend register update"]
pub type TSARU_W<'a> = crate::BitWriter<'a, u32, PTPTSCR_SPEC, bool, 5>;
impl R {
    #[doc = "Bit 0 - Time stamp enable"]
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Time stamp fine or coarse update"]
    #[inline(always)]
    pub fn tsfcu(&self) -> TSFCU_R {
        TSFCU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Time stamp system time initialize"]
    #[inline(always)]
    pub fn tssti(&self) -> TSSTI_R {
        TSSTI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Time stamp system time update"]
    #[inline(always)]
    pub fn tsstu(&self) -> TSSTU_R {
        TSSTU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Time stamp interrupt trigger enable"]
    #[inline(always)]
    pub fn tsite(&self) -> TSITE_R {
        TSITE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Time stamp addend register update"]
    #[inline(always)]
    pub fn tsaru(&self) -> TSARU_R {
        TSARU_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Time stamp enable"]
    #[inline(always)]
    pub fn tse(&mut self) -> TSE_W {
        TSE_W::new(self)
    }
    #[doc = "Bit 1 - Time stamp fine or coarse update"]
    #[inline(always)]
    pub fn tsfcu(&mut self) -> TSFCU_W {
        TSFCU_W::new(self)
    }
    #[doc = "Bit 2 - Time stamp system time initialize"]
    #[inline(always)]
    pub fn tssti(&mut self) -> TSSTI_W {
        TSSTI_W::new(self)
    }
    #[doc = "Bit 3 - Time stamp system time update"]
    #[inline(always)]
    pub fn tsstu(&mut self) -> TSSTU_W {
        TSSTU_W::new(self)
    }
    #[doc = "Bit 4 - Time stamp interrupt trigger enable"]
    #[inline(always)]
    pub fn tsite(&mut self) -> TSITE_W {
        TSITE_W::new(self)
    }
    #[doc = "Bit 5 - Time stamp addend register update"]
    #[inline(always)]
    pub fn tsaru(&mut self) -> TSARU_W {
        TSARU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet PTP time stamp control register (ETH_PTPTSCR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptptscr](index.html) module"]
pub struct PTPTSCR_SPEC;
impl crate::RegisterSpec for PTPTSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptptscr::R](R) reader structure"]
impl crate::Readable for PTPTSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptptscr::W](W) writer structure"]
impl crate::Writable for PTPTSCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PTPTSCR to value 0"]
impl crate::Resettable for PTPTSCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}