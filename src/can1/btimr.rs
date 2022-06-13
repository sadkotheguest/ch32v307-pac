#[doc = "Register `BTIMR` reader"]
pub struct R(crate::R<BTIMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BTIMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BTIMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BTIMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BTIMR` writer"]
pub struct W(crate::W<BTIMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BTIMR_SPEC>;
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
impl From<crate::W<BTIMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BTIMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SILM` reader - SILM"]
pub type SILM_R = crate::BitReader<bool>;
#[doc = "Field `SILM` writer - SILM"]
pub type SILM_W<'a> = crate::BitWriter<'a, u32, BTIMR_SPEC, bool, 31>;
#[doc = "Field `LBKM` reader - LBKM"]
pub type LBKM_R = crate::BitReader<bool>;
#[doc = "Field `LBKM` writer - LBKM"]
pub type LBKM_W<'a> = crate::BitWriter<'a, u32, BTIMR_SPEC, bool, 30>;
#[doc = "Field `SJW` reader - SJW"]
pub type SJW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SJW` writer - SJW"]
pub type SJW_W<'a> = crate::FieldWriter<'a, u32, BTIMR_SPEC, u8, u8, 2, 24>;
#[doc = "Field `TS2` reader - TS2"]
pub type TS2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TS2` writer - TS2"]
pub type TS2_W<'a> = crate::FieldWriter<'a, u32, BTIMR_SPEC, u8, u8, 3, 20>;
#[doc = "Field `TS1` reader - TS1"]
pub type TS1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TS1` writer - TS1"]
pub type TS1_W<'a> = crate::FieldWriter<'a, u32, BTIMR_SPEC, u8, u8, 4, 16>;
#[doc = "Field `BRP` reader - BRP"]
pub type BRP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BRP` writer - BRP"]
pub type BRP_W<'a> = crate::FieldWriter<'a, u32, BTIMR_SPEC, u16, u16, 10, 0>;
impl R {
    #[doc = "Bit 31 - SILM"]
    #[inline(always)]
    pub fn silm(&self) -> SILM_R {
        SILM_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30 - LBKM"]
    #[inline(always)]
    pub fn lbkm(&self) -> LBKM_R {
        LBKM_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bits 24:25 - SJW"]
    #[inline(always)]
    pub fn sjw(&self) -> SJW_R {
        SJW_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 20:22 - TS2"]
    #[inline(always)]
    pub fn ts2(&self) -> TS2_R {
        TS2_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 16:19 - TS1"]
    #[inline(always)]
    pub fn ts1(&self) -> TS1_R {
        TS1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 0:9 - BRP"]
    #[inline(always)]
    pub fn brp(&self) -> BRP_R {
        BRP_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - SILM"]
    #[inline(always)]
    pub fn silm(&mut self) -> SILM_W {
        SILM_W::new(self)
    }
    #[doc = "Bit 30 - LBKM"]
    #[inline(always)]
    pub fn lbkm(&mut self) -> LBKM_W {
        LBKM_W::new(self)
    }
    #[doc = "Bits 24:25 - SJW"]
    #[inline(always)]
    pub fn sjw(&mut self) -> SJW_W {
        SJW_W::new(self)
    }
    #[doc = "Bits 20:22 - TS2"]
    #[inline(always)]
    pub fn ts2(&mut self) -> TS2_W {
        TS2_W::new(self)
    }
    #[doc = "Bits 16:19 - TS1"]
    #[inline(always)]
    pub fn ts1(&mut self) -> TS1_W {
        TS1_W::new(self)
    }
    #[doc = "Bits 0:9 - BRP"]
    #[inline(always)]
    pub fn brp(&mut self) -> BRP_W {
        BRP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BTIMR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [btimr](index.html) module"]
pub struct BTIMR_SPEC;
impl crate::RegisterSpec for BTIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [btimr::R](R) reader structure"]
impl crate::Readable for BTIMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [btimr::W](W) writer structure"]
impl crate::Writable for BTIMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BTIMR to value 0"]
impl crate::Resettable for BTIMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}