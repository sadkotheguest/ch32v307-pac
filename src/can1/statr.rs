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
#[doc = "Field `RX` reader - RX"]
pub type RX_R = crate::BitReader<bool>;
#[doc = "Field `SAMP` reader - SAMP"]
pub type SAMP_R = crate::BitReader<bool>;
#[doc = "Field `RXM` reader - RXM"]
pub type RXM_R = crate::BitReader<bool>;
#[doc = "Field `TXM` reader - TXM"]
pub type TXM_R = crate::BitReader<bool>;
#[doc = "Field `SLAKI` reader - SLAKI"]
pub type SLAKI_R = crate::BitReader<bool>;
#[doc = "Field `SLAKI` writer - SLAKI"]
pub type SLAKI_W<'a> = crate::BitWriter<'a, u32, STATR_SPEC, bool, 4>;
#[doc = "Field `WKUI` reader - WKUI"]
pub type WKUI_R = crate::BitReader<bool>;
#[doc = "Field `WKUI` writer - WKUI"]
pub type WKUI_W<'a> = crate::BitWriter<'a, u32, STATR_SPEC, bool, 3>;
#[doc = "Field `ERRI` reader - ERRI"]
pub type ERRI_R = crate::BitReader<bool>;
#[doc = "Field `ERRI` writer - ERRI"]
pub type ERRI_W<'a> = crate::BitWriter<'a, u32, STATR_SPEC, bool, 2>;
#[doc = "Field `SLAK` reader - SLAK"]
pub type SLAK_R = crate::BitReader<bool>;
#[doc = "Field `INAK` reader - INAK"]
pub type INAK_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 11 - RX"]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - SAMP"]
    #[inline(always)]
    pub fn samp(&self) -> SAMP_R {
        SAMP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - RXM"]
    #[inline(always)]
    pub fn rxm(&self) -> RXM_R {
        RXM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - TXM"]
    #[inline(always)]
    pub fn txm(&self) -> TXM_R {
        TXM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 4 - SLAKI"]
    #[inline(always)]
    pub fn slaki(&self) -> SLAKI_R {
        SLAKI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - WKUI"]
    #[inline(always)]
    pub fn wkui(&self) -> WKUI_R {
        WKUI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - ERRI"]
    #[inline(always)]
    pub fn erri(&self) -> ERRI_R {
        ERRI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - SLAK"]
    #[inline(always)]
    pub fn slak(&self) -> SLAK_R {
        SLAK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - INAK"]
    #[inline(always)]
    pub fn inak(&self) -> INAK_R {
        INAK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - SLAKI"]
    #[inline(always)]
    pub fn slaki(&mut self) -> SLAKI_W {
        SLAKI_W::new(self)
    }
    #[doc = "Bit 3 - WKUI"]
    #[inline(always)]
    pub fn wkui(&mut self) -> WKUI_W {
        WKUI_W::new(self)
    }
    #[doc = "Bit 2 - ERRI"]
    #[inline(always)]
    pub fn erri(&mut self) -> ERRI_W {
        ERRI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "STATR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statr](index.html) module"]
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