#[doc = "Register `CTLR` reader"]
pub struct R(crate::R<CTLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTLR` writer"]
pub struct W(crate::W<CTLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTLR_SPEC>;
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
impl From<crate::W<CTLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PG` reader - Programming"]
pub type PG_R = crate::BitReader<bool>;
#[doc = "Field `PG` writer - Programming"]
pub type PG_W<'a> = crate::BitWriter<'a, u32, CTLR_SPEC, bool, 0>;
#[doc = "Field `PER` reader - Page Erase"]
pub type PER_R = crate::BitReader<bool>;
#[doc = "Field `PER` writer - Page Erase"]
pub type PER_W<'a> = crate::BitWriter<'a, u32, CTLR_SPEC, bool, 1>;
#[doc = "Field `MER` reader - Mass Erase"]
pub type MER_R = crate::BitReader<bool>;
#[doc = "Field `MER` writer - Mass Erase"]
pub type MER_W<'a> = crate::BitWriter<'a, u32, CTLR_SPEC, bool, 2>;
#[doc = "Field `OBPG` reader - Option byte programming"]
pub type OBPG_R = crate::BitReader<bool>;
#[doc = "Field `OBPG` writer - Option byte programming"]
pub type OBPG_W<'a> = crate::BitWriter<'a, u32, CTLR_SPEC, bool, 4>;
#[doc = "Field `OBER` reader - Option byte erase"]
pub type OBER_R = crate::BitReader<bool>;
#[doc = "Field `OBER` writer - Option byte erase"]
pub type OBER_W<'a> = crate::BitWriter<'a, u32, CTLR_SPEC, bool, 5>;
#[doc = "Field `STRT` reader - Start"]
pub type STRT_R = crate::BitReader<bool>;
#[doc = "Field `STRT` writer - Start"]
pub type STRT_W<'a> = crate::BitWriter<'a, u32, CTLR_SPEC, bool, 6>;
#[doc = "Field `LOCK` reader - Lock"]
pub type LOCK_R = crate::BitReader<bool>;
#[doc = "Field `LOCK` writer - Lock"]
pub type LOCK_W<'a> = crate::BitWriter<'a, u32, CTLR_SPEC, bool, 7>;
#[doc = "Field `OBWRE` reader - Option bytes write enable"]
pub type OBWRE_R = crate::BitReader<bool>;
#[doc = "Field `OBWRE` writer - Option bytes write enable"]
pub type OBWRE_W<'a> = crate::BitWriter<'a, u32, CTLR_SPEC, bool, 9>;
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader<bool>;
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a> = crate::BitWriter<'a, u32, CTLR_SPEC, bool, 10>;
#[doc = "Field `EOPIE` reader - End of operation interrupt enable"]
pub type EOPIE_R = crate::BitReader<bool>;
#[doc = "Field `EOPIE` writer - End of operation interrupt enable"]
pub type EOPIE_W<'a> = crate::BitWriter<'a, u32, CTLR_SPEC, bool, 12>;
#[doc = "Field `FLOCK` reader - Fast programmable lock"]
pub type FLOCK_R = crate::BitReader<bool>;
#[doc = "Field `FLOCK` writer - Fast programmable lock"]
pub type FLOCK_W<'a> = crate::BitWriter<'a, u32, CTLR_SPEC, bool, 15>;
#[doc = "Field `FTPG` reader - Fast programming"]
pub type FTPG_R = crate::BitReader<bool>;
#[doc = "Field `FTPG` writer - Fast programming"]
pub type FTPG_W<'a> = crate::BitWriter<'a, u32, CTLR_SPEC, bool, 16>;
#[doc = "Field `FTER` reader - Fast erase"]
pub type FTER_R = crate::BitReader<bool>;
#[doc = "Field `FTER` writer - Fast erase"]
pub type FTER_W<'a> = crate::BitWriter<'a, u32, CTLR_SPEC, bool, 17>;
#[doc = "Field `BUFLOAD` reader - Buffer load"]
pub type BUFLOAD_R = crate::BitReader<bool>;
#[doc = "Field `BUFLOAD` writer - Buffer load"]
pub type BUFLOAD_W<'a> = crate::BitWriter<'a, u32, CTLR_SPEC, bool, 18>;
#[doc = "Field `BUFRST` reader - Buffer reset"]
pub type BUFRST_R = crate::BitReader<bool>;
#[doc = "Field `BUFRST` writer - Buffer reset"]
pub type BUFRST_W<'a> = crate::BitWriter<'a, u32, CTLR_SPEC, bool, 19>;
impl R {
    #[doc = "Bit 0 - Programming"]
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Page Erase"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mass Erase"]
    #[inline(always)]
    pub fn mer(&self) -> MER_R {
        MER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Option byte programming"]
    #[inline(always)]
    pub fn obpg(&self) -> OBPG_R {
        OBPG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Option byte erase"]
    #[inline(always)]
    pub fn ober(&self) -> OBER_R {
        OBER_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Start"]
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Lock"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Option bytes write enable"]
    #[inline(always)]
    pub fn obwre(&self) -> OBWRE_R {
        OBWRE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - End of operation interrupt enable"]
    #[inline(always)]
    pub fn eopie(&self) -> EOPIE_R {
        EOPIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Fast programmable lock"]
    #[inline(always)]
    pub fn flock(&self) -> FLOCK_R {
        FLOCK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Fast programming"]
    #[inline(always)]
    pub fn ftpg(&self) -> FTPG_R {
        FTPG_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Fast erase"]
    #[inline(always)]
    pub fn fter(&self) -> FTER_R {
        FTER_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Buffer load"]
    #[inline(always)]
    pub fn bufload(&self) -> BUFLOAD_R {
        BUFLOAD_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Buffer reset"]
    #[inline(always)]
    pub fn bufrst(&self) -> BUFRST_R {
        BUFRST_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Programming"]
    #[inline(always)]
    pub fn pg(&mut self) -> PG_W {
        PG_W::new(self)
    }
    #[doc = "Bit 1 - Page Erase"]
    #[inline(always)]
    pub fn per(&mut self) -> PER_W {
        PER_W::new(self)
    }
    #[doc = "Bit 2 - Mass Erase"]
    #[inline(always)]
    pub fn mer(&mut self) -> MER_W {
        MER_W::new(self)
    }
    #[doc = "Bit 4 - Option byte programming"]
    #[inline(always)]
    pub fn obpg(&mut self) -> OBPG_W {
        OBPG_W::new(self)
    }
    #[doc = "Bit 5 - Option byte erase"]
    #[inline(always)]
    pub fn ober(&mut self) -> OBER_W {
        OBER_W::new(self)
    }
    #[doc = "Bit 6 - Start"]
    #[inline(always)]
    pub fn strt(&mut self) -> STRT_W {
        STRT_W::new(self)
    }
    #[doc = "Bit 7 - Lock"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W::new(self)
    }
    #[doc = "Bit 9 - Option bytes write enable"]
    #[inline(always)]
    pub fn obwre(&mut self) -> OBWRE_W {
        OBWRE_W::new(self)
    }
    #[doc = "Bit 10 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W {
        ERRIE_W::new(self)
    }
    #[doc = "Bit 12 - End of operation interrupt enable"]
    #[inline(always)]
    pub fn eopie(&mut self) -> EOPIE_W {
        EOPIE_W::new(self)
    }
    #[doc = "Bit 15 - Fast programmable lock"]
    #[inline(always)]
    pub fn flock(&mut self) -> FLOCK_W {
        FLOCK_W::new(self)
    }
    #[doc = "Bit 16 - Fast programming"]
    #[inline(always)]
    pub fn ftpg(&mut self) -> FTPG_W {
        FTPG_W::new(self)
    }
    #[doc = "Bit 17 - Fast erase"]
    #[inline(always)]
    pub fn fter(&mut self) -> FTER_W {
        FTER_W::new(self)
    }
    #[doc = "Bit 18 - Buffer load"]
    #[inline(always)]
    pub fn bufload(&mut self) -> BUFLOAD_W {
        BUFLOAD_W::new(self)
    }
    #[doc = "Bit 19 - Buffer reset"]
    #[inline(always)]
    pub fn bufrst(&mut self) -> BUFRST_W {
        BUFRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctlr](index.html) module"]
pub struct CTLR_SPEC;
impl crate::RegisterSpec for CTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctlr::R](R) reader structure"]
impl crate::Readable for CTLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctlr::W](W) writer structure"]
impl crate::Writable for CTLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTLR to value 0x80"]
impl crate::Resettable for CTLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}