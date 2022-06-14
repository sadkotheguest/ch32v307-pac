#[doc = "Register `DCTRL` reader"]
pub struct R(crate::R<DCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCTRL` writer"]
pub struct W(crate::W<DCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCTRL_SPEC>;
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
impl From<crate::W<DCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTEN` reader - DTEN"]
pub type DTEN_R = crate::BitReader<bool>;
#[doc = "Field `DTEN` writer - DTEN"]
pub type DTEN_W<'a> = crate::BitWriter<'a, u32, DCTRL_SPEC, bool, 0>;
#[doc = "Field `DTDIR` reader - DTDIR"]
pub type DTDIR_R = crate::BitReader<bool>;
#[doc = "Field `DTDIR` writer - DTDIR"]
pub type DTDIR_W<'a> = crate::BitWriter<'a, u32, DCTRL_SPEC, bool, 1>;
#[doc = "Field `DTMODE` reader - DTMODE"]
pub type DTMODE_R = crate::BitReader<bool>;
#[doc = "Field `DTMODE` writer - DTMODE"]
pub type DTMODE_W<'a> = crate::BitWriter<'a, u32, DCTRL_SPEC, bool, 2>;
#[doc = "Field `DMAEN` reader - DMAEN"]
pub type DMAEN_R = crate::BitReader<bool>;
#[doc = "Field `DMAEN` writer - DMAEN"]
pub type DMAEN_W<'a> = crate::BitWriter<'a, u32, DCTRL_SPEC, bool, 3>;
#[doc = "Field `DBLOCKSIZE` reader - DBLOCKSIZE"]
pub type DBLOCKSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBLOCKSIZE` writer - DBLOCKSIZE"]
pub type DBLOCKSIZE_W<'a> = crate::FieldWriter<'a, u32, DCTRL_SPEC, u8, u8, 4, 4>;
#[doc = "Field `PWSTART` reader - PWSTART"]
pub type PWSTART_R = crate::BitReader<bool>;
#[doc = "Field `PWSTART` writer - PWSTART"]
pub type PWSTART_W<'a> = crate::BitWriter<'a, u32, DCTRL_SPEC, bool, 8>;
#[doc = "Field `PWSTOP` reader - PWSTOP"]
pub type PWSTOP_R = crate::BitReader<bool>;
#[doc = "Field `PWSTOP` writer - PWSTOP"]
pub type PWSTOP_W<'a> = crate::BitWriter<'a, u32, DCTRL_SPEC, bool, 9>;
#[doc = "Field `RWMOD` reader - RWMOD"]
pub type RWMOD_R = crate::BitReader<bool>;
#[doc = "Field `RWMOD` writer - RWMOD"]
pub type RWMOD_W<'a> = crate::BitWriter<'a, u32, DCTRL_SPEC, bool, 10>;
#[doc = "Field `SDIOEN` reader - SDIOEN"]
pub type SDIOEN_R = crate::BitReader<bool>;
#[doc = "Field `SDIOEN` writer - SDIOEN"]
pub type SDIOEN_W<'a> = crate::BitWriter<'a, u32, DCTRL_SPEC, bool, 11>;
impl R {
    #[doc = "Bit 0 - DTEN"]
    #[inline(always)]
    pub fn dten(&self) -> DTEN_R {
        DTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DTDIR"]
    #[inline(always)]
    pub fn dtdir(&self) -> DTDIR_R {
        DTDIR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DTMODE"]
    #[inline(always)]
    pub fn dtmode(&self) -> DTMODE_R {
        DTMODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMAEN"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - DBLOCKSIZE"]
    #[inline(always)]
    pub fn dblocksize(&self) -> DBLOCKSIZE_R {
        DBLOCKSIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - PWSTART"]
    #[inline(always)]
    pub fn pwstart(&self) -> PWSTART_R {
        PWSTART_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PWSTOP"]
    #[inline(always)]
    pub fn pwstop(&self) -> PWSTOP_R {
        PWSTOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RWMOD"]
    #[inline(always)]
    pub fn rwmod(&self) -> RWMOD_R {
        RWMOD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SDIOEN"]
    #[inline(always)]
    pub fn sdioen(&self) -> SDIOEN_R {
        SDIOEN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DTEN"]
    #[inline(always)]
    pub fn dten(&mut self) -> DTEN_W {
        DTEN_W::new(self)
    }
    #[doc = "Bit 1 - DTDIR"]
    #[inline(always)]
    pub fn dtdir(&mut self) -> DTDIR_W {
        DTDIR_W::new(self)
    }
    #[doc = "Bit 2 - DTMODE"]
    #[inline(always)]
    pub fn dtmode(&mut self) -> DTMODE_W {
        DTMODE_W::new(self)
    }
    #[doc = "Bit 3 - DMAEN"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W::new(self)
    }
    #[doc = "Bits 4:7 - DBLOCKSIZE"]
    #[inline(always)]
    pub fn dblocksize(&mut self) -> DBLOCKSIZE_W {
        DBLOCKSIZE_W::new(self)
    }
    #[doc = "Bit 8 - PWSTART"]
    #[inline(always)]
    pub fn pwstart(&mut self) -> PWSTART_W {
        PWSTART_W::new(self)
    }
    #[doc = "Bit 9 - PWSTOP"]
    #[inline(always)]
    pub fn pwstop(&mut self) -> PWSTOP_W {
        PWSTOP_W::new(self)
    }
    #[doc = "Bit 10 - RWMOD"]
    #[inline(always)]
    pub fn rwmod(&mut self) -> RWMOD_W {
        RWMOD_W::new(self)
    }
    #[doc = "Bit 11 - SDIOEN"]
    #[inline(always)]
    pub fn sdioen(&mut self) -> SDIOEN_W {
        SDIOEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDIO data control register (SDIO_DCTRL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dctrl](index.html) module"]
pub struct DCTRL_SPEC;
impl crate::RegisterSpec for DCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dctrl::R](R) reader structure"]
impl crate::Readable for DCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dctrl::W](W) writer structure"]
impl crate::Writable for DCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCTRL to value 0"]
impl crate::Resettable for DCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}