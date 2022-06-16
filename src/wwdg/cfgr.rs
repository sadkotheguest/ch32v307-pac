#[doc = "Register `CFGR` reader"]
pub struct R(crate::R<CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR` writer"]
pub struct W(crate::W<CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR_SPEC>;
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
impl From<crate::W<CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `W` reader - 7-bit window value"]
pub type W_R = crate::FieldReader<u8, u8>;
#[doc = "Field `W` writer - 7-bit window value"]
pub type W_W<'a> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 7, 0>;
#[doc = "Field `WDGTB` reader - Timer Base"]
pub type WDGTB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WDGTB` writer - Timer Base"]
pub type WDGTB_W<'a> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 2, 7>;
#[doc = "Field `EWI` reader - Early Wakeup Interrupt"]
pub type EWI_R = crate::BitReader<bool>;
#[doc = "Field `EWI` writer - Early Wakeup Interrupt"]
pub type EWI_W<'a> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, 9>;
impl R {
    #[doc = "Bits 0:6 - 7-bit window value"]
    #[inline(always)]
    pub fn w(&self) -> W_R {
        W_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:8 - Timer Base"]
    #[inline(always)]
    pub fn wdgtb(&self) -> WDGTB_R {
        WDGTB_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - Early Wakeup Interrupt"]
    #[inline(always)]
    pub fn ewi(&self) -> EWI_R {
        EWI_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - 7-bit window value"]
    #[inline(always)]
    pub fn w(&mut self) -> W_W {
        W_W::new(self)
    }
    #[doc = "Bits 7:8 - Timer Base"]
    #[inline(always)]
    pub fn wdgtb(&mut self) -> WDGTB_W {
        WDGTB_W::new(self)
    }
    #[doc = "Bit 9 - Early Wakeup Interrupt"]
    #[inline(always)]
    pub fn ewi(&mut self) -> EWI_W {
        EWI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register (WWDG_CFR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr](index.html) module"]
pub struct CFGR_SPEC;
impl crate::RegisterSpec for CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr::R](R) reader structure"]
impl crate::Readable for CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr::W](W) writer structure"]
impl crate::Writable for CFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGR to value 0x7f"]
impl crate::Resettable for CFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7f
    }
}