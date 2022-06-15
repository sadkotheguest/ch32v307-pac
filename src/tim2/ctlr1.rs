#[doc = "Register `CTLR1` reader"]
pub struct R(crate::R<CTLR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTLR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTLR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTLR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTLR1` writer"]
pub struct W(crate::W<CTLR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTLR1_SPEC>;
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
impl From<crate::W<CTLR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTLR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CKD` reader - Clock division"]
pub type CKD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKD` writer - Clock division"]
pub type CKD_W<'a> = crate::FieldWriter<'a, u32, CTLR1_SPEC, u8, u8, 2, 8>;
#[doc = "Field `ARPE` reader - Auto-reload preload enable"]
pub type ARPE_R = crate::BitReader<bool>;
#[doc = "Field `ARPE` writer - Auto-reload preload enable"]
pub type ARPE_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 7>;
#[doc = "Field `CMS` reader - Center-aligned mode selection"]
pub type CMS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMS` writer - Center-aligned mode selection"]
pub type CMS_W<'a> = crate::FieldWriter<'a, u32, CTLR1_SPEC, u8, u8, 2, 5>;
#[doc = "Field `DIR` reader - Direction"]
pub type DIR_R = crate::BitReader<bool>;
#[doc = "Field `DIR` writer - Direction"]
pub type DIR_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 4>;
#[doc = "Field `OPM` reader - One-pulse mode"]
pub type OPM_R = crate::BitReader<bool>;
#[doc = "Field `OPM` writer - One-pulse mode"]
pub type OPM_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 3>;
#[doc = "Field `URS` reader - Update request source"]
pub type URS_R = crate::BitReader<bool>;
#[doc = "Field `URS` writer - Update request source"]
pub type URS_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 2>;
#[doc = "Field `UDIS` reader - Update disable"]
pub type UDIS_R = crate::BitReader<bool>;
#[doc = "Field `UDIS` writer - Update disable"]
pub type UDIS_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 1>;
#[doc = "Field `CEN` reader - Counter enable"]
pub type CEN_R = crate::BitReader<bool>;
#[doc = "Field `CEN` writer - Counter enable"]
pub type CEN_W<'a> = crate::BitWriter<'a, u32, CTLR1_SPEC, bool, 0>;
impl R {
    #[doc = "Bits 8:9 - Clock division"]
    #[inline(always)]
    pub fn ckd(&self) -> CKD_R {
        CKD_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 7 - Auto-reload preload enable"]
    #[inline(always)]
    pub fn arpe(&self) -> ARPE_R {
        ARPE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Center-aligned mode selection"]
    #[inline(always)]
    pub fn cms(&self) -> CMS_R {
        CMS_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 4 - Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - One-pulse mode"]
    #[inline(always)]
    pub fn opm(&self) -> OPM_R {
        OPM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Update request source"]
    #[inline(always)]
    pub fn urs(&self) -> URS_R {
        URS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Update disable"]
    #[inline(always)]
    pub fn udis(&self) -> UDIS_R {
        UDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Counter enable"]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:9 - Clock division"]
    #[inline(always)]
    pub fn ckd(&mut self) -> CKD_W {
        CKD_W::new(self)
    }
    #[doc = "Bit 7 - Auto-reload preload enable"]
    #[inline(always)]
    pub fn arpe(&mut self) -> ARPE_W {
        ARPE_W::new(self)
    }
    #[doc = "Bits 5:6 - Center-aligned mode selection"]
    #[inline(always)]
    pub fn cms(&mut self) -> CMS_W {
        CMS_W::new(self)
    }
    #[doc = "Bit 4 - Direction"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W::new(self)
    }
    #[doc = "Bit 3 - One-pulse mode"]
    #[inline(always)]
    pub fn opm(&mut self) -> OPM_W {
        OPM_W::new(self)
    }
    #[doc = "Bit 2 - Update request source"]
    #[inline(always)]
    pub fn urs(&mut self) -> URS_W {
        URS_W::new(self)
    }
    #[doc = "Bit 1 - Update disable"]
    #[inline(always)]
    pub fn udis(&mut self) -> UDIS_W {
        UDIS_W::new(self)
    }
    #[doc = "Bit 0 - Counter enable"]
    #[inline(always)]
    pub fn cen(&mut self) -> CEN_W {
        CEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctlr1](index.html) module"]
pub struct CTLR1_SPEC;
impl crate::RegisterSpec for CTLR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctlr1::R](R) reader structure"]
impl crate::Readable for CTLR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctlr1::W](W) writer structure"]
impl crate::Writable for CTLR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTLR1 to value 0"]
impl crate::Resettable for CTLR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}