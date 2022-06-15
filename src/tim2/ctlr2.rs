#[doc = "Register `CTLR2` reader"]
pub struct R(crate::R<CTLR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTLR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTLR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTLR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTLR2` writer"]
pub struct W(crate::W<CTLR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTLR2_SPEC>;
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
impl From<crate::W<CTLR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTLR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TI1S` reader - TI1 selection"]
pub type TI1S_R = crate::BitReader<bool>;
#[doc = "Field `TI1S` writer - TI1 selection"]
pub type TI1S_W<'a> = crate::BitWriter<'a, u32, CTLR2_SPEC, bool, 7>;
#[doc = "Field `MMS` reader - Master mode selection"]
pub type MMS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MMS` writer - Master mode selection"]
pub type MMS_W<'a> = crate::FieldWriter<'a, u32, CTLR2_SPEC, u8, u8, 3, 4>;
#[doc = "Field `CCDS` reader - Capture/compare DMA selection"]
pub type CCDS_R = crate::BitReader<bool>;
#[doc = "Field `CCDS` writer - Capture/compare DMA selection"]
pub type CCDS_W<'a> = crate::BitWriter<'a, u32, CTLR2_SPEC, bool, 3>;
#[doc = "Field `CCUS` reader - Update selection"]
pub type CCUS_R = crate::BitReader<bool>;
#[doc = "Field `CCUS` writer - Update selection"]
pub type CCUS_W<'a> = crate::BitWriter<'a, u32, CTLR2_SPEC, bool, 2>;
#[doc = "Field `CCPC` reader - Compare selection"]
pub type CCPC_R = crate::BitReader<bool>;
#[doc = "Field `CCPC` writer - Compare selection"]
pub type CCPC_W<'a> = crate::BitWriter<'a, u32, CTLR2_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 7 - TI1 selection"]
    #[inline(always)]
    pub fn ti1s(&self) -> TI1S_R {
        TI1S_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Master mode selection"]
    #[inline(always)]
    pub fn mms(&self) -> MMS_R {
        MMS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline(always)]
    pub fn ccds(&self) -> CCDS_R {
        CCDS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Update selection"]
    #[inline(always)]
    pub fn ccus(&self) -> CCUS_R {
        CCUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 0 - Compare selection"]
    #[inline(always)]
    pub fn ccpc(&self) -> CCPC_R {
        CCPC_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - TI1 selection"]
    #[inline(always)]
    pub fn ti1s(&mut self) -> TI1S_W {
        TI1S_W::new(self)
    }
    #[doc = "Bits 4:6 - Master mode selection"]
    #[inline(always)]
    pub fn mms(&mut self) -> MMS_W {
        MMS_W::new(self)
    }
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline(always)]
    pub fn ccds(&mut self) -> CCDS_W {
        CCDS_W::new(self)
    }
    #[doc = "Bit 2 - Update selection"]
    #[inline(always)]
    pub fn ccus(&mut self) -> CCUS_W {
        CCUS_W::new(self)
    }
    #[doc = "Bit 0 - Compare selection"]
    #[inline(always)]
    pub fn ccpc(&mut self) -> CCPC_W {
        CCPC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctlr2](index.html) module"]
pub struct CTLR2_SPEC;
impl crate::RegisterSpec for CTLR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctlr2::R](R) reader structure"]
impl crate::Readable for CTLR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctlr2::W](W) writer structure"]
impl crate::Writable for CTLR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTLR2 to value 0"]
impl crate::Resettable for CTLR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}