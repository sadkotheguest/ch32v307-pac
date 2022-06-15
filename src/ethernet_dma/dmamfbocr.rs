#[doc = "Register `DMAMFBOCR` reader"]
pub struct R(crate::R<DMAMFBOCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAMFBOCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAMFBOCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAMFBOCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MFC` reader - Missed frames by the controller"]
pub type MFC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OMFC` reader - Overflow bit for missed frame counter"]
pub type OMFC_R = crate::BitReader<bool>;
#[doc = "Field `MFA` reader - Missed frames by the application"]
pub type MFA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OFOC` reader - Overflow bit for FIFO overflow counter"]
pub type OFOC_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:15 - Missed frames by the controller"]
    #[inline(always)]
    pub fn mfc(&self) -> MFC_R {
        MFC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Overflow bit for missed frame counter"]
    #[inline(always)]
    pub fn omfc(&self) -> OMFC_R {
        OMFC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:27 - Missed frames by the application"]
    #[inline(always)]
    pub fn mfa(&self) -> MFA_R {
        MFA_R::new(((self.bits >> 17) & 0x07ff) as u16)
    }
    #[doc = "Bit 28 - Overflow bit for FIFO overflow counter"]
    #[inline(always)]
    pub fn ofoc(&self) -> OFOC_R {
        OFOC_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "Ethernet DMA missed frame and buffer overflow counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamfbocr](index.html) module"]
pub struct DMAMFBOCR_SPEC;
impl crate::RegisterSpec for DMAMFBOCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmamfbocr::R](R) reader structure"]
impl crate::Readable for DMAMFBOCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMAMFBOCR to value 0"]
impl crate::Resettable for DMAMFBOCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}